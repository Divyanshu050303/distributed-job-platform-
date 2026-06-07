#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Admin,
    Recruiter,
    User,
}

impl Role {
    pub fn from_str(role: &str) -> Option<Self> {
        match role {
            "Admin" => Some(Self::Admin),
            "Recruiter" => Some(Self::Recruiter),
            "User" => Some(Self::User),
            _ => None,
        }
    }
}
