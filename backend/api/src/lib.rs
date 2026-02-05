use chrono::NaiveDate;
use poem_openapi::{ApiResponse, Object, payload::Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yams_core::{
    domain::{self, Email},
    use_cases,
};

mod address;
mod animal;
pub mod api;
mod client;
pub mod errors;

pub use address::*;
pub use animal::*;
pub use client::*;
