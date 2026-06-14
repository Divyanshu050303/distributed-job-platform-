use serde::Deserialize;
use utoipa::IntoParams;

use crate::enums::{employment_type::EmploymentType, job_status::JobStatus, work_mode::WorkMode};

#[derive(Debug, Deserialize, IntoParams)]
pub struct JobQueryParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,

    pub status: Option<JobStatus>,
    pub employment_type: Option<EmploymentType>,
    pub work_mode: Option<WorkMode>,

    pub location: Option<String>,
    pub search: Option<String>,
}
