use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum AccountError {
    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Account not found")]
    AccountNotFound,

    #[error("Weak password: {0}")]
    WeakPassword(String),

    #[error("Name cannot be empty")]
    InvalidName,
}
