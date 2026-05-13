import { BrowserProvider, type Eip1193Provider } from "ethers";

import { KNOWN_CHAINS } from "./chains";
import {
  UnsupportedChainError,
  UserRejectedError,
  WalletNotInstalledError,
} from "./errors";

interface EthereumWindow {
  ethereum?: Eip1193Provider & {
    request(args: { method: string; params?: unknown[] | object }): Promise<unknown>;
  };
}

function getEthereum() {
  const eth = (globalThis as unknown as EthereumWindow).ethereum;
  if (!eth || typeof eth.request !== "function") {
    throw new WalletNotInstalledError();
  }
  return eth;
}

async function ensureNetwork(chainId: number): Promise<void> {
  const meta = KNOWN_CHAINS[chainId];
  if (!meta) throw new UnsupportedChainError(chainId);

  const eth = getEthereum();
  const current = (await eth.request({ method: "eth_chainId" })) as string;
  if (parseInt(current, 16) === parseInt(meta.chainId, 16)) return;

  try {
    await eth.request({
      method: "wallet_switchEthereumChain",
      params: [{ chainId: meta.chainId }],
    });
  } catch (err) {
    const code = (err as { code?: number })?.code;
    if (code === 4902) {
      // Chain unknown to wallet — add it.
      await eth.request({
        method: "wallet_addEthereumChain",
        params: [meta],
      });
    } else if (code === 4001) {
      throw new UserRejectedError("network switch");
    } else {
      throw err;
    }
  }
}

/**
 * Return the currently connected wallet address, if any. Does not prompt.
 */
export async function getWalletAddress(): Promise<string | null> {
  try {
    const eth = getEthereum();
    const accounts = (await eth.request({ method: "eth_accounts" })) as string[];
    return accounts.length > 0 ? (accounts[0] ?? null) : null;
  } catch {
    return null;
  }
}

/**
 * Connect the wallet and switch to the given chain. Prompts the user.
 */
export async function connectWallet(chainId: number): Promise<string> {
  const eth = getEthereum();
  await ensureNetwork(chainId);

  let accounts: string[];
  try {
    accounts = (await eth.request({ method: "eth_requestAccounts" })) as string[];
  } catch (err) {
    if ((err as { code?: number })?.code === 4001) {
      throw new UserRejectedError("wallet connection");
    }
    throw err;
  }

  const account = accounts[0];
  if (!account) {
    throw new UserRejectedError("wallet connection");
  }
  return account;
}

/**
 * Internal: get a connected signer for the given chain.
 */
export async function getSigner(chainId: number) {
  await connectWallet(chainId);
  const eth = getEthereum();
  // ethers v6 BrowserProvider accepts the EIP-1193 provider directly.
  const provider = new BrowserProvider(eth as Eip1193Provider);
  return provider.getSigner();
}
