use crate::structs::user_role::UserRole;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub role: UserRole,
    pub exp: usize,
}
