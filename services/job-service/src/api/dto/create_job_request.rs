use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::enums::employment_type::EmploymentType;
use crate::enums::job_status::JobStatus;
use crate::enums::work_mode::WorkMode;
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateJobRequest {
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
    #[schema(example = "INR")]
    pub currency: String,
    #[schema(example = 1)]
    pub openings: i32,
    pub status: JobStatus,
    pub expires_at: Option<DateTime<Utc>>,
}
