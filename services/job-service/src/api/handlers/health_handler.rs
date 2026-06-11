use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
}

#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (
            status = 200,
            description = "Health check successful",
            body = HealthResponse
        )
    )
)]
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        service: "job-service".to_string(),
    })
}
