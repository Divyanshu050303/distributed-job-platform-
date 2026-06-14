use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::api::response::api_response::ApiResponse;
use crate::app_state::AppState;
use crate::application::job_service::JobService;
use crate::errors::app_error::AppError;
use axum::extract::{Path, State};
use uuid::Uuid;

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
#[utoipa::path(
    delete,
    path = "/api/v1/jobs/{id}",
    tag = "Jobs",
    params(
        ("id" = Uuid, Path, description = "Job ID")
    ),
    responses(
        (status = 200, description = "Job deleted successfully"),
        (status = 404, description = "Job not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_job(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    JobService::delete_job(&state.db, id).await?;

    Ok(Json(ApiResponse::success("Job deleted successfully", ())))
}
