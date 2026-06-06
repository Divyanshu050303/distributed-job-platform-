use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct PasswordResetToken {
    pub id: Uuid,

    pub user_id: Uuid,

    pub token_hash: String,

    pub attempts: i32,

    pub expires_at: NaiveDateTime,

    pub used_at: Option<NaiveDateTime>,

    pub created_at: NaiveDateTime,
}
