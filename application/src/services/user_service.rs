use std::sync::Arc;
use bcrypt::{hash, verify, DEFAULT_COST};
use validator::Validate;
use crate::repositories::user_repository::UserRepository;
use crate::dto::request::auth_request::{RegisterRequest, LoginRequest};
use crate::dto::response::auth_response::{RegisterResponse, LoginResponse, UserInfo};
use crate::entities::user::NewUser;
use crate::error::ApiError;
use crate::utils::jwt::JwtService;

pub struct UserService {
    user_repository: Arc<dyn UserRepository>,
    jwt_service: JwtService,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self {
            user_repository,
            jwt_service: JwtService::default(),
        }
    }

    pub fn new_with_jwt(user_repository: Arc<dyn UserRepository>, jwt_service: JwtService) -> Self {
        Self {
            user_repository,
            jwt_service,
        }
    }

    /// 사용자 회원가입
    pub async fn register(&self, request: RegisterRequest) -> Result<RegisterResponse, ApiError> {
        // 입력 데이터 유효성 검사
        request.validate()?;

        // 이메일 중복 확인
        if let Some(_) = self.user_repository.find_by_email(&request.email).await? {
            return Err(ApiError::Conflict("이미 존재하는 이메일입니다".to_string()));
        }

        // 사용자명 중복 확인
        if let Some(_) = self.user_repository.find_by_username(&request.username).await? {
            return Err(ApiError::Conflict("이미 존재하는 사용자명입니다".to_string()));
        }

        // 비밀번호 해싱
        let password_hash = hash(&request.password, DEFAULT_COST)?;

        // 새 사용자 생성
        let new_user = NewUser {
            email: request.email,
            username: request.username,
            password_hash,
        };

        let user = self.user_repository.create(new_user).await?;

        Ok(RegisterResponse::from(user))
    }

    /// 사용자 로그인
    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse, ApiError> {
        // 입력 데이터 유효성 검사
        request.validate()?;

        // 사용자 조회
        let user = self
            .user_repository
            .find_by_email(&request.email)
            .await?
            .ok_or_else(|| ApiError::Unauthorized("잘못된 이메일 또는 비밀번호입니다".to_string()))?;

        // 비밀번호 검증
        if !verify(&request.password, &user.password_hash)? {
            return Err(ApiError::Unauthorized("잘못된 이메일 또는 비밀번호입니다".to_string()));
        }

        // JWT 토큰 생성
        let access_token = self.jwt_service.generate_token(user.id, &user.email, &user.username)?;
        let expires_in = self.jwt_service.expires_in_seconds();

        Ok(LoginResponse {
            access_token,
            token_type: "Bearer".to_string(),
            expires_in,
            user: UserInfo::from(user),
        })
    }

    /// 사용자 ID로 조회
    pub async fn get_user_by_id(&self, id: uuid::Uuid) -> Result<Option<UserInfo>, ApiError> {
        let user = self.user_repository.find_by_id(id).await?;
        Ok(user.map(UserInfo::from))
    }

    /// 이메일로 사용자 조회
    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<UserInfo>, ApiError> {
        let user = self.user_repository.find_by_email(email).await?;
        Ok(user.map(UserInfo::from))
    }

    /// 사용자명으로 사용자 조회
    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<UserInfo>, ApiError> {
        let user = self.user_repository.find_by_username(username).await?;
        Ok(user.map(UserInfo::from))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::user_repository::tests::MockUserRepository;
    use crate::entities::user::User;
    use chrono::Utc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_register_success() {
        let mut mock_repo = MockUserRepository::new();

        // 이메일 중복 확인 - 없음
        mock_repo
            .expect_find_by_email()
            .with(mockall::predicate::eq("test@example.com"))
            .times(1)
            .returning(|_| Ok(None));

        // 사용자명 중복 확인 - 없음
        mock_repo
            .expect_find_by_username()
            .with(mockall::predicate::eq("testuser"))
            .times(1)
            .returning(|_| Ok(None));

        // 사용자 생성
        let user_id = Uuid::new_v4();
        let now = Utc::now();
        mock_repo
            .expect_create()
            .times(1)
            .returning(move |new_user| {
                Ok(User {
                    id: user_id,
                    email: new_user.email,
                    username: new_user.username,
                    password_hash: new_user.password_hash,
                    created_at: now,
                    updated_at: now,
                })
            });

        let service = UserService::new(Arc::new(mock_repo));
        let request = RegisterRequest {
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            password: "password123".to_string(),
        };

        let result = service.register(request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.email, "test@example.com");
        assert_eq!(response.username, "testuser");
    }

    #[tokio::test]
    async fn test_register_duplicate_email() {
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
            .with(mockall::predicate::eq("test@example.com"))
            .times(1)
            .returning(move |_| Ok(Some(existing_user.clone())));

        let service = UserService::new(Arc::new(mock_repo));
        let request = RegisterRequest {
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            password: "password123".to_string(),
        };

        let result = service.register(request).await;
        assert!(result.is_err());

        if let Err(ApiError::Conflict(msg)) = result {
            assert_eq!(msg, "이미 존재하는 이메일입니다");
        } else {
            panic!("Expected Conflict error");
        }
    }

    #[tokio::test]
    async fn test_login_success() {
        let mut mock_repo = MockUserRepository::new();

        let password_hash = hash("password123", DEFAULT_COST).unwrap();
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
            .with(mockall::predicate::eq("test@example.com"))
            .times(1)
            .returning(move |_| Ok(Some(user.clone())));

        let service = UserService::new(Arc::new(mock_repo));
        let request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = service.login(request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.token_type, "Bearer");
        assert_eq!(response.expires_in, 86400);
        assert_eq!(response.user.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_login_invalid_email() {
        let mut mock_repo = MockUserRepository::new();

        mock_repo
            .expect_find_by_email()
            .with(mockall::predicate::eq("nonexistent@example.com"))
            .times(1)
            .returning(|_| Ok(None));

        let service = UserService::new(Arc::new(mock_repo));
        let request = LoginRequest {
            email: "nonexistent@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = service.login(request).await;
        assert!(result.is_err());

        if let Err(ApiError::Unauthorized(msg)) = result {
            assert_eq!(msg, "잘못된 이메일 또는 비밀번호입니다");
        } else {
            panic!("Expected Unauthorized error");
        }
    }
}
