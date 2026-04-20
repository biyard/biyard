use crate::common::types::Partition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[derive(api_doc_macros::ApiDocSchema)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct TokenBalanceResponse {
    /// Project identifier.
    #[field_doc(en = "Project identifier.", ko = "프로젝트 식별자.")]
    #[cfg_attr(feature = "server", schemars(with = "String"))]
    pub project_id: Partition,
    /// User identifier.
    #[field_doc(en = "User identifier.", ko = "유저 식별자.")]
    pub meta_user_id: String,
    /// Token balance in smallest units.
    #[field_doc(en = "Token balance in smallest units.", ko = "토큰 잔액 (최소 단위).")]
    pub balance: i64,
    /// Latest mint transaction hash.
    #[field_doc(en = "Latest mint transaction hash.", ko = "최근 민트 트랜잭션 해시.")]
    pub tx_hash: Option<String>,
    /// Creation timestamp (Unix epoch seconds).
    #[field_doc(en = "Creation timestamp (Unix epoch seconds).", ko = "생성 타임스탬프 (Unix epoch 초).")]
    pub created_at: i64,
    /// Last update timestamp (Unix epoch seconds).
    #[field_doc(en = "Last update timestamp (Unix epoch seconds).", ko = "마지막 업데이트 타임스탬프 (Unix epoch 초).")]
    pub updated_at: i64,
}
