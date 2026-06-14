use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),

    Unauthorized,

    Forbidden,

    NotFound,

    Conflict(String),

    Validation(String),

    InternalServerError,

    JobNotFound,
}

#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),

            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),

            AppError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_string()),

            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),

            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),

            AppError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),

            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),

            AppError::JobNotFound => (StatusCode::NOT_FOUND, "Job not found".to_string()),
        };

        (
            status,
            Json(ErrorResponse {
                success: false,
                message,
            }),
        )
            .into_response()
    }
}
