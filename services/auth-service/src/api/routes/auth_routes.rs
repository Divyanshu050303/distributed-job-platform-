use axum::{Router, routing::post};

use crate::{api::handlers::auth_handler::register, app_state::AppState};

pub fn auth_routes() -> Router<AppState> {
    Router::new().route("/api/v1/auth/register", post(register))
}
