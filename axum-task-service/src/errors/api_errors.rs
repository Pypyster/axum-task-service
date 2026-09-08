use super::service_errors::ServiceError;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub struct ApiError(pub ServiceError);

impl From<ServiceError> for ApiError {
    fn from(err: ServiceError) -> Self {
        ApiError(err)
    }
}

#[derive(Serialize)]
struct ErrorBody {
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self.0 {
            ServiceError::NotFound { resource, id } => (
                StatusCode::NOT_FOUND,
                format!("{resource} with id {id} not found"),
            ),
            ServiceError::Forbidden(reason) => (StatusCode::FORBIDDEN, reason.clone()),
            ServiceError::Domain(domain_err) => (StatusCode::BAD_REQUEST, domain_err.to_string()),
            ServiceError::Repo(db_err) => {
                tracing::error!("database error: {db_err}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".to_string(),
                )
            }
        };

        (status, Json(ErrorBody { message })).into_response()
    }
}
