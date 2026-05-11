import { ProxyRequestError } from "./errors";
import { submitClaim } from "./claim";
import { getClaimHistory, type OnChainClaimEvent } from "./history";
import { connectWallet, getWalletAddress } from "./wallet";
import type {
  ClaimSignatureResponse,
  ClaimSubmitResult,
  ClaimableResponse,
  TokenInfo,
} from "./types";

/**
 * Function the SDK uses to talk to the partner proxy. Defaults to fetching
 * `${baseUrl}${path}` with JSON body. Override to inject auth headers,
 * CSRF tokens, or replace with a non-fetch transport entirely.
 */
export type BiyardFetcher = (path: string, init: RequestInit) => Promise<Response>;

export interface BiyardClaimOptions {
  /**
   * Base URL of the partner's proxy endpoints. The SDK appends `/claimable`,
   * `/claim-signature`, etc. Example: `"/api/biyard"`.
   *
   * Either `baseUrl` or `fetcher` is required.
   */
  baseUrl?: string;
  /**
   * Replace the default fetch transport. Receives a path like `/claimable`
   * and a `RequestInit`; must return a `Response`. Useful when the partner
   * routes through a non-REST transport or needs custom headers.
   */
  fetcher?: BiyardFetcher;
  /**
   * Target chain id (EVM). 8217 = Kaia Mainnet, 1001 = Kaia Kairos, 1 = Ethereum.
   */
  chainId: number;
}

/**
 * Biyard browser SDK entry point.
 *
 * Flow:
 *   1. Partner backend mints `meta_user_id` from its own session and forwards
 *      requests to Biyard with the partner's Biyard API key.
 *   2. This SDK calls the partner's proxy endpoints — never Biyard directly.
 *   3. SDK collects the EIP-712 permit from Biyard (via partner proxy) and
 *      submits the on-chain `BrandToken.claim(...)` transaction with the
 *      user's wallet.
 *
 * Security: the SDK never sends `meta_user_id`. The partner proxy MUST
 * derive it from its own authenticated session, not from the request body.
 */
export class BiyardClaim {
  private readonly fetcher: BiyardFetcher;
  readonly chainId: number;

  constructor(opts: BiyardClaimOptions) {
    if (!opts.baseUrl && !opts.fetcher) {
      throw new Error("BiyardClaim: either `baseUrl` or `fetcher` is required");
    }
    this.chainId = opts.chainId;
    this.fetcher =
      opts.fetcher ??
      ((path, init) => fetch(`${opts.baseUrl}${path}`, init));
  }

  /**
   * GET `${baseUrl}/claimable` — list months with claimable tokens for the
   * currently authenticated user. The partner proxy resolves the user from
   * its own session.
   */
  async getClaimable(): Promise<ClaimableResponse> {
    return this.request<ClaimableResponse>("/claimable", { method: "GET" });
  }

  /**
   * GET `${baseUrl}/token` — public token metadata (name, symbol, decimals,
   * contract address). Not user-specific, but still routed through the
   * partner proxy so the API key stays server-side.
   */
  async getTokenInfo(): Promise<TokenInfo> {
    const raw = await this.request<Partial<TokenInfo>>("/token", {
      method: "GET",
    });
    return {
      name: raw.name ?? "",
      symbol: raw.symbol ?? "",
      description: raw.description ?? null,
      contract_address: raw.contract_address ?? null,
      chain_id: raw.chain_id ?? null,
      start_month: raw.start_month ?? null,
      // Biyard does not yet return decimals; ERC-20 default is 18.
      decimals: raw.decimals ?? 18,
    };
  }

  /**
   * Request a claim signature from the partner proxy for the given month.
   * Returns the EIP-712 permit response from Biyard.
   *
   * The SDK does not send `meta_user_id`; the partner proxy MUST inject it
   * from its session.
   */
  async getClaimSignature(
    month: string,
    walletAddress: string,
  ): Promise<ClaimSignatureResponse> {
    return this.request<ClaimSignatureResponse>("/claim-signature", {
      method: "POST",
      body: JSON.stringify({ month, wallet_address: walletAddress }),
      headers: { "content-type": "application/json" },
    });
  }

  /**
   * One-call claim flow:
   *   - connect wallet (prompts user if needed)
   *   - fetch claim signature from partner proxy
   *   - submit `BrandToken.claim(...)` on-chain (prompts user to sign tx)
   */
  async claim(month: string): Promise<ClaimSubmitResult> {
    const walletAddress = await connectWallet(this.chainId);
    const sig = await this.getClaimSignature(month, walletAddress);
    return submitClaim(sig);
  }

  /**
   * Connect wallet and switch to the configured chain. Returns the address.
   */
  connectWallet(): Promise<string> {
    return connectWallet(this.chainId);
  }

  /**
   * Read past on-chain `Claimed` events for the given wallet. Hits the chain
   * directly via a public RPC; does not go through the partner proxy or
   * Biyard. Used by the widget to show "already claimed" state after a page
   * reload, including from a different browser / device.
   *
   * Requires the BrandToken contract address (from `getTokenInfo()`).
   */
  async getClaimHistory(
    walletAddress: string,
    contractAddress: string,
  ): Promise<OnChainClaimEvent[]> {
    return getClaimHistory(walletAddress, contractAddress, this.chainId);
  }

  /**
   * Currently connected wallet address, or null. Does not prompt.
   */
  getWalletAddress(): Promise<string | null> {
    return getWalletAddress();
  }

  private async request<T>(path: string, init: RequestInit): Promise<T> {
    const res = await this.fetcher(path, init);
    if (!res.ok) {
      const body = await res.text().catch(() => "");
      throw new ProxyRequestError(res.status, body, path);
    }
    return (await res.json()) as T;
  }
}
