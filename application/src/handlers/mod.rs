//! Handlers module
//!
//! Contains HTTP handlers (controllers) for API endpoints.

pub mod auth_handler;
pub mod health_handler;

pub use health_handler::HealthHandler;
