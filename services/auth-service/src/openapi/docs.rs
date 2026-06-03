use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::handlers::health_handler::health_check,
        crate::api::handlers::auth_handler::register
    ),
    components(
        schemas(
            crate::api::handlers::health_handler::HealthResponse,

            crate::api::dto::register_request::RegisterRequest,
            crate::api::dto::register_response::RegisterResponse
        )
    ),
    tags(
        (name = "Health", description = "Health APIs"),
        (name = "Authentication", description = "Authentication APIs")
    )
)]
pub struct ApiDoc;
