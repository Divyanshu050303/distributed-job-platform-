use crate::api::response::api_response::ApiResponse;
use crate::{
    api::dto::{
        login_request::LoginRequest, login_response::LoginResponse,
        logout_all_request::LogoutAllRequest, logout_request::LogoutRequest,
        profile_response::ProfileResponse, refresh_token_request::RefreshTokenRequest,
        refresh_token_response::RefreshTokenResponse, register_request::RegisterRequest,
        register_response::RegisterResponse, session_response::SessionResponse,
    },
    app_state::AppState,
    application::auth_service::AuthService,
    auth::current_user::CurrentUser,
    errors::app_error::AppError,
};
use axum::extract::Path;
use axum::{Json, extract::State};
use uuid::Uuid;

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
) -> Result<Json<ApiResponse<RegisterResponse>>, AppError> {
    let response = AuthService::register(&state.db, request).await?;

    Ok(Json(ApiResponse::success(
        "User registered successfully",
        response,
    )))
}
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "Authentication",
    request_body = LoginRequest,
    responses(
        (
            status = 200,
            body = LoginResponse
        )
    )
)]
pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, AppError> {
    let response = AuthService::login(&state.db, &state.config, request).await?;

    Ok(Json(ApiResponse::success("Login successful", response)))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh-token",
    tag = "Authentication",
    request_body = RefreshTokenRequest,
    responses(
        (
            status = 200,
            body = RefreshTokenResponse
        )
    )
)]
pub async fn refresh_token(
    State(state): State<AppState>,
    Json(request): Json<RefreshTokenRequest>,
) -> Result<Json<ApiResponse<RefreshTokenResponse>>, AppError> {
    let response = AuthService::refresh_token(&state.db, &state.config, request).await?;

    Ok(Json(ApiResponse::success(
        "Refresh token successful",
        response,
    )))
}
#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    tag = "Authentication",
    request_body = LogoutRequest,
    responses(
        (
            status = 200,
            description = "Logged out successfully"
        )
    ),
      security(
        ("bearer_auth" = [])
    )
)]
pub async fn logout(
    State(state): State<AppState>,
    Json(request): Json<LogoutRequest>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    AuthService::logout(&state.db, request).await?;

    Ok(Json(ApiResponse::empty("Logged out successfully")))
}
#[utoipa::path(
    post,
    path = "/api/v1/auth/logout-all",
    tag = "Authentication",
    request_body = LogoutAllRequest,
    responses(
        (
            status = 200,
            description = "Logged out from all devices"
        )
    ),
      security(
        ("bearer_auth" = [])
    )
)]
pub async fn logout_all(
    State(state): State<AppState>,
    Json(request): Json<LogoutAllRequest>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    AuthService::logout_all(&state.db, request).await?;

    Ok(Json(ApiResponse::empty("Logged out from all devices")))
}

#[utoipa::path(
    get,
    path = "/api/v1/auth/profile",
    tag = "Authentication",
    responses(
        (
            status = 200,
            description = "Current authenticated user",
            body = ProfileResponse
        )
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn profile(
    current_user: CurrentUser,
) -> Result<Json<ApiResponse<ProfileResponse>>, AppError> {
    Ok(Json(ApiResponse::success(
        "Current user fetched successfully",
        ProfileResponse {
            user_id: current_user.user_id,
            email: current_user.email,
            role: current_user.role,
        },
    )))
}
#[utoipa::path(
    get,
    path = "/api/v1/auth/sessions",
    tag = "Authentication",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (
            status = 200,
            description = "Sessions fetched successfully",
            body = Vec<SessionResponse>
        )
    )
)]
pub async fn sessions(
    State(state): State<AppState>,
    current_user: CurrentUser,
) -> Result<Json<ApiResponse<Vec<SessionResponse>>>, AppError> {
    let sessions = AuthService::get_sessions(&state.db, current_user.user_id).await?;

    Ok(Json(ApiResponse::success(
        "Sessions fetched successfully",
        sessions,
    )))
}
#[utoipa::path(
    delete,
    path = "/api/v1/auth/sessions/{session_id}",
    tag = "Authentication",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("session_id" = Uuid, Path, description = "Session ID")
    ),
    responses(
        (
            status = 200,
            description = "Session revoked successfully"
        )
    )
)]
pub async fn revoke_session(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(session_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    AuthService::revoke_session(&state.db, current_user.user_id, session_id).await?;

    Ok(Json(ApiResponse::empty("Session revoked successfully")))
}
