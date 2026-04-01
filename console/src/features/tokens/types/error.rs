use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum TokenError {
    #[error("Token not found")]
    TokenNotFound,

    #[error("Insufficient tokens")]
    InsufficientTokens,

    #[error("Invalid token amount")]
    InvalidTokenAmount,

    #[error("Token already exists")]
    TokenAlreadyExists,

    #[error("Token balance not found")]
    TokenBalanceNotFound,
}
