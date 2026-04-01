# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.
Detailed conventions are in `.claude/rules/` (auto-loaded, scoped by file glob).

## Project Overview

Biyard is a Launchpad-like SaaS and PaaS platform that enables users to create projects and manage points and tokens over blockchain through our PaaS APIs.

**Repository:** https://github.com/biyard/biyard

- **SaaS/PaaS Platform:** Launchpad-style service for blockchain projects
- **PaaS APIs:** Services consume Biyard APIs to manage points and tokens on blockchain
- **Target Users:** Project creators who need blockchain token/point management infrastructure

## Important Constraints

- **DO NOT modify or add code in the `api/` package.** The `api/` package is legacy and no longer under active development. All new backend work should go through Dioxus fullstack server functions in `console/`.

## Monorepo Structure

Rust workspace managed by Cargo. Frontend migrating to Dioxus fullstack.

```
biyard/
├── api/              # Rust backend (LEGACY — do not develop)
├── console/          # Dioxus fullstack frontend (SSR + WASM hydration)
├── landing/          # Dioxus landing page (SSR + WASM)
├── packages/         # Shared Rust libraries
│   ├── btracing/     # Tracing wrapper (v0.1.*)
│   ├── by-axum/      # Axum framework wrapper (v0.2.*)
│   ├── by-macros/    # Procedural macros — DynamoEntity, DynamoEnum (v0.6.*)
│   └── by-types/     # Shared types (v0.3.*)
├── contracts/        # Solidity smart contracts (Hardhat)
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
cd console
DYNAMO_TABLE_PREFIX=biyard-dev dx serve --port 8000 --web   # Console dev server
dx build --release @client --features web --platform web \
  @server --features server --platform server               # Production build

cd landing
dx serve --port 8001 --web                                  # Landing dev server
```

## Technology Stack

- **Backend:** Rust 2024, Axum (via by-axum), Tokio, DynamoDB (AWS SDK), Askama (SSR), Serde, Schemars
- **Frontend:** Dioxus 0.7 fullstack (SSR + WASM), TailwindCSS v4, wasm_bindgen (JS interop)
- **Database:** DynamoDB single-table design, DynamoEntity derive macro (by-macros)
- **Blockchain:** Solidity (Hardhat), ethers-rs
- **Infrastructure:** AWS CDK, Lambda, S3, Docker

## Build Verification

1. `cargo build` — backend compiles
2. `cargo test` — backend tests pass
3. `cd console && dx build` — console frontend compiles
4. `cd landing && dx build` — landing frontend compiles

## Frontend Code Quality Rules

### File Size & Structure
- **Single file must not exceed ~300 lines.** If a page or component grows beyond this, split into a directory module (e.g., `home.rs` → `home/mod.rs` + sub-files).
- **Split by logical section**, not by technical layer. Each visual section (Hero, FAQ, Footer, etc.) should be its own file.
- **Remove dead code immediately.** Do not leave unused components, structs, or constants in the codebase.

### Avoid `dangerous_inner_html`
- **Never use `dangerous_inner_html` for CSS.** Extract to external `.css` files in `assets/` and load via `document::Link { rel: "stylesheet", href: asset!("/assets/file.css") }`.
- **Never use `dangerous_inner_html` for JavaScript.** Extract to external `.js` files in `assets/` and load via `document::Script { src: asset!("/assets/file.js") }`.
- **Prefer native RSX SVG elements** over `dangerous_inner_html` for SVGs. Dioxus 0.7 supports `svg {}`, `path {}`, `circle {}`, `rect {}`, `line {}` etc. natively. Use snake_case attributes: `view_box`, `stroke_width`, `stroke_linecap`, `stroke_dasharray`.
- **Only use `dangerous_inner_html` for SVGs** when they contain elements not well-supported in RSX (e.g., `<text>`, `<defs>`, `<marker>`). In that case, define as named constants in a dedicated `svgs.rs` file.

### Inline Styles
- Prefer TailwindCSS utility classes over inline `style` attributes where possible.
- For CSS animations (`@keyframes`), define in external CSS files, not inline Rust string constants.

## Platform Considerations

- **Blockchain:** PaaS APIs handle blockchain ops; design for async processing (webhooks, callbacks)
- **Multi-Tenancy:** Projects isolated via DynamoDB partition keys; validate ownership on every request
