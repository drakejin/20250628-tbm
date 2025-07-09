use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use crate::utils::jwt::{JwtService, Claims};
use crate::error::ApiError;

/// JWT 인증 미들웨어에서 사용할 사용자 정보
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: uuid::Uuid,
    pub email: String,
    pub username: String,
}

impl From<Claims> for AuthUser {
    fn from(claims: Claims) -> Self {
        Self {
            id: uuid::Uuid::parse_str(&claims.sub).unwrap_or_default(),
            email: claims.email,
            username: claims.username,
        }
    }
}

/// JWT 토큰 인증 미들웨어
pub async fn auth_middleware(
    State(jwt_service): State<Arc<JwtService>>,
    mut request: Request,
    next: Next,
) -> Result<Response, ApiError> {
    // Authorization 헤더에서 토큰 추출
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| ApiError::Unauthorized("Authorization 헤더가 필요합니다".to_string()))?;

    // Bearer 토큰 형식 확인
    if !auth_header.starts_with("Bearer ") {
        return Err(ApiError::Unauthorized("Bearer 토큰 형식이 아닙니다".to_string()));
    }

    // 토큰 추출
    let token = auth_header.trim_start_matches("Bearer ");

    // 토큰 검증
    let claims = jwt_service.verify_token(token)?;

    // 사용자 정보를 request extensions에 저장
    let auth_user = AuthUser::from(claims);
    request.extensions_mut().insert(auth_user);

    Ok(next.run(request).await)
}

/// 선택적 JWT 인증 미들웨어 (토큰이 없어도 통과)
pub async fn optional_auth_middleware(
    State(jwt_service): State<Arc<JwtService>>,
    mut request: Request,
    next: Next,
) -> Response {
    // Authorization 헤더에서 토큰 추출 시도
    if let Some(auth_header) = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
    {
        // Bearer 토큰 형식 확인
        if auth_header.starts_with("Bearer ") {
            let token = auth_header.trim_start_matches("Bearer ");

            // 토큰 검증 시도 (실패해도 계속 진행)
            if let Ok(claims) = jwt_service.verify_token(token) {
                let auth_user = AuthUser::from(claims);
                request.extensions_mut().insert(auth_user);
            }
        }
    }

    next.run(request).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        middleware,
        response::Response,
        routing::get,
        Router,
    };
    use tower::ServiceExt;
    use crate::utils::jwt::JwtService;
    use uuid::Uuid;

    async fn test_handler() -> &'static str {
        "success"
    }

    async fn auth_test_handler(
        request: Request<Body>,
    ) -> Result<&'static str, StatusCode> {
        // request extensions에서 AuthUser 확인
        if request.extensions().get::<AuthUser>().is_some() {
            Ok("authenticated")
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }

    #[tokio::test]
    async fn test_auth_middleware_with_valid_token() {
        let jwt_service = Arc::new(JwtService::default());
        let user_id = Uuid::new_v4();
        let token = jwt_service.generate_token(user_id, "test@example.com", "testuser").unwrap();

        let app = Router::new()
            .route("/protected", get(test_handler))
            .layer(middleware::from_fn_with_state(
                jwt_service.clone(),
                auth_middleware,
            ))
            .with_state(jwt_service);

        let request = Request::builder()
            .uri("/protected")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_auth_middleware_without_token() {
        let jwt_service = Arc::new(JwtService::default());

        let app = Router::new()
            .route("/protected", get(test_handler))
            .layer(middleware::from_fn_with_state(
                jwt_service.clone(),
                auth_middleware,
            ))
            .with_state(jwt_service);

        let request = Request::builder()
            .uri("/protected")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_auth_middleware_with_invalid_token() {
        let jwt_service = Arc::new(JwtService::default());

        let app = Router::new()
            .route("/protected", get(test_handler))
            .layer(middleware::from_fn_with_state(
                jwt_service.clone(),
                auth_middleware,
            ))
            .with_state(jwt_service);

        let request = Request::builder()
            .uri("/protected")
            .header(AUTHORIZATION, "Bearer invalid_token")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_optional_auth_middleware_without_token() {
        let jwt_service = Arc::new(JwtService::default());

        let app = Router::new()
            .route("/optional", get(auth_test_handler))
            .layer(middleware::from_fn_with_state(
                jwt_service.clone(),
                optional_auth_middleware,
            ))
            .with_state(jwt_service);

        let request = Request::builder()
            .uri("/optional")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        // 토큰이 없어도 통과하지만, AuthUser가 없으므로 handler에서 UNAUTHORIZED 반환
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_optional_auth_middleware_with_valid_token() {
        let jwt_service = Arc::new(JwtService::default());
        let user_id = Uuid::new_v4();
        let token = jwt_service.generate_token(user_id, "test@example.com", "testuser").unwrap();

        let app = Router::new()
            .route("/optional", get(auth_test_handler))
            .layer(middleware::from_fn_with_state(
                jwt_service.clone(),
                optional_auth_middleware,
            ))
            .with_state(jwt_service);

        let request = Request::builder()
            .uri("/optional")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
