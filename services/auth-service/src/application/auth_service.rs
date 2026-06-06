use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    api::dto::{login_request::LoginRequest, login_response::LoginResponse},
    auth::{jwt::JwtService, password::PasswordService, token_hash::TokenHashService},
    config::app_config::AppConfig,
};
use crate::{
    api::dto::{
        logout_all_request::LogoutAllRequest, logout_request::LogoutRequest,
        refresh_token_request::RefreshTokenRequest, refresh_token_response::RefreshTokenResponse,
        register_request::RegisterRequest, register_response::RegisterResponse,
        session_response::SessionResponse, user_request::CreateUserParams,
    },
    errors::app_error::AppError,
    repositories::{
        refresh_token_repository::RefreshTokenRepository, role_repository::RoleRepository,
        session_repository::SessionRepository, user_repository::UserRepository,
    },
};

pub struct AuthService;

impl AuthService {
    pub async fn register(
        pool: &PgPool,
        request: RegisterRequest,
    ) -> Result<RegisterResponse, AppError> {
        let existing_user = UserRepository::find_by_email(pool, &request.email)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        if existing_user.is_some() {
            return Err(AppError::UserAlreadyExists);
        }

        let role = RoleRepository::find_by_name(pool, "User")
            .await
            .map_err(|_| AppError::InternalServerError)?;

        let role = role.ok_or(AppError::RoleNotFound)?;

        let password_hash = PasswordService::hash_password(&request.password)
            .map_err(|_| AppError::InternalServerError)?;

        let user = UserRepository::create_user(
            pool,
            CreateUserParams {
                email: request.email.clone(),
                password_hash,
                first_name: request.first_name.clone(),
                last_name: request.last_name.clone(),
                role_id: role.id,
            },
        )
        .await
        .map_err(|_| AppError::InternalServerError)?;

        Ok(RegisterResponse {
            id: user.id,
            email: user.email,
            first_name: user.first_name.unwrap_or_default(),
            last_name: user.last_name.unwrap_or_default(),
            is_verified: user.is_verified,
        })
    }
    pub async fn login(
        pool: &PgPool,
        config: &AppConfig,
        request: LoginRequest,
    ) -> Result<LoginResponse, AppError> {
        let user = UserRepository::find_by_email(pool, &request.email)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        let user = user.ok_or(AppError::InvalidCredentials)?;

        let valid = PasswordService::verify_password(&request.password, &user.password_hash)
            .map_err(|_| AppError::InternalServerError)?;

        if !valid {
            return Err(AppError::InvalidCredentials);
        }
        let session = SessionRepository::create(pool, user.id, None, None)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        let access_token = JwtService::generate_access_token(
            user.id,
            user.email.clone(),
            "User".to_string(),
            config,
        )?;

        let refresh_token = uuid::Uuid::new_v4().to_string();
        let token_hash = TokenHashService::hash(&refresh_token);
        RefreshTokenRepository::create(pool, user.id, session.id, token_hash)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        Ok(LoginResponse {
            access_token,

            refresh_token,

            token_type: "Bearer".to_string(),

            expires_in: 900,
        })
    }
    pub async fn refresh_token(
        pool: &PgPool,
        config: &AppConfig,
        request: RefreshTokenRequest,
    ) -> Result<RefreshTokenResponse, AppError> {
        // 1. Hash incoming refresh token

        let request_token_hash = TokenHashService::hash(&request.refresh_token);

        // 2. Find refresh token

        let existing_token = RefreshTokenRepository::find_by_hash(pool, &request_token_hash)
            .await
            .map_err(|_| AppError::InternalServerError)?
            .ok_or(AppError::InvalidRefreshToken)?;

        // 3. Check revoked

        if existing_token.revoked {
            return Err(AppError::InvalidRefreshToken);
        }

        // 4. Load user

        let user = UserRepository::find_by_id(pool, existing_token.user_id)
            .await
            .map_err(|_| AppError::InternalServerError)?
            .ok_or(AppError::UserNotFound)?;

        // 5. Generate new access token

        let access_token = JwtService::generate_access_token(
            user.id,
            user.email.clone(),
            "User".to_string(),
            config,
        )?;

        // 6. Generate new refresh token

        let new_refresh_token = uuid::Uuid::new_v4().to_string();

        let token_hash = TokenHashService::hash(&new_refresh_token);

        // 7. Store new refresh token

        RefreshTokenRepository::create(pool, user.id, existing_token.session_id, token_hash)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        // 8. Revoke old refresh token

        RefreshTokenRepository::revoke(pool, existing_token.id)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        // 9. Return response

        Ok(RefreshTokenResponse {
            access_token,
            refresh_token: new_refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: 900,
        })
    }
    pub async fn logout(pool: &PgPool, request: LogoutRequest) -> Result<(), AppError> {
        let token_hash = TokenHashService::hash(&request.refresh_token);

        let refresh_token = RefreshTokenRepository::find_by_hash(pool, &token_hash)
            .await
            .map_err(|_| AppError::InternalServerError)?
            .ok_or(AppError::InvalidRefreshToken)?;

        RefreshTokenRepository::revoke(pool, refresh_token.id)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        Ok(())
    }
    pub async fn logout_all(pool: &PgPool, request: LogoutAllRequest) -> Result<(), AppError> {
        RefreshTokenRepository::revoke_all_by_user(pool, request.user_id)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        SessionRepository::deactivate_all_by_user(pool, request.user_id)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        Ok(())
    }
    pub async fn get_sessions(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<SessionResponse>, AppError> {
        let sessions = SessionRepository::find_by_user_id(pool, user_id)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        Ok(sessions
            .into_iter()
            .map(|session| SessionResponse {
                session_id: session.id,

                device_name: session.device_name,

                user_agent: session.user_agent,

                ip_address: session.ip_address,

                is_active: session.is_active,

                last_activity_at: session.last_activity_at,

                created_at: session.created_at,
            })
            .collect())
    }
    pub async fn revoke_session(
        pool: &PgPool,
        current_user_id: Uuid,
        session_id: Uuid,
    ) -> Result<(), AppError> {
        let session = SessionRepository::find_by_id(pool, session_id)
            .await
            .map_err(|_| AppError::InternalServerError)?
            .ok_or(AppError::SessionNotFound)?;

        if session.user_id != current_user_id {
            return Err(AppError::Unauthorized);
        }

        SessionRepository::deactivate(pool, session_id)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        RefreshTokenRepository::revoke_by_session(pool, session_id)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        Ok(())
    }
}
