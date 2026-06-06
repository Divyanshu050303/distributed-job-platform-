use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct RefreshToken {
    pub id: Uuid,

    pub user_id: Uuid,

    pub session_id: Uuid,

    pub token_hash: String,

    pub revoked: bool,

    pub revoked_at: Option<NaiveDateTime>,

    pub expires_at: NaiveDateTime,

    pub created_at: NaiveDateTime,
}
