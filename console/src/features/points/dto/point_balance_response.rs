use crate::common::types::Partition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[derive(api_doc_macros::ApiDocSchema)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct PointBalanceResponse {
    #[field_doc(en = "Project identifier.", ko = "프로젝트 식별자.")]
    #[cfg_attr(feature = "server", schemars(with = "String"))]
    pub project_id: Partition,
    #[field_doc(en = "User identifier.", ko = "유저 식별자.")]
    pub meta_user_id: String,
    #[field_doc(en = "Month in YYYY-MM format.", ko = "월 (YYYY-MM 형식).")]
    pub month: String,
    #[field_doc(en = "Current point balance.", ko = "현재 포인트 잔액.")]
    pub balance: i64,
    #[field_doc(en = "Total points earned this month.", ko = "이번 달 총 획득 포인트.")]
    pub total_earned: i64,
    #[field_doc(en = "Total points spent this month.", ko = "이번 달 총 사용 포인트.")]
    pub total_spent: i64,
    #[field_doc(en = "Last update timestamp (Unix epoch seconds).", ko = "마지막 업데이트 타임스탬프 (Unix epoch 초).")]
    pub updated_at: i64,
    #[field_doc(en = "Total points across all users in the project.", ko = "프로젝트 전체 유저의 총 포인트.")]
    #[serde(default)]
    pub project_total_points: i64,
    #[field_doc(en = "Monthly token supply for the project.", ko = "프로젝트의 월간 토큰 공급량.")]
    #[serde(default)]
    pub monthly_token_supply: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointBalancesResponse {
    pub balances: Vec<PointBalanceResponse>,
    pub total_balance: i64,
}
