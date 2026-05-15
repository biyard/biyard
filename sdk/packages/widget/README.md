# `@biyard/widget`

Drop-in Biyard Web Components. Framework-agnostic — works in vanilla HTML, Vue, Svelte, server-rendered pages, or any host where a `<script type="module">` runs.

## Components

| Tag | Purpose |
|---|---|
| `<biyard-claim>` | Wallet connect + on-chain claim flow (modal or inline). |
| `<biyard-balance>` | Point balance + on-chain token balance card. |
| `<biyard-transactions>` | Paginated point activity list. |
| `<biyard-monthly-summary>` | Per-month earned / spent / balance summary. |

## Quick start (CDN)

```html
<script type="module" src="https://cdn.biyard.io/widget.js"></script>

<biyard-balance base-url="/api/biyard" month="2026-01"></biyard-balance>
<biyard-monthly-summary base-url="/api/biyard"></biyard-monthly-summary>
<biyard-transactions base-url="/api/biyard" limit="10"></biyard-transactions>
<biyard-claim base-url="/api/biyard" month="2026-01"></biyard-claim>
```

The CDN entry auto-registers every tag.

## Quick start (npm)

```bash
pnpm add @biyard/widget @biyard/sdk ethers
```

```ts
import { defineBiyardWidgets } from "@biyard/widget";
defineBiyardWidgets();
```

Or import + register individually:

```ts
import { defineBiyardBalance, defineBiyardClaim } from "@biyard/widget";
defineBiyardBalance();
defineBiyardClaim();
```

## Shared attributes

These work on every Biyard widget:

| Attribute | Default | Description |
|---|---|---|
| `base-url` | required | Partner proxy base URL. |
| `theme` | `auto` | `auto` / `light` / `dark`. |
| `lang` | (auto-detect) | `en` / `ko`. Falls back to `<html lang>` / `navigator.language`. |
| `branding` | `default` | `default` / `minimal` / `none` — controls the 11px footer attribution. |
| `title` | (per-widget i18n) | Override the card title. |

Plus per-widget attributes documented in the source.

## Theming

Set any of these CSS custom properties on the host element:

```css
biyard-balance, biyard-claim, biyard-transactions, biyard-monthly-summary {
  --biyard-color-accent: #6366f1;
  --biyard-color-accent-foreground: #ffffff;
  --biyard-color-bg: #ffffff;
  --biyard-color-surface: #f9fafb;
  --biyard-color-text: #111827;
  --biyard-color-muted: #6b7280;
  --biyard-color-border: #e5e7eb;
  --biyard-color-danger: #dc2626;
  --biyard-color-success: #059669;
  --biyard-font-family: "Inter", system-ui, sans-serif;
  --biyard-radius: 12px;
  --biyard-spacing: 4px;
}
```

`--biyard-font-family` defaults to `inherit`.

## Phishing protection

`<biyard-claim>` renders the on-chain `verifyingContract` address (truncated, with hover-tooltip + clickable link to the chain explorer) before invoking the wallet's signature prompt. Users should verify it matches what their wallet shows in the EIP-712 dialog.

## Where the data comes from

The widgets call the partner's proxy endpoints under `base-url`:

| Widget | Endpoints called |
|---|---|
| `<biyard-claim>` | `/token`, `/claimable`, `/claim-signature`, and the wallet RPC for `Claimed` events. |
| `<biyard-balance>` | `/token`, `/balance`, wallet RPC `balanceOf`. |
| `<biyard-transactions>` | `/transactions` (with `?limit` + `?bookmark` pagination). |
| `<biyard-monthly-summary>` | `/monthly-summaries`. |

See [`docs/integration.md`](../../docs/integration.md) for the partner-proxy contract.
