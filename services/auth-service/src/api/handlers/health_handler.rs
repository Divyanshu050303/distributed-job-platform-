use crate::{app_state::AppState, repositories::role_repository::RoleRepository};
use axum::{Json, extract::State};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
}

#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (
            status = 200,
            description = "Health Check",
            body = HealthResponse
        )
    )
)]
pub async fn health_check(State(state): State<AppState>) -> Json<HealthResponse> {
    let role = RoleRepository::find_by_name(&state.db, "User")
        .await
        .unwrap();

    println!("{:#?}", role);

    Json(HealthResponse {
        status: "ok".to_string(),
    })
}
