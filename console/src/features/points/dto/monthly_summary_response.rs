use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(api_doc_macros::ApiDocSchema)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct MonthlySummaryItem {
    /// Month in YYYY-MM format.
    #[field_doc(en = "Month in YYYY-MM format.", ko = "월 (YYYY-MM 형식).")]
    pub month: String,
    /// Total points earned by the user this month.
    #[field_doc(en = "Total points earned by the user this month.", ko = "해당 월에 유저가 획득한 총 포인트.")]
    pub total_earned: i64,
    /// Total points spent by the user this month.
    #[field_doc(en = "Total points spent by the user this month.", ko = "해당 월에 유저가 사용한 총 포인트.")]
    pub total_spent: i64,
    /// Net point balance for this month.
    #[field_doc(en = "Net point balance for this month.", ko = "해당 월 순 포인트 잔액.")]
    pub balance: i64,
    /// Total points across all users in the project for this month.
    #[field_doc(en = "Total points across all users in the project for this month.", ko = "해당 월 프로젝트 내 전체 유저의 총 포인트.")]
    pub project_total_points: i64,
    /// Monthly token supply configured for the project.
    #[field_doc(en = "Monthly token supply configured for the project.", ko = "프로젝트에 설정된 월간 토큰 공급량.")]
    pub monthly_token_supply: i64,
    /// Whether the user has already exchanged points for tokens this month.
    #[field_doc(en = "Whether the user has already exchanged points for tokens this month.", ko = "해당 월에 유저가 이미 포인트를 토큰으로 교환했는지 여부.")]
    pub exchanged: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(api_doc_macros::ApiDocSchema)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct MonthlySummariesResponse {
    /// List of monthly summaries, ordered chronologically.
    #[field_doc(en = "List of monthly summaries, ordered chronologically.", ko = "시간순으로 정렬된 월별 요약 목록.")]
    pub months: Vec<MonthlySummaryItem>,
}
