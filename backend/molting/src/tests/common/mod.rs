//! Shared test doubles and helpers for molting integration tests.

use std::cell::Cell;
use std::sync::{Arc, Mutex};

use crate::{AppliableMigration, DownMigration, MigrationTarget, UpMigration};
use async_trait::async_trait;

/// Test error type for migration failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestError {
    GetVersionFailed,
    ApplyFailed,
    Custom(String),
}

impl std::fmt::Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestError::GetVersionFailed => write!(f, "get_version_failed"),
            TestError::ApplyFailed => write!(f, "apply_failed"),
            TestError::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for TestError {}

/// Fake migration target that stores current version in memory and can inject errors.
pub struct FakeMigrationTarget {
    current_version: Cell<Option<usize>>,
    pub fail_get_version: Cell<Option<TestError>>,
    pub fail_apply: Cell<Option<TestError>>,
    /// If set, fail on the N-th call to apply_migration (1-based).
    pub fail_on_apply_call: Cell<Option<usize>>,
    apply_call_count: Cell<usize>,
}

impl FakeMigrationTarget {
    pub fn new(initial_version: Option<usize>) -> Self {
        Self {
            current_version: Cell::new(initial_version),
            fail_get_version: Cell::new(None),
            fail_apply: Cell::new(None),
            fail_on_apply_call: Cell::new(None),
            apply_call_count: Cell::new(0),
        }
    }

    pub fn current_version(&self) -> Option<usize> {
        self.current_version.get()
    }
}

#[async_trait]
impl MigrationTarget<(), TestError> for FakeMigrationTarget {
    fn get_current_version(&self) -> Result<Option<usize>, TestError> {
        if let Some(ref e) = self.fail_get_version.take() {
            return Err(e.clone());
        }
        Ok(self.current_version.get())
    }

    async fn apply_migration(
        &mut self,
        new_version: Option<usize>,
        implementation: impl AppliableMigration<(), TestError> + Send,
    ) -> Result<(), TestError> {
        let count = self.apply_call_count.get() + 1;
        self.apply_call_count.set(count);
        if self.fail_on_apply_call.get() == Some(count) {
            return Err(self.fail_apply.take().unwrap_or(TestError::ApplyFailed));
        }
        if self.fail_on_apply_call.get().is_none() {
            if let Some(e) = self.fail_apply.take() {
                return Err(e);
            }
        }
        implementation.run(&mut ()).await?;
        self.current_version.set(new_version);
        Ok(())
    }
}

/// Type alias for target used in tests (no T state, just ()).
pub type TestTarget = FakeMigrationTarget;

/// Wrap a fake up migration in `Arc<dyn UpMigration<(), TestError>>` for use with the registry.
pub fn arc_up(m: FakeUpMigration) -> Arc<dyn UpMigration<(), TestError>> {
    Arc::new(m)
}

/// Wrap a fake down migration in `Arc<dyn DownMigration<(), TestError>>` for use with the registry.
pub fn arc_down(m: FakeDownMigration) -> Arc<dyn DownMigration<(), TestError>> {
    Arc::new(m)
}

/// Shared log type for recording (version, direction): true = up, false = down.
pub type AppliedLog = Arc<Mutex<Vec<(usize, bool)>>>;

/// Fake up-only migration that records applied version and direction.
#[derive(Clone)]
pub struct FakeUpMigration {
    pub version: usize,
    pub description: Option<&'static str>,
    pub applied_log: AppliedLog,
}

impl FakeUpMigration {
    pub fn new(version: usize, applied_log: AppliedLog) -> Self {
        Self {
            version,
            description: None,
            applied_log,
        }
    }

    pub fn with_description(
        version: usize,
        description: &'static str,
        applied_log: AppliedLog,
    ) -> Self {
        Self {
            version,
            description: Some(description),
            applied_log,
        }
    }
}

#[async_trait]
impl UpMigration<(), TestError> for FakeUpMigration {
    fn version(&self) -> usize {
        self.version
    }

    fn description(&self) -> Option<&'static str> {
        self.description
    }

    async fn up(&self, _target: &mut ()) -> Result<(), TestError> {
        self.applied_log.lock().unwrap().push((self.version, true));
        Ok(())
    }
}

/// Fake reversible migration that records applied version and direction.
pub struct FakeDownMigration {
    pub version: usize,
    pub description: Option<&'static str>,
    pub applied_log: AppliedLog,
}

impl FakeDownMigration {
    pub fn new(version: usize, applied_log: AppliedLog) -> Self {
        Self {
            version,
            description: None,
            applied_log,
        }
    }

    pub fn with_description(
        version: usize,
        description: &'static str,
        applied_log: AppliedLog,
    ) -> Self {
        Self {
            version,
            description: Some(description),
            applied_log,
        }
    }
}

#[async_trait]
impl UpMigration<(), TestError> for FakeDownMigration {
    fn version(&self) -> usize {
        self.version
    }

    fn description(&self) -> Option<&'static str> {
        self.description
    }

    async fn up(&self, _target: &mut ()) -> Result<(), TestError> {
        self.applied_log.lock().unwrap().push((self.version, true));
        Ok(())
    }
}

#[async_trait]
impl DownMigration<(), TestError> for FakeDownMigration {
    async fn down(&self, _target: &mut ()) -> Result<(), TestError> {
        self.applied_log.lock().unwrap().push((self.version, false));
        Ok(())
    }
}
