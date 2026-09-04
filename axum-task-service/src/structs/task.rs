use sea_orm::entity::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use crate::entities::tasks::Model as TaskModel;
use crate::structs::task_status::TaskStatus;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub status: TaskStatus,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

impl TryFrom<TaskModel> for Task {
    type Error = String;
    fn try_from(model: TaskModel) -> Result<Self, Self::Error> {
        Ok(Task {
            id: model.id,
            name: model.name,
            status: TaskStatus::try_from(model.status)?,
            created_at: model.created_at,
            updated_at: model.updated_at,
        })
    }
}

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    pub name: String,
    pub status: TaskStatus,
}

#[derive(Deserialize)]
pub struct UpdateTaskRequest {
    pub name: Option<String>,
    pub status: Option<TaskStatus>,
}