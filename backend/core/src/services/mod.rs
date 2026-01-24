pub mod address;
pub mod animal;
pub mod client;
pub mod event;

pub use address::AddressService;
pub use animal::{AnimalService, RaceService};
pub use client::ClientService;
pub use event::{EventService, SeminarService};
