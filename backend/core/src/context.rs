use std::sync::Arc;
use crate::ports::repository::*;

pub struct YamsContext {
    pub address_repo: Arc<dyn AddressRepository>,
    pub client_repo: Arc<dyn ClientRepository>,
    pub animal_repo: Arc<dyn AnimalRepository>,
    pub race_repo: Arc<dyn RaceRepository>,
    pub event_repo: Arc<dyn EventRepository>,
    pub seminar_repo: Arc<dyn SeminarRepository>,
}

impl YamsContext {
    pub fn new(
        address_repo: Arc<dyn AddressRepository>,
        client_repo: Arc<dyn ClientRepository>,
        animal_repo: Arc<dyn AnimalRepository>,
        race_repo: Arc<dyn RaceRepository>,
        event_repo: Arc<dyn EventRepository>,
        seminar_repo: Arc<dyn SeminarRepository>,
    ) -> Self {
        Self {
            address_repo,
            client_repo,
            animal_repo,
            race_repo,
            event_repo,
            seminar_repo,
        }
    }
}
