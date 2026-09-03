use std::ops::{ControlFlow, Deref, DerefMut};

use async_trait::async_trait;
use error_stack::{FrameKind, Report, ResultExt};
use std::cmp::Ordering;
use std::fmt::Debug;

use crate::application::ThreadSafeError;
use crate::ports::{
    BehandlungRepository, HaustierRepository, KlientRepository, LeistungRepository,
    ProduktRepository, RechnungRepository, RepositoryError, RepositoryResult, SeminarRepository,
    SeminarTerminRepository,
};

/// Active unit of work. `commit` / `rollback` consume it.
///
/// Drop panics if neither `commit` nor `rollback` ran (forgotten conclusion).
/// Does **not** call [`UnitOfWorkImpl::rollback`] — that is async and fallible.
#[must_use = "UnitOfWork must be committed or rolled back"]
pub struct UnitOfWork<'a> {
    implementation: Option<Box<dyn UnitOfWorkImpl + 'a>>,
}

impl<'a> UnitOfWork<'a> {
    pub(crate) fn owned(implementation: Box<dyn UnitOfWorkImpl + 'a>) -> Self {
        Self {
            implementation: Some(implementation),
        }
    }

    pub(crate) fn locked(inner: &'a dyn UnitOfWorkImpl) -> Self {
        Self {
            implementation: Some(Box::new(LockedUnitOfWorkImpl { inner })),
        }
    }

    pub(crate) fn as_impl(&self) -> &dyn UnitOfWorkImpl {
        self.implementation
            .as_deref()
            .expect("UnitOfWork already committed or rolled back")
    }
}

impl UnitOfWork<'_> {
    pub async fn commit(mut self) -> RepositoryResult<()> {
        self.take_impl().commit().await
    }

    pub async fn rollback(mut self) -> RepositoryResult<()> {
        self.take_impl().rollback().await
    }

    /// Commit on `Ok`, rollback on `Err`. Pass a function to translate a `Report<RepositoryError>` from commit into a `Report<C>`.
    #[inline]
    pub async fn finish_with<T, C, F>(
        self,
        result: Result<T, Report<C>>,
        context_from_commit: F,
    ) -> Result<T, Report<C>>
    where
        C: ThreadSafeError,
        F: FnOnce(&Report<RepositoryError>) -> C,
    {
        match result {
            Ok(value) => {
                self.commit().await.map_err(|e| {
                    let new_context = context_from_commit(&e);
                    e.change_context(new_context)
                })?;
                Ok(value)
            }
            Err(mut error) => {
                if let Err(_rollback_error) = self.rollback().await {
                    // Atach the rollback report to the outer error
                    let _ = error.frames_mut(|frame| {
                        let FrameKind::Context(_) = frame.kind() else {
                            return ControlFlow::Continue(());
                        };

                        // Cannot attach right now because report does not support
                        // TODO: Somehow fix this later (upstream feat)

                        ControlFlow::Break(())
                    });
                }
                Err(error)
            }
        }
    }

    /// Commit on `Ok`, rollback on `Err`. Pass a function to translate a `Report<RepositoryError>` from commit into a `Report<C>`.
    #[inline]
    pub fn finish<T, C: ThreadSafeError>(
        self,
        result: Result<T, Report<C>>,
        context: C,
    ) -> impl Future<Output = Result<T, Report<C>>> {
        self.finish_with(result, |_| context)
    }

    fn take_impl(&mut self) -> Box<dyn UnitOfWorkImpl + '_> {
        self.implementation
            .take()
            .expect("UnitOfWork already committed or rolled back")
    }

    pub fn klienten(&self) -> &dyn KlientRepository {
        self.as_impl().klienten()
    }

    pub fn haustiere(&self) -> &dyn HaustierRepository {
        self.as_impl().haustiere()
    }

    pub fn produkte(&self) -> &dyn ProduktRepository {
        self.as_impl().produkte()
    }

    pub fn behandlungen(&self) -> &dyn BehandlungRepository {
        self.as_impl().behandlungen()
    }

    pub fn leistungen(&self) -> &dyn LeistungRepository {
        self.as_impl().leistungen()
    }

    pub fn rechnungen(&self) -> &dyn RechnungRepository {
        self.as_impl().rechnungen()
    }

    pub fn seminare(&self) -> &dyn SeminarRepository {
        self.as_impl().seminare()
    }

    pub fn seminar_termine(&self) -> &dyn SeminarTerminRepository {
        self.as_impl().seminar_termine()
    }
}

impl Drop for UnitOfWork<'_> {
    fn drop(&mut self) {
        if self.implementation.is_some() && !std::thread::panicking() {
            panic!("UnitOfWork dropped without commit or rollback");
        }
    }
}

struct LockedUnitOfWorkImpl<'a> {
    inner: &'a dyn UnitOfWorkImpl,
}

#[async_trait]
impl UnitOfWorkImpl for LockedUnitOfWorkImpl<'_> {
    async fn commit(self: Box<Self>) -> RepositoryResult<()> {
        Ok(())
    }

    async fn rollback(self: Box<Self>) -> RepositoryResult<()> {
        Ok(())
    }

    fn klienten(&self) -> &dyn KlientRepository {
        self.inner.klienten()
    }

    fn haustiere(&self) -> &dyn HaustierRepository {
        self.inner.haustiere()
    }

    fn produkte(&self) -> &dyn ProduktRepository {
        self.inner.produkte()
    }

    fn behandlungen(&self) -> &dyn BehandlungRepository {
        self.inner.behandlungen()
    }

    fn leistungen(&self) -> &dyn LeistungRepository {
        self.inner.leistungen()
    }

    fn rechnungen(&self) -> &dyn RechnungRepository {
        self.inner.rechnungen()
    }

    fn seminare(&self) -> &dyn SeminarRepository {
        self.inner.seminare()
    }

    fn seminar_termine(&self) -> &dyn SeminarTerminRepository {
        self.inner.seminar_termine()
    }
}

/// UoW Provider
#[async_trait]
pub trait UnitOfWorkImpl: Send + Sync {
    async fn commit(self: Box<Self>) -> RepositoryResult<()>;
    async fn rollback(self: Box<Self>) -> RepositoryResult<()>;

    fn klienten(&self) -> &dyn KlientRepository;
    fn haustiere(&self) -> &dyn HaustierRepository;
    fn produkte(&self) -> &dyn ProduktRepository;
    fn behandlungen(&self) -> &dyn BehandlungRepository;
    fn leistungen(&self) -> &dyn LeistungRepository;
    fn rechnungen(&self) -> &dyn RechnungRepository;
    fn seminare(&self) -> &dyn SeminarRepository;
    fn seminar_termine(&self) -> &dyn SeminarTerminRepository;
}

#[async_trait]
pub trait UnitOfWorkProvider: Send + Sync {
    async fn begin(&self) -> RepositoryResult<Box<dyn UnitOfWorkImpl>>;
}

pub struct Versioned<T> {
    version: u64,
    data: T,
}

impl<T> Deref for Versioned<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Versioned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> PartialEq for Versioned<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
    }
}

impl<T> PartialOrd for Versioned<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.version.partial_cmp(&other.version)
    }
}

impl<T> Versioned<T> {
    pub fn new(version: u64, data: T) -> Self {
        Self { version, data }
    }

    pub fn init(data: T) -> Self {
        Self { version: 0, data }
    }

    pub fn v(&self) -> u64 {
        self.version
    }

    pub fn increment(&mut self) -> u64 {
        self.version += 1;
        self.version
    }

    pub fn incremented(mut self) -> Self {
        self.increment();
        self
    }

    pub fn into_data(self) -> T {
        self.data
    }

    pub fn cloned_data(&self) -> T
    where
        T: Clone,
    {
        self.data.clone()
    }
}

impl<T> Clone for Versioned<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            version: self.version,
            data: self.cloned_data(),
        }
    }
}

impl<T> Debug for Versioned<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Versioned")
            .field("version", &self.version)
            .field("data", &self.data)
            .finish()
    }
}
