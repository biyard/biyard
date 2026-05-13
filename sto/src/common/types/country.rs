use crate::common::*;

/// 국가. 표시명/국기 이모지 변환은 view 단 헬퍼에서 처리.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum,
)]
pub enum Country {
    #[default]
    Unknown,
    Kr,
    Us,
    Sg,
    Eu,
    Other,
}
