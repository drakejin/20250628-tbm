use regex::Regex;
use std::sync::LazyLock;

/// Username validation regex: alphanumeric characters and underscores only
pub static USERNAME_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[a-zA-Z0-9_]+$").expect("Invalid username regex")
});

/// Email validation helper
pub fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.len() <= 255
}

/// Username validation helper
pub fn is_valid_username(username: &str) -> bool {
    username.len() >= 3 && username.len() <= 50 && USERNAME_REGEX.is_match(username)
}

/// Password validation helper
pub fn is_valid_password(password: &str) -> bool {
    password.len() >= 8 && password.len() <= 128
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_username_validation() {
        assert!(is_valid_username("user123"));
        assert!(is_valid_username("test_user"));
        assert!(is_valid_username("abc"));

        assert!(!is_valid_username("ab")); // too short
        assert!(!is_valid_username("user-name")); // contains dash
        assert!(!is_valid_username("user@name")); // contains @
        assert!(!is_valid_username("")); // empty
    }

    #[test]
    fn test_password_validation() {
        assert!(is_valid_password("password123"));
        assert!(is_valid_password("12345678"));

        assert!(!is_valid_password("1234567")); // too short
        assert!(!is_valid_password("")); // empty
    }

    #[test]
    fn test_email_validation() {
        assert!(is_valid_email("test@example.com"));
        assert!(is_valid_email("user@domain.org"));

        assert!(!is_valid_email("invalid-email"));
        assert!(!is_valid_email(""));
    }
}
