use ethers::prelude::*;
use std::sync::Arc;

use crate::common::SupportedChain;

abigen!(
    BiyardTokenContract,
    r#"[
        function mint(address to, uint256 amount) external
        function addMinter(address account) external
        function minters(address account) external view returns (bool)
        function totalSupply() external view returns (uint256)
        function balanceOf(address account) external view returns (uint256)
        function name() external view returns (string)
        function symbol() external view returns (string)
        event Transfer(address indexed from, address indexed to, uint256 value)
    ]"#
);

abigen!(
    FloorPriceTreasuryContract,
    r#"[
        function mintRewardTokens(address to, uint256 amount, string reason) external
    ]"#
);

fn deployer_private_key() -> String {
    std::env::var("DEPLOYER_PRIVATE_KEY").expect("DEPLOYER_PRIVATE_KEY must be set")
}

fn deployer_wallet(chain_id: u64) -> Result<LocalWallet, String> {
    deployer_private_key()
        .parse::<LocalWallet>()
        .map_err(|e| format!("Invalid private key: {e}"))
        .map(|wallet| wallet.with_chain_id(chain_id))
}

pub fn deployer_address(chain_id: u64) -> Result<Address, String> {
    Ok(deployer_wallet(chain_id)?.address())
}

fn default_rpc_url(chain_id: u64) -> Option<&'static str> {
    match chain_id {
        1001 => Some("https://public-en-kairos.node.kaia.io"),
        8217 => Some("https://public-en.node.kaia.io"),
        _ => None,
    }
}

fn rpc_url_for_chain(chain_id: u64) -> Result<String, String> {
    if let Ok(url) = std::env::var(format!("RPC_URL_{chain_id}")) {
        return Ok(url);
    }
    default_rpc_url(chain_id)
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Unsupported chain ID: {chain_id}"))
}

pub fn provider(chain_id: u64) -> Result<Provider<Http>, String> {
    let url = rpc_url_for_chain(chain_id)?;
    Provider::<Http>::try_from(url).map_err(|e| format!("Failed to create provider: {e}"))
}

pub fn signer(chain_id: u64) -> Result<SignerMiddleware<Provider<Http>, LocalWallet>, String> {
    let provider = provider(chain_id)?;
    let wallet = deployer_wallet(chain_id)?;
    Ok(SignerMiddleware::new(provider, wallet))
}

fn env_for_chain(base: &str, chain_id: u64) -> Option<String> {
    std::env::var(format!("{base}_{chain_id}"))
        .ok()
        .or_else(|| std::env::var(base).ok())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn parse_address(var_name: &str, value: &str) -> Result<Address, String> {
    value
        .parse::<Address>()
        .map_err(|e| format!("Invalid {var_name}: {e}"))
}

pub fn stable_token_address(chain_id: u64) -> Result<Address, String> {
    let value = env_for_chain("STABLE_TOKEN_ADDRESS", chain_id).ok_or_else(|| {
        format!("STABLE_TOKEN_ADDRESS_{chain_id} or STABLE_TOKEN_ADDRESS must be set")
    })?;
    parse_address("stable token address", &value)
}

fn project_owner_address(chain_id: u64) -> Result<Address, String> {
    if let Some(value) = env_for_chain("PROJECT_OWNER_ADDRESS", chain_id) {
        return parse_address("project owner address", &value);
    }

    deployer_address(chain_id)
}

async fn deploy_contract(
    chain_id: u64,
    bytecode_env: &str,
    constructor_args: Vec<u8>,
) -> Result<(Address, TxHash), String> {
    let _ = SupportedChain::from_chain_id(chain_id)
        .ok_or_else(|| format!("Unsupported chain: {chain_id}"))?;
    let client = Arc::new(signer(chain_id)?);

    let bytecode_hex =
        std::env::var(bytecode_env).map_err(|_| format!("{bytecode_env} env var not set"))?;

    let bytecode = hex::decode(bytecode_hex.trim_start_matches("0x"))
        .map_err(|e| format!("Invalid bytecode hex for {bytecode_env}: {e}"))?;

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

pub async fn deploy_token(
    chain_id: u64,
    name: &str,
    symbol: &str,
    initial_supply: u64,
    max_supply: u64,
) -> Result<(Address, TxHash), String> {
    let constructor_args = ethers::abi::encode(&[
        ethers::abi::Token::String(name.to_string()),
        ethers::abi::Token::String(symbol.to_string()),
        ethers::abi::Token::Uint(U256::from(initial_supply)),
        ethers::abi::Token::Uint(U256::from(max_supply)),
    ]);

    deploy_contract(chain_id, "BIYARD_TOKEN_BYTECODE", constructor_args).await
}

pub async fn deploy_floor_price_treasury(
    chain_id: u64,
    brand_token_address: Address,
    treasury_reserve_bps: u64,
) -> Result<(Address, TxHash, Address, Address), String> {
    let stable_token = stable_token_address(chain_id)?;
    let project_owner = project_owner_address(chain_id)?;

    let constructor_args = ethers::abi::encode(&[
        ethers::abi::Token::Address(brand_token_address),
        ethers::abi::Token::Address(stable_token),
        ethers::abi::Token::Address(project_owner),
        ethers::abi::Token::Uint(U256::from(treasury_reserve_bps)),
    ]);

    let (treasury_address, tx_hash) =
        deploy_contract(chain_id, "FLOOR_PRICE_TREASURY_BYTECODE", constructor_args).await?;

    Ok((treasury_address, tx_hash, stable_token, project_owner))
}

pub async fn mint_on_chain(
    chain_id: u64,
    contract_address: &str,
    to_address: &str,
    amount: u64,
) -> Result<TxHash, String> {
    let client = Arc::new(signer(chain_id)?);

    let contract_addr: Address = contract_address
        .parse()
        .map_err(|e| format!("Invalid contract address: {e}"))?;

    let to_addr: Address = to_address
        .parse()
        .map_err(|e| format!("Invalid recipient address: {e}"))?;

    let contract = BiyardTokenContract::new(contract_addr, client);
    let call = contract.mint(to_addr, U256::from(amount));

    let pending = call
        .send()
        .await
        .map_err(|e| format!("Mint tx failed: {e}"))?;

    let tx_hash = pending.tx_hash();

    pending
        .await
        .map_err(|e| format!("Waiting for mint receipt failed: {e}"))?
        .ok_or("No mint receipt".to_string())?;

    Ok(tx_hash)
}

pub async fn mint_reward_on_chain(
    chain_id: u64,
    treasury_contract_address: &str,
    to_address: &str,
    amount: u64,
    reason: &str,
) -> Result<TxHash, String> {
    let client = Arc::new(signer(chain_id)?);

    let treasury_addr: Address = treasury_contract_address
        .parse()
        .map_err(|e| format!("Invalid treasury contract address: {e}"))?;

    let to_addr: Address = to_address
        .parse()
        .map_err(|e| format!("Invalid recipient address: {e}"))?;

    let contract = FloorPriceTreasuryContract::new(treasury_addr, client);
    let call = contract.mint_reward_tokens(to_addr, U256::from(amount), reason.to_string());

    let pending = call
        .send()
        .await
        .map_err(|e| format!("Treasury mint tx failed: {e}"))?;

    let tx_hash = pending.tx_hash();

    pending
        .await
        .map_err(|e| format!("Waiting for treasury mint receipt failed: {e}"))?
        .ok_or("No treasury mint receipt".to_string())?;

    Ok(tx_hash)
}

/// Server calls addMinter on the contract so the user wallet can call mint().
/// Gas paid by server (DEPLOYER_PRIVATE_KEY).
pub async fn add_minter(
    chain_id: u64,
    contract_address: &str,
    minter_address: &str,
) -> Result<TxHash, String> {
    let client = Arc::new(signer(chain_id)?);

    let contract_addr: Address = contract_address
        .parse()
        .map_err(|e| format!("Invalid contract address: {e}"))?;

    let minter_addr: Address = minter_address
        .parse()
        .map_err(|e| format!("Invalid minter address: {e}"))?;

    let contract = BiyardTokenContract::new(contract_addr, client);
    let call = contract.add_minter(minter_addr);

    let pending = call
        .send()
        .await
        .map_err(|e| format!("addMinter tx failed: {e}"))?;

    let tx_hash = pending.tx_hash();

    pending
        .await
        .map_err(|e| format!("Waiting for addMinter receipt failed: {e}"))?
        .ok_or("No addMinter receipt".to_string())?;

    Ok(tx_hash)
}

/// Check if an address is already a minter on the contract.
pub async fn is_minter(
    chain_id: u64,
    contract_address: &str,
    address: &str,
) -> Result<bool, String> {
    let prov = provider(chain_id)?;

    let contract_addr: Address = contract_address
        .parse()
        .map_err(|e| format!("Invalid contract address: {e}"))?;

    let addr: Address = address
        .parse()
        .map_err(|e| format!("Invalid address: {e}"))?;

    let contract = BiyardTokenContract::new(contract_addr, Arc::new(prov));

    contract
        .minters(addr)
        .call()
        .await
        .map_err(|e| format!("minters() call failed: {e}"))
}

/// Verify a mint tx on-chain: check receipt exists, correct contract, and extract amount.
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

    // Parse Transfer event to find mint amount
    // Transfer(address indexed from, address indexed to, uint256 value)
    // from=0x0 means mint
    let transfer_sig: H256 = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
        .parse()
        .unwrap();

    for log in &receipt.logs {
        if log.topics.first() == Some(&transfer_sig) && log.topics.len() >= 3 {
            let from = H160::from(log.topics[1]);
            if from == H160::zero() {
                // This is a mint event
                let to = H160::from(log.topics[2]);
                let amount = U256::from_big_endian(&log.data);
                return Ok((format!("{to:?}"), amount.as_u64()));
            }
        }
    }

    Err("No mint Transfer event found in tx".to_string())
}

/// Encode mint(address,uint256) calldata for the user to sign locally.
pub fn encode_mint_calldata(to_address: &str, amount: u64) -> Result<String, String> {
    let to_addr: Address = to_address
        .parse()
        .map_err(|e| format!("Invalid address: {e}"))?;

    let calldata = BiyardTokenContractCalls::Mint(MintCall {
        to: to_addr,
        amount: U256::from(amount),
    });

    Ok(format!(
        "0x{}",
        hex::encode(ethers::abi::AbiEncode::encode(calldata))
    ))
}

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

    let contract = BiyardTokenContract::new(contract_addr, Arc::new(provider));

    let balance = contract
        .balance_of(account_addr)
        .call()
        .await
        .map_err(|e| format!("Balance query failed: {e}"))?;

    Ok(balance.as_u64())
}
