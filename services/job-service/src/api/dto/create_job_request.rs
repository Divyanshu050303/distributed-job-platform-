use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateJobRequest {
    pub title: String,
    pub description: String,
    pub company_name: String,
    pub employment_type: String,
    pub work_mode: String,
    pub location: String,
    pub experience_min: i32,
    pub experience_max: i32,
    pub salary_min: Option<i64>,
    pub salary_max: Option<i64>,
    #[schema(example = "INR")]
    pub currency: String,
    #[schema(example = 1)]
    pub openings: i32,
    pub expires_at: Option<chrono::NaiveDateTime>,
}
