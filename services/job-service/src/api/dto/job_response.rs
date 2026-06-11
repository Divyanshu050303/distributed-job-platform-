use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct JobResponse {
    pub id: Uuid,
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
    pub currency: String,
    pub openings: i32,
    pub status: String,
    pub created_by: Uuid,
    pub published_at: Option<NaiveDateTime>,
    pub expires_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
