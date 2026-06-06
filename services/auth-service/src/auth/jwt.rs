use chrono::{Duration, Utc};

use jsonwebtoken::{EncodingKey, Header, encode};

use uuid::Uuid;

use crate::{auth::claims::Claims, config::app_config::AppConfig, errors::app_error::AppError};
use jsonwebtoken::{DecodingKey, Validation, decode};
pub struct JwtService;

impl JwtService {
    pub fn generate_access_token(
        user_id: Uuid,
        email: String,
        role: String,
        config: &AppConfig,
    ) -> Result<String, AppError> {
        let now = Utc::now();

        let expiry = now + Duration::minutes(config.jwt_access_token_expiry_minutes);

        let claims = Claims {
            sub: user_id.to_string(),

            email,

            role,

            exp: expiry.timestamp() as usize,

            iat: now.timestamp() as usize,

            iss: config.jwt_issuer.clone(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
        )
        .map_err(|_| AppError::InternalServerError)
    }
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
