# syntax=docker/dockerfile:1.7
# Build context must be the sdk/ workspace root, not the example folder.
#   cd sdk && docker build -f examples/nextjs-basic/Dockerfile -t biyard-sdk-example .

FROM node:20-alpine AS base
RUN corepack enable && corepack prepare pnpm@10.15.0 --activate
WORKDIR /app

# ─── Dependencies layer ──────────────────────────────────────────────
FROM base AS deps
COPY pnpm-workspace.yaml pnpm-lock.yaml package.json ./
COPY packages/widget/package.json packages/widget/
COPY packages/core/package.json packages/core/
COPY examples/nextjs-basic/package.json examples/nextjs-basic/
RUN --mount=type=cache,id=pnpm,target=/root/.local/share/pnpm/store \
    pnpm install --frozen-lockfile

# ─── Builder layer ───────────────────────────────────────────────────
FROM base AS builder
COPY --from=deps /app/node_modules ./node_modules
COPY --from=deps /app/packages/widget/node_modules ./packages/widget/node_modules
COPY --from=deps /app/packages/core/node_modules ./packages/core/node_modules
COPY --from=deps /app/examples/nextjs-basic/node_modules ./examples/nextjs-basic/node_modules
COPY . .

# Server-side env vars are NOT baked into the image — they are injected at
# runtime via docker-compose env_file. Only NEXT_PUBLIC_* would need to be
# present at build time, and this app does not use any.
RUN pnpm --filter "@biyard/sdk" build && \
    pnpm --filter "@biyard/widget" build && \
    pnpm --filter "biyard-sdk-example-nextjs" build

# ─── Runner layer ────────────────────────────────────────────────────
FROM node:20-alpine AS runner
WORKDIR /app
ENV NODE_ENV=production
ENV PORT=3000
ENV HOSTNAME=0.0.0.0

RUN addgroup --system --gid 1001 nodejs && \
    adduser --system --uid 1001 nextjs

COPY --from=builder --chown=nextjs:nodejs \
    /app/examples/nextjs-basic/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs \
    /app/examples/nextjs-basic/.next/static ./examples/nextjs-basic/.next/static

USER nextjs
EXPOSE 3000
CMD ["node", "examples/nextjs-basic/server.js"]
