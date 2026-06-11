use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateJobResponse {
    pub id: Uuid,
    pub title: String,
    pub company_name: String,
    pub status: String,
    pub created_at: NaiveDateTime,
}
