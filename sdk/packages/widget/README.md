# `@biyard/widget`

Drop-in `<biyard-claim>` Web Component for embedding the Biyard token-claim flow in any web page.

The widget is designed to feel like a part of the partner's product. Default theme follows the partner's surrounding colors and font; the only Biyard mark is a single 11px footer link ("Secured by Biyard ↗") that can be set to `minimal` or hidden entirely.

## Quick start (CDN)

```html
<script type="module" src="https://cdn.biyard.io/widget.js"></script>

<biyard-claim
  base-url="/api/biyard"
  chain-id="1001"
  month="2026-01"
></biyard-claim>
```

## Quick start (npm)

```bash
pnpm add @biyard/widget @biyard/sdk ethers
```

```ts
import { defineBiyardClaim } from "@biyard/widget";
defineBiyardClaim();
```

```html
<biyard-claim base-url="/api/biyard" chain-id="1001" month="2026-01"></biyard-claim>
```

## Attributes

| Attribute | Default | Description |
|---|---|---|
| `base-url` | required | Partner proxy base URL. |
| `chain-id` | required | EVM chain id (e.g. `1001` Kaia Kairos, `8217` Kaia mainnet). |
| `month` | required | Month to claim, e.g. `"2026-01"`. |
| `mode` | `modal` | `modal` (button → centered dialog) or `inline` (renders the card directly). |
| `theme` | `auto` | `auto` (follow OS), `light`, or `dark`. |
| `branding` | `default` | `default` (Secured by Biyard ↗), `minimal` (via Biyard), or `none`. |
| `label` | `Claim` | Trigger button label (modal mode only). |
| `title` | `Claim tokens` | Title shown inside the card. |
| `subtitle` | — | Optional subtitle line under the title. |
| `amount` | (auto) | Raw uint256 amount override. If omitted, widget fetches from `${baseUrl}/claimable`. |
| `symbol` | (auto) | Token symbol override. Auto-fetched from `${baseUrl}/token` if omitted. |
| `decimals` | `18` | Decimals for amount display. |

## Theming

Set any of these CSS custom properties on the host element. They're the entire theming surface:

```css
biyard-claim {
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

`--biyard-font-family` defaults to `inherit`, so the widget picks up the host page's typeface automatically.

## Events

The widget dispatches:

| Event | Detail |
|---|---|
| `biyard-claim-success` | `{ tx_hash: string }` |
| `biyard-claim-error` | `{ error: Error }` |
| `biyard-open` / `biyard-close` | — (modal mode) |

```js
document.querySelector("biyard-claim").addEventListener(
  "biyard-claim-success",
  (e) => console.log("tx:", e.detail.tx_hash),
);
```

## Phishing protection

Before invoking the wallet's signature prompt, the widget renders the on-chain `verifyingContract` address (truncated) and chain name in the review panel. Users should verify these match what their wallet shows in the EIP-712 prompt.
