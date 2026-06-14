use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};

use crate::{
    api::handlers::{
        health_handler::delete_job,
        job_handler::{create_job, get_job, get_jobs, update_job},
    },
    app_state::AppState,
    middleware::auth_middleware::auth_middleware,
};

pub fn routes(state: AppState) -> Router<AppState> {
    let public_routes = Router::new()
        .route("/api/v1/jobs/{id}", get(get_job))
        .route("/api/v1/jobs", get(get_jobs));

    let protected_routes = Router::new()
        .route("/api/v1/jobs", post(create_job))
        .route("/api/v1/jobs/{id}", put(update_job))
        .route("/api/v1/jobs/{id}", delete(delete_job))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    protected_routes.merge(public_routes)
}
