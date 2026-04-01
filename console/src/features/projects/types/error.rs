use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum ProjectError {
    #[error("Project not found")]
    ProjectNotFound,

    #[error("Project access denied")]
    ProjectAccessDenied,

    #[error("Invalid exchange rate")]
    InvalidExchangeRate,

    #[error("Insufficient supply")]
    InsufficientSupply,
}
