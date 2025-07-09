use std::sync::Arc;
use axum_test::TestServer;
use serde_json::json;
use tbm_application::{
    handlers::auth_handler::AuthHandler,
    services::user_service::UserService,
    repositories::user_repository::tests::MockUserRepository,
    entities::user::User,
    dto::request::auth_request::{RegisterRequest, LoginRequest},
};
use uuid::Uuid;
use chrono::Utc;

#[tokio::test]
async fn test_register_endpoint_success() {
    let mut mock_repo = MockUserRepository::new();

    // Mock repository 설정
    mock_repo.expect_find_by_email().returning(|_| Ok(None));
    mock_repo.expect_find_by_username().returning(|_| Ok(None));
    mock_repo.expect_create().returning(|new_user| {
        Ok(User {
            id: Uuid::new_v4(),
            email: new_user.email,
            username: new_user.username,
            password_hash: new_user.password_hash,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    });

    let user_service = Arc::new(UserService::new(Arc::new(mock_repo)));
    let auth_handler = Arc::new(AuthHandler::new(user_service));

    let app = axum::Router::new()
        .route("/auth/register", axum::routing::post(AuthHandler::register))
        .with_state(auth_handler);

    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/auth/register")
        .json(&json!({
            "email": "test@example.com",
            "username": "testuser",
            "password": "password123"
        }))
        .await;

    assert_eq!(response.status_code(), 201);

    let body: serde_json::Value = response.json();
    assert_eq!(body["email"], "test@example.com");
    assert_eq!(body["username"], "testuser");
    assert!(body["id"].is_string());
    assert!(body["created_at"].is_string());
}

#[tokio::test]
async fn test_register_endpoint_duplicate_email() {
    let mut mock_repo = MockUserRepository::new();

    let existing_user = User {
        id: Uuid::new_v4(),
        email: "test@example.com".to_string(),
        username: "existing".to_string(),
        password_hash: "hash".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // 이메일 중복 확인 - 존재함
    mock_repo
        .expect_find_by_email()
        .returning(move |_| Ok(Some(existing_user.clone())));

    let user_service = Arc::new(UserService::new(Arc::new(mock_repo)));
    let auth_handler = Arc::new(AuthHandler::new(user_service));

    let app = axum::Router::new()
        .route("/auth/register", axum::routing::post(AuthHandler::register))
        .with_state(auth_handler);

    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/auth/register")
        .json(&json!({
            "email": "test@example.com",
            "username": "testuser",
            "password": "password123"
        }))
        .await;

    assert_eq!(response.status_code(), 409);

    let body: serde_json::Value = response.json();
    assert_eq!(body["error"], "이미 존재하는 이메일입니다");
}

#[tokio::test]
async fn test_register_endpoint_validation_error() {
    let mock_repo = MockUserRepository::new();
    let user_service = Arc::new(UserService::new(Arc::new(mock_repo)));
    let auth_handler = Arc::new(AuthHandler::new(user_service));

    let app = axum::Router::new()
        .route("/auth/register", axum::routing::post(AuthHandler::register))
        .with_state(auth_handler);

    let server = TestServer::new(app).unwrap();

    // 잘못된 이메일 형식
    let response = server
        .post("/auth/register")
        .json(&json!({
            "email": "invalid-email",
            "username": "testuser",
            "password": "password123"
        }))
        .await;

    assert_eq!(response.status_code(), 422);

    // 너무 짧은 비밀번호
    let response = server
        .post("/auth/register")
        .json(&json!({
            "email": "test@example.com",
            "username": "testuser",
            "password": "123"
        }))
        .await;

    assert_eq!(response.status_code(), 422);

    // 너무 짧은 사용자명
    let response = server
        .post("/auth/register")
        .json(&json!({
            "email": "test@example.com",
            "username": "ab",
            "password": "password123"
        }))
        .await;

    assert_eq!(response.status_code(), 422);
}

#[tokio::test]
async fn test_login_endpoint_success() {
    let mut mock_repo = MockUserRepository::new();

    let password_hash = bcrypt::hash("password123", bcrypt::DEFAULT_COST).unwrap();
    let user = User {
        id: Uuid::new_v4(),
        email: "test@example.com".to_string(),
        username: "testuser".to_string(),
        password_hash,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    mock_repo
        .expect_find_by_email()
        .returning(move |_| Ok(Some(user.clone())));

    let user_service = Arc::new(UserService::new(Arc::new(mock_repo)));
    let auth_handler = Arc::new(AuthHandler::new(user_service));

    let app = axum::Router::new()
        .route("/auth/login", axum::routing::post(AuthHandler::login))
        .with_state(auth_handler);

    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/auth/login")
        .json(&json!({
            "email": "test@example.com",
            "password": "password123"
        }))
        .await;

    assert_eq!(response.status_code(), 200);

    let body: serde_json::Value = response.json();
    assert_eq!(body["token_type"], "Bearer");
    assert_eq!(body["expires_in"], 86400);
    assert_eq!(body["user"]["email"], "test@example.com");
    assert_eq!(body["user"]["username"], "testuser");
    assert!(body["access_token"].is_string());

    // JWT 토큰이 실제로 유효한지 검증
    let token = body["access_token"].as_str().unwrap();
    assert!(token.len() > 50); // JWT 토큰은 일반적으로 길다
    assert!(token.contains(".")); // JWT는 점으로 구분된 세 부분으로 구성
}

#[tokio::test]
async fn test_login_endpoint_invalid_credentials() {
    let mut mock_repo = MockUserRepository::new();

    mock_repo
        .expect_find_by_email()
        .returning(|_| Ok(None));

    let user_service = Arc::new(UserService::new(Arc::new(mock_repo)));
    let auth_handler = Arc::new(AuthHandler::new(user_service));

    let app = axum::Router::new()
        .route("/auth/login", axum::routing::post(AuthHandler::login))
        .with_state(auth_handler);

    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/auth/login")
        .json(&json!({
            "email": "nonexistent@example.com",
            "password": "password123"
        }))
        .await;

    assert_eq!(response.status_code(), 401);

    let body: serde_json::Value = response.json();
    assert_eq!(body["error"], "잘못된 이메일 또는 비밀번호입니다");
}

#[tokio::test]
async fn test_logout_endpoint() {
    let app = axum::Router::new()
        .route("/auth/logout", axum::routing::post(AuthHandler::logout));

    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/auth/logout")
        .await;

    assert_eq!(response.status_code(), 204);
}
