use crate::common::*;

/// STO 모집/발행 상태.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum,
)]
pub enum StoStatus {
    #[default]
    Unknown,
    /// 증권신고서 제출 (모집 진행)
    Filed,
    /// 발행 완료 (모집 완료)
    Issued,
    /// 철회
    Withdrawn,
    /// 청산 완료
    Liquidated,
}
