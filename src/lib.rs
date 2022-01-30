pub mod auth;
mod entities;

pub type BoxedError = Box<dyn std::error::Error + Send + Sync>;


