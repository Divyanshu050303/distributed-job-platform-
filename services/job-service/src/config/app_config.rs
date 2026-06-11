use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app_name: String,
    pub server_host: String,
    pub server_port: u16,
    pub database_url: String,
    pub jwt_secret: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            app_name: "job-service".to_string(),

            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),

            server_port: env::var("JOB_SERVICE_PORT")
                .unwrap_or_else(|_| "8082".to_string())
                .parse()
                .expect("JOB_SERVICE_PORT must be a valid number"),

            database_url: env::var("DATABASE_URL_JOB_SERVICE")
                .expect("DATABASE_URL_JOB_SERVICE must be set"),

            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
        }
    }
}
