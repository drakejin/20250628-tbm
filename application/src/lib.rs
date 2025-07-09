//! TBM Application Library
//!
//! This library provides the core functionality for the TBM application,
//! following modern Rust server API development patterns.

pub mod config;
pub mod dto;
pub mod entities;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod repositories;
pub mod services;
pub mod utils;

pub use error::ApiError;
