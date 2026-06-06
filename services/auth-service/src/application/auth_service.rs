use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    api::dto::{
        forgot_password_request::ForgotPasswordRequest, logout_all_request::LogoutAllRequest,
        logout_request::LogoutRequest, refresh_token_request::RefreshTokenRequest,
        refresh_token_response::RefreshTokenResponse, register_request::RegisterRequest,
        register_response::RegisterResponse, send_verification_request::SendVerificationRequest,
        session_response::SessionResponse, user_request::CreateUserParams,
        verify_email_request::VerifyEmailRequest,
    },
    errors::app_error::AppError,
    repositories::{
        email_verification_repository::EmailVerificationRepository,
        password_reset_repository::PasswordResetRepository,
        refresh_token_repository::RefreshTokenRepository, role_repository::RoleRepository,
        session_repository::SessionRepository, user_repository::UserRepository,
    },
};
use crate::{
    api::dto::{
        login_request::LoginRequest, login_response::LoginResponse,
        reset_password_request::ResetPasswordRequest,
    },
    auth::{jwt::JwtService, password::PasswordService, token_hash::TokenHashService},
    config::app_config::AppConfig,
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
    pub async fn forgot_password(
        pool: &PgPool,
        request: ForgotPasswordRequest,
    ) -> Result<(), AppError> {
        let user = UserRepository::find_by_email(pool, &request.email)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        if let Some(user) = user {
            let reset_token = uuid::Uuid::new_v4().to_string();

            let token_hash = TokenHashService::hash(&reset_token);

            PasswordResetRepository::create(pool, user.id, token_hash)
                .await
                .map_err(|_| AppError::InternalServerError)?;

            println!("Password Reset Token: {}", reset_token);
        }

        Ok(())
    }
    pub async fn reset_password(
        pool: &PgPool,
        request: ResetPasswordRequest,
    ) -> Result<(), AppError> {
        let token_hash = TokenHashService::hash(&request.token);

        let reset_token = PasswordResetRepository::find_by_hash(pool, &token_hash)
            .await
            .map_err(|_| AppError::InternalServerError)?
            .ok_or(AppError::InvalidResetToken)?;

        if reset_token.used_at.is_some() {
            return Err(AppError::ResetTokenAlreadyUsed);
        }

        if reset_token.expires_at < chrono::Utc::now().naive_utc() {
            return Err(AppError::ResetTokenExpired);
        }

        let password_hash = PasswordService::hash_password(&request.new_password)
            .map_err(|_| AppError::InternalServerError)?;

        UserRepository::update_password(pool, reset_token.user_id, password_hash)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        PasswordResetRepository::mark_used(pool, reset_token.id)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        RefreshTokenRepository::revoke_all_by_user(pool, reset_token.user_id)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        SessionRepository::deactivate_all_by_user(pool, reset_token.user_id)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        Ok(())
    }
    pub async fn send_verification(
        pool: &PgPool,
        request: SendVerificationRequest,
    ) -> Result<(), AppError> {
        println!("=== SEND VERIFICATION START ===");
        println!("Email: {}", request.email);

        let user = UserRepository::find_by_email(pool, &request.email)
            .await
            .map_err(|e| {
                println!("Database Error: {:?}", e);
                AppError::InternalServerError
            })?;

        println!("User Found: {:?}", user);

        if let Some(user) = user {
            println!("User ID: {}", user.id);
            println!("Is Verified: {}", user.is_verified);

            if user.is_verified {
                println!("User already verified");
                return Ok(());
            }

            let token = uuid::Uuid::new_v4().to_string();

            println!("Generated Token: {}", token);

            let token_hash = TokenHashService::hash(&token);

            println!("Generated Hash: {}", token_hash);

            EmailVerificationRepository::create(pool, user.id, token_hash)
                .await
                .map_err(|e| {
                    println!("Create Verification Error: {:?}", e);
                    AppError::InternalServerError
                })?;

            println!("Email Verification Token: {}", token);
            println!("Verification record created successfully");
        } else {
            println!("User not found");
        }

        println!("=== SEND VERIFICATION END ===");

        Ok(())
    }
    pub async fn verify_email(pool: &PgPool, request: VerifyEmailRequest) -> Result<(), AppError> {
        let token_hash = TokenHashService::hash(&request.token);

        let verification = EmailVerificationRepository::find_by_hash(pool, &token_hash)
            .await
            .map_err(|_| AppError::InternalServerError)?
            .ok_or(AppError::InvalidVerificationToken)?;

        if verification.verified_at.is_some() {
            return Err(AppError::EmailAlreadyVerified);
        }

        if verification.expires_at < chrono::Utc::now().naive_utc() {
            return Err(AppError::VerificationTokenExpired);
        }

        UserRepository::verify_email(pool, verification.user_id)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        EmailVerificationRepository::mark_verified(pool, verification.id)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        Ok(())
    }
}
