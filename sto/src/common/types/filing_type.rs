use crate::common::*;

/// 공시 문서 유형. DART 코드를 기반으로 하되, 누락 가능성을 대비해 `Other` fallback 포함.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum,
)]
pub enum FilingType {
    #[default]
    Unknown,
    /// 증권신고서 (투자계약증권)
    SecuritiesRegistration,
    /// 정정증권신고서
    Corrected,
    /// 투자설명서
    Prospectus,
    /// 증권발행실적보고서
    IssuanceReport,
    /// 정기보고 (사업보고서 등)
    Periodic,
    /// 주요사항보고서
    Material,
    /// 분류 외
    Other,
}
