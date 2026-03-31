# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.
Detailed conventions are in `.claude/rules/` (auto-loaded, scoped by file glob).

## Project Overview

Biyard is a Launchpad-like SaaS and PaaS platform that enables users to create projects and manage points and tokens over blockchain through our PaaS APIs.

**Repository:** https://github.com/biyard/biyard

- **SaaS/PaaS Platform:** Launchpad-style service for blockchain projects
- **PaaS APIs:** Services consume Biyard APIs to manage points and tokens on blockchain
- **Target Users:** Project creators who need blockchain token/point management infrastructure

## Monorepo Structure

Rust workspace managed by Cargo. Frontend migrating to Dioxus fullstack.

```
biyard/
├── api/              # Rust backend (Axum REST APIs + Askama SSR)
├── app/              # Dioxus fullstack frontend (SSR + WASM hydration)
├── packages/         # Shared Rust libraries
│   ├── btracing/     # Tracing wrapper (v0.1.*)
│   ├── by-axum/      # Axum framework wrapper (v0.2.*)
│   ├── by-macros/    # Procedural macros — DynamoEntity, DynamoEnum (v0.6.*)
│   └── by-types/     # Shared types (v0.3.*)
├── contracts/        # Solidity smart contracts (Hardhat)
├── console/          # React admin dashboard (legacy, migrating to Dioxus)
├── landing/          # React landing page (legacy, migrating to Dioxus)
└── cdk/              # AWS CDK infrastructure
```

## Development Methodology

**Feature-Driven Development:** Organize code by business domain, not technical layers.
Each feature is self-contained under `features/<name>/` with its own models, DTOs, types, and utils.

## Quick-Start Commands

### Backend
```bash
cd api
cargo build          # Build
cargo run            # Run server
cargo test           # Run tests
cargo fmt            # Format
cargo clippy         # Lint
```

### Frontend (Dioxus)
```bash
cd app
DYNAMO_TABLE_PREFIX=biyard-dev dx serve --port 8000 --web   # Dev server
dx build --release @client --features web --platform web \
  @server --features server --platform server               # Production build
```

### Legacy Frontend (console/landing)
```bash
cd console && pnpm install && pnpm dev    # Console dev server
cd landing && pnpm install && pnpm dev    # Landing dev server
```

## Technology Stack

- **Backend:** Rust 2024, Axum (via by-axum), Tokio, DynamoDB (AWS SDK), Askama (SSR), Serde, Schemars
- **Frontend:** Dioxus 0.7 fullstack (SSR + WASM), TailwindCSS v4, wasm_bindgen (JS interop)
- **Database:** DynamoDB single-table design, DynamoEntity derive macro (by-macros)
- **Blockchain:** Solidity (Hardhat), ethers-rs
- **Infrastructure:** AWS CDK, Lambda, S3, Docker
- **Legacy Frontend:** React 19, Vite, TypeScript, Shadcn UI, React Query

## Build Verification

1. `cargo build` — backend compiles
2. `cargo test` — backend tests pass
3. `dx build` — Dioxus frontend compiles (when app/ is set up)
4. `cd console && pnpm build` — legacy console builds

## Platform Considerations

- **Blockchain:** PaaS APIs handle blockchain ops; design for async processing (webhooks, callbacks)
- **Multi-Tenancy:** Projects isolated via DynamoDB partition keys; validate ownership on every request
