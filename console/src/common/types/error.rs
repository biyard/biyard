use serde::{Deserialize, Serialize};

use crate::features::accounts::AccountError;
use crate::features::credentials::CredentialError;
use crate::features::enterprises::EnterpriseError;
use crate::features::points::PointError;
use crate::features::projects::ProjectError;
use crate::features::tokens::TokenError;

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum Error {
    #[error("Unknown: {0}")]
    Unknown(String),

    #[error("Invalid partition key: {0}")]
    InvalidPartitionKey(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("No session found")]
    NoSessionFound,

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Forbidden")]
    Forbidden,

    #[error("Duplicate: {0}")]
    Duplicate(String),

    #[error("Invalid bookmark")]
    InvalidBookmark,

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error(transparent)]
    Account(#[from] AccountError),

    #[error(transparent)]
    Credential(#[from] CredentialError),

    #[error(transparent)]
    Enterprise(#[from] EnterpriseError),

    #[error(transparent)]
    Project(#[from] ProjectError),

    #[error(transparent)]
    Point(#[from] PointError),

    #[error(transparent)]
    Token(#[from] TokenError),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Unknown(s)
    }
}

impl From<dioxus::prelude::ServerFnError> for Error {
    fn from(e: dioxus::prelude::ServerFnError) -> Self {
        Error::Unknown(format!("Server function error: {}", e))
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for Error {
    fn into_response(self) -> dioxus::fullstack::axum::response::Response {
        use dioxus::fullstack::AsStatusCode;
        use dioxus::fullstack::axum::http::StatusCode;
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
            Error::Unauthorized | Error::NoSessionFound => StatusCode::UNAUTHORIZED,
            Error::Forbidden => StatusCode::FORBIDDEN,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::BadRequest(_)
            | Error::InvalidPartitionKey(_)
            | Error::InvalidBookmark
            | Error::Duplicate(_)
            | Error::ValidationError(_) => StatusCode::BAD_REQUEST,
            Error::Account(e) => match e {
                AccountError::InvalidCredentials => StatusCode::UNAUTHORIZED,
                AccountError::AccountNotFound => StatusCode::NOT_FOUND,
                AccountError::EmailAlreadyExists
                | AccountError::WeakPassword(_)
                | AccountError::InvalidName => StatusCode::BAD_REQUEST,
            },
            Error::Credential(e) => match e {
                CredentialError::InvalidApiKey => StatusCode::UNAUTHORIZED,
                CredentialError::CredentialNotFound => StatusCode::NOT_FOUND,
                CredentialError::CredentialLimitExceeded => StatusCode::BAD_REQUEST,
            },
            Error::Enterprise(e) => match e {
                EnterpriseError::EnterpriseNotFound
                | EnterpriseError::InvitationNotFound
                | EnterpriseError::MemberNotFound => StatusCode::NOT_FOUND,
                EnterpriseError::EnterpriseAccessDenied => StatusCode::FORBIDDEN,
                EnterpriseError::InvitationExpired
                | EnterpriseError::InvitationAlreadyAccepted
                | EnterpriseError::InvitationRevoked
                | EnterpriseError::LastOwnerCannotLeave
                | EnterpriseError::LastOwnerCannotDemote
                | EnterpriseError::AccountAlreadyInEnterprise
                | EnterpriseError::InvalidEnterpriseName
                | EnterpriseError::InvalidEnterpriseSlug => StatusCode::BAD_REQUEST,
            },
            Error::Project(e) => match e {
                ProjectError::ProjectNotFound => StatusCode::NOT_FOUND,
                ProjectError::ProjectAccessDenied => StatusCode::FORBIDDEN,
                ProjectError::InvalidExchangeRate | ProjectError::InsufficientSupply => {
                    StatusCode::BAD_REQUEST
                }
            },
            Error::Point(e) => match e {
                PointError::PointBalanceNotFound
                | PointError::MetaUserNotFound
                | PointError::PointAggregationNotFound => StatusCode::NOT_FOUND,
                PointError::InsufficientPoints | PointError::InvalidPointAmount => {
                    StatusCode::BAD_REQUEST
                }
            },
            Error::Token(e) => match e {
                TokenError::TokenNotFound
                | TokenError::TokenBalanceNotFound
                | TokenError::NotDeployed => StatusCode::NOT_FOUND,
                TokenError::InsufficientTokens
                | TokenError::InvalidTokenAmount
                | TokenError::TokenAlreadyExists
                | TokenError::AlreadyDeployed => StatusCode::BAD_REQUEST,
                TokenError::DeployFailed(_) | TokenError::MintFailed(_) => {
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            },
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "server")]
impl From<aws_sdk_dynamodb::Error> for Error {
    fn from(e: aws_sdk_dynamodb::Error) -> Self {
        Error::InternalServerError(format!("DynamoDB error: {}", e))
    }
}

#[cfg(feature = "server")]
impl From<serde_dynamo::Error> for Error {
    fn from(e: serde_dynamo::Error) -> Self {
        Error::InternalServerError(format!("SerdeDynamo error: {}", e))
    }
}

#[cfg(feature = "server")]
impl From<tower_sessions::session::Error> for Error {
    fn from(e: tower_sessions::session::Error) -> Self {
        Error::InternalServerError(format!("Session error: {}", e))
    }
}

#[cfg(feature = "server")]
impl From<validator::ValidationErrors> for Error {
    fn from(e: validator::ValidationErrors) -> Self {
        Error::ValidationError(e.to_string())
    }
}

impl From<base64::DecodeError> for Error {
    fn from(e: base64::DecodeError) -> Self {
        Error::InternalServerError(format!("Base64 decode error: {}", e))
    }
}
