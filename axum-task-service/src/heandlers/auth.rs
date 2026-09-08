use std::sync::Arc;

use axum::{
    extract::{Request, State}, http::{
        StatusCode, header,
    }, middleware::Next, response::Response,
};
use jsonwebtoken::{
    decode,
    Algorithm,
    DecodingKey,
    Validation,
    errors::Error as JwtError,
};

use crate::{handlers::routes::AppState, structs::{
    claims::Claims,
    user_role::UserRole,
}};

fn decode_token(
    token: &str,
    jwt_secret: &str,
) -> Result<Claims, JwtError> {
    let validation = Validation::new(Algorithm::HS256);

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &validation,
    )?;

    Ok(token_data.claims)
}

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
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

    let claims = match decode_token(token, state.jwt_secret.as_ref()) {
        Ok(claims) => claims,

        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

pub async fn admin_request(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let claims = match request.extensions().get::<Claims>() {
        Some(claims) => claims,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    if claims.role != UserRole::Admin {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(request).await)
}