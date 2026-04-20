use ethers::prelude::*;
use std::sync::Arc;

use crate::common::SupportedChain;

use super::{deployer_address, signer, BrandTokenContract};

const BRAND_TOKEN_BYTECODE: &str =
    include_str!(concat!(env!("OUT_DIR"), "/BRAND_TOKEN_BYTECODE.hex"));

const TREASURY_BYTECODE: &str =
    include_str!(concat!(env!("OUT_DIR"), "/TREASURY_BYTECODE.hex"));

const MULTISIG_BYTECODE: &str =
    include_str!(concat!(env!("OUT_DIR"), "/MULTISIG_BYTECODE.hex"));

async fn deploy_contract(
    chain_id: u64,
    bytecode_hex: &str,
    constructor_args: Vec<u8>,
) -> Result<(Address, TxHash), String> {
    let _ = SupportedChain::from_chain_id(chain_id)
        .ok_or_else(|| format!("Unsupported chain: {chain_id}"))?;
    let client = Arc::new(signer(chain_id)?);

    let bytecode = hex::decode(bytecode_hex.trim_start_matches("0x"))
        .map_err(|e| format!("Invalid embedded bytecode hex: {e}"))?;

    let mut deploy_data = bytecode;
    deploy_data.extend_from_slice(&constructor_args);

    let tx = TransactionRequest::new()
        .data(deploy_data)
        .chain_id(chain_id);

    let pending = client
        .send_transaction(tx, None)
        .await
        .map_err(|e| format!("Deploy tx failed: {e}"))?;

    let tx_hash = pending.tx_hash();

    let receipt = pending
        .await
        .map_err(|e| format!("Waiting for deploy receipt failed: {e}"))?
        .ok_or("No deploy receipt")?;

    let contract_address = receipt
        .contract_address
        .ok_or("No contract address in receipt")?;

    Ok((contract_address, tx_hash))
}

#[derive(Debug, Clone)]
pub struct BrandSystemDeployment {
    pub multisig_address: Address,
    pub multisig_tx_hash: TxHash,
    pub token_address: Address,
    pub token_tx_hash: TxHash,
    pub treasury_address: Address,
    pub treasury_tx_hash: TxHash,
    pub stable_token_address: Address,
}

pub async fn deploy_brand_system(
    chain_id: u64,
    token_name: &str,
    token_symbol: &str,
    monthly_emission: u128,
    decay_rate_bps: u16,
    stable_token_addr: Address,
    distribution_wallets: Vec<Address>,
    distribution_bps: Vec<u16>,
    start_timestamp: u64,
) -> Result<BrandSystemDeployment, String> {
    let deployer = deployer_address(chain_id)?;

    // 1. Deploy Multisig (1-of-1 with deployer)
    let ms_args = ethers::abi::encode(&[
        ethers::abi::Token::Array(vec![ethers::abi::Token::Address(deployer)]),
        ethers::abi::Token::Uint(U256::from(1u64)),
    ]);
    let (multisig_addr, ms_tx) = deploy_contract(chain_id, MULTISIG_BYTECODE, ms_args).await?;

    // 2. Deploy BrandToken (owner = deployer initially, will transfer)
    let token_args = ethers::abi::encode(&[
        ethers::abi::Token::String(token_name.to_string()),
        ethers::abi::Token::String(token_symbol.to_string()),
        ethers::abi::Token::Uint(U256::from(monthly_emission)),
        ethers::abi::Token::Uint(U256::from(decay_rate_bps)),
        ethers::abi::Token::Address(deployer),
        ethers::abi::Token::Address(deployer),
        ethers::abi::Token::Uint(U256::from(start_timestamp)),
    ]);
    let (token_addr, token_tx) =
        deploy_contract(chain_id, BRAND_TOKEN_BYTECODE, token_args).await?;

    // 3. Deploy Treasury
    let treasury_args = ethers::abi::encode(&[
        ethers::abi::Token::Address(stable_token_addr),
        ethers::abi::Token::Address(token_addr),
        ethers::abi::Token::Address(multisig_addr),
    ]);
    let (treasury_addr, treasury_tx) =
        deploy_contract(chain_id, TREASURY_BYTECODE, treasury_args).await?;

    // 4. Configure: set distribution slots, then transfer ownership to multisig
    let client = Arc::new(signer(chain_id)?);
    let token_contract = BrandTokenContract::new(token_addr, client.clone());

    // 4a. Set distribution slots (if any)
    if !distribution_wallets.is_empty() {
        token_contract
            .set_distribution_slots(distribution_wallets, distribution_bps)
            .send()
            .await
            .map_err(|e| format!("setDistributionSlots failed: {e}"))?
            .await
            .map_err(|e| format!("setDistributionSlots receipt failed: {e}"))?;
    }

    token_contract
        .transfer_ownership(multisig_addr)
        .send()
        .await
        .map_err(|e| format!("transferOwnership failed: {e}"))?
        .await
        .map_err(|e| format!("transferOwnership receipt failed: {e}"))?;

    Ok(BrandSystemDeployment {
        multisig_address: multisig_addr,
        multisig_tx_hash: ms_tx,
        token_address: token_addr,
        token_tx_hash: token_tx,
        treasury_address: treasury_addr,
        treasury_tx_hash: treasury_tx,
        stable_token_address: stable_token_addr,
    })
}
