use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;
use serde_json::json;

use crate::{
    db::user::{
        check_password_hash, create_user, delete_user_by_id, delete_user_by_phone, find_user_by_id,
        find_user_by_phone, list_users,
    }, handlers::auth::create_token, structs::user::{LoginRequest, RegisterRequest},
};

pub async fn get_all_users_handler(
    State(pool): State<PgPool>
) -> impl IntoResponse {
    match list_users(pool).await {
        Ok(users_list) => (StatusCode::OK, Json(users_list)).into_response(),

        Err(err) => {
            eprintln!("Failed to get users: {err}");

            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load users").into_response()
        }
    }
}

pub async fn get_user_handler_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match find_user_by_id(pool, id).await {
        Ok(Some(user)) => (StatusCode::OK, Json(user)).into_response(),

        Ok(None) => {
            let msg = format!("No user with id: {id}");
            (StatusCode::NOT_FOUND, msg).into_response()
        }

        Err(err) => {
            eprintln!("Failed to get user: {err}");

            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load user").into_response()
        }
    }
}

pub async fn get_user_handler_by_phone(
    State(pool): State<PgPool>,
    Path(phone): Path<String>,
) -> impl IntoResponse {
    match find_user_by_phone(pool, phone.clone()).await {
        Ok(Some(user)) => (StatusCode::OK, Json(user)).into_response(),

        Ok(None) => {
            let msg = format!("No user with phone: {phone}");
            (StatusCode::NOT_FOUND, msg).into_response()
        }

        Err(err) => {
            eprintln!("Failed to get user: {err}");

            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load user").into_response()
        }
    }
}

pub async fn create_user_handler(
    State(pool): State<PgPool>,
    Json(user): Json<RegisterRequest>,
) -> impl IntoResponse {
    match create_user(pool, user).await {
        Ok(new_user) => {
            let msg = format!("User created with id: {}", new_user.id);

            (StatusCode::CREATED, msg).into_response()
        }

        Err(err) if err == "User with this phone already exists" => {
            (StatusCode::CONFLICT, err).into_response()
        }

        Err(err) => {
            eprintln!("Failed to create user: {err}");

            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user").into_response()
        }
    }
}

pub async fn delete_user_handler_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match delete_user_by_id(pool, id).await {
        Ok(()) => (StatusCode::OK, "User and all user tasks deleted").into_response(),

        Err(err) => {
            eprintln!("Failed to delete user: {err}");

            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete user").into_response()
        }
    }
}

pub async fn delete_user_handler_by_phone(
    State(pool): State<PgPool>,
    Path(phone): Path<String>,
) -> impl IntoResponse {
    match delete_user_by_phone(pool, phone).await {
        Ok(()) => (StatusCode::OK, "User and all user tasks deleted").into_response(),

        Err(err) => {
            eprintln!("Failed to delete user: {err}");

            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete user").into_response()
        }
    }
}

pub async fn login_handler(
    State(pool): State<PgPool>,
    Json(login_data): Json<LoginRequest>,
) -> impl IntoResponse {
    let phone = login_data.phone.clone();

    let is_password_valid = match check_password_hash(pool.clone(), login_data).await {
        Ok(is_valid) => is_valid,

        Err(err) => {
            eprintln!("Failed to check password: {err}");

            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to login")
                .into_response();
        }
    };

    if !is_password_valid {
        return (
            StatusCode::UNAUTHORIZED,
            "Invalid phone or password",
        )
            .into_response();
    }

    let user = match find_user_by_phone(pool, phone).await {
        Ok(Some(user)) => user,

        Ok(None) => {
            return (
                StatusCode::UNAUTHORIZED,
                "Invalid phone or password",
            )
                .into_response();
        }

        Err(err) => {
            eprintln!("Failed to find user after password check: {err}");

            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to login")
                .into_response();
        }
    };

    let jwt_secret = match std::env::var("JWT_SECRET") {
        Ok(secret) => secret,

        Err(_) => {
            eprintln!("JWT_SECRET is not set");

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server configuration error",
            )
                .into_response();
        }
    };

    let token = match create_token(user.id, user.role, &jwt_secret) {
        Ok(token) => token,

        Err(err) => {
            eprintln!("Failed to create token: {err}");

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create token",
            )
                .into_response();
        }
    };

    (
        StatusCode::OK, 
        Json(json!({ "token": token }))
        .into_response()
    )
        .into_response()
}