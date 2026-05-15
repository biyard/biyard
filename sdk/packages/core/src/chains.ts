/**
 * Kaia + Ethereum network metadata used for `wallet_addEthereumChain` /
 * `wallet_switchEthereumChain` requests.
 *
 * Add new entries here as Biyard expands chain support.
 */

export interface ChainMetadata {
  chainId: string; // 0x-prefixed hex
  chainName: string;
  nativeCurrency: { name: string; symbol: string; decimals: number };
  rpcUrls: string[];
  blockExplorerUrls: string[];
}

export const KNOWN_CHAINS: Record<number, ChainMetadata> = {
  1001: {
    chainId: "0x3e9",
    chainName: "Kaia Kairos Testnet",
    nativeCurrency: { name: "KAIA", symbol: "KAIA", decimals: 18 },
    rpcUrls: ["https://public-en-kairos.node.kaia.io"],
    blockExplorerUrls: ["https://kairos.kaiascan.io"],
  },
  8217: {
    chainId: "0x2019",
    chainName: "Kaia Mainnet",
    nativeCurrency: { name: "KAIA", symbol: "KAIA", decimals: 18 },
    rpcUrls: ["https://public-en.node.kaia.io"],
    blockExplorerUrls: ["https://kaiascan.io"],
  },
  1: {
    chainId: "0x1",
    chainName: "Ethereum Mainnet",
    nativeCurrency: { name: "ETH", symbol: "ETH", decimals: 18 },
    rpcUrls: ["https://cloudflare-eth.com"],
    blockExplorerUrls: ["https://etherscan.io"],
  },
};
