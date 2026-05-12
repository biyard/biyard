use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum Error {
    #[error("Unknown: {0}")]
    Unknown(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Unknown(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Unknown(s.to_string())
    }
}
