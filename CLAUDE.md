# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.
Detailed conventions are in `.claude/rules/` (auto-loaded, scoped by file glob).

## Project Overview

Biyard is a Launchpad-like SaaS and PaaS platform that enables users to create projects and manage points and tokens over blockchain through our PaaS APIs.

**Repository:** https://github.com/biyard/biyard

- **SaaS/PaaS Platform:** Launchpad-style service for blockchain projects
- **PaaS APIs:** Services consume Biyard APIs to manage points and tokens on blockchain
- **Target Users:** Project creators who need blockchain token/point management infrastructure

## Domain Decisions (Session Memory)

These decisions are agreed and should be treated as default unless explicitly changed by the user.

### Terminology
- **UI label:** Use **Brand(브랜드)** in user-facing screens.
- **Backend/domain canonical name:** Keep **Project** in code, routes, and data model for now.
- **Rule:** Do not rename `Project` structs/modules/routes (`/v1/projects/*`) unless user explicitly requests a full migration.
- **Meaning:** In current product scope, **Brand == Project (presentation alias)**.

### Multi-Tenancy Direction (B2B)
- Service is B2B, so tenancy should center on **Organization**.
- Membership is attached to **Organization** (not directly to project) as the primary access boundary.
- Project/Brand belongs to one Organization.
- Users join Organizations via membership and then access Organization-owned projects.

### Relationship Model (Target)
- `Organization 1:N Membership`
- `User 1:N Membership`
- `Organization 1:N Project`
- `Project 1:N Token`
- `Project 1:N PointTransaction / PointBalance / AuditLog`

### RBAC Baseline (Organization Scope)
- `Owner`: organization billing, membership, all projects, destructive ops
- `Admin`: manage projects/tokens/points and member invitations, no owner transfer
- `Viewer`: read-only access to dashboard/usage/audit

### Token Immutability Rule
- Token name/symbol are treated as immutable after issuance (blockchain-aligned expectation).
- UI/API should avoid post-issuance rename flows.

### Current Implementation Guardrails
- Current code still uses `account_id`-based ownership in several places. This is acceptable short-term.
- Near-term product work should prioritize UI wording and feature completion over risky schema migration.
- If Organization migration is started later, do staged migration with dual-read/dual-write plan.

### DynamoDB Modeling Notes (Planned)
- Keep single-table design and query-first access patterns.
- Recommended entities:
  - `PK=ORG#{org_id}, SK=META`
  - `PK=ORG#{org_id}, SK=MEMBER#{user_id}`
  - `PK=ORG#{org_id}, SK=PROJECT#{project_id}`
  - `PK=PROJECT#{project_id}, SK=META`
  - `PK=PROJECT#{project_id}, SK=TOKEN#{token_id}`
  - `PK=PROJECT#{project_id}, SK=AUDIT#{timestamp}#{event_id}`
- Use GSIs for:
  - user -> organizations lookup
  - cross-project admin/audit listing
  - time-ordered event retrieval when needed

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
