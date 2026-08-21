use std::sync::{Arc, Mutex};

use bon::Builder;
use yams_core::ports::Clock;

pub mod animal_management;
pub mod client_management;

#[derive(Builder, Default)]
pub struct TestAdapters {
    pub clock: Option<Arc<dyn Clock>>,
}
