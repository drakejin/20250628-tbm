//! Health check response DTO

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    /// Service status
    pub status: String,
    /// Current timestamp
    pub timestamp: DateTime<Utc>,
    /// Service version
    pub version: String,
    /// Environment
    pub environment: String,
}

impl HealthResponse {
    /// Create a new healthy response
    pub fn healthy() -> Self {
        Self {
            status: "healthy".to_string(),
            timestamp: Utc::now(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            environment: std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
        }
    }
}
