# Biyard SDK Integration Guide

This guide explains how a partner service embeds the Biyard token-claim flow on its own site.

## Architecture

```
   Browser                Partner server                Biyard
   ─────────              ─────────────                 ──────
   @biyard/sdk
   or
   <biyard-claim>  ──►   /api/biyard/*       ──►       api.biyard.co
                          (proxy, holds              (Authorization:
                           Biyard API key)            Bearer <api_key>)
```

- The browser SDK / widget calls **the partner's own proxy endpoints**, never `api.biyard.co` directly.
- The partner proxy is a thin pass-through that:
  1. Authenticates the request with the partner's existing session.
  2. Looks up the `meta_user_id` from that session (NOT from the request body).
  3. Calls Biyard with the partner's API key (`Authorization: Bearer biyard_…`).
  4. Returns Biyard's response unchanged.

## Security rule (read this)

> **The partner proxy MUST derive `meta_user_id` from its own server-side session. It MUST NOT accept `meta_user_id` from the browser.**

If you take `meta_user_id` from the request body or query string, any user can impersonate any other user and claim their tokens.

### DO

```ts
// partner backend
app.post("/api/biyard/claim-signature", async (req, res) => {
  const user = await authenticate(req); // partner's own session
  const { month, wallet_address } = req.body;

  const r = await biyardServer({
    meta_user_id: user.id, // ← from session, never from req.body
    month,
    wallet_address,
  });
  res.json(r);
});
```

### DON'T

```ts
// VULNERABLE — never do this.
app.post("/api/biyard/claim-signature", async (req, res) => {
  const { meta_user_id, month, wallet_address } = req.body; // ❌
  const r = await biyardServer({ meta_user_id, month, wallet_address });
  res.json(r);
});
```

## Browser integration

### Option 1: SDK (`@biyard/sdk`)

```bash
pnpm add @biyard/sdk ethers
```

```ts
import { BiyardClaim } from "@biyard/sdk";

const biyard = new BiyardClaim({
  baseUrl: "/api/biyard",
  chainId: 8217, // Kaia Mainnet
});

const { months } = await biyard.getClaimable();
const result = await biyard.claim("2025-04");
console.log(result.tx_hash);
```

Works in any framework (React, Vue, Svelte, vanilla). React is just one consumer — see the example app.

### Option 2: Drop-in widget (`@biyard/widget`)

```html
<script type="module" src="https://cdn.biyard.io/widget.js"></script>

<biyard-claim
  base-url="/api/biyard"
  chain-id="8217"
  month="2025-04"
></biyard-claim>
```

No build step required. Works in vanilla HTML, server-rendered pages, Wordpress, Vue/Svelte/Angular components — anywhere a `<script type="module">` runs.

Listen for events:

```js
document.querySelector("biyard-claim").addEventListener(
  "biyard-claim-success",
  (e) => console.log("tx:", e.detail.tx_hash),
);
```

## Partner proxy endpoints

The SDK and widget call these endpoints under `baseUrl`:

| Method | Path | Body | Returns |
|---|---|---|---|
| GET | `/claimable` | — | `ClaimableResponse` |
| POST | `/claim-signature` | `{ month, wallet_address }` | `ClaimSignatureResponse` |

Each proxy endpoint forwards to the corresponding Biyard endpoint:

| Partner proxy | Biyard endpoint |
|---|---|
| `GET /api/biyard/claimable` | `GET /v1/projects/:project_id/tokens/claimable?meta_user_id=...` |
| `POST /api/biyard/claim-signature` | `POST /v1/projects/:project_id/tokens/claim-signature` |

See [`examples/nextjs-basic/lib/biyard-server.ts`](../examples/nextjs-basic/lib/biyard-server.ts) for a reference implementation.

## DTOs

```ts
interface ClaimableMonth {
  month: string;
  user_points: number;
  total_points: number;
  claimable_tokens: string;   // raw ERC-20 units, string to avoid precision loss
  already_claimed: string;
  remaining: string;
}

interface ClaimableResponse {
  months: ClaimableMonth[];
}

interface ClaimSignatureResponse {
  month_index: string;
  amount: string;
  max_claimable: string;
  nonce: string;
  deadline: string;
  signature: string;          // 0x-prefixed hex
  contract_address: string;
  chain_id: number;
}
```

## Supported chains

| Chain | `chainId` |
|---|---|
| Kaia Mainnet | `8217` |
| Kaia Kairos Testnet | `1001` |
| Ethereum Mainnet | `1` |

## Wallets

The SDK uses the EIP-1193 provider exposed at `window.ethereum`. This covers MetaMask, Kaia Wallet, and most browser wallets. Mobile WalletConnect via QR is not bundled in v1 — partners that need it can wrap their own connection layer and pass a pre-connected signer (see core source).
