use ethers::prelude::*;
use std::sync::Arc;

use super::{provider, BrandTokenContract, Erc20Contract, TreasuryContract};

pub async fn get_on_chain_balance(
    chain_id: u64,
    contract_address: &str,
    account_address: &str,
) -> Result<u64, String> {
    let provider = provider(chain_id)?;

    let contract_addr: Address = contract_address
        .parse()
        .map_err(|e| format!("Invalid contract address: {e}"))?;

    let account_addr: Address = account_address
        .parse()
        .map_err(|e| format!("Invalid account address: {e}"))?;

    let contract = Erc20Contract::new(contract_addr, Arc::new(provider));

    let balance = contract
        .balance_of(account_addr)
        .call()
        .await
        .map_err(|e| format!("Balance query failed: {e}"))?;

    Ok(balance.as_u64())
}

pub async fn verify_mint_tx(
    chain_id: u64,
    contract_address: &str,
    tx_hash_str: &str,
) -> Result<(String, u64), String> {
    let prov = provider(chain_id)?;

    let tx_hash: TxHash = tx_hash_str
        .parse()
        .map_err(|e| format!("Invalid tx hash: {e}"))?;

    let receipt = prov
        .get_transaction_receipt(tx_hash)
        .await
        .map_err(|e| format!("Failed to get receipt: {e}"))?
        .ok_or("Transaction not found or not yet mined")?;

    let expected_addr: Address = contract_address
        .parse()
        .map_err(|e| format!("Invalid contract address: {e}"))?;

    let to_addr = receipt.to.ok_or("Transaction has no 'to' address")?;
    if to_addr != expected_addr {
        return Err(format!(
            "Tx target {to_addr:?} does not match contract {expected_addr:?}"
        ));
    }

    if receipt.status != Some(1.into()) {
        return Err("Transaction reverted".to_string());
    }

    let transfer_sig: H256 =
        "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
            .parse()
            .unwrap();

    for log in &receipt.logs {
        if log.topics.first() == Some(&transfer_sig) && log.topics.len() >= 3 {
            let from = H160::from(log.topics[1]);
            if from == H160::zero() {
                let to = H160::from(log.topics[2]);
                let amount = U256::from_big_endian(&log.data);
                return Ok((format!("{to:?}"), amount.as_u64()));
            }
        }
    }

    Err("No mint Transfer event found in tx".to_string())
}

#[derive(Debug, Clone)]
pub struct TreasuryStatus {
    pub treasury_balance_raw: u128,
    pub stable_decimals: u8,
    pub stable_symbol: String,
    pub total_supply_raw: u128,
    pub circulating_supply_raw: u128,
    pub treasury_held_tokens_raw: u128,
    pub token_decimals: u8,
    pub token_symbol: String,
    pub floor_price_raw_1e18: u128,
    pub current_month: u64,
}

pub async fn get_treasury_status(
    chain_id: u64,
    treasury_contract_address: &str,
    brand_token_address: &str,
) -> Result<TreasuryStatus, String> {
    let prov = Arc::new(provider(chain_id)?);

    let treasury_addr: Address = treasury_contract_address
        .parse()
        .map_err(|e| format!("Invalid treasury address: {e}"))?;

    let token_addr: Address = brand_token_address
        .parse()
        .map_err(|e| format!("Invalid brand token address: {e}"))?;

    let treasury = TreasuryContract::new(treasury_addr, prov.clone());
    let brand_token = Erc20Contract::new(token_addr, prov.clone());
    let brand_token_full = BrandTokenContract::new(token_addr, prov.clone());

    let stable_addr = treasury
        .stable_token()
        .call()
        .await
        .map_err(|e| format!("stableToken() call failed: {e}"))?;

    let stable = Erc20Contract::new(stable_addr, prov.clone());

    let (treasury_balance, stable_decimals, stable_symbol) = tokio::try_join!(
        async {
            stable
                .balance_of(treasury_addr)
                .call()
                .await
                .map_err(|e| format!("stable balanceOf() failed: {e}"))
        },
        async {
            stable
                .decimals()
                .call()
                .await
                .map_err(|e| format!("stable decimals() failed: {e}"))
        },
        async {
            stable
                .symbol()
                .call()
                .await
                .map_err(|e| format!("stable symbol() failed: {e}"))
        },
    )?;

    let (total_supply, circulating_supply, treasury_held, floor_price, token_decimals, token_symbol, current_month) =
        tokio::try_join!(
            async {
                brand_token
                    .total_supply()
                    .call()
                    .await
                    .map_err(|e| format!("token totalSupply() failed: {e}"))
            },
            async {
                treasury
                    .circulating_supply()
                    .call()
                    .await
                    .map_err(|e| format!("circulatingSupply() failed: {e}"))
            },
            async {
                brand_token
                    .balance_of(treasury_addr)
                    .call()
                    .await
                    .map_err(|e| format!("token balanceOf(treasury) failed: {e}"))
            },
            async {
                treasury
                    .get_price()
                    .call()
                    .await
                    .map_err(|e| format!("getPrice() failed: {e}"))
            },
            async {
                brand_token
                    .decimals()
                    .call()
                    .await
                    .map_err(|e| format!("token decimals() failed: {e}"))
            },
            async {
                brand_token
                    .symbol()
                    .call()
                    .await
                    .map_err(|e| format!("token symbol() failed: {e}"))
            },
            async {
                brand_token_full
                    .current_month()
                    .call()
                    .await
                    .map_err(|e| format!("currentMonth() failed: {e}"))
            },
        )?;

    Ok(TreasuryStatus {
        treasury_balance_raw: treasury_balance.as_u128(),
        stable_decimals,
        stable_symbol,
        total_supply_raw: total_supply.as_u128(),
        circulating_supply_raw: circulating_supply.as_u128(),
        treasury_held_tokens_raw: treasury_held.as_u128(),
        token_decimals,
        token_symbol,
        floor_price_raw_1e18: floor_price.as_u128(),
        current_month: current_month.as_u64(),
    })
}
