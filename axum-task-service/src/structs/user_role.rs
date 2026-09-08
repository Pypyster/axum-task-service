use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    User,
    Admin,
}

impl From<UserRole> for String {
    fn from(value: UserRole) -> Self {
        match value {
            UserRole::Admin => "admin".to_string(),
            UserRole::User => "user".to_string(),
        }
    }
}

impl TryFrom<String> for UserRole {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "admin" => Ok(UserRole::Admin),
            "user" => Ok(UserRole::User),
            other => Err(format!("Unknown user role: {}", other)),
        }
    }
}
