//! Health endpoint integration tests

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::Value;
use std::sync::Arc;
use tbm_application::{
    dto::response::HealthResponse,
    handlers::HealthHandler,
    services::HealthService,
};
use tower::ServiceExt;

async fn create_test_app() -> axum::Router {
    let health_service = Arc::new(HealthService::new());
    let health_handler = Arc::new(HealthHandler::new(health_service));

    axum::Router::new()
        .route("/health", axum::routing::get(HealthHandler::health_check))
        .with_state(health_handler)
}

#[tokio::test]
async fn test_health_check_success() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let health_response: HealthResponse = serde_json::from_slice(&body).unwrap();

    assert_eq!(health_response.status, "healthy");
    assert_eq!(health_response.version, "0.1.0");
}

#[tokio::test]
async fn test_health_check_response_format() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    // Check that all required fields are present
    assert!(json.get("status").is_some());
    assert!(json.get("timestamp").is_some());
    assert!(json.get("version").is_some());
    assert!(json.get("environment").is_some());
}
