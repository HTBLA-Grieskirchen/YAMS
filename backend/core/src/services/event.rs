use std::sync::Arc;
use uuid::Uuid;
use crate::models::{Event, NewEvent, Seminar, NewSeminar};
use crate::ports::{EventRepository, SeminarRepository};
use crate::error::Result;

pub struct EventService {
    repo: Arc<dyn EventRepository>,
}

impl EventService {
    pub fn new(repo: Arc<dyn EventRepository>) -> Self {
        Self { repo }
    }

    pub async fn get_all(&self) -> Result<Vec<Event>> {
        self.repo.find_all().await
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Event>> {
        self.repo.find_by_id(id).await
    }

    pub async fn create(&self, event: NewEvent) -> Result<Event> {
        self.repo.create(event).await
    }

    pub async fn update(&self, event: Event) -> Result<Event> {
        self.repo.update(event).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        self.repo.delete(id).await
    }
}

pub struct SeminarService {
    repo: Arc<dyn SeminarRepository>,
}

impl SeminarService {
    pub fn new(repo: Arc<dyn SeminarRepository>) -> Self {
        Self { repo }
    }

    pub async fn get_all(&self) -> Result<Vec<Seminar>> {
        self.repo.find_all().await
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Seminar>> {
        self.repo.find_by_id(id).await
    }

    pub async fn create(&self, seminar: NewSeminar) -> Result<Seminar> {
        self.repo.create(seminar).await
    }

    pub async fn update(&self, seminar: Seminar) -> Result<Seminar> {
        self.repo.update(seminar).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        self.repo.delete(id).await
    }
}
