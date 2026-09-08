use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;

use crate::{
    db::task::{
        delete_task,get_all_tasks,
        get_tasks_by_user_id,
    },
    errors::{DomainError, ServiceError},
    repository::task_repository::NewTask,
    service::task_service::TaskService,
    structs::{
        claims::Claims,
        task::{CreateTaskRequest, UpdateTaskRequest},
        user_role::UserRole,
    },
};

pub async fn get_all_tasks_handler(
    State(task_service): State<Arc<TaskService>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let result = if claims.role == UserRole::Admin {
        task_service.get_all_tasks_as_admin().await
    } else {
       task_service.get_all_for_user(claims.sub).await
    };

    match result {
        Ok(tasks) => (StatusCode::OK, Json(tasks)).into_response(),

        Err(err) => {
            eprintln!("Failed to get tasks: {err}");

            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load tasks").into_response()
        }
    }
}

pub async fn get_task_handler(
    State(task_service): State<Arc<TaskService>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let result = if claims.role == UserRole::Admin {
        task_service.get_task_as_admin(id).await
    } else {
        task_service.get_task(id, claims.sub).await
    };

    match result {
        Ok(task) => (StatusCode::OK, Json(task)).into_response(),

        Err(ServiceError::NotFound { .. }) => {
            let msg = format!("No available task with id: {id}");
          (StatusCode::NOT_FOUND, msg).into_response()
        }

        Err(err) => {
            eprintln!("Failed to get task: {err}");
 
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load task").into_response()
        }
    }  
}

pub async fn create_task_handler(
    State(task_service): State<Arc<TaskService>>,
    Extension(claims): Extension<Claims>,
    Json(task): Json<CreateTaskRequest>,
) -> impl IntoResponse {
    if claims.role != UserRole::Admin {
        return (StatusCode::FORBIDDEN, "Only admin can create tasks").into_response();
    }

    let new_task = NewTask {
        name: task.name,
        user_id: task.user_id,
        status: task.status,
    };

    match task_service.create_task(new_task).await {
        Ok(task) => (StatusCode::CREATED, Json(task)).into_response(),

        Err(ServiceError::Domain(DomainError::EmptyName)) => {
            (StatusCode::BAD_REQUEST, "Task name cannot be empty").into_response()
        }

        Err(err) => {
            eprintln!("Failed to create task: {err}");

            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create task").into_response()
        }
    }
}

pub async fn update_task_handler(
    State(task_service): State<Arc<TaskService>>,
    Path(id): Path<i32>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<UpdateTaskRequest>,
) -> impl IntoResponse {
    let result = if claims.role == UserRole::Admin {
        task_service.update_all_task(id, req).await
    } else {
        task_service
            .update_status_for_user(id, claims.sub, req)
            .await
    };

    match result {
        Ok(()) => (StatusCode::OK, "Task updated").into_response(),

        Err(ServiceError::Domain(DomainError::ReopenCancelledTask)) => {
            (StatusCode::CONFLICT, "Canceled task cannot be updated").into_response()
        }

        Err(ServiceError::Domain(DomainError::UserCanUpdateOnlyStatus)) => {
            (StatusCode::FORBIDDEN, "User can update only task status").into_response()
        }

        Err(ServiceError::NotFound { .. }) => {
            let msg = format!("No available task with id: {id}");
            (StatusCode::NOT_FOUND, msg).into_response()
        }

        Err(err) => {
            eprintln!("Failed to update task: {err}");

            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update task").into_response()
        }
    }
}

pub async fn delete_task_handler(
    State(task_service): State<Arc<TaskService>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    if claims.role != UserRole::Admin {
        return (StatusCode::FORBIDDEN, "Only admin can delete tasks").into_response();
    }

    match task_service.delete_task_as_admin(id).await {
        Ok(()) => (StatusCode::OK, "Task deleted").into_response(),

        Err(ServiceError::NotFound { .. }) => {
            let msg = format!("No task with id: {id}");
            (StatusCode::NOT_FOUND, msg).into_response()
        }

        Err(err) => {
            eprintln!("Failed to delete task: {err}");

            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete task").into_response()
        }
    }
}
