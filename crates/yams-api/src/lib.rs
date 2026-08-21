mod address;
mod animal;
pub mod api;
mod client;
#[cfg(feature = "openapi")]
mod spec;
#[cfg(feature = "openapi")]
pub use spec::*;
pub mod errors;

pub use address::*;
pub use animal::*;
pub use api::{YamsApi, requests, responses};
pub use client::*;
