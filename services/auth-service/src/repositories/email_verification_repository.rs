use crate::domain::email_verification::EmailVerification;
use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;
pub struct EmailVerificationRepository;

impl EmailVerificationRepository {
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        token_hash: String,
    ) -> Result<(), sqlx::Error> {
        let expires_at = Utc::now().naive_utc() + Duration::hours(24);

        sqlx::query(
            r#"
            INSERT INTO email_verifications (
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
    ) -> Result<Option<EmailVerification>, sqlx::Error> {
        sqlx::query_as::<_, EmailVerification>(
            r#"
            SELECT *
            FROM email_verifications
            WHERE token_hash = $1
            "#,
        )
        .bind(token_hash)
        .fetch_optional(pool)
        .await
    }

    pub async fn mark_verified(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE email_verifications
            SET verified_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
