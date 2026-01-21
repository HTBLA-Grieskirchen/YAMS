use std::sync::Arc;
use uuid::Uuid;
use crate::models::{Event, NewEvent, Seminar, NewSeminar};
use crate::context::YamsContext;
use crate::error::Result;

pub struct EventService {
    ctx: Arc<YamsContext>,
}

impl EventService {
    pub fn new(ctx: Arc<YamsContext>) -> Self {
        Self { ctx }
    }

    pub async fn get_all(&self) -> Result<Vec<Event>> {
        self.ctx.event_repo.find_all().await
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Event>> {
        self.ctx.event_repo.find_by_id(id).await
    }

    pub async fn create(&self, event: NewEvent) -> Result<Event> {
        self.ctx.event_repo.create(event).await
    }

    pub async fn update(&self, event: Event) -> Result<Event> {
        self.ctx.event_repo.update(event).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        self.ctx.event_repo.delete(id).await
    }
}

pub struct SeminarService {
    ctx: Arc<YamsContext>,
}

impl SeminarService {
    pub fn new(ctx: Arc<YamsContext>) -> Self {
        Self { ctx }
    }

    pub async fn get_all(&self) -> Result<Vec<Seminar>> {
        self.ctx.seminar_repo.find_all().await
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Seminar>> {
        self.ctx.seminar_repo.find_by_id(id).await
    }

    pub async fn create(&self, seminar: NewSeminar) -> Result<Seminar> {
        self.ctx.seminar_repo.create(seminar).await
    }

    pub async fn update(&self, seminar: Seminar) -> Result<Seminar> {
        self.ctx.seminar_repo.update(seminar).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        self.ctx.seminar_repo.delete(id).await
    }
}
