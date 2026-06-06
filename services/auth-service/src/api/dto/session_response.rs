use chrono::NaiveDateTime;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct SessionResponse {
    pub session_id: Uuid,

    pub device_name: Option<String>,

    pub user_agent: Option<String>,

    pub ip_address: Option<String>,

    pub is_active: bool,

    pub last_activity_at: NaiveDateTime,

    pub created_at: NaiveDateTime,
}
