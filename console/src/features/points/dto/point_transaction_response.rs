use crate::common::types::Partition;
use crate::features::points::TransactionType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[derive(api_doc_macros::ApiDocSchema)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct PointTransactionResponse {
    /// Project identifier.
    #[field_doc(en = "Project identifier.", ko = "프로젝트 식별자.")]
    #[cfg_attr(feature = "server", schemars(with = "String"))]
    pub project_id: Partition,
    /// User identifier involved in this transaction.
    #[field_doc(en = "User identifier involved in this transaction.", ko = "해당 트랜잭션에 관련된 유저 식별자.")]
    pub meta_user_id: String,
    /// Month in YYYY-MM format.
    #[field_doc(en = "Month in YYYY-MM format.", ko = "월 (YYYY-MM 형식).")]
    pub month: String,
    /// Transaction type: Award, Deduct, Transfer, or Exchange.
    #[field_doc(en = "Transaction type: Award, Deduct, Transfer, or Exchange.", ko = "트랜잭션 유형: 지급, 차감, 이전, 교환.")]
    #[cfg_attr(feature = "server", schemars(with = "String"))]
    pub transaction_type: TransactionType,
    /// Point amount. Negative for outgoing transfers/deductions.
    #[field_doc(en = "Point amount. Negative for outgoing transfers/deductions.", ko = "포인트 수량. 출금/차감 시 음수.")]
    pub amount: i64,
    /// Counterparty user ID (for Transfer type).
    #[field_doc(en = "Counterparty user ID (for Transfer type).", ko = "이전 대상 유저 ID (Transfer 유형 시).")]
    pub target_user_id: Option<String>,
    /// Human-readable memo.
    #[field_doc(en = "Human-readable memo.", ko = "사람이 읽을 수 있는 메모.")]
    pub description: Option<String>,
    /// Creation timestamp (Unix epoch seconds).
    #[field_doc(en = "Creation timestamp (Unix epoch seconds).", ko = "생성 타임스탬프 (Unix epoch 초).")]
    pub created_at: i64,
}
