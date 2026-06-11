use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct JobBookmark {
    pub id: Uuid,

    pub user_id: Uuid,

    pub job_id: Uuid,

    pub created_at: NaiveDateTime,
}
