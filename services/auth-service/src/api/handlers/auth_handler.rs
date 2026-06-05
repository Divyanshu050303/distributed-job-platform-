use axum::{Json, extract::State};

use crate::{
    api::dto::{register_request::RegisterRequest, register_response::RegisterResponse},
    app_state::AppState,
    application::auth_service::AuthService,
    errors::app_error::AppError,
};

#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    tag = "Authentication",
    request_body = RegisterRequest,
    responses(
        (
            status = 201,
            description = "User registered successfully",
            body = RegisterResponse
        )
    )
)]
pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, AppError> {
    let response = AuthService::register(&state.db, request).await?;

    Ok(Json(response))
}
