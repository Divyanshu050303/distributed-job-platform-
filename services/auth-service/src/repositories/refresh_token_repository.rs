use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::refresh_token::RefreshToken;

pub struct RefreshTokenRepository;

impl RefreshTokenRepository {
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        session_id: Uuid,
        token_hash: String,
    ) -> Result<RefreshToken, sqlx::Error> {
        let expires_at = Utc::now().naive_utc() + Duration::days(30);

        sqlx::query_as::<_, RefreshToken>(
            r#"
            INSERT INTO refresh_tokens (
                user_id,
                session_id,
                token_hash,
                revoked,
                expires_at
            )
            VALUES (
                $1,
                $2,
                $3,
                false,
                $4
            )
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(session_id)
        .bind(token_hash)
        .bind(expires_at)
        .fetch_one(pool)
        .await
    }
    pub async fn find_by_hash(
        pool: &PgPool,
        token_hash: &str,
    ) -> Result<Option<RefreshToken>, sqlx::Error> {
        sqlx::query_as::<_, RefreshToken>(
            r#"
        SELECT *
        FROM refresh_tokens
        WHERE token_hash = $1
        "#,
        )
        .bind(token_hash)
        .fetch_optional(pool)
        .await
    }
    pub async fn revoke(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
        UPDATE refresh_tokens
        SET
            revoked = true,
            revoked_at = NOW()
        WHERE id = $1
        "#,
        )
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }
    pub async fn revoke_all_by_user(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
        UPDATE refresh_tokens
        SET
            revoked = true,
            revoked_at = NOW()
        WHERE user_id = $1
        "#,
        )
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
