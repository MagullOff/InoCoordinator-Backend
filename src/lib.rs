#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

pub mod api;
pub mod repos;
pub mod schema;
pub mod services;
pub mod types;
pub mod errors;

pub use errors::Errors;
