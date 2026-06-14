mod api;
mod app_state;
mod application;
mod auth;
mod config;
mod domain;
mod enums;
mod errors;
mod infrastructure;
mod middleware;
mod openapi;
mod repositories;

use axum::Router;
use dotenvy::dotenv;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    api::routes::{health_routes::health_routes, job_routes},
    app_state::AppState,
    config::app_config::AppConfig,
    infrastructure::postgres::connection::create_pool,
    openapi::docs::ApiDoc,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Load configuration
    let config = AppConfig::from_env();

    // Create database pool
    let db_pool = create_pool(&config.database_url)
        .await
        .expect("Failed to connect to database");

    // Application state
    let state = AppState {
        db: db_pool,
        config: config.clone(),
    };

    // Build router
    let app = Router::new()
        .merge(health_routes())
        .merge(job_routes::routes(state.clone()))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state);

    let address = format!("{}:{}", config.server_host, config.server_port);

    println!("{} running on http://{}", config.app_name, address);

    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app).await.expect("Server failed");
}
