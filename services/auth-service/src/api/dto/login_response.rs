use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub access_token: String,

    pub refresh_token: String,

    pub token_type: String,

    pub expires_in: i64,
}
