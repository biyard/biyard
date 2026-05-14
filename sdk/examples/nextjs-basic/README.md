# `examples/nextjs-basic`

Minimal Next.js (App Router) integration of `@biyard/sdk` + `@biyard/react`.

## What this shows

- A partner backend (Next.js Route Handlers under `app/api/biyard/`) that proxies two Biyard endpoints with the partner's API key.
- A client page that uses `<BiyardProvider>`, `useClaimable`, and `<ClaimButton>` to render claimable months and trigger the wallet flow.
- The **security rule** in practice: `meta_user_id` is read from `currentMetaUserId()` on the server, never from the request body. See [`lib/session.ts`](./lib/session.ts) and the comments in the route handlers.

## Run

```bash
cp .env.example .env.local
# fill BIYARD_API_KEY + BIYARD_PROJECT_ID
pnpm install        # from repo root
pnpm --filter biyard-sdk-example-nextjs dev
```

Open <http://localhost:3100>.

To impersonate different demo users locally:

```
curl -H 'x-demo-user: demo_user_42' http://localhost:3100/api/biyard/claimable
```

## Files

- [`lib/biyard-server.ts`](./lib/biyard-server.ts) — server-only Biyard fetcher (holds API key).
- [`lib/session.ts`](./lib/session.ts) — stub partner session that returns `meta_user_id`. Replace with real auth.
- [`app/api/biyard/claimable/route.ts`](./app/api/biyard/claimable/route.ts) — proxy for `GET /claimable`.
- [`app/api/biyard/claim-signature/route.ts`](./app/api/biyard/claim-signature/route.ts) — proxy for `POST /claim-signature`.
- [`app/page.tsx`](./app/page.tsx) — client UI.
