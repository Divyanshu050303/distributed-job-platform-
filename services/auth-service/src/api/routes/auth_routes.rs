use axum::{
    Router, middleware,
    routing::{delete, get, post},
};

use crate::{
    api::handlers::auth_handler::{
        admin_dashboard, forgot_password, login, logout, logout_all, profile, refresh_token,
        register, reset_password, revoke_session, send_verification, sessions, verify_email,
    },
    app_state::AppState,
    middleware::auth_middleware::auth_middleware,
};

pub fn routes(state: AppState) -> Router<AppState> {
    let public_routes = Router::new()
        .route("/api/v1/auth/register", post(register))
        .route("/api/v1/auth/login", post(login))
        .route("/api/v1/auth/refresh-token", post(refresh_token))
        .route("/api/v1/auth/reset-password", post(reset_password))
        .route("/api/v1/auth/forgot-password", post(forgot_password))
        .route("/api/v1/auth/send-verification", post(send_verification))
        .route("/api/v1/auth/verify-email", post(verify_email));

    let protected_routes = Router::new()
        .route("/api/v1/auth/logout", post(logout))
        .route("/api/v1/auth/logout-all", post(logout_all))
        .route("/api/v1/auth/profile", get(profile))
        .route("/api/v1/auth/sessions", get(sessions))
        .route("/api/v1/auth/sessions/{session_id}", delete(revoke_session))
        .route("/api/v1/admin/dashboard", get(admin_dashboard))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    public_routes.merge(protected_routes)
}
