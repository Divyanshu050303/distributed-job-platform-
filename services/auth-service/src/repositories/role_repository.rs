use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::role::Role;

pub struct RoleRepository;

impl RoleRepository {
    pub async fn find_by_id(pool: &PgPool, role_id: Uuid) -> Result<Option<Role>, sqlx::Error> {
        sqlx::query_as::<_, Role>(
            r#"
            SELECT *
            FROM roles
            WHERE id = $1
            "#,
        )
        .bind(role_id)
        .fetch_optional(pool)
        .await
    }
    pub async fn find_by_name(pool: &PgPool, role_name: &str) -> Result<Option<Role>, sqlx::Error> {
        sqlx::query_as::<_, Role>(
            r#"
            SELECT *
            FROM roles
            WHERE name = $1
            "#,
        )
        .bind(role_name)
        .fetch_optional(pool)
        .await
    }
}
