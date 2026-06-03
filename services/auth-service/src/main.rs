mod api;
mod app_state;
mod application;
mod auth;
mod config;
mod domain;
mod errors;
mod infrastructure;
mod openapi;
mod repositories;

use axum::Router;
use dotenvy::dotenv;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    api::routes::{auth_routes::auth_routes, health_routes::health_routes},
    app_state::AppState,
    config::app_config::AppConfig,
    infrastructure::postgres::connection::create_pool,
    openapi::docs::ApiDoc,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = AppConfig::from_env();

    let db_pool = create_pool(&config.database_url)
        .await
        .expect("Failed to connect database");

    let state = AppState { db: db_pool };

    let app = Router::new()
        .merge(health_routes())
        .merge(auth_routes())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state);

    let address = format!("{}:{}", config.server_host, config.server_port);

    println!("{} running on {}", config.app_name, address);

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
