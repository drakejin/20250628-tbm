use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(email(message = "유효한 이메일 주소를 입력해주세요"))]
    pub email: String,

    #[validate(length(min = 3, max = 50, message = "사용자명은 3-50자 사이여야 합니다"))]
    #[validate(regex(path = "crate::utils::validation::USERNAME_REGEX", message = "사용자명은 영문, 숫자, 언더스코어만 사용 가능합니다"))]
    pub username: String,

    #[validate(length(min = 8, max = 128, message = "비밀번호는 8-128자 사이여야 합니다"))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(email(message = "유효한 이메일 주소를 입력해주세요"))]
    pub email: String,

    #[validate(length(min = 1, message = "비밀번호를 입력해주세요"))]
    pub password: String,
}
