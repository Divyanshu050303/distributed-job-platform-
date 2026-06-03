use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Role {
    pub id: Uuid,

    pub name: String,

    pub description: Option<String>,

    pub created_at: NaiveDateTime,

    pub updated_at: NaiveDateTime,
}
