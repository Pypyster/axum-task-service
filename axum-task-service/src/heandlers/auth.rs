use axum::Extension;
use axum::extract::{Request, State};
use axum::http::{StatusCode, header};
use axum::middleware::Next;
use axum::response::Response;
use chrono::{Duration, Utc};
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error as JwtError,
};
use sqlx::PgPool;

use crate::structs::claims::Claims;
use crate::structs::user_role::UserRole;

const TOKEN_LIFETIME_HOURS: i64 = 24;

pub fn create_token(
    user_id: i32, 
    role: UserRole, 
    jwt_secret: &str
) -> Result<String, JwtError> {
    let exp = (Utc::now() + Duration::hours(TOKEN_LIFETIME_HOURS)).timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        role,
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
}

pub fn decode_token(
    token: &str, 
    jwt_secret: &str
) -> Result<Claims, JwtError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

pub async fn auth_middleware(
    mut request: Request, 
    next: Next
) -> Result<Response, StatusCode> {
    let authorization = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    let token = match authorization {
        Some(value) => match value.strip_prefix("Bearer ") {
            Some(token) if !token.is_empty() => token,
            _ => return Err(StatusCode::UNAUTHORIZED),
        },
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let jwt_secret = match std::env::var("JWT_SECRET") {
        Ok(secret) => secret,
        Err(_) => {
            eprintln!("JWT_SECRET is not set");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let claims = match decode_token(token, &jwt_secret) {
        Ok(claims) => claims,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

pub async fn admin_request(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let claims = match req.extensions().get::<Claims>() {
        Some(claims) => claims,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    if claims.role != UserRole::Admin {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(req).await)
}
