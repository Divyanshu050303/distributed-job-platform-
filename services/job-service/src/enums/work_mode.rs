use serde::{Deserialize, Serialize};
use sqlx::Type;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Type, ToSchema, PartialEq)]
#[sqlx(type_name = "work_mode", rename_all = "PascalCase")]
pub enum WorkMode {
    Remote,
    Hybrid,
    Onsite,
}
