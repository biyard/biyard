use crate::common::*;

#[derive(Debug, Clone, PartialEq, Eq, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum)]
pub enum EntityType {
    #[default]
    None,

    // Timestamp
    TS(String),

    // Account feature
    Account,

    // Session feature
    Session,

    // Credential feature
    Credential,

    // Project feature
    Project,

    // Point feature
    Month(String),
    PointTransaction(String),
    Token,
    TokenBalance,
    MonthlyPointAggregation,
    User(String),

    // Update
    Update,

    // Contact
    Contact,
}
