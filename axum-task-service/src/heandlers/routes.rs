use axum::{
    Router, middleware,
    routing::{delete, get, post},
};
use sqlx::PgPool;
use std::sync::Arc;
use tower::ServiceBuilder;

use crate::{
    db::{
        task::{create_table_tasks, get_pool},
        user::create_table_users,
    },
    handlers::{
        auth::{admin_request, auth_middleware},
        tasks::{
            create_task_handler,
            delete_task_handler,
            get_all_tasks_handler,
            get_task_handler,
            update_task_handler,
        },
        users::{
            create_user_handler,
            delete_user_handler_by_id,
            get_all_users_handler,
            get_user_handler_by_id,
            get_user_handler_by_phone,
            login_handler,
        },
    },
    repository::{
        pg_task_repository::PgTaskRepos,
        pg_user_repository::PgUserRepos,
        task_repository::TaskRepository,
        user_repository::UserRepository,
    },
    service::{
        task_service::TaskService,
        user_service::UserService,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub task_service: Arc<TaskService>,
    pub user_service: Arc<UserService>,
    pub jwt_secret: Arc<str>,
}


pub fn create_routes(state: Arc<AppState>) -> Router {
    let public_routes = Router::new()
        .route("/register", post(create_user_handler))
        .route("/login", post(login_handler));

    let admin_user_routes = Router::new()
        .route("/users", get(get_all_users_handler))
        .route("/users/id/{id}", get(get_user_handler_by_id))
        .route("/users/phone/{phone}", get(get_user_handler_by_phone))
        .route("/users/id/{id}", delete(delete_user_handler_by_id))
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn_with_state(
                    state.clone(), 
                 auth_middleware
                ))
                .layer(middleware::from_fn(admin_request)),
        );

    let task_routes = Router::new()
        .route(
            "/tasks",
            get(get_all_tasks_handler).post(create_task_handler),
        )
        .route(
            "/tasks/{id}",
            get(get_task_handler)
                .put(update_task_handler)
                .delete(delete_task_handler),
        )
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware
        ));

    Router::new()
        .merge(public_routes)
        .merge(admin_user_routes)
        .merge(task_routes)
        .with_state(state)
}

pub async fn app() -> Result<Router, sqlx::Error> {
    let pool = get_pool().await?;

    create_table_users(&pool).await?;

    create_table_tasks(&pool).await?;

    let task_repo: Arc<dyn TaskRepository> =
        Arc::new(PgTaskRepos::new(pool.clone()));

    let task_service = Arc::new(TaskService::new(task_repo));

    let user_repo: Arc<dyn UserRepository> =
        Arc::new(PgUserRepos::new(pool.clone()));

    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET is not set");

    let user_service = Arc::new(UserService::new(
        user_repo,
        jwt_secret.clone(),
    ));

    let state = Arc::new(AppState {
        task_service,
        user_service,
        jwt_secret: Arc::from(jwt_secret),
    });

    Ok(create_routes(state))
}
