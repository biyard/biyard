use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum AccountError {
    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Account not found")]
    AccountNotFound,
}
