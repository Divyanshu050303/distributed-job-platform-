use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfileResponse {
    pub user_id: Uuid,

    pub email: String,

    pub role: String,
}
