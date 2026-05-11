import {
  BrowserProvider,
  Contract,
  JsonRpcProvider,
  type AbstractProvider,
  type EventLog,
  type Eip1193Provider,
} from "ethers";

import { KNOWN_CHAINS } from "./chains";
import { UnsupportedChainError } from "./errors";

interface EthereumWindow {
  ethereum?: Eip1193Provider;
}

const CLAIMED_ABI = [
  "event Claimed(address indexed user, uint256 indexed month, uint256 amount, uint256 nonce)",
] as const;

export interface OnChainClaimEvent {
  /** 0-indexed month within the token's lifecycle (matches `month_index` from the signature). */
  month_index: string;
  /** Claimed amount in raw uint256 (ERC-20 units). */
  amount: string;
  /** EIP-712 nonce used at claim time. */
  nonce: string;
  /** Transaction hash that produced the event. */
  tx_hash: string;
  /** Block number the event was mined in. */
  block_number: number;
}

/**
 * Read past `Claimed(address indexed user, uint256 indexed month, …)` events
 * for the given wallet from the BrandToken contract.
 *
 * Provider resolution:
 *   1. `opts.provider` if given.
 *   2. The user's wallet (`window.ethereum`) — this is the **default in the
 *      browser** because public RPC endpoints typically lack CORS headers,
 *      so direct `fetch` to them fails. Wallet-internal RPC has no CORS
 *      constraint.
 *   3. Fallback to the public RPC URL configured in `KNOWN_CHAINS`. This
 *      path works in Node / Deno but generally fails in browsers.
 *
 * On-chain is the authoritative source. A user who claimed on a different
 * browser / device still sees their prior claim here.
 *
 * @param walletAddress - 0x-prefixed wallet address to filter by.
 * @param contractAddress - BrandToken contract address.
 * @param chainId - Chain id.
 * @param opts.provider - Optional provider override.
 * @param opts.fromBlock - Earliest block to scan. Default `0`.
 * @param opts.toBlock - Latest block to scan. Default `"latest"`.
 */
export async function getClaimHistory(
  walletAddress: string,
  contractAddress: string,
  chainId: number,
  opts: {
    provider?: AbstractProvider;
    fromBlock?: number | "earliest";
    toBlock?: number | "latest";
  } = {},
): Promise<OnChainClaimEvent[]> {
  const provider = opts.provider ?? defaultProviderFor(chainId);
  const contract = new Contract(contractAddress, CLAIMED_ABI, provider);

  const filter = contract.filters.Claimed!(walletAddress);
  const logs = await contract.queryFilter(
    filter,
    opts.fromBlock ?? 0,
    opts.toBlock ?? "latest",
  );

  return logs.map((log) => {
    // ethers v6 returns EventLog when ABI matches.
    const e = log as EventLog;
    const args = e.args;
    return {
      month_index: (args[1] as bigint).toString(),
      amount: (args[2] as bigint).toString(),
      nonce: (args[3] as bigint).toString(),
      tx_hash: log.transactionHash,
      block_number: log.blockNumber,
    };
  });
}

function defaultProviderFor(chainId: number): AbstractProvider {
  // Prefer the wallet's injected provider — public RPC endpoints lack CORS
  // headers and will fail when fetched directly from the browser.
  const eth = (globalThis as unknown as EthereumWindow).ethereum;
  if (eth) {
    return new BrowserProvider(eth, chainId);
  }
  const meta = KNOWN_CHAINS[chainId];
  if (!meta) throw new UnsupportedChainError(chainId);
  const url = meta.rpcUrls[0];
  if (!url) throw new UnsupportedChainError(chainId);
  return new JsonRpcProvider(url, chainId);
}

