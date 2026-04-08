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
        function stableToken() external view returns (address)
        function getFloorPrice() external view returns (uint256)
        function getCirculatingSupply() external view returns (uint256)
    ]"#
);

abigen!(
    Erc20Contract,
    r#"[
        function balanceOf(address account) external view returns (uint256)
        function totalSupply() external view returns (uint256)
        function decimals() external view returns (uint8)
        function symbol() external view returns (string)
    ]"#
);

fn deployer_private_key() -> Result<&'static str, String> {
    option_env!("DEPLOYER_PRIVATE_KEY")
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "DEPLOYER_PRIVATE_KEY must be set at build time".to_string())
}

fn deployer_wallet(chain_id: u64) -> Result<LocalWallet, String> {
    deployer_private_key()?
        .parse::<LocalWallet>()
        .map_err(|e| format!("Invalid private key: {e}"))
        .map(|wallet| wallet.with_chain_id(chain_id))
}

pub fn deployer_address(chain_id: u64) -> Result<Address, String> {
    Ok(deployer_wallet(chain_id)?.address())
}

fn rpc_url_for_chain(chain_id: u64) -> Result<&'static str, String> {
    match chain_id {
        31337 => Ok("http://localhost:8545"),
        1001 => Ok("https://public-en-kairos.node.kaia.io"),
        8217 => Ok("https://public-en.node.kaia.io"),
        _ => Err(format!("Unsupported chain ID: {chain_id}")),
    }
}

/// Built-in stable token (USDT) address per chain.
///
/// - **8217 (Kaia mainnet):** native Tether USDT, launched May 2025. Verified
///   via Kaia docs (`docs.kaia.io/build/tutorials/how-to-send-usdt-tokens-using-kaia-sdk/`).
/// - **1001 (Kairos testnet):** same address as mainnet. Verified on-chain via
///   `eth_getTransactionReceipt` against `public-en-kairos.node.kaia.io` —
///   a Kairos faucet tx emits a standard ERC20 Transfer log whose contract
///   address is `0xd077a400968890eacc75cdc901f0356c943e4fdb`.
/// - **31337 (local Anvil/Hardhat):** intentionally `None`. Every
///   `npx hardhat node` run produces a fresh MockUSDT at a different address,
///   so a hardcoded value would be misleading. Treasury deploy on 31337 is
///   expected to fail until we either auto-deploy MockUSDT in this code path
///   or thread the freshly-deployed address through explicitly.
fn default_stable_token(chain_id: u64) -> Option<&'static str> {
    match chain_id {
        31337 => None,
        1001 => Some("0xd077a400968890eacc75cdc901f0356c943e4fdb"),
        8217 => Some("0xd077a400968890eacc75cdc901f0356c943e4fdb"),
        _ => None,
    }
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

pub fn stable_token_address(chain_id: u64) -> Result<Address, String> {
    let value = default_stable_token(chain_id)
        .ok_or_else(|| format!("No built-in stable token address for chain {chain_id}"))?;
    value
        .parse::<Address>()
        .map_err(|e| format!("Invalid stable token address for chain {chain_id}: {e}"))
}

/// The project-owner address is currently always the deployer wallet.
/// A separate per-project owner is intentionally deferred until we add
/// real project-owner key management; until then, do not introduce a
/// `PROJECT_OWNER_ADDRESS*` env var.
fn project_owner_address(chain_id: u64) -> Result<Address, String> {
    deployer_address(chain_id)
}

/// Compile-time bytecode for `BiyardToken.sol`, produced by `build.rs`
/// from the Hardhat artifact under `contracts/artifacts/`.
const BIYARD_TOKEN_BYTECODE: &str =
    include_str!(concat!(env!("OUT_DIR"), "/BIYARD_TOKEN_BYTECODE.hex"));

/// Compile-time bytecode for `FloorPriceTreasury.sol`, produced by `build.rs`.
const FLOOR_PRICE_TREASURY_BYTECODE: &str = include_str!(concat!(
    env!("OUT_DIR"),
    "/FLOOR_PRICE_TREASURY_BYTECODE.hex"
));

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

    deploy_contract(chain_id, BIYARD_TOKEN_BYTECODE, constructor_args).await
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
        deploy_contract(chain_id, FLOOR_PRICE_TREASURY_BYTECODE, constructor_args).await?;

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

/// Snapshot of on-chain treasury state for a brand project.
///
/// Every field comes straight from live RPC reads; nothing is
/// mirrored in DynamoDB. A missing `treasury_contract_address` on the
/// project's token record means the treasury has not been deployed
/// yet and callers should treat the whole status as unavailable.
#[derive(Debug, Clone)]
pub struct TreasuryStatus {
    /// Raw USDT balance held by the treasury contract (stable token units).
    pub treasury_balance_raw: u128,
    /// Decimal places of the stable token (typically 6 for USDT).
    pub stable_decimals: u8,
    /// Stable token symbol (e.g. "USDT").
    pub stable_symbol: String,
    /// `totalSupply` of the brand token (raw units).
    pub total_supply_raw: u128,
    /// `getCirculatingSupply()` from the treasury contract (raw units).
    /// This is `totalSupply - treasuryHeld` as defined on-chain.
    pub circulating_supply_raw: u128,
    /// Brand token decimals.
    pub token_decimals: u8,
    /// Raw floor price returned by `getFloorPrice()`, scaled by 1e18.
    /// `0` when `circulating_supply_raw` is zero.
    pub floor_price_raw_1e18: u128,
}

/// Read a full treasury snapshot directly from chain state.
///
/// Resolves every value at the same block-ish window so the returned
/// floor price, treasury balance, and supplies are mutually consistent
/// enough to display on the console.
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

    let treasury = FloorPriceTreasuryContract::new(treasury_addr, prov.clone());
    let brand_token = Erc20Contract::new(token_addr, prov.clone());

    // Resolve stable token address from the treasury contract itself
    // so we never have to trust a cached value from DynamoDB.
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

    let (total_supply, circulating_supply, floor_price, token_decimals) = tokio::try_join!(
        async {
            brand_token
                .total_supply()
                .call()
                .await
                .map_err(|e| format!("token totalSupply() failed: {e}"))
        },
        async {
            treasury
                .get_circulating_supply()
                .call()
                .await
                .map_err(|e| format!("getCirculatingSupply() failed: {e}"))
        },
        async {
            treasury
                .get_floor_price()
                .call()
                .await
                .map_err(|e| format!("getFloorPrice() failed: {e}"))
        },
        async {
            brand_token
                .decimals()
                .call()
                .await
                .map_err(|e| format!("token decimals() failed: {e}"))
        },
    )?;

    Ok(TreasuryStatus {
        treasury_balance_raw: treasury_balance.as_u128(),
        stable_decimals,
        stable_symbol,
        total_supply_raw: total_supply.as_u128(),
        circulating_supply_raw: circulating_supply.as_u128(),
        token_decimals,
        floor_price_raw_1e18: floor_price.as_u128(),
    })
}
