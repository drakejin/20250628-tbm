//! TBM Application Main Entry Point
//!
//! This is the main entry point for the TBM application server.

use std::sync::Arc;

use axum::{routing::{get, post}, Router};
use sqlx::PgPool;
use tbm_application::{
    config::AppConfig,
    handlers::{HealthHandler, auth_handler::AuthHandler},
    services::{HealthService, user_service::UserService},
    repositories::user_repository::PostgresUserRepository,
    dto::response::HealthResponse,
    dto::request::auth_request::{RegisterRequest, LoginRequest},
    dto::response::auth_response::{RegisterResponse, LoginResponse, UserInfo},
};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, Level};
use tracing_subscriber;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        tbm_application::handlers::health_handler::HealthHandler::health_check,
        tbm_application::handlers::auth_handler::AuthHandler::register,
        tbm_application::handlers::auth_handler::AuthHandler::login,
        tbm_application::handlers::auth_handler::AuthHandler::logout,
    ),
    components(schemas(
        HealthResponse,
        RegisterRequest,
        LoginRequest,
        RegisterResponse,
        LoginResponse,
        UserInfo,
    )),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Authentication", description = "User authentication endpoints")
    ),
    info(
        title = "TBM Application API",
        version = "0.1.0",
        description = "A modern Rust server API following best practices"
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    // Load configuration
    let config = AppConfig::from_env();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(match config.log_level.as_str() {
            "trace" => Level::TRACE,
            "debug" => Level::DEBUG,
            "info" => Level::INFO,
            "warn" => Level::WARN,
            "error" => Level::ERROR,
            _ => Level::INFO,
        })
        .init();

    info!("Starting TBM Application server...");
    info!("Environment: {}", config.environment);
    info!("Server address: {}", config.server_address());

    // Initialize database connection
    info!("Connecting to database...");
    let pool = PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    // Run database migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    info!("Database connected and migrations completed");

    // Initialize repositories
    let user_repository = Arc::new(PostgresUserRepository::new(pool.clone()));

    // Initialize services
    let health_service = Arc::new(HealthService::new());
    let user_service = Arc::new(UserService::new(user_repository));

    // Initialize handlers
    let health_handler = Arc::new(HealthHandler::new(health_service));
    let auth_handler = Arc::new(AuthHandler::new(user_service));

    // Build the application router
    let app = Router::new()
        .route("/health", get(HealthHandler::health_check))
        .with_state(health_handler)
        .route("/api/v1/auth/register", post(AuthHandler::register))
        .route("/api/v1/auth/login", post(AuthHandler::login))
        .route("/api/v1/auth/logout", post(AuthHandler::logout))
        .with_state(auth_handler)
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", ApiDoc::openapi())
        )
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
        );

    // Start the server
    let listener = tokio::net::TcpListener::bind(&config.server_address())
        .await
        .expect("Failed to bind to address");

    info!("Server listening on {}", config.server_address());
    info!("Swagger UI available at http://{}/swagger-ui", config.server_address());

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
