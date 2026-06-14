use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::enums::{employment_type::EmploymentType, work_mode::WorkMode};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateJobRequest {
    pub title: String,
    pub description: String,
    pub company_name: String,
    pub employment_type: EmploymentType,
    pub work_mode: WorkMode,
    pub location: String,
    pub experience_min: i32,
    pub experience_max: i32,
    pub salary_min: i64,
    pub salary_max: i64,
    pub currency: String,
    pub openings: i32,
    pub expires_at: NaiveDateTime,
}
