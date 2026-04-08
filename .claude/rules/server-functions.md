---
globs: ["console/src/features/**/controllers/**/*.rs"]
---

# Server Handlers (Console Backend)

All backend work lives in `console/` as handler functions under
`console/src/features/<feature>/controllers/*.rs`. There is no separate `api/`
crate.

## Handler Macro

Handlers use `#[get("/v1/...", auth: SomeAuth)]` / `#[post(...)]` /
`#[put(...)]` / `#[patch(...)]` / `#[delete(...)]` from **`by-macros`** (pinned
to `biyard/ratel.git`). This is **not** stock Dioxus `#[server]`.

```rust
use crate::common::{ProjectPartition, Result};
use crate::features::projects::ProjectResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::ProjectViewerAuth;

#[get("/v1/projects/:project_id", auth: ProjectViewerAuth)]
pub async fn get_project_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
) -> Result<ProjectResponse> {
    Ok(auth.project.into())
}
```

Reference: [console/src/features/projects/controllers/get_project.rs](console/src/features/projects/controllers/get_project.rs)

## File Layout

- **One handler per file.** Do not group multiple handlers in one file.
- **File path mirrors URL path** (excluding path parameters):
  - `GET  /v1/projects`                  → `features/projects/controllers/list_projects.rs`
  - `POST /v1/projects`                  → `features/projects/controllers/create_project.rs`
  - `GET  /v1/projects/:project_id`      → `features/projects/controllers/get_project.rs`
  - `PUT  /v1/projects/:project_id`      → `features/projects/controllers/update_project.rs`
  - `GET  /v1/projects/:pid/tokens`      → `features/projects/controllers/tokens/list_tokens.rs`
- Export handlers from the feature's `controllers/mod.rs`.

## Body Parameters

The `#[post]` / `#[put]` / `#[patch]` macro wraps all handler body arguments
into an auto-generated `___Body_Serialize___` struct, so the **HTTP wire format
uses the handler parameter names as JSON field names**. Callers (both Dioxus
frontend clients and external HTTP clients) must send
`{"<param_name>": <value>, ...}`.

### Naming rule

- **Prefer flat, meaningful parameter names over a single `req` wrapper.**
  Each parameter becomes a top-level JSON field, which is more idiomatic REST
  and keeps external clients from having to look up the Rust variable name.
- **Exception — collection bodies.** When the body is a `Vec<T>`, a `HashMap`,
  or another collection that can't be naturally split into multiple named
  fields, declare a single meaningful parameter name (e.g., `transactions`,
  `items`) rather than `req`. Never name it `req`.

```rust
// Preferred — flat, named fields
#[post("/v1/projects/:project_id/tokens/mint/:user_id", auth: ProjectAuth)]
pub async fn mint_tokens_handler(
    project_id: ProjectPartition,
    user_id: String,
    amount: i64,
    memo: Option<String>,
) -> Result<TokenBalanceResponse> { ... }
// Wire format: {"amount": 100, "memo": "..."}

// Collection body — use a meaningful name, never `req`
#[post("/v1/projects/:project_id/points", auth: ProjectAuth)]
pub async fn transact_points_handler(
    project_id: ProjectPartition,
    transactions: Vec<TransactPointsRequest>,
) -> Result<Vec<TransactPointsResponse>> { ... }
// Wire format: {"transactions": [{...}, {...}]}

// Wrong — `req` leaks the Rust binding into the HTTP contract
pub async fn transact_points_handler(
    project_id: ProjectPartition,
    req: Vec<TransactPointsRequest>,
) -> Result<Vec<TransactPointsResponse>> { ... }
// Wire format: {"req": [{...}]}  ← unclear field name in the public API
```

## Path & Query Parameters

- **Path params** use `:param` in the macro URL and appear as function arguments.
- **Query params** must be declared in the macro URL with `?name1&name2` (no braces):
  ```rust
  // Correct — query params after ? without braces
  #[get("/v1/projects?limit&bookmark", auth: EnterpriseContextAuth)]
  pub async fn list_projects_handler(
      limit: i32,
      bookmark: Option<String>,
  ) -> Result<ListResponse<ProjectResponse>> { ... }

  // Wrong — missing query params (treated as body → error on GET)
  #[get("/v1/projects", auth: EnterpriseContextAuth)]

  // Wrong — braces around query params
  #[get("/v1/projects?{limit}&{bookmark}", auth: EnterpriseContextAuth)]
  ```

## Server-Only Code

Server-side imports (auth extractors, DynamoDB config, model types) must be
gated behind `#[cfg(feature = "server")]` so the WASM client build stays lean:

```rust
#[cfg(feature = "server")]
use crate::common::CommonConfig;
#[cfg(feature = "server")]
use crate::common::EnterpriseContextAuth;
#[cfg(feature = "server")]
use crate::features::projects::Project;
```

## Auth Extractors

Auth extractors live under `console/src/common/` (e.g., `ProjectViewerAuth`,
`EnterpriseContextAuth`). The macro injects an `auth` binding into the handler
body:

- `auth.account` — authenticated account
- `auth.enterprise` — current enterprise/organization context
- `auth.project` — project resolved from `:project_id` (for project-scoped auth)

Choose the **least-privileged** extractor that satisfies the handler's needs.

## Return Type

Handlers return `crate::common::Result<T>` where `T` is the feature's public
response DTO (e.g., `ProjectResponse`, `ListResponse<ProjectResponse>`). Use
`.into()` to convert domain models to DTOs — do not expose raw DynamoDB entity
structs.

## DynamoDB Access

Use `DynamoEntity`-derived CRUD helpers — see [dynamodb-patterns.md](dynamodb-patterns.md).
Obtain the client via `CommonConfig::default().dynamodb()`.

```rust
let config = CommonConfig::default();
let cli = config.dynamodb();
let (projects, bookmark) =
    Project::find_by_organization_id(cli, &auth.enterprise.pk, opt).await?;
```

## Multi-Tenancy

- Include the organization/project partition key in every query — do not scan.
- Trust the auth extractor's resolved context (`auth.enterprise.pk`,
  `auth.project.pk`); do not re-derive tenancy from request input.
- Never accept a raw `organization_id` or `project_id` from the client when an
  extractor already provides it.
