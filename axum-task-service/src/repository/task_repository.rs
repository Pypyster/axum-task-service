use crate::structs::task::{Task, UpdateTaskRequest};
use crate::structs::task_status::TaskStatus;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct NewTask {
    pub name: String,
    pub status: TaskStatus,
    pub user_id: i32,
}

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn create(&self, task: NewTask) -> Result<Task, sqlx::Error>;
    async fn find_by_id(&self, id: i32) -> Result<Option<Task>, sqlx::Error>;
    async fn find_by_id_and_user(
        &self,
        task_id: i32,
        user_id: i32,
    ) -> Result<Option<Task>, sqlx::Error>;
    async fn get_all(&self) -> Result<Vec<Task>, sqlx::Error>;
    async fn get_by_user(&self, user_id: i32) -> Result<Vec<Task>, sqlx::Error>;
    async fn update_all_task(&self, task: UpdateTaskRequest, id: i32) -> Result<bool, sqlx::Error>;
    async fn update_status_for_user(
        &self,
        id: i32,
        user_id: i32,
        task: UpdateTaskRequest,
    ) -> Result<bool, sqlx::Error>;
    async fn delete(&self, id: i32) -> Result<bool, sqlx::Error>;
}
