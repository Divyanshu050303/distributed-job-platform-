use axum::{
    Json,
    extract::{Path, Query, State},
};
use uuid::Uuid;

use crate::{
    api::{
        dto::{
            create_job_request::CreateJobRequest, create_job_response::CreateJobResponse,
            job_list_response::JobListResponse, job_query_params::JobQueryParams,
            job_response::JobResponse,
            update_job_request::UpdateJobRequest,
        },
        response::api_response::ApiResponse,
    },
    app_state::AppState,
    application::job_service::JobService,
    auth::current_user::CurrentUser,
    enums::{
        employment_type::EmploymentType, job_status::JobStatus, work_mode::WorkMode,
    },
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
    tag = "Jobs",
    params(
        ("id" = Uuid, Path, description = "Job ID")
    ),
    responses(
        (
            status = 200,
            description = "Job fetched successfully",
            body = JobResponse
        )
       
    )
    
)]
pub async fn get_job(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<JobResponse>>, AppError> {
    println!("ID: {:?}", id);
    let response = JobService::get_job(&state.db, id).await.map_err(|e| {
        println!("Error: {:?}", e);
        AppError::InternalServerError
    })?;
    println!("Response: {:?}", response);
    Ok(Json(ApiResponse::success(
        "Job Fetched Successfully",
        response,
    )))
}
#[utoipa::path(
    get,
    path = "/api/v1/jobs",
    tag = "Jobs",
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Number of items per page"),
        ("status" = Option<JobStatus>, Query, description = "Job status"),
        ("employment_type" = Option<EmploymentType>, Query, description = "Employment type"),
        ("work_mode" = Option<WorkMode>, Query, description = "Work mode"),
        ("location" = Option<String>, Query, description = "Location"),
        ("search" = Option<String>, Query, description = "Search query")
    ),
    responses(
        (
            status = 200,
            description = "Jobs fetched successfully",
            body = JobListResponse
        )
    )
)]
pub async fn get_jobs(
    State(state): State<AppState>,
    Query(params): Query<JobQueryParams>,
) -> Result<Json<ApiResponse<JobListResponse>>, AppError> {
    let response = JobService::get_Jobs(&state.db, params).await?;
    Ok(Json(ApiResponse::success("Jobs fetched successfully", response)))
}
#[utoipa::path(
    put,
    path = "/api/v1/jobs/{id}",
    tag = "Jobs",
    request_body = UpdateJobRequest,
    params(
        ("id" = Uuid, Path, description = "Job ID")
    ),
    responses(
        (status = 200, description = "Job updated successfully", body = JobResponse),
        (status = 404, description = "Job not found")
    )
)]
pub async fn update_job(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateJobRequest>,
) -> Result<Json<ApiResponse<JobResponse>>, AppError> {
    let response = JobService::update_job(
        &state.db,
        id,
        request,
    )
    .await?;
    Ok(Json(ApiResponse::success("Job updated successfully", response)))
}