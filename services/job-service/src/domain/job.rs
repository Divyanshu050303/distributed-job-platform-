use chrono::NaiveDateTime;
use sqlx::FromRow;

use crate::enums::employment_type::EmploymentType;
use crate::enums::job_status::JobStatus;
use crate::enums::work_mode::WorkMode;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Job {
    pub id: Uuid,

    pub title: String,
    pub description: String,

    pub company_name: String,

    pub employment_type: EmploymentType,
    pub work_mode: WorkMode,

    pub location: String,

    pub experience_min: i32,
    pub experience_max: i32,

    pub salary_min: Option<i64>,
    pub salary_max: Option<i64>,
    pub currency: String,

    pub openings: i32,

    pub status: JobStatus,

    pub created_by: Uuid,

    pub published_at: Option<NaiveDateTime>,
    pub expires_at: Option<NaiveDateTime>,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    pub deleted_at: Option<NaiveDateTime>,
}
