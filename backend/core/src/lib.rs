#![feature(associated_type_defaults)]
#![feature(never_type)]
#![feature(error_generic_member_access)]

pub mod application;
pub mod domain;
pub mod service;

pub use application::App;
pub use service::ports;
pub use service::use_cases;
