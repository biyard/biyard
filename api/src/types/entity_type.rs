use crate::*;
use serde_with::{DeserializeFromStr, SerializeDisplay};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    SerializeDisplay,
    DeserializeFromStr,
    Default,
    DynamoEnum,
    JsonSchema,
    OperationIo,
)]
pub enum EntityType {
    #[default]
    None,

    // Timestamp
    TS(i64), // TS#<unix_timestamp>

    // Account feature
    Account,

    // Session feature
    Session,

    // Credential feature
    Credential,

    // Project feature
    Project,

    // Point feature
    #[schemars(description = "Sort key: MONTH#<YYYY-MM>")]
    Month(String),
    #[schemars(description = "Sort key: PointTransaction#<transaction_id>")]
    PointTransaction(String),
    Token,
    TokenBalance,
    MonthlyPointAggregation,

    // Update
    Update,

    // Contact
    Contact,
}
