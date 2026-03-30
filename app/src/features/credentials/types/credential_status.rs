use crate::common::*;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq,
    serde_repr::Serialize_repr, serde_repr::Deserialize_repr,
    Default, DynamoEnum,
)]
#[repr(u8)]
pub enum CredentialStatus {
    #[default]
    Active = 1,
    Revoked = 2,
}
