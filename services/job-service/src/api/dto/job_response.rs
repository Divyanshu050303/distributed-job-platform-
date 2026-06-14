use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::enums::employment_type::EmploymentType;

use crate::enums::work_mode::WorkMode;

use crate::enums::job_status::JobStatus;
use crate::domain::job::Job;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct JobResponse {
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
}

impl From<Job> for JobResponse {
    fn from(job: Job) -> Self {
        Self {
            id: job.id,
            title: job.title,
            description: job.description,
            company_name: job.company_name,
            employment_type: job.employment_type,
            work_mode: job.work_mode,
            location: job.location,
            experience_min: job.experience_min,
            experience_max: job.experience_max,
            salary_min: job.salary_min,
            salary_max: job.salary_max,
            currency: job.currency,
            openings: job.openings,
            status: job.status,
            created_by: job.created_by,
            published_at: job.published_at,
            expires_at: job.expires_at,
            created_at: job.created_at,
            updated_at: job.updated_at,
        }
    }
}
