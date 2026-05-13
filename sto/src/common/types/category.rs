use crate::common::*;

/// 자산 카테고리. DB 에는 영문 UPPER_SNAKE 로 저장.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, SerializeDisplay, DeserializeFromStr, Default,
    DynamoEnum,
)]
pub enum Category {
    #[default]
    Unknown,
    Music,
    Art,
    RealEstate,
    Livestock,
}
