use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::job_response::JobResponse;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct JobListResponse {
    pub items: Vec<JobResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}
