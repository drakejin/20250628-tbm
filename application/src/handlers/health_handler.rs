//! Health check handler
//!
//! Provides HTTP handlers for health check endpoints.

use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::Json};

use crate::dto::response::HealthResponse;
use crate::error::ApiError;
use crate::services::HealthService;

#[derive(Debug, Clone)]
pub struct HealthHandler {
    health_service: Arc<HealthService>,
}

impl HealthHandler {
    /// Create a new health handler instance
    pub fn new(health_service: Arc<HealthService>) -> Self {
        Self { health_service }
    }

    /// Health check endpoint
    #[utoipa::path(
        get,
        path = "/health",
        responses(
            (status = 200, description = "Service is healthy", body = HealthResponse)
        ),
        tag = "Health"
    )]
    pub async fn health_check(
        State(handler): State<Arc<HealthHandler>>,
    ) -> Result<(StatusCode, Json<HealthResponse>), ApiError> {
        let health = handler.health_service.get_health().await?;
        Ok((StatusCode::OK, Json(health)))
    }
}
