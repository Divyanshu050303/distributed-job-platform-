use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub user_id: Uuid,
    pub email: String,
    pub role: String,
}
