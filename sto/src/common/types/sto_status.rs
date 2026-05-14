use crate::common::*;

/// STO 모집/발행 상태.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum,
    Translate,
)]
pub enum StoStatus {
    #[default]
    #[translate(en = "—", ko = "—")]
    Unknown,
    /// 증권신고서 제출 (모집 진행)
    #[translate(en = "Open", ko = "공모 진행")]
    Filed,
    /// 발행 완료 (모집 완료)
    #[translate(en = "Issued", ko = "발행 완료")]
    Issued,
    /// 철회
    #[translate(en = "Withdrawn", ko = "철회")]
    Withdrawn,
    /// 청산 완료
    #[translate(en = "Liquidated", ko = "청산 완료")]
    Liquidated,
}

impl StoStatus {
    /// 상태에 매핑되는 텍스트 색상 클래스 (테일윈드 토큰).
    pub fn color_class(self) -> &'static str {
        use StoStatus::*;
        match self {
            Issued | Liquidated | Filed => "text-brand",
            Withdrawn => "text-warning",
            Unknown => "text-foreground-muted",
        }
    }
}
