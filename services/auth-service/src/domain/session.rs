use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Session {
    pub id: Uuid,

    pub user_id: Uuid,

    pub device_name: Option<String>,

    pub user_agent: Option<String>,

    pub ip_address: Option<String>,

    pub last_activity_at: NaiveDateTime,

    pub is_active: bool,

    pub expires_at: NaiveDateTime,

    pub created_at: NaiveDateTime,
}
