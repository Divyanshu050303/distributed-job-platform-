use sqlx::PgPool;
use uuid::Uuid;

use crate::{api::dto::user_request::CreateUserParams, domain::user::User};

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT *
            FROM users
            WHERE email = $1
              AND deleted_at IS NULL
            "#,
        )
        .bind(email)
        .fetch_optional(pool)
        .await
    }

    pub async fn create_user(pool: &PgPool, params: CreateUserParams) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (
                email,
                password_hash,
                first_name,
                last_name,
                role_id
            )
            VALUES (
                $1,
                $2,
                $3,
                $4,
                $5
            )
            RETURNING *
            "#,
        )
        .bind(params.email)
        .bind(params.password_hash)
        .bind(params.first_name)
        .bind(params.last_name)
        .bind(params.role_id)
        .fetch_one(pool)
        .await
    }
    pub async fn find_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
        SELECT *
        FROM users
        WHERE id = $1
        "#,
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await
    }
    pub async fn update_password(
        pool: &PgPool,
        user_id: Uuid,
        password_hash: String,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
        UPDATE users
        SET password_hash = $1
        WHERE id = $2
        "#,
        )
        .bind(password_hash)
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(())
    }
    pub async fn verify_email(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
        UPDATE users
        SET is_verified = true
        WHERE id = $1
        "#,
        )
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
