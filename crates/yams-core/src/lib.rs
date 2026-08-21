#![feature(associated_type_defaults)]
#![feature(never_type)]
#![feature(error_generic_member_access)]

pub mod adapters;
pub mod application;
pub mod domain;
pub mod service;

pub use application::{App, ErrorReportExt, ResultReport, ThreadSafeError, ports, uow};
