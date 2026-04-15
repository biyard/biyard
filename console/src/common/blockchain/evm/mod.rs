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

pub(crate) fn deployer_private_key() -> Result<&'static str, String> {
    option_env!("DEPLOYER_PRIVATE_KEY")
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "DEPLOYER_PRIVATE_KEY must be set at build time".to_string())
}

pub(crate) fn deployer_wallet(chain_id: u64) -> Result<LocalWallet, String> {
    deployer_private_key()?
        .parse::<LocalWallet>()
        .map_err(|e| format!("Invalid private key: {e}"))
        .map(|wallet| wallet.with_chain_id(chain_id))
}

pub fn deployer_address(chain_id: u64) -> Result<Address, String> {
    Ok(deployer_wallet(chain_id)?.address())
}

pub(crate) fn rpc_url_for_chain(chain_id: u64) -> Result<&'static str, String> {
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

mod deploy;
mod queries;
mod transactions;

pub use deploy::*;
pub use queries::*;
pub use transactions::*;
