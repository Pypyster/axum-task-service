use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;

use crate::{
    db::task::{
        create_task, delete_task, find_task_by_id, find_task_by_id_and_user_id, get_all_tasks, get_tasks_by_user_id, update_all_task, update_task_status_for_user,
    }, structs::{
        claims::Claims,
        task::{CreateTaskRequest, UpdateTaskRequest},
        user_role::UserRole,
    },
};

pub async fn get_all_tasks_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let result = if claims.role == UserRole::Admin {
        get_all_tasks(&pool).await
    } else {
        get_tasks_by_user_id(&pool, claims.sub).await
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
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let result = if claims.role == UserRole::Admin {
        find_task_by_id(&pool, id).await
    } else {
        find_task_by_id_and_user_id(&pool, id, claims.sub).await
    };

    match result {
        Ok(Some(task)) => (StatusCode::OK, Json(task)).into_response(),

        Ok(None) => {
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
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(task): Json<CreateTaskRequest>,
) -> impl IntoResponse {
    if claims.role != UserRole::Admin {
        return (StatusCode::FORBIDDEN, "Only admin can create tasks")
            .into_response();
    }

    match create_task(&pool, task).await {
        Ok(task) => (StatusCode::CREATED, Json(task)).into_response(),

        Err(err) => {
            eprintln!("Failed to create task: {err}");

            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create task")
                .into_response()
        }
    }
}

pub async fn update_task_handler(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<UpdateTaskRequest>,
) -> impl IntoResponse {
    let result = if claims.role == UserRole::Admin {
        update_all_task(&pool, req, id).await
    } else {
        if req.name.is_some() {
            return (
                StatusCode::FORBIDDEN,
                "User can update only task status",
            )
                .into_response();
        }

        update_task_status_for_user(&pool, req, id, claims.sub).await
    };

    match result {
        Ok(true) => (StatusCode::OK, "Task updated").into_response(),

        Ok(false) => {
            let msg = format!("No available task with id: {id}");
            (StatusCode::NOT_FOUND, msg).into_response()
        }

        Err(err) => {
            eprintln!("Failed to update task: {err}");

            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update task")
                .into_response()
        }
    }
}

pub async fn delete_task_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    if claims.role != UserRole::Admin {
        return (StatusCode::FORBIDDEN, "Only admin can delete tasks")
            .into_response();
    }

    match delete_task(&pool, id).await {
        Ok(true) => (StatusCode::OK, "Task deleted").into_response(),

        Ok(false) => {
            let msg = format!("No task with id: {id}");
            (StatusCode::NOT_FOUND, msg).into_response()
        }

        Err(err) => {
            eprintln!("Failed to delete task: {err}");

            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete task")
                .into_response()
        }
    }
}
