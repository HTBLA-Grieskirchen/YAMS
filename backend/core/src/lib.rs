#![feature(associated_type_defaults)]
#![feature(never_type)]
#![feature(error_generic_member_access)]

pub mod app;
pub mod domain;
pub mod service;

pub use app::App;
pub use service::ports;
pub use service::use_cases;
