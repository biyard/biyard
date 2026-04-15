use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(api_doc_macros::ApiDocSchema)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct TransactPointsResponse {
    #[field_doc(en = "Unique identifier for this transaction.", ko = "트랜잭션 고유 식별자.")]
    pub transaction_id: String,
    #[field_doc(en = "Month the transaction was recorded in (YYYY-MM).", ko = "트랜잭션이 기록된 월 (YYYY-MM).")]
    pub month: String,
    #[field_doc(en = "User identifier involved in the transaction.", ko = "트랜잭션에 관련된 유저 식별자.")]
    pub meta_user_id: String,
    #[field_doc(en = "Type: Award, Deduct, Transfer, or Exchange.", ko = "유형: Award, Deduct, Transfer, Exchange.")]
    pub transaction_type: String,
    #[field_doc(en = "Point amount. Negative for outgoing.", ko = "포인트 수량. 차감/이체 시 음수.")]
    pub amount: i64,
}
