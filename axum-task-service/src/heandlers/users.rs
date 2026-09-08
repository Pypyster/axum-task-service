use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Serialize;

use crate::{
    errors::ApiError,
    handlers::routes::AppState,
    structs::user::{LoginRequest, RegisterRequest, User},
};

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub token: String,
    pub token_type: String,
}

pub async fn get_all_users_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<User>>, ApiError> {
    let users = state.user_service.get_all_users().await?;

    Ok(Json(users))
}

pub async fn get_user_handler_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<User>, ApiError> {
    let user = state.user_service.get_user_by_id(id).await?;

    Ok(Json(user))
}

pub async fn get_user_handler_by_phone(
    State(state): State<Arc<AppState>>,
    Path(phone): Path<String>,
) -> Result<Json<User>, ApiError> {
    let user = state.user_service.get_user_by_phone(phone).await?;

    Ok(Json(user))
}

pub async fn create_user_handler(
    State(state): State<Arc<AppState>>,
    Json(user): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<User>), ApiError> {
    let created_user = state.user_service.create_user(user).await?;

    Ok((StatusCode::CREATED, Json(created_user)))
}

pub async fn delete_user_handler_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, ApiError> {
    state.user_service.delete_user_by_id(id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(login_data): Json<LoginRequest>,
) -> Result<Json<TokenResponse>, ApiError> {
    let token = state.user_service.login(login_data).await?;

    Ok(Json(TokenResponse {
        token,
        token_type: "Bearer".to_string(),
    }))
}
