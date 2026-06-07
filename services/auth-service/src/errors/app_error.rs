use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    InternalServerError,

    UserAlreadyExists,

    RoleNotFound,

    InvalidCredentials,

    Unauthorized,
    InvalidRefreshToken,
    RefreshTokenExpired,
    UserNotFound,
    SessionNotFound,
    InvalidResetToken,
    ResetTokenExpired,
    ResetTokenAlreadyUsed,
    InvalidVerificationToken,
    VerificationTokenExpired,
    EmailAlreadyVerified,
    Forbidden,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
    success: bool,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::UserAlreadyExists => (
                StatusCode::CONFLICT,
                Json(ErrorResponse {
                    message: "User already exists".into(),
                    success: false,
                }),
            )
                .into_response(),

            AppError::RoleNotFound => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    message: "Role not found".into(),
                    success: false,
                }),
            )
                .into_response(),

            AppError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    message: "Invalid credentials".into(),
                    success: false,
                }),
            )
                .into_response(),

            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    message: "Unauthorized".into(),
                    success: false,
                }),
            )
                .into_response(),

            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: "Internal server error".into(),
                    success: false,
                }),
            )
                .into_response(),
            AppError::InvalidRefreshToken => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    message: "Invalid refresh token".into(),
                    success: false,
                }),
            )
                .into_response(),
            AppError::RefreshTokenExpired => (
                StatusCode::GONE,
                Json(ErrorResponse {
                    message: "Refresh token expired".into(),
                    success: false,
                }),
            )
                .into_response(),
            AppError::UserNotFound => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    message: "User not found".into(),
                    success: false,
                }),
            )
                .into_response(),
            AppError::SessionNotFound => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    message: "Session not found".into(),
                    success: false,
                }),
            )
                .into_response(),
            AppError::InvalidResetToken => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    message: "Invalid reset token".into(),
                    success: false,
                }),
            )
                .into_response(),
            AppError::ResetTokenExpired => (
                StatusCode::GONE,
                Json(ErrorResponse {
                    message: "Reset token expired".into(),
                    success: false,
                }),
            )
                .into_response(),
            AppError::ResetTokenAlreadyUsed => (
                StatusCode::GONE,
                Json(ErrorResponse {
                    message: "Reset token already used".into(),
                    success: false,
                }),
            )
                .into_response(),
            AppError::InvalidVerificationToken => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    message: "Invalid verification token".into(),
                    success: false,
                }),
            )
                .into_response(),
            AppError::VerificationTokenExpired => (
                StatusCode::GONE,
                Json(ErrorResponse {
                    message: "Verification token expired".into(),
                    success: false,
                }),
            )
                .into_response(),
            AppError::EmailAlreadyVerified => (
                StatusCode::CONFLICT,
                Json(ErrorResponse {
                    message: "Email already verified".into(),
                    success: false,
                }),
            )
                .into_response(),
            AppError::Forbidden => (
                StatusCode::FORBIDDEN,
                Json(ErrorResponse {
                    message: "Forbidden".into(),
                    success: false,
                }),
            )
                .into_response(),
        }
    }
}
