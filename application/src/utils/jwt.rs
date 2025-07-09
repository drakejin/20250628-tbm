use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::error::ApiError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // Subject (user ID)
    pub email: String,
    pub username: String,
    pub exp: i64,     // Expiration time
    pub iat: i64,     // Issued at
}

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    expires_in: Duration,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
            expires_in: Duration::hours(24), // 24시간
        }
    }

    /// JWT 토큰 생성
    pub fn generate_token(&self, user_id: Uuid, email: &str, username: &str) -> Result<String, ApiError> {
        let now = Utc::now();
        let exp = now + self.expires_in;

        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            username: username.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| ApiError::Internal(format!("JWT 토큰 생성 실패: {}", e)))
    }

    /// JWT 토큰 검증 및 클레임 추출
    pub fn verify_token(&self, token: &str) -> Result<Claims, ApiError> {
        decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map(|data| data.claims)
            .map_err(|e| ApiError::Unauthorized(format!("유효하지 않은 토큰: {}", e)))
    }

    /// 토큰 만료 시간 (초 단위)
    pub fn expires_in_seconds(&self) -> i64 {
        self.expires_in.num_seconds()
    }
}

impl Default for JwtService {
    fn default() -> Self {
        // 개발 환경용 기본 시크릿 (실제 운영에서는 환경변수로 관리)
        Self::new("your-secret-key-change-this-in-production")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_verify_token() {
        let jwt_service = JwtService::default();
        let user_id = Uuid::new_v4();
        let email = "test@example.com";
        let username = "testuser";

        // 토큰 생성
        let token = jwt_service.generate_token(user_id, email, username).unwrap();
        assert!(!token.is_empty());

        // 토큰 검증
        let claims = jwt_service.verify_token(&token).unwrap();
        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.email, email);
        assert_eq!(claims.username, username);
    }

    #[test]
    fn test_verify_invalid_token() {
        let jwt_service = JwtService::default();
        let invalid_token = "invalid.token.here";

        let result = jwt_service.verify_token(invalid_token);
        assert!(result.is_err());
    }

    #[test]
    fn test_expires_in_seconds() {
        let jwt_service = JwtService::default();
        assert_eq!(jwt_service.expires_in_seconds(), 86400); // 24시간 = 86400초
    }
}
