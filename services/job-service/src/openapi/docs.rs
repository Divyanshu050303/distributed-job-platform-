use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::handlers::health_handler::health_check,
        crate::api::handlers::job_handler::create_job,
        crate::api::handlers::job_handler::get_job,
        crate::api::handlers::job_handler::get_jobs,
    ),
    components(
        schemas(
            crate::api::handlers::health_handler::HealthResponse,

            crate::api::dto::create_job_request::CreateJobRequest,
            crate::api::dto::create_job_response::CreateJobResponse,
            crate::api::dto::job_response::JobResponse,
            crate::api::dto::job_list_response::JobListResponse,
        )
    ),
    tags(
        (name = "Health", description = "Health APIs"),
        (name = "Jobs", description = "Job Management APIs")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);

        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}
