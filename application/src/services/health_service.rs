//! Health service
//!
//! Provides business logic for health check operations.

use crate::dto::response::HealthResponse;
use crate::error::ApiError;

#[derive(Debug, Clone)]
pub struct HealthService;

impl HealthService {
    /// Create a new health service instance
    pub fn new() -> Self {
        Self
    }

    /// Get health status
    pub async fn get_health(&self) -> Result<HealthResponse, ApiError> {
        // In a real application, you might check database connectivity,
        // external service availability, etc.
        Ok(HealthResponse::healthy())
    }
}

impl Default for HealthService {
    fn default() -> Self {
        Self::new()
    }
}
