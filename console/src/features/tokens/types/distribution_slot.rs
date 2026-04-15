use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[derive(api_doc_macros::ApiDocSchema)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct DistributionSlotEntry {
    /// Wallet address to receive tokens.
    #[field_doc(en = "Wallet address to receive tokens.", ko = "토큰을 수령할 지갑 주소.")]
    pub wallet: String,
    /// Share in basis points (e.g. 5000 = 50%).
    #[field_doc(en = "Share in basis points (e.g. 5000 = 50%).", ko = "분배 비율 (베이시스 포인트, 예: 5000 = 50%).")]
    pub bps: u16,
}
