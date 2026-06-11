use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    api::{
        dto::{
            create_job_request::CreateJobRequest, create_job_response::CreateJobResponse,
            job_response::JobResponse,
        },
        response::api_response::ApiResponse,
    },
    app_state::AppState,
    application::job_service::JobService,
    auth::current_user::CurrentUser,
    errors::app_error::AppError,
};

#[utoipa::path(
    post,
    path = "/api/v1/jobs",
    tag = "jobs",
    request_body = CreateJobRequest,
    responses(
        (
            status = 201,
            description = "Job created successfully",
            body = CreateJobResponse
        )
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_job(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(request): Json<CreateJobRequest>,
) -> Result<Json<ApiResponse<CreateJobResponse>>, AppError> {
    let response = JobService::create_job(&state.db, request, current_user.user_id).await?;
    Ok(Json(ApiResponse::success(
        "Job created successfully",
        response,
    )))
}

#[utoipa::path(
    get,
    path = "/api/v1/jobs/{id}",
    tag = "jobs",
    responses(
        (
            status = 200,
            description = "Job retrieved successfully",
            body = JobResponse
        )
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_job(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<JobResponse>>, AppError> {
    let response = JobService::get_job(&state.db, id).await?;
    Ok(Json(ApiResponse::success(
        "Job Fetched Successfully",
        response,
    )))
}
