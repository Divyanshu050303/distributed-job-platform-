use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct JobSkill {
    pub id: Uuid,

    pub job_id: Uuid,

    pub skill_name: String,

    pub is_mandatory: bool,

    pub created_at: NaiveDateTime,
}
