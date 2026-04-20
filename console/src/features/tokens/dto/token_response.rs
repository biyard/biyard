use crate::common::types::Partition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(api_doc_macros::ApiDocSchema)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct TokenResponse {
    /// Token identifier.
    #[field_doc(en = "Token identifier.", ko = "토큰 식별자.")]
    #[cfg_attr(feature = "server", schemars(with = "String"))]
    pub pk: Partition,
    /// Token name (e.g. "My Brand Token").
    #[field_doc(en = "Token name (e.g. \"My Brand Token\").", ko = "토큰 이름 (예: \"My Brand Token\").")]
    pub name: String,
    /// Token symbol (e.g. "MBT"). Immutable after deployment.
    #[field_doc(en = "Token symbol (e.g. \"MBT\"). Immutable after deployment.", ko = "토큰 심볼 (예: \"MBT\"). 배포 후 변경 불가.")]
    pub symbol: String,
    /// Current circulating supply in smallest token units.
    #[field_doc(en = "Current circulating supply in smallest token units.", ko = "현재 유통량 (최소 토큰 단위).")]
    pub circulating_supply: i64,
    /// Token description.
    #[field_doc(en = "Token description.", ko = "토큰 설명.")]
    pub description: Option<String>,
    /// Deployed ERC-20 contract address.
    #[field_doc(en = "Deployed ERC-20 contract address.", ko = "배포된 ERC-20 컨트랙트 주소.")]
    pub contract_address: Option<String>,
    /// Treasury contract address.
    #[field_doc(en = "Treasury contract address.", ko = "트레저리 컨트랙트 주소.")]
    pub treasury_contract_address: Option<String>,
    /// Multisig wallet address.
    #[field_doc(en = "Multisig wallet address.", ko = "멀티시그 지갑 주소.")]
    pub multisig_address: Option<String>,
    /// Stable token address used for treasury (e.g. USDT).
    #[field_doc(en = "Stable token address used for treasury (e.g. USDT).", ko = "트레저리에 사용되는 스테이블 토큰 주소 (예: USDT).")]
    pub stable_token_address: Option<String>,
    /// Blockchain chain ID (e.g. 1001 for Kaia Kairos).
    #[field_doc(en = "Blockchain chain ID (e.g. 1001 for Kaia Kairos).", ko = "블록체인 체인 ID (예: Kaia Kairos의 경우 1001).")]
    pub chain_id: Option<u64>,
    /// Transaction hash of the token deployment.
    #[field_doc(en = "Transaction hash of the token deployment.", ko = "토큰 배포 트랜잭션 해시.")]
    pub deployment_tx_hash: Option<String>,
    /// Transaction hash of the treasury deployment.
    #[field_doc(en = "Transaction hash of the treasury deployment.", ko = "트레저리 배포 트랜잭션 해시.")]
    pub treasury_deployment_tx_hash: Option<String>,
    /// Transaction hash of the multisig deployment.
    #[field_doc(en = "Transaction hash of the multisig deployment.", ko = "멀티시그 배포 트랜잭션 해시.")]
    pub multisig_deployment_tx_hash: Option<String>,
    /// Treasury reserve rate in basis points (e.g. 2000 = 20%).
    #[field_doc(en = "Treasury reserve rate in basis points (e.g. 2000 = 20%).", ko = "트레저리 준비율 (베이시스 포인트, 예: 2000 = 20%).")]
    pub treasury_reserve_bps: u64,
    /// Monthly token emission amount.
    #[field_doc(en = "Monthly token emission amount.", ko = "월간 토큰 발행량.")]
    pub monthly_emission: i64,
    /// Monthly emission decay rate in basis points (e.g. 500 = 5%).
    #[field_doc(en = "Monthly emission decay rate in basis points (e.g. 500 = 5%).", ko = "월간 발행량 감소율 (베이시스 포인트, 예: 500 = 5%).")]
    pub decay_rate_bps: u16,
    /// Token distribution slots with wallet addresses and basis point shares.
    #[field_doc(en = "Token distribution slots with wallet addresses and basis point shares.", ko = "토큰 분배 슬롯 (지갑 주소 및 베이시스 포인트 비율).")]
    pub distribution_slots: Vec<crate::features::tokens::DistributionSlotEntry>,
    /// Last month tokens were minted (YYYY-MM).
    #[field_doc(en = "Last month tokens were minted (YYYY-MM).", ko = "마지막 토큰 발행 월 (YYYY-MM).")]
    #[serde(default)]
    pub last_minted_month: Option<String>,
    /// Whether a deployment is currently in progress.
    #[field_doc(en = "Whether a deployment is currently in progress.", ko = "배포가 현재 진행 중인지 여부.")]
    #[serde(default)]
    pub deploying: bool,
    /// First month of token emission (YYYY-MM).
    #[field_doc(en = "First month of token emission (YYYY-MM).", ko = "토큰 발행 시작 월 (YYYY-MM).")]
    #[serde(default)]
    pub start_month: Option<String>,
    /// Creation timestamp (Unix epoch seconds).
    #[field_doc(en = "Creation timestamp (Unix epoch seconds).", ko = "생성 타임스탬프 (Unix epoch 초).")]
    pub created_at: i64,
    /// Last update timestamp (Unix epoch seconds).
    #[field_doc(en = "Last update timestamp (Unix epoch seconds).", ko = "마지막 업데이트 타임스탬프 (Unix epoch 초).")]
    pub updated_at: i64,
}
