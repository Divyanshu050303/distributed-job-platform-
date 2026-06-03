use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::user::User;

pub struct UserRepository;

pub struct CreateUserParams {
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub role_id: Uuid,
}
