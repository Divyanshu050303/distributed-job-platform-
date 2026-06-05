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
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::UserAlreadyExists => (
                StatusCode::CONFLICT,
                Json(ErrorResponse {
                    message: "User already exists".into(),
                }),
            )
                .into_response(),

            AppError::RoleNotFound => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    message: "Role not found".into(),
                }),
            )
                .into_response(),

            AppError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    message: "Invalid credentials".into(),
                }),
            )
                .into_response(),

            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    message: "Unauthorized".into(),
                }),
            )
                .into_response(),

            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: "Internal server error".into(),
                }),
            )
                .into_response(),
        }
    }
}
