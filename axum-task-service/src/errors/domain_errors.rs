use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Task name can't be empy")]
    EmptyName,
    #[error("Can not reopen cancelled task")]
    ReopenCancelledTask,
}
