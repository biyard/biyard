# API Documentation System

Public API endpoints are documented via the `#[api_doc]` macro and rendered at
`/docs/api` (Dioxus SSR page) and `/docs/api.json` (OpenAPI 3.1 JSON).

## Adding a New Public API Endpoint

### 1. Annotate the handler

Add `#[api_doc_macros::api_doc(...)]` **above** the `#[get]`/`#[post]`/etc. attribute:

```rust
#[api_doc_macros::api_doc(
    group = "Points",
    summary = "Transact points",
    summary_ko = "포인트 트랜잭션",
)]
#[post("/v1/projects/:project_id/points", auth: ProjectAuth)]
pub async fn transact_points_handler(...) -> Result<...> { ... }
```

**Parameters:**
| Name | Required | Description |
|---|---|---|
| `group` | yes | Category name shown in docs sidebar (e.g., "Points", "Tokens", "Projects") |
| `summary` | yes | One-line English summary |
| `summary_ko` | no | Korean summary (falls back to English if omitted) |
| `description` | no | Detailed English description (falls back to `///` doc comments) |
| `description_ko` | no | Korean description |

### 2. Add `ApiDocSchema` derive to request/response DTOs

For any DTO (struct or enum) that appears as a body parameter or response type:

```rust
#[derive(Serialize, Deserialize, api_doc_macros::ApiDocSchema)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct MyResponse {
    #[field_doc(en = "Unique identifier.", ko = "고유 식별자.")]
    pub id: String,

    #[field_doc(en = "Creation timestamp (Unix epoch seconds).", ko = "생성 타임스탬프 (Unix epoch 초).")]
    pub created_at: i64,
}
```

For enums (both unit and data variants):

```rust
#[derive(api_doc_macros::ApiDocSchema)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum TransactionType {
    Award,
    Deduct,
    Transfer,
    Exchange,
}
```

**Key points:**
- `ApiDocSchema` derive is **not** behind `cfg_attr` — it must always be present
  so `#[field_doc]` attributes are recognized in all build targets (server + WASM).
- `schemars::JsonSchema` stays behind `#[cfg_attr(feature = "server", ...)]`.
- Every `pub` field should have `#[field_doc(en = "...", ko = "...")]`.
- For custom types like `Partition`, `TransactionType`, add
  `#[cfg_attr(feature = "server", schemars(with = "String"))]`.
- **No manual schema registry step required.** The `ApiDocSchema` derive
  automatically registers the type via `inventory::submit!`.

### 3. Verify

```bash
RUSTFLAGS="-D warnings" DYNAMO_TABLE_PREFIX=biyard-dev cargo build -p console --features server
RUSTFLAGS="-D warnings" DYNAMO_TABLE_PREFIX=biyard-dev cargo build -p console --features web --target wasm32-unknown-unknown
```

## Architecture

### Role Split

| Concern | Handled by |
|---|---|
| Field structure (type, required, flatten, oneOf) | `schemars` (JSON Schema) |
| i18n descriptions (en/ko) | `#[field_doc]` via `ApiDocSchema` derive |
| Auto-registration (no match statements) | `inventory::submit!` via `ApiDocSchema` derive |
| Enum variants + data fields | `schemars` (oneOf, const) |

### How It Works

1. `ApiDocSchema` derive generates:
   - `field_docs()` — i18n descriptions from `#[field_doc]`
   - `inventory::submit!(ApiSchemaEntry { ... })` — registers the type with
     `schema_fn` (calls `schemars::schema_for!`) and `field_docs`
2. At runtime, `schema_with_i18n(type_name)` looks up the inventory to get
   the JSON Schema + i18n docs merged — no manual match statement needed.
3. The docs page (`api_docs.rs`) parses the JSON Schema for field structure
   and overlays i18n descriptions from `field_docs`.
4. The OpenAPI handler (`openapi.rs`) uses `schema_for_type()` directly for
   accurate JSON Schema output including `oneOf`, `flatten`, etc.

## What Gets Documented

Only handlers with `#[api_doc]` appear in docs. Handlers without it are invisible
to the documentation system. **Public API = endpoints external developers call
with Bearer token.** Console-only UI endpoints (accounts, enterprises, etc.)
should NOT have `#[api_doc]`.

## Current Public Endpoints (13)

- **Points (6):** transact, aggregation, user balance, user transactions, monthly summaries, all transactions
- **Tokens (5):** create, get, update, deploy, mint
- **Projects (2):** get project, treasury status

## File Locations

| Component | Path |
|---|---|
| Shared runtime types (`ApiEndpointMeta`, `ApiSchemaEntry`) | `packages/api-doc-types/src/lib.rs` |
| `#[api_doc]` attribute macro | `packages/api-doc-macros/src/api_doc.rs` |
| `ApiDocSchema` derive macro | `packages/api-doc-macros/src/api_doc_schema.rs` |
| Re-export module | `console/src/common/types/api_doc_meta.rs` |
| Docs page (Dioxus) | `console/src/features/console/pages/api_docs.rs` |
| Docs page i18n | `console/src/features/console/i18n.rs` (`ApiDocsTranslate`) |
| OpenAPI JSON handler | `console/src/common/openapi.rs` |
| Test dapp pages | `console/src/common/blockchain/dapp.rs` |
| Route registration | `console/src/route.rs` (`/docs/api`) |
