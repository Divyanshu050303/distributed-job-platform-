use utoipa::{Modify, OpenApi};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::handlers::health_handler::health_check,
        crate::api::handlers::auth_handler::register,
         crate::api::handlers::auth_handler::login,
         crate::api::handlers::auth_handler::refresh_token,
         crate::api::handlers::auth_handler::logout,
         crate::api::handlers::auth_handler::logout_all,
         crate::api::handlers::auth_handler::profile,
    ),
    components(
        schemas(
            crate::api::handlers::health_handler::HealthResponse,

            crate::api::dto::register_request::RegisterRequest,
            crate::api::dto::register_response::RegisterResponse,
             crate::api::dto::login_request::LoginRequest,
            crate::api::dto::login_response::LoginResponse,
            crate::api::dto::refresh_token_request::RefreshTokenRequest,
            crate::api::dto::refresh_token_response::RefreshTokenResponse,
            crate::api::dto::logout_request::LogoutRequest,
            crate::api::dto::logout_all_request::LogoutAllRequest,
            crate::api::dto::profile_response::ProfileResponse
        )
    ),
    tags(
        (name = "Health", description = "Health APIs"),
        (name = "Authentication", description = "Authentication APIs")
    ),
      modifiers(&SecurityAddon)
)]
pub struct ApiDoc;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();

        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}
