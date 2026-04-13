use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum PointError {
    #[error("Point balance not found")]
    PointBalanceNotFound,

    #[error("Insufficient points")]
    InsufficientPoints,

    #[error("Invalid point amount")]
    InvalidPointAmount,

    #[error("Meta user not found")]
    MetaUserNotFound,

    #[error("Point aggregation not found")]
    PointAggregationNotFound,

    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),
}
