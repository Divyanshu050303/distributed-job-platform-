use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct JobStatusHistory {
    pub id: Uuid,

    pub job_id: Uuid,

    pub old_status: Option<String>,

    pub new_status: String,

    pub changed_by: Uuid,

    pub created_at: NaiveDateTime,
}
