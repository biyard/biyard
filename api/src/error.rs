use crate::*;

#[derive(Debug, thiserror::Error, RestError, aide::OperationIo)]
pub enum Error {
    #[error("Unknown")]
    #[rest_error(code = 1)]
    Unknown(String),
    #[error("Internal server error: {0}")]
    #[rest_error(status = 500)]
    InternalServerError(String),

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
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Unknown(s)
    }
}
