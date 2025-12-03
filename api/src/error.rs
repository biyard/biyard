use tmpl_renderer::RenderError;

use crate::*;

#[derive(Debug, thiserror::Error, RestError, aide::OperationIo)]
pub enum Error {
    #[error("Web error: {0}")]
    #[rest_error(code = 0)]
    WebError(#[from] RenderError),

    #[error("Unknown")]
    #[rest_error(code = 1)]
    Unknown(String),
    #[error("Internal server error: {0}")]
    #[rest_error(status = 500)]
    InternalServerError(String),
    #[error("Forbidden")]
    #[rest_error(status = 403)]
    Forbidden,

    // Session/Authentication error (50 ~ 99)
    #[error("Session error")]
    #[rest_error(code = 50)]
    SessionError(#[from] tower_sessions::session::Error),
    #[error("No session found")]
    NoSessionFound,
    #[error("Unauthorized")]
    #[rest_error(status = 401)]
    Unauthorized,

    // DynamoDB Common Errors (100-199)
    #[error("Not Found")]
    #[rest_error(code = 100)]
    DynamoError(#[from] aws_sdk_dynamodb::Error),
    #[error("Validation error: {0}")]
    ValidationError(#[from] validator::ValidationError),
    #[error("Validation errors: {0}")]
    #[rest_error(code = 101, status = 400)]
    ValidationErrors(#[from] validator::ValidationErrors),
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
    #[error("SerdeJson error: {0}")]
    SerdeJson(#[from] serde_json::Error),

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

    // Project errors (500-599)
    #[error("Project not found")]
    #[rest_error(code = 500, status = 404)]
    ProjectNotFound,
    #[error("Project access denied")]
    #[rest_error(code = 501, status = 403)]
    ProjectAccessDenied,
    #[error("Invalid exchange rate")]
    #[rest_error(code = 502, status = 400)]
    InvalidExchangeRate,
    #[error("Insufficient supply")]
    #[rest_error(code = 503, status = 400)]
    InsufficientSupply,

    // Point errors (600-699)
    #[error("Point balance not found")]
    #[rest_error(code = 600, status = 404)]
    PointBalanceNotFound,
    #[error("Insufficient points")]
    #[rest_error(code = 601, status = 400)]
    InsufficientPoints,
    #[error("Invalid point amount")]
    #[rest_error(code = 602, status = 400)]
    InvalidPointAmount,
    #[error("Meta user not found")]
    #[rest_error(code = 603, status = 404)]
    MetaUserNotFound,

    // Token errors (700-799)
    #[error("Token not found")]
    #[rest_error(code = 700, status = 404)]
    TokenNotFound,
    #[error("Insufficient tokens")]
    #[rest_error(code = 701, status = 400)]
    InsufficientTokens,
    #[error("Invalid token amount")]
    #[rest_error(code = 702, status = 400)]
    InvalidTokenAmount,
    #[error("Token already exists")]
    #[rest_error(code = 703, status = 400)]
    TokenAlreadyExists,
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Unknown(s)
    }
}
