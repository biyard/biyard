use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum CredentialError {
    #[error("Credential not found")]
    CredentialNotFound,

    #[error("Invalid API key")]
    InvalidApiKey,

    #[error("Credential limit exceeded")]
    CredentialLimitExceeded,
}
