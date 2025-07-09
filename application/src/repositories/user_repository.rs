use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::entities::user::{User, NewUser};
use crate::error::ApiError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: NewUser) -> Result<User, ApiError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, ApiError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, ApiError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, ApiError>;
    async fn update(&self, id: Uuid, user: NewUser) -> Result<User, ApiError>;
    async fn delete(&self, id: Uuid) -> Result<(), ApiError>;
}

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: NewUser) -> Result<User, ApiError> {
        let id = Uuid::new_v4();
        let now = chrono::Utc::now();

        let created_user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, email, username, password_hash, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, email, username, password_hash, created_at, updated_at
            "#,
            id,
            user.email,
            user.username,
            user.password_hash,
            now,
            now
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created_user)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, ApiError> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, username, password_hash, created_at, updated_at FROM users WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, ApiError> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, username, password_hash, created_at, updated_at FROM users WHERE email = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, ApiError> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, username, password_hash, created_at, updated_at FROM users WHERE username = $1",
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn update(&self, id: Uuid, user: NewUser) -> Result<User, ApiError> {
        let now = chrono::Utc::now();

        let updated_user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET email = $2, username = $3, password_hash = $4, updated_at = $5
            WHERE id = $1
            RETURNING id, email, username, password_hash, created_at, updated_at
            "#,
            id,
            user.email,
            user.username,
            user.password_hash,
            now
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated_user)
    }

    async fn delete(&self, id: Uuid) -> Result<(), ApiError> {
        sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;

    mock! {
        pub UserRepository {}

        #[async_trait]
        impl UserRepository for UserRepository {
            async fn create(&self, user: NewUser) -> Result<User, ApiError>;
            async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, ApiError>;
            async fn find_by_email(&self, email: &str) -> Result<Option<User>, ApiError>;
            async fn find_by_username(&self, username: &str) -> Result<Option<User>, ApiError>;
            async fn update(&self, id: Uuid, user: NewUser) -> Result<User, ApiError>;
            async fn delete(&self, id: Uuid) -> Result<(), ApiError>;
        }
    }

    pub use MockUserRepository;
}
