use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: Uuid,

    pub email: String,

    pub password_hash: String,

    pub first_name: Option<String>,

    pub last_name: Option<String>,

    pub role_id: Uuid,

    pub is_active: bool,

    pub is_verified: bool,

    pub failed_login_attempts: i32,

    pub locked_until: Option<NaiveDateTime>,

    pub last_login_at: Option<NaiveDateTime>,

    pub created_at: NaiveDateTime,

    pub updated_at: NaiveDateTime,

    pub deleted_at: Option<NaiveDateTime>,
}
