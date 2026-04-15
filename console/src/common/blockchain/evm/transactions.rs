use ethers::prelude::*;
use std::sync::Arc;

use super::{deployer_address, deployer_wallet, signer, BusdtContract, Erc20Contract};

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
