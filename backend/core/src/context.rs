use crate::services::*;
use std::sync::Arc;

pub struct YamsContext {
    pub address_service: Arc<AddressService>,
    pub client_service: Arc<ClientService>,
    pub animal_service: Arc<AnimalService>,
    pub race_service: Arc<RaceService>,
    pub event_service: Arc<EventService>,
    pub seminar_service: Arc<SeminarService>,
}

impl YamsContext {
    pub fn new(
        address_service: Arc<AddressService>,
        client_service: Arc<ClientService>,
        animal_service: Arc<AnimalService>,
        race_service: Arc<RaceService>,
        event_service: Arc<EventService>,
        seminar_service: Arc<SeminarService>,
    ) -> Self {
        Self {
            address_service,
            client_service,
            animal_service,
            race_service,
            event_service,
            seminar_service,
        }
    }
}
