use serde::{Deserialize, Serialize};

use crate::common::rpc::WasmTreasuryStatus;
use crate::features::tokens::TokenResponse;

/// Live on-chain snapshot of a brand project's treasury.
///
/// All values come from an RPC read against the treasury and brand
/// token contracts. When `deployed = false` every other field is
/// zero / empty and the frontend should treat the whole payload as
/// unavailable (the brand still needs to deploy its contracts).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[derive(api_doc_macros::ApiDocSchema)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct TreasuryStatusResponse {
    /// Whether the treasury + brand token contracts have been deployed
    /// for this project.
    #[field_doc(en = "Whether the treasury and brand token contracts have been deployed for this project.", ko = "해당 프로젝트의 트레저리 및 브랜드 토큰 컨트랙트 배포 여부.")]
    pub deployed: bool,

    /// Chain id the contracts live on (e.g. 1001 for Kaia Kairos).
    #[field_doc(en = "Chain ID the contracts live on (e.g. 1001 for Kaia Kairos).", ko = "컨트랙트가 배포된 블록체인 체인 ID (예: Kaia Kairos의 경우 1001).")]
    pub chain_id: Option<u64>,

    /// Treasury contract address (hex string, checksummed/lowercased
    /// from the provider).
    #[field_doc(en = "Treasury contract address (hex string).", ko = "트레저리 컨트랙트 주소 (16진수 문자열).")]
    pub treasury_contract_address: Option<String>,
    /// Brand token contract address.
    #[field_doc(en = "Brand token contract address.", ko = "브랜드 토큰 컨트랙트 주소.")]
    pub brand_token_address: Option<String>,

    /// Raw treasury balance in stable token units (as string to avoid
    /// JSON 53-bit precision loss on large u128 values).
    #[field_doc(en = "Raw treasury balance in stable token units (string to avoid JSON precision loss).", ko = "스테이블 토큰 단위의 트레저리 원시 잔액 (JSON 정밀도 손실 방지를 위해 문자열).")]
    pub treasury_balance_raw: String,
    /// Stable token decimals (e.g. 6 for USDT).
    #[field_doc(en = "Stable token decimals (e.g. 6 for USDT).", ko = "스테이블 토큰 소수점 자릿수 (예: USDT의 경우 6).")]
    pub stable_decimals: u8,
    /// Stable token symbol (e.g. "USDT").
    #[field_doc(en = "Stable token symbol (e.g. \"USDT\").", ko = "스테이블 토큰 심볼 (예: \"USDT\").")]
    pub stable_symbol: String,
    /// Whether the stable token has a public faucet mint (e.g. BUSDT).
    /// When true, the console can mint + deposit for demo purposes.
    #[field_doc(en = "Whether the stable token has a public faucet mint for demo purposes.", ko = "데모용 스테이블 토큰 공개 파우셋 민트 지원 여부.")]
    pub stable_mintable: bool,

    /// Brand token `totalSupply` (raw units).
    #[field_doc(en = "Brand token total supply in raw units.", ko = "브랜드 토큰 총 발행량 (원시 단위).")]
    pub total_supply_raw: String,
    /// Brand token `circulatingSupply` from the treasury contract.
    #[field_doc(en = "Brand token circulating supply from the treasury contract.", ko = "트레저리 컨트랙트 기준 브랜드 토큰 유통량.")]
    pub circulating_supply_raw: String,
    /// Brand tokens held by the Treasury (bought back, out of circulation).
    #[field_doc(en = "Brand tokens held by the treasury (bought back, out of circulation).", ko = "트레저리가 보유한 브랜드 토큰 (바이백, 유통 제외).")]
    #[serde(default)]
    pub treasury_held_tokens_raw: String,
    /// Brand token decimals.
    #[field_doc(en = "Brand token decimals.", ko = "브랜드 토큰 소수점 자릿수.")]
    pub token_decimals: u8,
    /// Brand token symbol (e.g. "TKN").
    #[field_doc(en = "Brand token symbol (e.g. \"TKN\").", ko = "브랜드 토큰 심볼 (예: \"TKN\").")]
    #[serde(default)]
    pub token_symbol: String,

    /// Raw floor price scaled by 1e18, matching `getFloorPrice()` on
    /// the Solidity contract. `0` when circulating supply is zero.
    #[field_doc(en = "Raw floor price scaled by 1e18, matching getFloorPrice() on-chain. 0 when circulating supply is zero.", ko = "온체인 getFloorPrice()와 동일한 1e18 스케일 원시 바닥가. 유통량이 0이면 0.")]
    pub floor_price_raw_1e18: String,

    /// On-chain `currentMonth()` index from the BrandToken contract.
    /// Includes `monthOffset` from `advanceMonth()` calls.
    #[field_doc(en = "On-chain current month index from the BrandToken contract.", ko = "BrandToken 컨트랙트의 온체인 현재 월 인덱스.")]
    #[serde(default)]
    pub current_month: u64,
}

impl TreasuryStatusResponse {
    pub fn from_wasm(wasm: &WasmTreasuryStatus, token: &TokenResponse) -> Self {
        let stable_mintable = token
            .stable_token_address
            .as_deref()
            .and_then(|addr| {
                use crate::common::SupportedChain;
                token.chain_id.and_then(|cid| {
                    SupportedChain::from_chain_id(cid).and_then(|chain| {
                        chain
                            .stable_token_options()
                            .into_iter()
                            .find(|opt| opt.address.eq_ignore_ascii_case(addr))
                            .map(|opt| opt.mintable)
                    })
                })
            })
            .unwrap_or(false);

        Self {
            deployed: true,
            chain_id: token.chain_id,
            treasury_contract_address: token.treasury_contract_address.clone(),
            brand_token_address: token.contract_address.clone(),
            treasury_balance_raw: wasm.treasury_balance_raw.clone(),
            stable_decimals: wasm.stable_decimals,
            stable_symbol: wasm.stable_symbol.clone(),
            stable_mintable,
            total_supply_raw: wasm.total_supply_raw.clone(),
            circulating_supply_raw: wasm.circulating_supply_raw.clone(),
            treasury_held_tokens_raw: wasm.treasury_held_tokens_raw.clone(),
            token_decimals: wasm.token_decimals,
            token_symbol: wasm.token_symbol.clone(),
            floor_price_raw_1e18: wasm.floor_price_raw_1e18.clone(),
            current_month: wasm.current_month,
        }
    }
}
