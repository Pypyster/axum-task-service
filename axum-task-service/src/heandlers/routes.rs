use axum::routing::{delete, get, patch, post};
use axum::Router;
use sea_orm::{Database, DatabaseConnection};

use crate::handlers::tasks::{create_task, delete_task, get_all_tasks, get_one_task, update_task};

async fn db() -> DatabaseConnection {
    let db_url = "postgres://postgres:PYPYster0312@localhost:5432/my_project.db";
    Database::connect(db_url).await.expect("Connection Error")
}

pub async fn app() -> Router {
    let db = db().await;

    Router::new()
        .route("/tasks", get(get_all_tasks).post(create_task))
        .route("/create_task", post(create_task))
        .route("/tasks/{id}", get(get_one_task).patch(update_task).delete(delete_task))
        .with_state(db)
}