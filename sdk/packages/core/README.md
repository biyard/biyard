# `@biyard/sdk`

Framework-agnostic browser SDK for Biyard token claim. Handles wallet connection (MetaMask / Kaia Wallet) + on-chain submit. Calls the partner's proxy endpoints, never Biyard directly.

## Install

```bash
pnpm add @biyard/sdk ethers
```

## Quick start

```ts
import { BiyardClaim } from "@biyard/sdk";

const biyard = new BiyardClaim({
  baseUrl: "/api/biyard", // partner proxy
  chainId: 8217,          // Kaia Mainnet
});

const { months } = await biyard.getClaimable();
const result = await biyard.claim("2025-04");
console.log("tx:", result.tx_hash);
```

## Partner proxy contract

The SDK expects the partner backend to expose, under `baseUrl`:

| Method | Path | Body | Returns |
|---|---|---|---|
| GET | `/claimable` | — | `ClaimableResponse` |
| POST | `/claim-signature` | `{ month, wallet_address }` | `ClaimSignatureResponse` |

The proxy MUST resolve `meta_user_id` from its own session and forward it to Biyard server-side. The SDK never sends `meta_user_id`.

See [docs/integration.md](../../docs/integration.md) for the full integration guide.
