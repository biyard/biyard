export { BiyardClaim } from "./client";
export type { BiyardClaimOptions, BiyardFetcher } from "./client";

export { submitClaim } from "./claim";
export { getClaimHistory } from "./history";
export type { OnChainClaimEvent } from "./history";
export { connectWallet, getWalletAddress } from "./wallet";
export { KNOWN_CHAINS } from "./chains";
export type { ChainMetadata } from "./chains";

export type {
  ClaimableMonth,
  ClaimableResponse,
  ClaimSignatureRequestBody,
  ClaimSignatureResponse,
  ClaimSubmitResult,
  TokenInfo,
} from "./types";

export {
  BiyardError,
  ProxyRequestError,
  UnsupportedChainError,
  UserRejectedError,
  WalletNotInstalledError,
} from "./errors";
