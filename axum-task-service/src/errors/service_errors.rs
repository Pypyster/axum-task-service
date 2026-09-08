use super::domain_errors::DomainError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("{resource} with id {id} not found")]
    NotFound { resource: String, id: i32 },

    #[error("{resource} with phone {phone} not found")]
    NotFoundPhone { resource: String, phone: String },

    #[error("forbidden: {0}")]
    Forbidden(String),

    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error("database error: {0}")]
    Repo(#[from] sqlx::Error),
}
