use crate::structs::task::{CreateTaskRequest, Task, UpdateTaskRequest};
use sqlx::PgPool;

pub async fn get_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    PgPool::connect(&database_url).await
}

pub async fn create_table_tasks(
    pool: &PgPool
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS tasks (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending'
                CHECK (status IN ('pending', 'in_process', 'done', 'cancel')),
            user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        ",
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn find_task_by_id(
    pool: &PgPool,
    id: i32
) -> Result<Option<Task>, sqlx::Error> {
    sqlx::query_as::<_, Task>(
        "
        SELECT id, name, status, user_id, created_at, updated_at
        FROM tasks
        WHERE id = $1
        ",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn find_task_by_id_and_user_id(
    pool: &PgPool,
    task_id: i32,
    user_id: i32,
) -> Result<Option<Task>, sqlx::Error> {
    sqlx::query_as::<_, Task>(
        "
        SELECT *
        FROM tasks
        WHERE id = $1 AND user_id = $2
        ",
    )
    .bind(task_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn get_tasks_by_user_id(
    pool: &PgPool,
    user_id: i32
) -> Result<Vec<Task>, sqlx::Error> {
    sqlx::query_as::<_, Task>(
        "
        SELECT *
        FROM tasks
        WHERE user_id = $1
        ORDER BY id
        ",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_all_tasks(
    pool: &PgPool
) -> Result<Vec<Task>, sqlx::Error> {
    sqlx::query_as::<_, Task>(
        "
        SELECT id, name, status, user_id, created_at, updated_at
        FROM tasks
        ORDER BY id
        ",
    )
    .fetch_all(pool)
    .await
}

pub async fn update_all_task(
    pool: &PgPool,
    req: UpdateTaskRequest,
    id: i32,
) -> Result<bool, sqlx::Error> {
    let status: Option<String> = req.status.map(String::from);

    let result = sqlx::query(
        "
        UPDATE tasks
        SET
            name = COALESCE($1, name),
            status = COALESCE($2, status),
            updated_at = NOW()
        WHERE id = $3
        ",
    )
    .bind(req.name)
    .bind(status)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() == 1)
}

pub async fn update_task_status_for_user(
    pool: &PgPool,
    req: UpdateTaskRequest,
    id_task: i32,
    id_user: i32
) -> Result<bool, sqlx::Error> {
    let status: Option<String> = req.status.map(String::from);

    let result = sqlx::query(
        "
        UPDATE tasks
        SET
            status = COALESCE($1, status),
            updated_at = NOW()
        WHERE id = $2
           AND user_id = $3
        ",
    )
    .bind(status)
    .bind(id_task)
    .bind(id_user)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() == 1)
}

pub async fn delete_task(
    pool: &PgPool,
    id: i32
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "
        DELETE FROM tasks
        WHERE id = $1
        ",
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() == 1)
}

pub async fn create_task(
    pool: &PgPool,
    task: CreateTaskRequest
) -> Result<Task, sqlx::Error> {
    let status: String = task.status.into();

    sqlx::query_as::<_, Task>(
        "
        INSERT INTO tasks (name, status, user_id)
        VALUES ($1, $2, $3)
        RETURNING id, name, status, user_id, created_at, updated_at
        ",
    )
    .bind(task.name)
    .bind(status)
    .bind(task.user_id)
    .fetch_one(pool)
    .await
}
