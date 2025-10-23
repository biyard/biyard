use crate::*;

#[derive(Debug, thiserror::Error, RestError, aide::OperationIo)]
pub enum Error {
    #[error("Unknown")]
    #[rest_error(code = 1)]
    Unknown(String),
    #[error("Internal server error: {0}")]
    #[rest_error(status = 500)]
    InternalServerError(String),

    // Session error (50 ~ 99)
    #[error("Session error")]
    #[rest_error(code = 50)]
    SessionError(#[from] tower_sessions::session::Error),
    #[error("No session found")]
    NoSessionFound,

    // DynamoDB Common Errors (100-199)
    #[error("Not Found")]
    #[rest_error(code = 100)]
    DynamoError(#[from] aws_sdk_dynamodb::Error),
    #[error("Validation error: {0}")]
    ValidationError(#[from] validator::ValidationError),
    #[error("SerdeDynamo error: {0}")]
    #[rest_error(status = 500)]
    DynamoEncodingError(#[from] serde_dynamo::Error),
    #[error("Invalid partition key: {0}")]
    InvalidPartitionKey(String),
    #[error("Bookmark is invalid")]
    InvalidBookmark,

    // Encoding errors (200-299)
    #[error("Base64 decode error: {0}")]
    #[rest_error(code = 200)]
    Base64DecodingError(#[from] base64::DecodeError),
    #[error("Decoding error: {0}")]
    Utf8Decoding(#[from] std::str::Utf8Error),

    // Account errors (300-399)
    #[error("Email already exists")]
    #[rest_error(code = 300, status = 400)]
    EmailAlreadyExists,
    #[error("Invalid credentials")]
    #[rest_error(code = 301, status = 401)]
    InvalidCredentials,
    #[error("Account not found")]
    #[rest_error(code = 302, status = 404)]
    AccountNotFound,

    // Credential errors (400-499)
    #[error("Credential not found")]
    #[rest_error(code = 400, status = 404)]
    CredentialNotFound,
    #[error("Invalid API key")]
    #[rest_error(code = 401, status = 401)]
    InvalidApiKey,
    #[error("Credential limit exceeded")]
    #[rest_error(code = 402, status = 400)]
    CredentialLimitExceeded,
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Unknown(s)
    }
}
