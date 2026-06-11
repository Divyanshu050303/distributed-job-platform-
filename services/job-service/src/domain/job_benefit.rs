use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct JobBenefit {
    pub id: Uuid,

    pub job_id: Uuid,

    pub benefit: String,

    pub created_at: NaiveDateTime,
}
