#![feature(associated_type_defaults)]
#![feature(never_type)]
#![feature(error_generic_member_access)]

pub mod application;
pub mod domain;
pub mod service;
pub mod adapters;

pub use application::{App, ResultReport, ThreadSafeError, ports, uow, ErrorReportExt};
