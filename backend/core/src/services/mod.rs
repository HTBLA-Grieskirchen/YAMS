pub mod address;
pub mod client;
pub mod animal;
pub mod event;

pub use address::AddressService;
pub use client::ClientService;
pub use animal::{AnimalService, RaceService};
pub use event::{EventService, SeminarService};
