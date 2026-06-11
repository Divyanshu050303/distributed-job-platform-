use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::{auth::claims::Claims, config::app_config::AppConfig, errors::app_error::AppError};

pub struct JwtService;

impl JwtService {
    pub fn validate_access_token(token: &str, config: &AppConfig) -> Result<Claims, AppError> {
        let decoded = decode::<Claims>(
            token,
            &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized)?;

        Ok(decoded.claims)
    }
}
