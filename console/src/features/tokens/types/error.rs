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

    #[error("Token already deployed on-chain")]
    AlreadyDeployed,

    #[error("On-chain deployment failed: {0}")]
    DeployFailed(String),

    #[error("On-chain mint failed: {0}")]
    MintFailed(String),

    #[error("Token not deployed on-chain")]
    NotDeployed,

    #[error("Treasury deposit failed: {0}")]
    DepositFailed(String),
}
