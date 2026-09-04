
use axum::extract::{Path};
use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, Json};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait};
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::DatabaseConnection;

use crate::structs::task::{CreateTaskRequest, Task, UpdateTaskRequest};
use crate::entities::tasks::ActiveModel as TaskActiveModel;
use crate::entities::tasks::Entity as TaskEntity;

pub async fn create_task(
    State(pool): State<DatabaseConnection>,
    Json(req): Json<CreateTaskRequest>,
) -> impl IntoResponse {
    let now = chrono::Utc::now();

    let new_task = TaskActiveModel {
        id: NotSet,
        name: Set(req.name),
        status: Set(req.status.into()),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
    };

    match new_task.insert(&pool).await {
        Ok(model) => match Task::try_from(model) {
            Ok(task) => (StatusCode::CREATED, Json(task)).into_response(),
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err).into_response(),
        },
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create task: {err}"),
        )
            .into_response(),
    }
}

pub async fn get_all_tasks(
    State(pool): State<DatabaseConnection>,
) -> impl IntoResponse {
    let models = match TaskEntity::find().all(&pool).await {
        Ok(models) => models,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch tasks: {err}"),
            )
                .into_response();
        }
    };

    let tasks: Result<Vec<Task>, String> = models.into_iter().map(Task::try_from).collect();

    match tasks {
        Ok(tasks) => (StatusCode::OK, Json(tasks)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err).into_response(),
    }
}

pub async fn get_one_task(
    State(pool): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let task_bd = match TaskEntity::find_by_id(id).one(&pool).await {
        Ok(task) => task,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch task: {err}"),
            )
                .into_response();
        }
    };

    let model = match task_bd {
        Some(model) => model,
        None => {
            return (
                StatusCode::NOT_FOUND,
                format!("Task with id {id} not found"),
            )
                .into_response();
        }
    };

    match Task::try_from(model) {
        Ok(task) => (StatusCode::OK, Json(task)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err).into_response(),
    }
}

pub async fn delete_task(
    State(pool): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let task_bd = match TaskEntity::find_by_id(id).one(&pool).await {
        Ok(task) => task,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch task: {err}"),
            )
                .into_response();
        }
    };

    let model = match task_bd {
        Some(model) => model,
        None => {
            return (
                StatusCode::NOT_FOUND,
                format!("Task with id {id} not found"),
            )
                .into_response();
        }
    };

    match model.delete(&pool).await {
        Ok(_) => (StatusCode::OK, "Deleted Task").into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete task: {err}"),
        )
            .into_response(),
    }
}

pub async fn update_task(
    State(pool): State<DatabaseConnection>, 
    Path(id): Path<i32>,
    Json(req): Json<UpdateTaskRequest>
) -> impl IntoResponse {
    let task_bd = match TaskEntity::find_by_id(id).one(&pool).await {
        Ok(task) => task,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch task: {err}"),
            )
                .into_response();
        }
    };

    let model = match task_bd {
        Some(model) => model,
        None => {
            return (
                StatusCode::NOT_FOUND,
                format!("Task with id {id} not found"),
            )
                .into_response();
        }
    };

    let mut model_act: TaskActiveModel = model.into();

    if let Some(name) = req.name {
        model_act.name = Set(name);
    };

    if let Some(status) = req.status {
        model_act.status = Set(status.into());
    };

    model_act.updated_at =Set(Utc::now().into());

    match model_act.update(&pool).await {
        Ok(updated) => match Task::try_from(updated) {
            Ok(task) => (StatusCode::OK, Json(task)).into_response(),
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err).into_response(),
        },
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to update task: {err}"),
        )
            .into_response(),
    }
    
}