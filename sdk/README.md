# Biyard SDK

Browser SDK for partner services to embed the Biyard token-claim flow.

## Packages

- [`@biyard/sdk`](./packages/core) — framework-agnostic TypeScript core. Wallet connect + on-chain claim submit + partner-proxy fetch. No React, no DOM rendering. Use this from React/Vue/Svelte/vanilla.
- [`@biyard/widget`](./packages/widget) — drop-in Web Component (`<biyard-claim>`). Framework-agnostic. The right choice for partners who just want a `<script>` tag + one HTML element.

There is intentionally no separate React package. React consumers use `@biyard/sdk` directly (see [`examples/nextjs-basic`](./examples/nextjs-basic)) or embed `<biyard-claim>` from `@biyard/widget`.

## Integration model

The SDK does **not** call the Biyard API directly. The partner runs a small proxy on its own backend that authenticates the end user with the partner's session, then forwards the request to Biyard with the partner's Biyard API key. The SDK is configured with the partner proxy's `baseUrl`.

See [docs/integration.md](./docs/integration.md) for the integration guide and the security rule that prevents `meta_user_id` forgery.

## Examples

- [`examples/nextjs-basic`](./examples/nextjs-basic) — Next.js (App Router) app with proxy endpoints + a React claim UI built on `@biyard/sdk`, plus a `/widget` page showing `<biyard-claim>` usage.

## Local development

```bash
pnpm install
pnpm build
pnpm --filter biyard-sdk-example-nextjs dev
```
