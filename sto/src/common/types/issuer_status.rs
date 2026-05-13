use crate::common::*;

/// 발행사 운영 상태. 시리즈/인가/위기 신호 등 자유 코멘트는 `Issuer.status_note` 에 둠.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum,
)]
pub enum IssuerStatus {
    #[default]
    Unknown,
    Operating,
    WoundDown,
}
