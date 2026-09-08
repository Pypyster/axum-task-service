use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    db::task::{
        create_task, delete_task, find_task_by_id, find_task_by_id_and_user_id, get_all_tasks,
        get_tasks_by_user_id, update_all_task, update_task_status_for_user,
    },
    repository::task_repository::{NewTask, TaskRepository},
    structs::task::{CreateTaskRequest, Task, UpdateTaskRequest},
};

pub struct PgTaskRepos {
    pool: PgPool,
}
impl PgTaskRepos {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskRepository for PgTaskRepos {
    async fn create(&self, task: NewTask) -> Result<Task, sqlx::Error> {
        let req = CreateTaskRequest {
            name: task.name,
            status: task.status,
            user_id: task.user_id,
        };
        create_task(&self.pool, req).await
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Task>, sqlx::Error> {
        find_task_by_id(&self.pool, id).await
    }

    async fn find_by_id_and_user(
        &self,
        task_id: i32,
        user_id: i32,
    ) -> Result<Option<Task>, sqlx::Error> {
        find_task_by_id_and_user_id(&self.pool, task_id, user_id).await
    }

    async fn get_by_user(&self, user_id: i32) -> Result<Vec<Task>, sqlx::Error> {
        get_tasks_by_user_id(&self.pool, user_id).await
    }

    async fn get_all(&self) -> Result<Vec<Task>, sqlx::Error> {
        get_all_tasks(&self.pool).await
    }

    async fn update_all_task(&self, task: UpdateTaskRequest, id: i32) -> Result<bool, sqlx::Error> {
        update_all_task(&self.pool, task, id).await
    }

    async fn update_status_for_user(
        &self,
        id: i32,
        user_id: i32,
        task: UpdateTaskRequest,
    ) -> Result<bool, sqlx::Error> {
        update_task_status_for_user(&self.pool, task, id, user_id).await
    }

    async fn delete(&self, id: i32) -> Result<bool, sqlx::Error> {
        delete_task(&self.pool, id).await
    }
}
