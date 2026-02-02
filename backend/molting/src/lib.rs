use std::{ops::Deref, pin::Pin, sync::Arc};

use async_trait::async_trait;

#[cfg(test)]
mod tests;

#[async_trait]
pub trait UpMigration<T: Send + Sync + 'static, E: std::error::Error + 'static>:
    Send + Sync + 'static
{
    /// The version ID of this migration. Must be unique and monotonically increasing.
    fn version(&self) -> usize;

    /// An optional description of this migration.
    fn description(&self) -> Option<&'static str> {
        None
    }

    /// Apply this migration to the target.
    async fn up(&self, target: &mut T) -> Result<(), E>;
}

#[async_trait]
impl<T: Send + Sync + 'static, E: std::error::Error + 'static> UpMigration<T, E>
    for Box<dyn UpMigration<T, E>>
{
    fn version(&self) -> usize {
        self.deref().version()
    }

    fn description(&self) -> Option<&'static str> {
        self.deref().description()
    }

    async fn up(&self, target: &mut T) -> Result<(), E> {
        self.deref().up(target).await
    }
}

#[async_trait]
pub trait DownMigration<T: Send + Sync + 'static, E: std::error::Error + 'static>:
    UpMigration<T, E>
{
    /// Revert this migration from the target.
    async fn down(&self, target: &mut T) -> Result<(), E>;
}

/// Internal trait for running a single migration step (up or down).
/// Public so that external `MigrationTarget` implementations can type the `apply_migration` argument.
pub trait AppliableMigration<T: Send + Sync + 'static, E: std::error::Error + 'static> {
    fn run(&self, target: &mut T) -> impl std::future::Future<Output = Result<(), E>> + Send;
}

struct ApplyMigrationUp<T: Send + Sync + 'static, E: std::error::Error + 'static>(
    Box<dyn UpMigration<T, E>>,
);

impl<T: Send + Sync + 'static, E: std::error::Error + 'static> AppliableMigration<T, E>
    for ApplyMigrationUp<T, E>
{
    async fn run(&self, target: &mut T) -> Result<(), E> {
        self.0.up(target).await
    }
}

struct ApplyMigrationDown<T: Send + Sync + 'static, E: std::error::Error + 'static>(
    Box<dyn DownMigration<T, E>>,
);

impl<T: Send + Sync + 'static, E: std::error::Error + 'static> AppliableMigration<T, E>
    for ApplyMigrationDown<T, E>
{
    async fn run(&self, target: &mut T) -> Result<(), E> {
        self.0.down(target).await
    }
}

#[async_trait]
impl<T: Send + Sync + 'static, E: std::error::Error + 'static> UpMigration<T, E>
    for Arc<dyn UpMigration<T, E>>
{
    fn version(&self) -> usize {
        self.deref().version()
    }
    fn description(&self) -> Option<&'static str> {
        self.deref().description()
    }
    async fn up(&self, target: &mut T) -> Result<(), E> {
        self.deref().up(target).await
    }
}

#[async_trait]
impl<T: Send + Sync + 'static, E: std::error::Error + 'static> UpMigration<T, E>
    for Arc<dyn DownMigration<T, E>>
{
    fn version(&self) -> usize {
        self.deref().version()
    }
    fn description(&self) -> Option<&'static str> {
        self.deref().description()
    }
    async fn up(&self, target: &mut T) -> Result<(), E> {
        self.deref().up(target).await
    }
}

#[async_trait]
impl<T: Send + Sync + 'static, E: std::error::Error + 'static> DownMigration<T, E>
    for Arc<dyn DownMigration<T, E>>
{
    async fn down(&self, target: &mut T) -> Result<(), E> {
        self.deref().down(target).await
    }
}

#[async_trait]
pub trait MigrationTarget<T: Send + Sync + 'static, E: std::error::Error + 'static> {
    /// Retrieve the currently applied version ID (if any).
    /// Returns None if no migrations have been applied.
    fn get_current_version(&self) -> Pin<Box<dyn Future<Output = Result<Option<usize>, E>> + '_>>;

    /// Apply a migration within a transactional context.
    ///
    /// This method handles the complete migration lifecycle:
    /// 1. Begins a transaction or context
    /// 2. Executes the provided migration operation
    /// 3. Records the new version number on success
    /// 4. Commits the transaction
    ///
    /// If any step fails, the transaction should be rolled back automatically.
    async fn apply_migration(
        &mut self,
        new_version: Option<usize>,
        implementation: impl AppliableMigration<T, E> + Send,
    ) -> Result<(), E>;
}

pub struct MigrationRegistry<M: ?Sized> {
    migrations: Vec<Arc<M>>,
}

impl<M: ?Sized> Default for MigrationRegistry<M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M: ?Sized> MigrationRegistry<M> {
    pub fn new() -> Self {
        Self { migrations: vec![] }
    }

    async fn get_versions<T: Send + Sync + 'static, E: std::error::Error + 'static>(
        &self,
        runner: &impl MigrationTarget<T, E>,
        target_version: Option<usize>,
    ) -> Result<(usize, usize), MigrationError<E>>
    where
        M: UpMigration<T, E>,
    {
        let current_ver = runner
            .get_current_version()
            .await
            .map_err(MigrationError::RunnerError)?;

        let latest_ver = self.migrations.last().map(|m| m.version()).unwrap_or(0);
        let target = target_version.unwrap_or(latest_ver);

        Ok((current_ver.unwrap_or(0), target))
    }
}

impl<T: Send + Sync + 'static, E: std::error::Error + 'static>
    MigrationRegistry<dyn UpMigration<T, E>>
{
    pub fn add(&mut self, migration: impl UpMigration<T, E>) -> &mut Self {
        self.add_dyn(Arc::new(migration) as Arc<dyn UpMigration<T, E>>);
        self
    }

    pub fn add_dyn(&mut self, migration: impl Into<Arc<dyn UpMigration<T, E>>>) -> &mut Self {
        self.migrations.push(migration.into());
        // Ensure strictly sorted order to guarantee deterministic chains
        self.migrations.sort_by_key(|m| m.version());
        self
    }

    /// Execute migrations to reach the target version.
    /// If target is None, migrate Up to the latest.
    pub async fn apply(
        &self,
        runner: &mut impl MigrationTarget<T, E>,
        target_version: Option<usize>,
    ) -> Result<(), MigrationError<E>> {
        let (current, target) = self.get_versions(runner, target_version).await?;

        println!("Migration Status: Current: {}, Target: {}", current, target);

        if current == target {
            println!("Context is up to date.");
            return Ok(());
        }

        // 2. Determine Direction and Range
        if target > current {
            apply_up_migrations(self.migrations.iter().cloned(), runner, current, target).await?;
        } else {
            // --- DOWN MIGRATIONS NOT SUPPORTED ---
            return Err(MigrationError::DownMigrationNotSupported);
        }

        Ok(())
    }
}

impl<T: Send + Sync + 'static, E: std::error::Error + 'static, I: Into<Arc<dyn UpMigration<T, E>>>>
    From<Vec<I>> for MigrationRegistry<dyn UpMigration<T, E>>
{
    fn from(migrations: Vec<I>) -> Self {
        let mut registry = Self::new();
        for migration in migrations {
            registry.add(migration.into());
        }
        registry
    }
}

impl<T: Send + Sync + 'static, E: std::error::Error + 'static>
    MigrationRegistry<dyn DownMigration<T, E>>
{
    pub fn add(&mut self, migration: impl DownMigration<T, E> + 'static) -> &mut Self {
        self.add_dyn(Arc::new(migration) as Arc<dyn DownMigration<T, E>>);
        self
    }

    pub fn add_dyn(&mut self, migration: impl Into<Arc<dyn DownMigration<T, E>>>) -> &mut Self {
        self.migrations.push(migration.into());
        // Ensure strictly sorted order to guarantee deterministic chains
        self.migrations.sort_by_key(|m| m.version());
        self
    }

    pub async fn apply(
        &self,
        runner: &mut impl MigrationTarget<T, E>,
        target_version: Option<usize>,
    ) -> Result<(), MigrationError<E>> {
        let (current, target) = self.get_versions(runner, target_version).await?;

        println!("Migration Status: Current: {}, Target: {}", current, target);

        if current == target {
            println!("Context is up to date.");
            return Ok(());
        }

        // 2. Determine Direction and Range
        if target > current {
            // --- UP MIGRATIONS ---
            apply_up_migrations(self.migrations.iter().cloned(), runner, current, target).await?;
        } else {
            apply_down_migrations(self.migrations.iter().cloned(), runner, current, target).await?;
        }

        Ok(())
    }
}

impl<
    T: Send + Sync + 'static,
    E: std::error::Error + 'static,
    I: Into<Arc<dyn DownMigration<T, E>>>,
> From<Vec<I>> for MigrationRegistry<dyn DownMigration<T, E>>
{
    fn from(migrations: Vec<I>) -> Self {
        let mut registry = Self::new();
        for migration in migrations {
            registry.add_dyn(migration.into());
        }
        registry
    }
}

#[derive(thiserror::Error, Debug)]
pub enum MigrationError<E: std::error::Error + 'static> {
    #[error("migration runner error: {0}")]
    RunnerError(#[from] E),
    #[error("migration failed: {id} - {source}")]
    MigrationFailed { id: usize, source: E },
    #[error("migration version mismatch: expected = {expected}; actual = {actual}")]
    VersionMismatch { expected: usize, actual: usize },
    #[error("down migrations are not supported")]
    DownMigrationNotSupported,
}

async fn apply_up_migrations<
    T: Send + Sync + 'static,
    E: std::error::Error + 'static,
    I: UpMigration<T, E> + 'static,
>(
    migrations: impl Iterator<Item = I>,
    runner: &mut impl MigrationTarget<T, E>,
    current_version: usize,
    target_version: usize,
) -> Result<(), MigrationError<E>> {
    // --- UP MIGRATIONS ---
    for migration in migrations {
        let m_version = migration.version();
        // Apply only if the migration is newer than current AND older/eq to target
        if m_version <= current_version || m_version > target_version {
            continue;
        }
        print!("Applying UP migration: {}", m_version);
        if let Some(description) = migration.description() {
            print!(" ({})", description);
        }
        println!();

        runner
            .apply_migration(Some(m_version), ApplyMigrationUp(Box::new(migration)))
            .await
            .map_err(|e| MigrationError::MigrationFailed {
                id: m_version,
                source: e,
            })?;
    }

    Ok(())
}

async fn apply_down_migrations<
    T: Send + Sync + 'static,
    E: std::error::Error + 'static,
    I: DownMigration<T, E> + 'static,
>(
    migrations: impl DoubleEndedIterator<Item = I>,
    runner: &mut impl MigrationTarget<T, E>,
    current_version: usize,
    target_version: usize,
) -> Result<(), MigrationError<E>> {
    // --- DOWN MIGRATIONS ---
    let mut migrations = migrations.rev();
    let mut cur_migration = migrations.next();
    while let Some(migration) = cur_migration.take() {
        let next_migration = migrations.next();
        let m_version = migration.version();
        // Apply only if the migration is newer than current AND older/eq to target
        if m_version > current_version || m_version <= target_version {
            continue;
        }

        print!("Applying DOWN migration: {}", m_version);
        if let Some(description) = migration.description() {
            print!(" ({})", description);
        }
        println!();

        runner
            .apply_migration(
                next_migration.as_ref().map(|m| m.version()),
                ApplyMigrationDown(Box::new(migration)),
            )
            .await
            .map_err(|e| MigrationError::MigrationFailed {
                id: m_version,
                source: e,
            })?;

        cur_migration = next_migration;
    }

    Ok(())
}
