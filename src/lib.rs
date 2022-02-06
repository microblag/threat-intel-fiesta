pub mod datastores;
mod entities;
pub mod routes;

pub type BoxedError = Box<dyn std::error::Error + Send + Sync>;


