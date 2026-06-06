use sqlx::PgPool;

use crate::config::app_config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,

    pub config: AppConfig,
}
