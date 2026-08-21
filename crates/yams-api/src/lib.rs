pub mod api;
pub mod schema;


#[cfg(feature = "openapi")]
mod spec;
#[cfg(feature = "openapi")]
pub use spec::*;
pub mod errors;

pub use api::{YamsApi, requests, responses};
