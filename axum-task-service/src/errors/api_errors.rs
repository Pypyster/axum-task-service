use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use super::{domain_errors::DomainError, service_errors::ServiceError};

pub struct ApiError(pub ServiceError);

impl From<ServiceError> for ApiError {
    fn from(error: ServiceError) -> Self {
        Self(error)
    }
}

#[derive(Debug, Serialize)]
struct ErrorBody {
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self.0 {
            ServiceError::NotFound { resource, id } => (
                StatusCode::NOT_FOUND,
                format!("{resource} with id {id} not found"),
            ),

            ServiceError::NotFoundPhone { resource, phone } => (
                StatusCode::NOT_FOUND,
                format!("{resource} with phone {phone} not found"),
            ),

            ServiceError::Forbidden(reason) => (StatusCode::FORBIDDEN, reason),

            ServiceError::Domain(domain_error) => {
                let status = domain_error_status(&domain_error);

                (status, domain_error.to_string())
            }

            ServiceError::Repo(db_error) => {
                tracing::error!("Database error: {db_error}");

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        (status, Json(ErrorBody { message })).into_response()
    }
}

fn domain_error_status(error: &DomainError) -> StatusCode {
    match error {
        DomainError::EmptyUserName
        | DomainError::EmptyPhone
        | DomainError::PasswordTooShort
        | DomainError::EmptyTaskName => StatusCode::BAD_REQUEST,

        DomainError::InvalidCredentials | DomainError::Unauthorized => StatusCode::UNAUTHORIZED,

        DomainError::PhoneAlreadyExists => StatusCode::CONFLICT,

        DomainError::ReopenCancelledTask | DomainError::UserCanUpdateOnlyStatus => {
            StatusCode::FORBIDDEN
        }
    }
}
