use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum ProjectError {
    #[error("Brand not found")]
    ProjectNotFound,

    #[error("Brand access denied")]
    ProjectAccessDenied,

    #[error("Invalid exchange rate")]
    InvalidExchangeRate,

    #[error("Insufficient supply")]
    InsufficientSupply,
}
