use ethers::prelude::*;
use std::sync::Arc;

use crate::common::SupportedChain;

abigen!(
    BrandTokenContract,
    r#"[
        function claim(uint256 month, uint256 amount, uint256 maxClaimable, uint256 nonce, uint256 deadline, bytes signature) external
        function monthRemaining(uint256 month) external view returns (uint256)
        function totalSupply() external view returns (uint256)
        function balanceOf(address account) external view returns (uint256)
        function name() external view returns (string)
        function symbol() external view returns (string)
        function decimals() external view returns (uint8)
        function setDistributionSlots(address[] wallets, uint16[] bps) external
        function setTreasury(address treasury) external
        function transferOwnership(address newOwner) external
        function owner() external view returns (address)
        function monthlyEmission() external view returns (uint256)
        function decayRateBps() external view returns (uint16)
        function currentMonth() external view returns (uint256)
        function currentMonthRemaining() external view returns (uint256)
        function cumulativeEmission() external view returns (uint256)
        function monthlyCeiling(uint256 month) external view returns (uint256)
        event Transfer(address indexed from, address indexed to, uint256 value)
        event Claimed(address indexed user, uint256 amount, uint256 nonce)
    ]"#
);

abigen!(
    TreasuryContract,
    r#"[
        function buyback(uint256 tokenAmount) external
        function getPrice() external view returns (uint256)
        function circulatingSupply() external view returns (uint256)
        function stableToken() external view returns (address)
        function brandToken() external view returns (address)
        function withdrawStable(address to, uint256 amount) external
        function withdrawToken(address token, address to, uint256 amount) external
    ]"#
);

abigen!(
    MultisigContract,
    r#"[
        function propose(address target, bytes data, uint256 value) external returns (uint256)
        function approve(uint256 proposalId) external
        function execute(uint256 proposalId) external
        function proposalCount() external view returns (uint256)
        function threshold() external view returns (uint256)
        function signerCount() external view returns (uint256)
        function isSigner(address) external view returns (bool)
    ]"#
);

abigen!(
    Erc20Contract,
    r#"[
        function balanceOf(address account) external view returns (uint256)
        function totalSupply() external view returns (uint256)
        function decimals() external view returns (uint8)
        function symbol() external view returns (string)
        function approve(address spender, uint256 amount) external returns (bool)
        function transfer(address to, uint256 amount) external returns (bool)
    ]"#
);

abigen!(
    BusdtContract,
    r#"[
        function mint(address to, uint256 amount) external
        function approve(address spender, uint256 amount) external returns (bool)
        function balanceOf(address account) external view returns (uint256)
        function decimals() external view returns (uint8)
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
        1001 => Ok("https://public-en-kairos.node.kaia.io"),
        8217 => Ok("https://public-en.node.kaia.io"),
        _ => Err(format!("Unsupported chain ID: {chain_id}")),
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

pub async fn transfer_brand_token(
    chain_id: u64,
    token_address: &str,
    to: &str,
    amount: U256,
) -> Result<TxHash, String> {
    let client = Arc::new(signer(chain_id)?);

    let token_addr: Address = token_address
        .parse()
        .map_err(|e| format!("Invalid token address: {e}"))?;

    let to_addr: Address = to
        .parse()
        .map_err(|e| format!("Invalid recipient address: {e}"))?;

    let token = Erc20Contract::new(token_addr, client);

    let transfer_call = token.transfer(to_addr, amount);
    let pending = transfer_call
        .send()
        .await
        .map_err(|e| format!("transfer failed: {e}"))?;

    let tx_hash = pending.tx_hash();
    pending
        .await
        .map_err(|e| format!("transfer receipt failed: {e}"))?;

    Ok(tx_hash)
}

pub async fn deposit_stable_to_treasury(
    chain_id: u64,
    stable_address: &str,
    treasury_address: &str,
    amount: U256,
) -> Result<TxHash, String> {
    let client = Arc::new(signer(chain_id)?);

    let stable_addr: Address = stable_address
        .parse()
        .map_err(|e| format!("Invalid stable token address: {e}"))?;

    let treasury_addr: Address = treasury_address
        .parse()
        .map_err(|e| format!("Invalid treasury address: {e}"))?;

    let busdt = BusdtContract::new(stable_addr, client.clone());

    let deployer = deployer_address(chain_id)?;
    busdt
        .mint(deployer, amount)
        .send()
        .await
        .map_err(|e| format!("BUSDT mint failed: {e}"))?
        .await
        .map_err(|e| format!("BUSDT mint receipt failed: {e}"))?;

    // Transfer directly to Treasury (no deposit() function needed —
    // Treasury reads its own balanceOf for floor price calculation)
    let stable = Erc20Contract::new(stable_addr, client);
    let transfer_call = stable.transfer(treasury_addr, amount);
    let pending = transfer_call
        .send()
        .await
        .map_err(|e| format!("BUSDT transfer to treasury failed: {e}"))?;

    let tx_hash = pending.tx_hash();
    pending
        .await
        .map_err(|e| format!("BUSDT transfer receipt failed: {e}"))?;

    Ok(tx_hash)
}

/// Generate an EIP-712 signature for a token claim.
pub fn sign_claim(
    chain_id: u64,
    token_address: &str,
    token_name: &str,
    to: &str,
    month: u64,
    amount: u128,
    max_claimable: u128,
    nonce: u64,
    deadline: u64,
) -> Result<Vec<u8>, String> {
    use sha3::{Digest, Keccak256};

    let wallet = deployer_wallet(chain_id)?;
    let token_addr: Address = token_address
        .parse()
        .map_err(|e| format!("Invalid token address: {e}"))?;
    let to_addr: Address = to
        .parse()
        .map_err(|e| format!("Invalid to address: {e}"))?;

    let type_hash = Keccak256::digest(
        b"Claim(address to,uint256 month,uint256 amount,uint256 maxClaimable,uint256 nonce,uint256 deadline)"
    );

    let struct_hash = Keccak256::digest(ethers::abi::encode(&[
        ethers::abi::Token::FixedBytes(type_hash.to_vec()),
        ethers::abi::Token::Address(to_addr),
        ethers::abi::Token::Uint(U256::from(month)),
        ethers::abi::Token::Uint(U256::from(amount)),
        ethers::abi::Token::Uint(U256::from(max_claimable)),
        ethers::abi::Token::Uint(U256::from(nonce)),
        ethers::abi::Token::Uint(U256::from(deadline)),
    ]));

    let domain_type_hash = Keccak256::digest(
        b"EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"
    );
    let name_hash = Keccak256::digest(token_name.as_bytes());
    let version_hash = Keccak256::digest(b"1");

    let domain_separator = Keccak256::digest(ethers::abi::encode(&[
        ethers::abi::Token::FixedBytes(domain_type_hash.to_vec()),
        ethers::abi::Token::FixedBytes(name_hash.to_vec()),
        ethers::abi::Token::FixedBytes(version_hash.to_vec()),
        ethers::abi::Token::Uint(U256::from(chain_id)),
        ethers::abi::Token::Address(token_addr),
    ]));

    let mut digest_input = Vec::with_capacity(66);
    digest_input.push(0x19);
    digest_input.push(0x01);
    digest_input.extend_from_slice(&domain_separator);
    digest_input.extend_from_slice(&struct_hash);
    let digest = Keccak256::digest(&digest_input);

    let signature = wallet
        .sign_hash(H256::from_slice(&digest))
        .map_err(|e| format!("Signing failed: {e}"))?;

    Ok(signature.to_vec())
}
