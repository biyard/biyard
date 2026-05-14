import { Contract, type ContractTransactionResponse } from "ethers";

import { getSigner } from "./wallet";
import type { ClaimSignatureResponse, ClaimSubmitResult } from "./types";

const BRAND_TOKEN_ABI = [
  "function claim(uint256 month, uint256 amount, uint256 maxClaimable, uint256 nonce, uint256 deadline, bytes signature) external",
  "function balanceOf(address) view returns (uint256)",
  "function decimals() view returns (uint8)",
  "function symbol() view returns (string)",
] as const;

type BrandTokenClaim = (
  month: bigint,
  amount: bigint,
  maxClaimable: bigint,
  nonce: bigint,
  deadline: bigint,
  signature: string,
) => Promise<ContractTransactionResponse>;

/**
 * Submit the server-signed claim permit on-chain. Prompts the user's wallet
 * to sign and send the transaction.
 *
 * The `sig` argument is the response returned by the partner proxy's
 * `/claim-signature` endpoint (which itself proxies Biyard's
 * `POST /v1/projects/:project_id/tokens/claim-signature`).
 */
export async function submitClaim(
  sig: ClaimSignatureResponse,
): Promise<ClaimSubmitResult> {
  const signer = await getSigner(sig.chain_id);
  const wallet_address = await signer.getAddress();

  const contract = new Contract(sig.contract_address, BRAND_TOKEN_ABI, signer);
  // ethers v6 attaches ABI methods dynamically; typed cast for strict TS.
  const claim = contract.getFunction("claim") as unknown as BrandTokenClaim;

  const tx = await claim(
    BigInt(sig.month_index),
    BigInt(sig.amount),
    BigInt(sig.max_claimable),
    BigInt(sig.nonce),
    BigInt(sig.deadline),
    sig.signature,
  );

  const receipt = await tx.wait();
  return {
    tx_hash: receipt?.hash ?? tx.hash,
    wallet_address,
  };
}
