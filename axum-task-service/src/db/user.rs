use sqlx::PgPool;

use crate::structs::user::{LoginRequest, RegisterRequest, User};

pub async fn create_table_users(pool: PgPool) -> Result<(), sqlx::Error> {
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
    .execute(&pool)
    .await?;

    Ok(())
}

pub async fn find_user_by_id(
    pool: PgPool, 
    id: i32
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "
        SELECT *
        FROM users
        WHERE id = $1
        ",
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
}

pub async fn find_user_by_phone(
    pool: PgPool, 
    phone: String
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "
        SELECT *
        FROM users WHERE phone = $1
        ",
    )
    .bind(&phone)
    .fetch_optional(&pool)
    .await
}

pub async fn create_password_hash(
    password: String
) -> Result<String, bcrypt::BcryptError> {
    tokio::task::spawn_blocking(move || bcrypt::hash(password, bcrypt::DEFAULT_COST))
        .await
        .expect("spawn_blocking task panicked")
}

pub async fn check_password_hash(
    pool: PgPool, 
    user: LoginRequest
) -> Result<bool, sqlx::Error> {
    let row: Option<(String,)> = sqlx::query_as("
        SELECT password_hash 
        FROM users 
        WHERE phone = $1"
    )
    .bind(user.phone)
    .fetch_optional(&pool)
    .await?;

    let Some((stored_hash,)) = row else {
        return Ok(false);
    };

    let is_valid = tokio::task::spawn_blocking(move || bcrypt::verify(user.password, &stored_hash))
        .await
        .expect("spawn_blocking task panicked")
        .unwrap_or(false);

    Ok(is_valid)
}

pub async fn create_user(
    pool: PgPool, 
    user: RegisterRequest
) -> Result<User, String> {
    let password_hash = create_password_hash(user.password)
        .await
        .map_err(|e| e.to_string())?;

    let result = sqlx::query_as::<_, User>(
        "
        INSERT INTO users (role, name, phone, password_hash)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        ",
    )
    .bind(String::from(user.role.clone()))    .bind(&user.name)
    .bind(&user.phone)
    .bind(&password_hash)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(new_user) => Ok(new_user),
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
            Err("User with this phone already exists".to_string())
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn delete_user_by_id(
    pool: PgPool,
    id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
        DELETE FROM users
        WHERE id = $1
        ",
    )
    .bind(id)
    .execute(&pool)
    .await?;

    Ok(())
}

pub async fn delete_user_by_phone(
    pool: PgPool,
    phone: String,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
        DELETE FROM users
        WHERE phone = $1
        ",
    )
    .bind(&phone)
    .execute(&pool)
    .await?;

    Ok(())
}

pub async fn list_users(pool: PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "
        SELECT *
        FROM users
        ORDER BY id
        ",
    )
    .fetch_all(&pool)
    .await
}
