use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};
use sqlx::PgPool;
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
            delete_user_handler_by_phone,
            get_all_users_handler,
            get_user_handler_by_id,
            get_user_handler_by_phone,
            login_handler,
        },
    },
};

pub fn create_routes(pool: PgPool) -> Router {
    let public_routes = Router::new()
        .route("/register", post(create_user_handler))
        .route("/login", post(login_handler));

    let admin_user_routes = Router::new()
    .route("/users", get(get_all_users_handler))
    .route("/users/id/{id}", get(get_user_handler_by_id))
    .route("/users/phone/{phone}", get(get_user_handler_by_phone))
    .route("/users/id/{id}", delete(delete_user_handler_by_id))
    .route(
        "/users/phone/{phone}",
        delete(delete_user_handler_by_phone),
    )
    .layer(
        ServiceBuilder::new()
            .layer(middleware::from_fn(auth_middleware))
            .layer(middleware::from_fn(admin_request)),
    );

    let task_routes = Router::new()
        .route(
            "/tasks",
            get(get_all_tasks_handler)
                .post(create_task_handler),
        )
        .route(
            "/tasks/{id}",
            get(get_task_handler)
                .put(update_task_handler)
                .delete(delete_task_handler),
        )
        .layer(middleware::from_fn(auth_middleware));

    Router::new()
        .merge(public_routes)
        .merge(admin_user_routes)
        .merge(task_routes)
        .with_state(pool)
}

pub async fn app() -> Result<Router, sqlx::Error> {
    let pool = get_pool().await?;
    
    create_table_users(pool.clone()).await?;

    create_table_tasks(&pool).await?;

    Ok(create_routes(pool))
}