use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::{
    api::handlers::auth_handler::{login, logout, logout_all, profile, refresh_token, register},
    app_state::AppState,
    middleware::auth_middleware::auth_middleware,
};

pub fn routes(state: AppState) -> Router<AppState> {
    let public_routes = Router::new()
        .route("/api/v1/auth/register", post(register))
        .route("/api/v1/auth/login", post(login))
        .route("/api/v1/auth/refresh-token", post(refresh_token));

    let protected_routes = Router::new()
        .route("/api/v1/auth/logout", post(logout))
        .route("/api/v1/auth/logout-all", post(logout_all))
        .route("/api/v1/auth/profile", get(profile))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    public_routes.merge(protected_routes)
}
