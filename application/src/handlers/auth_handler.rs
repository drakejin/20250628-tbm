use std::sync::Arc;
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use crate::services::user_service::UserService;
use crate::dto::request::auth_request::{RegisterRequest, LoginRequest};
use crate::dto::response::auth_response::{RegisterResponse, LoginResponse};
use crate::error::ApiError;

pub struct AuthHandler {
    user_service: Arc<UserService>,
}

impl AuthHandler {
    pub fn new(user_service: Arc<UserService>) -> Self {
        Self { user_service }
    }

    /// 회원가입
    #[utoipa::path(
        post,
        path = "/auth/register",
        request_body = RegisterRequest,
        responses(
            (status = 201, description = "회원가입 성공", body = RegisterResponse),
            (status = 400, description = "잘못된 요청 데이터"),
            (status = 409, description = "이미 존재하는 이메일/사용자명"),
            (status = 422, description = "유효성 검사 실패")
        ),
        tag = "Authentication"
    )]
    pub async fn register(
        State(handler): State<Arc<AuthHandler>>,
        Json(request): Json<RegisterRequest>,
    ) -> Result<(StatusCode, Json<RegisterResponse>), ApiError> {
        let response = handler.user_service.register(request).await?;
        Ok((StatusCode::CREATED, Json(response)))
    }

    /// 로그인
    #[utoipa::path(
        post,
        path = "/auth/login",
        request_body = LoginRequest,
        responses(
            (status = 200, description = "로그인 성공", body = LoginResponse),
            (status = 401, description = "잘못된 인증 정보"),
            (status = 422, description = "유효성 검사 실패")
        ),
        tag = "Authentication"
    )]
    pub async fn login(
        State(handler): State<Arc<AuthHandler>>,
        Json(request): Json<LoginRequest>,
    ) -> Result<Json<LoginResponse>, ApiError> {
        let response = handler.user_service.login(request).await?;
        Ok(Json(response))
    }

    /// 로그아웃 (현재는 더미 구현)
    #[utoipa::path(
        post,
        path = "/auth/logout",
        responses(
            (status = 204, description = "로그아웃 성공")
        ),
        tag = "Authentication",
        security(
            ("bearer_auth" = [])
        )
    )]
    pub async fn logout() -> Result<StatusCode, ApiError> {
        // JWT 토큰 무효화 로직이 여기에 들어갈 예정
        // 현재는 단순히 204 상태 코드만 반환
        Ok(StatusCode::NO_CONTENT)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::user_repository::tests::MockUserRepository;
    use crate::dto::response::auth_response::UserInfo;
    use axum::extract::State;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_register_success() {
        let mut mock_repo = MockUserRepository::new();

        // Mock repository 설정
        mock_repo.expect_find_by_email().returning(|_| Ok(None));
        mock_repo.expect_find_by_username().returning(|_| Ok(None));
        mock_repo.expect_create().returning(|new_user| {
            Ok(crate::entities::user::User {
                id: Uuid::new_v4(),
                email: new_user.email,
                username: new_user.username,
                password_hash: new_user.password_hash,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
        });

        let user_service = Arc::new(UserService::new(Arc::new(mock_repo)));
        let handler = Arc::new(AuthHandler::new(user_service));

        let request = RegisterRequest {
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            password: "password123".to_string(),
        };

        let result = AuthHandler::register(State(handler), Json(request)).await;
        assert!(result.is_ok());

        let (status, response) = result.unwrap();
        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(response.email, "test@example.com");
        assert_eq!(response.username, "testuser");
    }

    #[tokio::test]
    async fn test_login_success() {
        let mut mock_repo = MockUserRepository::new();

        let password_hash = bcrypt::hash("password123", bcrypt::DEFAULT_COST).unwrap();
        let user = crate::entities::user::User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            password_hash,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        mock_repo
            .expect_find_by_email()
            .returning(move |_| Ok(Some(user.clone())));

        let user_service = Arc::new(UserService::new(Arc::new(mock_repo)));
        let handler = Arc::new(AuthHandler::new(user_service));

        let request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = AuthHandler::login(State(handler), Json(request)).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.token_type, "Bearer");
        assert_eq!(response.expires_in, 86400);
        assert_eq!(response.user.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_logout() {
        let result = AuthHandler::logout().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), StatusCode::NO_CONTENT);
    }
}
