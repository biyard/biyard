/**
 * Wire-format DTOs returned by the partner proxy. These mirror the Rust
 * structs on the Biyard backend (see console/src/features/tokens/controllers/claim_mint.rs).
 *
 * Kept hand-mirrored for now. If the DTO surface grows, generate from
 * schemars JSON schema instead.
 */

export interface ClaimableMonth {
  month: string;
  user_points: number;
  total_points: number;
  /** Raw ERC-20 token units (includes decimals). String to avoid JSON precision loss. */
  claimable_tokens: string;
  already_claimed: string;
  remaining: string;
}

export interface ClaimableResponse {
  months: ClaimableMonth[];
}

/**
 * Public token metadata. Mirrors the `TokenResponse` returned by Biyard's
 * `GET /v1/projects/:project_id/tokens`. Only the fields a partner UI is
 * likely to render — Biyard returns additional bookkeeping fields that the
 * SDK consumer should not need.
 */
export interface TokenInfo {
  name: string;
  symbol: string;
  description: string | null;
  contract_address: string | null;
  chain_id: number | null;
  start_month: string | null;
  /** Display decimals. Not currently returned by Biyard — defaults to 18. */
  decimals: number;
}

export interface ClaimSignatureResponse {
  month_index: string;
  amount: string;
  max_claimable: string;
  nonce: string;
  deadline: string;
  /** 0x-prefixed hex signature. */
  signature: string;
  contract_address: string;
  chain_id: number;
}

/**
 * Body the SDK sends to the partner proxy when requesting a claim signature.
 * The partner proxy MUST inject `meta_user_id` from its own session — the
 * SDK never sends it.
 */
export interface ClaimSignatureRequestBody {
  month: string;
  wallet_address: string;
}

/**
 * On-chain submit result.
 */
export interface ClaimSubmitResult {
  tx_hash: string;
  wallet_address: string;
}
