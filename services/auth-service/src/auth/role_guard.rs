use crate::{
    auth::{current_user::CurrentUser, roles::Role},
    errors::app_error::AppError,
};

pub fn require_role(current_user: &CurrentUser, allowed_roles: &[Role]) -> Result<(), AppError> {
    let user_role = Role::from_str(&current_user.role).ok_or(AppError::Forbidden)?;

    if allowed_roles.contains(&user_role) {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}
