pub mod address;
pub mod animal;
pub mod client;
pub mod event;

pub use address::{Address, NewAddress};
pub use animal::{Animal, NewAnimal, NewRace, Race};
pub use client::{Client, NewClient};
pub use event::{Event, NewEvent, NewSeminar, Seminar};
