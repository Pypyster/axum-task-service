use sqlx::PgPool;

use crate::structs::{user::User, user_role::UserRole};

pub async fn create_table_users(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            role TEXT NOT NULL DEFAULT 'user'
                CHECK ( role IN ('user', 'admin')),
            phone TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL
        )
        ",
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn find_user_by_id(pool: &PgPool, id: i32) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "
        SELECT *
        FROM users
        WHERE id = $1
        ",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn find_user_by_phone(pool: &PgPool, phone: String) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "
        SELECT *
        FROM users WHERE phone = $1
        ",
    )
    .bind(&phone)
    .fetch_optional(pool)
    .await
}

pub async fn create_user(
    pool: &PgPool,
    name: String,
    phone: String,
    role: UserRole,
    password_hash: String,
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "
        INSERT INTO users (role, name, phone, password_hash)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        ",
    )
    .bind(String::from(role.clone()))
    .bind(name)
    .bind(phone)
    .bind(password_hash)
    .fetch_one(pool)
    .await
}

pub async fn delete_user_by_id(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "
        DELETE FROM users
        WHERE id = $1
        ",
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn delete_user_by_phone(pool: &PgPool, phone: String) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
        DELETE FROM users
        WHERE phone = $1
        ",
    )
    .bind(&phone)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn list_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "
        SELECT *
        FROM users
        ORDER BY id
        ",
    )
    .fetch_all(pool)
    .await
}
