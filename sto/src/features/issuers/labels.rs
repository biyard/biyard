use crate::common::IssuerStatus;
use crate::features::issuers::IssuersTranslate;

/// 발행사 상태 → 표시 라벨 (i18n).
pub fn issuer_status_label(s: IssuerStatus, t: &IssuersTranslate) -> &'static str {
    use IssuerStatus::*;
    match s {
        Operating => t.status_operating,
        WoundDown => t.status_wound_down,
        Unknown => t.status_unknown,
    }
}
