use serde::{Deserialize, Serialize};
use sqlx::Type;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Type, ToSchema, PartialEq)]
#[sqlx(type_name = "employment_type", rename_all = "PascalCase")]
pub enum EmploymentType {
    FullTime,
    PartTime,
    Contract,
    Internship,
    Freelance,
}
