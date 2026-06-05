use sqlx::PgPool;

use crate::{
    api::dto::{
        register_request::RegisterRequest, register_response::RegisterResponse,
        user_request::CreateUserParams,
    },
    auth::password::PasswordService,
    errors::app_error::AppError,
    repositories::{role_repository::RoleRepository, user_repository::UserRepository},
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
}
