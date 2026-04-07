use crate::common::*;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum,
)]
pub enum PasswordScheme {
    #[default]
    LegacySha3,
    BcryptV1,
}
