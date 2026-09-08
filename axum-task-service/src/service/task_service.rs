use std::sync::Arc;

use crate::errors::{DomainError, ServiceError};
use crate::repository::task_repository::{NewTask, TaskRepository};
use crate::structs::{
    task::{Task, UpdateTaskRequest},
    task_status::TaskStatus,
};
pub struct TaskService {
    repo: Arc<dyn TaskRepository>,
}

impl TaskService {
    pub fn new(repo: Arc<dyn TaskRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_task(&self, new_task: NewTask) -> Result<Task, ServiceError> {
        if new_task.name.trim().is_empty() {
            return Err(ServiceError::Domain(DomainError::EmptyTaskName));
        }

        let task = self.repo.create(new_task).await?;
        Ok(task)
    }

    pub async fn get_task(&self, id: i32, user_id: i32) -> Result<Task, ServiceError> {
        self.repo
            .find_by_id_and_user(id, user_id)
            .await?
            .ok_or(ServiceError::NotFound {
                resource: "task".into(),
                id,
            })
    }

    pub async fn get_task_as_admin(&self, id: i32) -> Result<Task, ServiceError> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or(ServiceError::NotFound {
                resource: "task".into(),
                id,
            })
    }

    pub async fn get_all_tasks_as_admin(&self) -> Result<Vec<Task>, ServiceError> {
        let tasks = self.repo.get_all().await?;
        Ok(tasks)
    }

    pub async fn get_all_for_user(&self, user_id: i32) -> Result<Vec<Task>, ServiceError> {
        let tasks = self.repo.get_by_user(user_id).await?;
        Ok(tasks)
    }

    pub async fn update_all_task(
        &self,
        id: i32,
        req: UpdateTaskRequest,
    ) -> Result<(), ServiceError> {
        let task = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or(ServiceError::NotFound {
                resource: "task".into(),
                id,
            })?;

        if task.status == TaskStatus::Cancel {
            return Err(ServiceError::Domain(DomainError::ReopenCancelledTask));
        }

        let updated = self.repo.update_all_task(req, id).await?;

        if !updated {
            return Err(ServiceError::NotFound {
                resource: "task".into(),
                id,
            });
        }

        Ok(())
    }

    pub async fn update_status_for_user(
        &self,
        id: i32,
        user_id: i32,
        req: UpdateTaskRequest,
    ) -> Result<(), ServiceError> {
        if req.name.is_some() {
            return Err(ServiceError::Domain(DomainError::UserCanUpdateOnlyStatus));
        }
        let task =
            self.repo
                .find_by_id_and_user(id, user_id)
                .await?
                .ok_or(ServiceError::NotFound {
                    resource: "task".into(),
                    id,
                })?;

        if task.status == TaskStatus::Cancel {
            return Err(ServiceError::Domain(DomainError::ReopenCancelledTask));
        }

        let updated = self.repo.update_status_for_user(id, user_id, req).await?;

        if !updated {
            return Err(ServiceError::NotFound {
                resource: "task".into(),
                id,
            });
        }

        Ok(())
    }

    pub async fn delete_task_as_admin(&self, id: i32) -> Result<(), ServiceError> {
        let deleted = self.repo.delete(id).await?;

        if !deleted {
            return Err(ServiceError::NotFound {
                resource: "task".into(),
                id,
            });
        }

        Ok(())
    }
}
