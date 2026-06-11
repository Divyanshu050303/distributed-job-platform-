use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::{
    api::handlers::job_handler::{create_job, get_job},
    app_state::AppState,
    middleware::auth_middleware::auth_middleware,
};

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/api/v1/jobs", post(create_job))
        .route("/api/v1/jobs/{id}", get(get_job))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
}
