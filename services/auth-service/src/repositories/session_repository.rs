use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::session::Session;

pub struct SessionRepository;

impl SessionRepository {
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        user_agent: Option<String>,
        ip_address: Option<String>,
    ) -> Result<Session, sqlx::Error> {
        let expires_at = Utc::now().naive_utc() + Duration::days(30);

        sqlx::query_as::<_, Session>(
            r#"
            INSERT INTO sessions (
                user_id,
                user_agent,
                ip_address,
                last_activity_at,
                is_active,
                expires_at
            )
            VALUES (
                $1,
                $2,
                $3,
                NOW(),
                true,
                $4
            )
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(user_agent)
        .bind(ip_address)
        .bind(expires_at)
        .fetch_one(pool)
        .await
    }
    pub async fn deactivate_all_by_user(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
        UPDATE sessions
        SET
            is_active = false
        WHERE user_id = $1
        "#,
        )
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
