use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[derive(api_doc_macros::ApiDocSchema)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct MonthlyPointAggregationResponse {
    #[field_doc(en = "Month in YYYY-MM format.", ko = "월 (YYYY-MM 형식).")]
    pub date: String,
    #[field_doc(en = "Total points in circulation this month.", ko = "이번 달 유통 중인 총 포인트.")]
    pub supplied_points: i64,
    #[field_doc(en = "Total points transferred between users.", ko = "유저 간 이체된 총 포인트.")]
    pub traded_points: i64,
    #[field_doc(en = "Total points awarded to users.", ko = "유저에게 지급된 총 포인트.")]
    pub awarded_points: i64,
    #[field_doc(en = "Total points deducted from users.", ko = "유저에게서 차감된 총 포인트.")]
    pub deducted_points: i64,
    #[field_doc(en = "Total points exchanged for tokens.", ko = "토큰으로 교환된 총 포인트.")]
    pub exchanged_points: i64,
}

impl MonthlyPointAggregationResponse {
    /// Zero-filled aggregation for a given month. Used when no aggregation
    /// row exists yet (fresh brand) so the API can return `200 OK` instead
    /// of `404`, keeping "no activity" out of the browser console error log.
    pub fn empty(date: String) -> Self {
        Self {
            date,
            ..Self::default()
        }
    }
}
