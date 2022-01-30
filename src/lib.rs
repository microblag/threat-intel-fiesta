pub mod auth;
pub mod schema;
mod models;

#[macro_use]
extern crate diesel;

pub type BoxedError = Box<dyn std::error::Error + Send + Sync>;


