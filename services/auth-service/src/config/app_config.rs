use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app_name: String,
    pub app_env: String,

    pub server_host: String,
    pub server_port: u16,

    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            app_name: env::var("APP_NAME").expect("APP_NAME missing"),

            app_env: env::var("APP_ENV").expect("APP_ENV missing"),

            server_host: env::var("SERVER_HOST").expect("SERVER_HOST missing"),

            server_port: env::var("SERVER_PORT")
                .expect("SERVER_PORT missing")
                .parse()
                .expect("Invalid SERVER_PORT"),

            database_url: env::var("DATABASE_URL").expect("DATABASE_URL missing"),
        }
    }
}
