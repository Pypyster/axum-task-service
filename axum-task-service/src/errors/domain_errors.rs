use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("User name cannot be empty")]
    EmptyUserName,
    #[error("Phone cannot be empty")]
    EmptyPhone,
    #[error("Password must contain at least 8 characters")]
    PasswordTooShort,
    #[error("Invalid phone or password")]
    InvalidCredentials,
    #[error("Authentication required")]
    Unauthorized,
    #[error("User with this phone already exists")]
    PhoneAlreadyExists,

    #[error("Task name cannot be empty")]
    EmptyTaskName,
    #[error("Cannot reopen cancelled task")]
    ReopenCancelledTask,
    #[error("User can update only task status")]
    UserCanUpdateOnlyStatus,
}
