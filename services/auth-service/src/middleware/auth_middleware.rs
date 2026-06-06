use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::{
    app_state::AppState,
    auth::{current_user::CurrentUser, jwt::JwtService},
};
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = JwtService::validate_access_token(token, &state.config)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let current_user = CurrentUser {
        user_id: claims.sub.parse().map_err(|_| StatusCode::UNAUTHORIZED)?,

        email: claims.email,

        role: claims.role,
    };

    request.extensions_mut().insert(current_user);

    Ok(next.run(request).await)
}
