use crate::common::*;

#[derive(
    Debug, Clone, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum, PartialEq, Eq,
)]
pub enum Need {
    #[default]
    GeneralInquiry,
    TechnicalSupport,
    PartnershipCollaboration,
    InvestmentFunding,
    FeedbackSuggestions,
}
