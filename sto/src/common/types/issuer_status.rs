use crate::common::*;

/// 발행사 운영 상태. 시리즈/인가/위기 신호 등 자유 코멘트는 `Issuer.status_note` 에 둠.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum,
    Translate,
)]
pub enum IssuerStatus {
    #[default]
    #[translate(en = "Unknown", ko = "상태 미상")]
    Unknown,
    #[translate(en = "Operating", ko = "운영 중")]
    Operating,
    #[translate(en = "Wound down", ko = "사업 종료")]
    WoundDown,
}
