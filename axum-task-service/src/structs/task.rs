use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, postgres::PgRow};

use crate::structs::task_status::TaskStatus;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub status: TaskStatus,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, PgRow> for Task {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let status_text: String = row.try_get("status")?;

        let status = TaskStatus::try_from(status_text).map_err(|err| {
            sqlx::Error::Decode(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                err,
            )))
        })?;

        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            status,
            user_id: row.try_get("user_id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub name: String,
    pub status: TaskStatus,
    pub user_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub name: Option<String>,
    pub status: Option<TaskStatus>,
}
