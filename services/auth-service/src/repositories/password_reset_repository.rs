use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::password_reset_token::PasswordResetToken;

pub struct PasswordResetRepository;

impl PasswordResetRepository {
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        token_hash: String,
    ) -> Result<(), sqlx::Error> {
        let expires_at = Utc::now().naive_utc() + Duration::minutes(30);

        sqlx::query(
            r#"
            INSERT INTO password_reset_tokens (
                user_id,
                token_hash,
                attempts,
                expires_at
            )
            VALUES ($1,$2,0,$3)
            "#,
        )
        .bind(user_id)
        .bind(token_hash)
        .bind(expires_at)
        .execute(pool)
        .await?;

        Ok(())
    }
    pub async fn find_by_hash(
        pool: &PgPool,
        token_hash: &str,
    ) -> Result<Option<PasswordResetToken>, sqlx::Error> {
        sqlx::query_as::<_, PasswordResetToken>(
            r#"
        SELECT *
        FROM password_reset_tokens
        WHERE token_hash = $1
        "#,
        )
        .bind(token_hash)
        .fetch_optional(pool)
        .await
    }
    pub async fn mark_used(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
        UPDATE password_reset_tokens
        SET used_at = NOW()
        WHERE id = $1
        "#,
        )
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
