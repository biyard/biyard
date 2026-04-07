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
- **UI label:** Use **Brand(브랜드)** in user-facing screens for projects.
- **UI label:** Use **Enterprise(기업)** in user-facing screens for the multi-tenant container. Do not use "Workspace", "Organization", or "Company" as alternative labels in UI copy, i18n keys, or URL slugs.
- **Backend/domain canonical name:** Keep **Project** in code, routes, and data model for now.
- **Backend/domain canonical name:** Use **Enterprise** in code, types, and URL slugs (e.g., `/enterprise/*`, `EnterpriseContextAuth`). Do not reintroduce "Organization" as a parallel concept in new code or docs.
- **Rule:** Do not rename `Project` structs/modules/routes (`/v1/projects/*`) unless user explicitly requests a full migration.
- **Meaning:** In current product scope, **Brand == Project (presentation alias)** and **Enterprise == the B2B tenant container**.

### Multi-Tenancy Direction (B2B)
- Service is B2B, so tenancy is centered on **Enterprise**.
- Membership is attached to **Enterprise** (not directly to project) as the primary access boundary.
- Project/Brand belongs to one Enterprise.
- Users join Enterprises via membership and then access Enterprise-owned projects.
- **Short-term assumption (current):** An authenticated account operates within a **single current Enterprise** resolved from session. There is **no enterprise identifier in URLs**, and multi-enterprise switching is deferred. When multi-enterprise is introduced later, it will require explicit URL-level redesign (e.g., `/e/:enterprise_slug/...`) and is an explicit scope change, not an incremental tweak.

### Relationship Model (Target)
- `Enterprise 1:N Membership`
- `User 1:N Membership`
- `Enterprise 1:N Project`
- `Project 1:1 Token` (see Token Cardinality Rule)
- `Project 1:N PointTransaction / PointBalance / AuditLog`

### RBAC Baseline (Enterprise Scope)
- `Owner`: enterprise billing, membership, all projects, destructive ops
- `Admin`: manage projects/tokens/points and member invitations, no owner transfer
- `Viewer`: read-only access to dashboard/usage/audit

### Token Cardinality Rule
- Each Project has **at most one Token** (1:1). Product scope does not support multiple tokens per brand.
- URL/route shape reflects this: `/projects/:project_id/token/...` (singular path segment), not `/tokens/:token_id/...`.
- Server handlers that operate on "the project's token" resolve it from the project context, not from a token id in the URL.
- If multi-token ever becomes a requirement, this is an explicit scope change and requires route redesign plus data model migration.

### Token Immutability Rule
- Token name/symbol are treated as immutable after issuance (blockchain-aligned expectation).
- UI/API should avoid post-issuance rename flows.

### Current Implementation Guardrails
- Current code still uses `account_id`-based ownership in several places. This is acceptable short-term.
- Near-term product work should prioritize UI wording and feature completion over risky schema migration.
- If deeper Enterprise-centric ownership migration is started later, do staged migration with dual-read/dual-write plan.

### DynamoDB Modeling Notes (Planned)
- Keep single-table design and query-first access patterns.
- Recommended entities:
  - `PK=ENT#{enterprise_id}, SK=META`
  - `PK=ENT#{enterprise_id}, SK=MEMBER#{user_id}`
  - `PK=ENT#{enterprise_id}, SK=PROJECT#{project_id}`
  - `PK=PROJECT#{project_id}, SK=META`
  - `PK=PROJECT#{project_id}, SK=TOKEN` (singular — 1:1 with project)
  - `PK=PROJECT#{project_id}, SK=AUDIT#{timestamp}#{event_id}`
- Use GSIs for:
  - user -> enterprises lookup
  - cross-project admin/audit listing
  - time-ordered event retrieval when needed

## Important Constraints

- **Backend lives in `console/src/features/*/controllers/*.rs`** as handler functions using the `#[get(...)]` / `#[post(...)]` / `#[put(...)]` / `#[patch(...)]` / `#[delete(...)]` macros from `by-macros`. There is no separate `api/` crate. See [.claude/rules/server-functions.md](.claude/rules/server-functions.md).
- **Do not add crates to the workspace root** (`Cargo.toml`) without discussion. `tmpl_renderer/` and `packages/console-interop/` are intentionally outside `[workspace] members`.
- **Keep Brand/Project terminology rule:** UI says *Brand*, code/routes stay *Project*. Do not rename structs/modules/routes from `project` → `brand`.

## Monorepo Structure

Rust workspace managed by Cargo. Both `console/` and `landing/` are Dioxus 0.7 fullstack apps (SSR + WASM hydration).

```
biyard/
├── console/               # Dioxus fullstack app — UI + backend handlers
│                          #   (src/features/*/controllers/*.rs)
├── landing/               # Dioxus fullstack marketing site
├── packages/
│   └── console-interop/   # Local shared crate (NOT in workspace members)
├── contracts/             # Solidity smart contracts (Hardhat)
├── cdk/                   # AWS CDK infrastructure (TypeScript)
├── tmpl_renderer/         # Standalone Askama renderer utility (NOT in workspace)
├── playwright/            # E2E test suite
├── scripts/               # localstack-init, dynamodb-table.json, web-entrypoint
├── docs/                  # Brand / concept SVGs only — NOT technical docs
├── Makefile               # CDK build/deploy, landing S3 sync, CloudFront invalidation
├── docker-compose.yaml    # Local dev: LocalStack (DynamoDB/S3/SQS) + DynamoDB Admin UI
└── Cargo.toml             # Workspace members: ["console", "landing"]
```

**External shared crates (not in `packages/`):**
- `btracing` — crates.io `0.1.6`
- `by-macros`, `dioxus-translate` — pinned to `biyard/ratel.git` rev `867cb9be7b173c92acdeefe237dd87b4adcaca1f`

## Development Methodology

**Feature-Driven Development:** Organize code by business domain, not technical layers.
Each feature is self-contained under `console/src/features/<name>/` with its own
`models/`, `dto/`, `types/`, `controllers/`, `components/`, `views/`, and `i18n.rs`.

## Quick-Start Commands

```bash
# Local infra (LocalStack: DynamoDB, S3, SQS + DynamoDB Admin UI)
docker compose up -d

# Console (Dioxus fullstack — UI + server handlers)
cd console
DYNAMO_TABLE_PREFIX=biyard-dev dx serve --port 8000 --web

# Landing (Dioxus fullstack)
cd landing
dx serve --port 8001 --web

# Workspace-wide checks
cargo build
cargo fmt
cargo clippy

# Production build (console)
cd console
dx build --release \
  @client --features web --platform web \
  @server --features server --platform server

# E2E tests
cd playwright && npx playwright test

# CDK deploy (see Makefile for targets)
make <target>
```

## Technology Stack

- **App framework:** Rust 2024, Dioxus 0.7 fullstack (feature-gated: `full`/`web`/`server`/`lambda`), Tokio
- **Server handlers:** `#[get/post/put/patch/delete]` macros from `by-macros` (no standalone Axum wrapper)
- **Database:** DynamoDB single-table design, `DynamoEntity` / `DynamoEnum` derive macros, `serde_dynamo`
- **Frontend:** TailwindCSS v4, `wasm-bindgen` / `web-sys` / `js-sys`
- **i18n:** `dioxus-translate` (`ko` feature enabled)
- **Blockchain:** Solidity (Hardhat), `ethers-rs`
- **Infrastructure:** AWS CDK (TypeScript), Lambda (`lambda_http`), S3, Docker, LocalStack for dev
- **E2E:** Playwright

## Build Verification

1. `cargo build` — workspace (`console` + `landing`) compiles
2. `cd console && dx build` — console fullstack compiles
3. `cd landing && dx build` — landing fullstack compiles
4. E2E via Playwright under `playwright/`

> Note: `console/` and `landing/` currently have no unit tests. `cargo test` will pass trivially.

## Frontend Code Quality Rules

### Internationalization Discipline
- **Do not hardcode user-facing copy in UI code.** Any text visible to users must go through i18n (`translate!`, `i18n.rs`, `use_translate()`).
- **Always localize all chrome text**, including:
  - page titles, section headers, descriptions
  - buttons, menu items, tabs, badges, empty states
  - placeholders, dialog titles/descriptions, success/error/info messages
  - `aria-label`, `title`, tooltip text, helper text
- **Allowed exceptions:** runtime data values only, such as brand names, user names, emails, IDs, API-returned content, transaction hashes, and other domain data that should be rendered as-is.
- **Preferred workflow for UI work:** define or update translation keys first, then wire the UI to those keys. Do not leave temporary hardcoded English/Korean strings in place.
- **File organization:**
  - small/local components: keep `translate!` in the same file when reasonable
  - page/layout/view-level text: use or create the feature's dedicated `i18n.rs`
- **Completion checklist for any UI change:** before finishing, scan for newly introduced raw UI strings in RSX and ensure they are localized.
- **Code review rule:** missing i18n is a real defect, not a polish item.

### File Size & Structure
- **Single file must not exceed ~300 lines.** If a page or component grows beyond this, split into a directory module (e.g., `home.rs` → `home/mod.rs` + sub-files).
- **Split by logical section**, not by technical layer. Each visual section (Hero, FAQ, Footer, etc.) should be its own file.
- **Remove dead code immediately.** Do not leave unused components, structs, or constants in the codebase.

### Avoid `dangerous_inner_html`
- **Never use `dangerous_inner_html` for CSS.** Extract to external `.css` files in `assets/` and load via `document::Link { rel: "stylesheet", href: asset!("/assets/file.css") }`.
- **Never use `dangerous_inner_html` for JavaScript.** Extract to external `.js` files in `assets/` and load via `document::Script { src: asset!("/assets/file.js") }`.
- **Prefer native RSX SVG elements** over `dangerous_inner_html` for SVGs. Dioxus 0.7 supports `svg {}`, `path {}`, `circle {}`, `rect {}`, `line {}` etc. natively. Use snake_case attributes: `view_box`, `stroke_width`, `stroke_linecap`, `stroke_dasharray`.
- **Only use `dangerous_inner_html` for SVGs** when they contain elements not well-supported in RSX (e.g., `<text>`, `<defs>`, `<marker>`). In that case, define as named constants in a dedicated `svgs.rs` file.

### SVG Assets
- **Separate SVGs into dedicated files, not inline Rust code.** Place `.svg` files under `assets/` and load via `asset!("/assets/icon.svg")`, or use native RSX SVG elements (`svg {}`, `path {}`, etc.) in a dedicated component file.
- **Do not inline raw SVG markup as string constants** in component files. This bloats components and makes icons hard to reuse.

### RSX String Interpolation
- **For `String` / `&str` values, embed directly without quotes.** Use `{var}` instead of `"{var}"`:
  ```rust
  // Correct — owned String or &str
  div { {name} }
  button { {label} }

  // Wrong — unnecessary format string wrapping
  div { "{name}" }
  button { "{label}" }
  ```
- **Use `"{var}"` only when concatenating with literal text** or interpolating non-string types:
  ```rust
  div { "Hello, {name}!" }      // concatenation
  span { "Count: {count}" }     // non-string type
  ```

### Inline Styles
- Prefer TailwindCSS utility classes over inline `style` attributes where possible.
- For CSS animations (`@keyframes`), define in external CSS files, not inline Rust string constants.

### Tailwind Sizing & Spacing
- **Avoid arbitrary pixel-based Tailwind utilities whenever possible.** Prefer scale/token-based classes such as `p-4`, `gap-3`, `rounded-2xl`, `h-10`, `max-w-sm` instead of `px-[18px]`, `mt-[13px]`, `w-[274px]`.
- **Use the Tailwind spacing/radius/size scale first.** Reach for arbitrary `[...]` values only when there is a clear visual requirement that cannot be expressed with the standard scale.
- **Do not use pixel-perfect one-off values by default.** If a layout feels like it needs many custom pixel values, adjust the component structure or choose the nearest design-system step instead.
- **Allowed exceptions:** hairlines/borders, exact asset dimensions, or interoperability constraints where the standard scale would materially break rendering.

## Platform Considerations

- **Blockchain:** PaaS APIs handle blockchain ops; design for async processing (webhooks, callbacks)
- **Multi-Tenancy:** Projects isolated via DynamoDB partition keys; validate ownership on every request
