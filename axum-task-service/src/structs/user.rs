use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, postgres::PgRow};

use crate::structs::user_role::UserRole;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,

    #[serde(skip_serializing)]
    pub password_hash: String,

    pub created_at: DateTime<Utc>,
    pub role: UserRole,
    pub phone: String,
}

impl<'r> FromRow<'r, PgRow> for User {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let role_text: String = row.try_get("role")?;

        let role = UserRole::try_from(role_text).map_err(|err| {
            sqlx::Error::Decode(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                err,
            )))
        })?;

        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            password_hash: row.try_get("password_hash")?,
            role,
            created_at: row.try_get("created_at")?,
            phone: row.try_get("phone")?,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub phone: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub phone: String,
    pub password: String,
    pub role: UserRole
}
