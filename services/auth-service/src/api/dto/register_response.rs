use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterResponse {
    pub id: Uuid,

    pub email: String,

    pub first_name: String,

    pub last_name: String,

    pub is_verified: bool,
}
