use axum::{Router, routing::get};

use crate::{api::handlers::health_handler::health_check, app_state::AppState};

pub fn health_routes() -> Router<AppState> {
    Router::new().route("/health", get(health_check))
}
