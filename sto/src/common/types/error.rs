use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum Error {
    #[error("Unknown: {0}")]
    Unknown(String),

    #[error("Invalid partition key: {0}")]
    InvalidPartitionKey(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Forbidden")]
    Forbidden,

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Invalid bookmark")]
    InvalidBookmark,
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

#[cfg(feature = "server")]
impl<E: std::fmt::Debug + 'static> From<aws_sdk_dynamodb::error::SdkError<E>> for Error {
    fn from(e: aws_sdk_dynamodb::error::SdkError<E>) -> Self {
        Error::InternalServerError(format!("dynamo: {e:?}"))
    }
}

#[cfg(feature = "server")]
impl From<serde_dynamo::Error> for Error {
    fn from(e: serde_dynamo::Error) -> Self {
        Error::InternalServerError(format!("serde_dynamo: {e}"))
    }
}

#[cfg(feature = "server")]
impl From<aws_sdk_dynamodb::Error> for Error {
    fn from(e: aws_sdk_dynamodb::Error) -> Self {
        Error::InternalServerError(format!("dynamo: {e}"))
    }
}

#[cfg(feature = "server")]
impl From<aws_sdk_dynamodb::error::BuildError> for Error {
    fn from(e: aws_sdk_dynamodb::error::BuildError) -> Self {
        Error::InternalServerError(format!("dynamo build: {e}"))
    }
}

impl From<base64::DecodeError> for Error {
    fn from(_: base64::DecodeError) -> Self {
        Error::InvalidBookmark
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::InternalServerError(format!("serde_json: {e}"))
    }
}
