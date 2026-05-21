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

impl From<dioxus::prelude::ServerFnError> for Error {
    fn from(e: dioxus::prelude::ServerFnError) -> Self {
        Error::Unknown(format!("Server function error: {e}"))
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for Error {
    fn into_response(self) -> dioxus::fullstack::axum::response::Response {
        use dioxus::fullstack::AsStatusCode;
        use dioxus::fullstack::axum::response::IntoResponse;

        let status = self.as_status_code();
        (status, self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for Error {
    fn as_status_code(&self) -> dioxus::fullstack::axum::http::StatusCode {
        use dioxus::fullstack::axum::http::StatusCode;
        match self {
            Error::Forbidden => StatusCode::FORBIDDEN,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::BadRequest(_)
            | Error::InvalidPartitionKey(_)
            | Error::InvalidBookmark => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
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
