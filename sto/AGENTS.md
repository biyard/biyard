# sto/ Agent Guide

Rules for working in the `sto/` app (Biyard STO information channel + Launchpad).
For general Dioxus 0.7 / Rust guidance see [/CLAUDE.md](../CLAUDE.md) and
[/console/AGENTS.md](../console/AGENTS.md).

This file only captures **domain decisions specific to this app** and **rules the
user has stated in past sessions**. Generic Rust/Dioxus rules are delegated to
the parent docs.

## 1. Domain Model

### 1.1 Single Table
- Table: `biyard-{stage}-sto` (DynamoEntity macro: `#[dynamo(table = "sto")]`)
- Composite `pk` / `sk`. Key conventions below.

### 1.2 Key Conventions
- `pk = STO#{uuid_v7}`
- `pk = ISSUER#{slug}`
- `sk = STO` — common STO row
- `sk = STO_META#{CATEGORY_UPPER_SNAKE}` — category-specific metadata row.
  e.g. `STO_META#MUSIC`, `STO_META#ART`, `STO_META#REAL_ESTATE`, `STO_META#LIVESTOCK`
- `sk = ISSUER` — issuer metadata
- `sk = FILING#{rcept_no}` — filing
- Time-sorted sk prefix: `TS#{epoch_ms}` (e.g. `TS#1776643200000`)

### 1.3 Rows Under the Same PK
- One STO = **1 common row + 0~1 category meta row + N filing rows**.
- An STO has exactly one meta row, only for its own category. A livestock STO
  never has a `STO_META#MUSIC` row attached.
- Detail fetch is **a single Query against `pk = STO#{uuid}`**. Branch on the
  `sk` prefix and deserialize each row into the appropriate struct.

### 1.4 Category Meta Row Uses Flat Columns
- ❌ Do not pack nested JSON like `meta = { kind: "Music", artist: "..." }` into
  a single attribute.
- ✅ The row's attributes themselves must be flat: `artist`, `rights_category`,
  `trust_no`, `year`, etc.
- Each category gets its own struct (`StoMetaMusic`, `StoMetaArt`,
  `StoMetaRealEstate`, `StoMetaLivestock`).
- Split model files by category as well (`models/sto_meta_music.rs`, etc.).
- On read, normalize into `StoMetaBundle` with four `Option<...>` slots and feed
  it to a converter like `into_detail`.

## 2. Domain Types (enum rules)

**Any controlled value used for search / filter / branching must be an enum.**
Never use raw `String`.

| Type | Variants | Location |
|---|---|---|
| `Category` | `Music / Art / RealEstate / Livestock / Unknown` | `common/types/category.rs` |
| `Country` | `Kr / Us / Sg / Eu / Other / Unknown` | `common/types/country.rs` |
| `Origin` | `Dart / Musicow / Unknown` | `common/types/origin.rs` (also reused for filing source — do not create a separate `FilingSource`) |
| `StoStatus` | `Filed / Issued / Withdrawn / Liquidated / Unknown` | `common/types/sto_status.rs` |
| `IssuerStatus` | `Operating / WoundDown / Unknown` | `common/types/issuer_status.rs` |
| `FilingType` | `SecuritiesRegistration / Corrected / Prospectus / IssuanceReport / Periodic / Material / Other / Unknown` | `common/types/filing_type.rs` |

### 2.1 Standard derive set for domain enums
Every domain enum uses the same derive bundle:

```rust
#[derive(
    Debug, Clone, Copy, PartialEq, Eq,
    SerializeDisplay, DeserializeFromStr, Default, DynamoEnum,
)]
```

- `DynamoEnum` auto-derives `Display + FromStr`. Variants serialize as
  `UPPER_SNAKE` (e.g. `Music` → `"MUSIC"`, `RealEstate` → `"REAL_ESTATE"`).
- So `"MUSIC".parse::<Category>().unwrap() == Category::Music`. Do not write
  custom `parse_xxx_filter()` helpers — `.parse()` is enough.

### 2.2 Presentation (flags, emojis, Korean labels) belongs in the view layer
- Never store display strings like `"🇰🇷 한국"` in the DB.
- Conversion helpers live in view modules:
  - `country_display(Country) -> &'static str`
  - `category_label(Category) -> &'static str`
  - `flag_for(Country) -> &'static str`
  - `status_label(StoStatus, &CatalogTranslate) -> &'static str`
  - `status_pill(StoStatus, &CatalogTranslate) -> Element`
  - `issuer_status_label(IssuerStatus) -> &'static str`

### 2.3 Separate free-form text from status values
- Do not mix free-form notes into a status enum.
  e.g. do not stuff `"Operating · NXT consortium member"` into `IssuerStatus`.
- Use `Issuer.status: IssuerStatus` (status value) +
  `Issuer.status_note: Option<String>` (free-form note) as separate fields.

## 3. Time Fields

- `Sto.issued_at`, `Filing.filed_at`, `created_at`, `updated_at` are all
  **`i64` epoch ms**.
- Do not store date strings like `"2023-09-19"` in the DB.
- GSI sort keys also use `TS#{epoch_ms}#{uuid}` (e.g.
  `TS#1776643200000#abc...`). Lex sort = time sort, by construction.
- Display helper: `format_date_ms(i64) -> String` (`YYYY-MM-DD`).
- Note: a UUID v7 timestamp and `issued_at` are **not** the same thing
  (DB-write time vs. filing/issuance time).

## 4. DynamoDB Access

### 4.1 Prefer by-macros helpers
- Single-entity reads use macro-generated helpers: `Sto::get(cli, pk, sk)`,
  `Sto::query(cli, pk, opt)`, etc.
- When a single PK contains **multiple entity types** (STO + STO_META + FILING),
  raw query is required. In that case, go through a single-table helper in
  `common/dynamodb.rs` rather than dropping `cli.query()...send().await` into a
  controller directly.
- Multi-write: use `crate::transact_write!` (`common/macros/dynamo.rs`).

### 4.2 UUID v7
- `uuid::Uuid::now_v7()`. v4 is forbidden.
  See [dynamodb-patterns.md](../.claude/rules/dynamodb-patterns.md).

## 5. Data Loading (frontend)

- **Use `use_loader`.** Do not use `use_server_future`. (Dioxus 0.7 fullstack
  recommended pattern is `use_loader`.)
- In layouts, do not use `use_loader(...)?` with `?` — wrap with `Ok(...)`.
- For the `SuspenseBoundary` pattern wrapping `Outlet`, see
  [/CLAUDE.md](../CLAUDE.md).

## 6. RSX / Style / i18n

Follow CLAUDE.md, but pay extra attention to the items this app has repeatedly
violated:

### 6.1 String interpolation in RSX
- For **`String` / `&str` values**: use `{var}` (expression block), not
  `"{var}"`.
  ```rust
  // OK
  span { {label} }
  div { {issuer.name} }

  // Wrong
  span { "{label}" }
  div { "{issuer.name}" }
  ```
- Literal + variable concatenation, or non-string types, may use `"{var}"`.
  ```rust
  span { "Count: {count}" }       // non-string
  div { "Hello, {name}!" }        // concatenation
  ```
- **Attribute values** (`class:`, `value:`, `href:`, etc.) may use `"{var}"`
  (e.g. `option { value: "{StoStatus::Issued}", ... }`). Attribute literals are
  evaluated separately.

### 6.2 i18n
- **No hardcoded Korean.** Add an i18n key the moment you introduce new text.
- Place a `translate!` block in `features/{feature}/i18n.rs` for large
  pages/views.
- Even small components with any text should reference keys. Single-word
  strings like `"발행사"` belong in i18n too.
- Enum labels like `"운영 중"`, `"사업 종료"` should eventually move to i18n.
  For now they live in `features/issuers/labels.rs` as a temporary helper.

### 6.3 RSX event handlers
- Do not inline `spawn`, `match`, or multi-step state updates inside
  `onclick: move |_| { ... }`. Hoist them above the `rsx!` block as
  `let on_xxx = move |_| { ... };`.

### 6.4 Hook order
- All `use_*` calls must be unconditional and at the top of the component.
  Never place a hook below `use_loader(...)?`.

## 7. Seed / Data Pipeline

### 7.1 Seed scripts
- `tools/build-sto-seed.py` produces `scripts/sto-seed-data.json`.
- Use English enum keys throughout (`MUSIC`, `ISSUED`, `KR`, `DART`).
- Category meta gets its own row, with flat attributes (same PK, different SK).
- Convert `issued_at` / `filed_at` to epoch ms during seed.

### 7.2 localstack
- `scripts/sto-init-entrypoint.sh` creates the table and seeds it.
- To re-seed: get explicit user approval, then drop → recreate → re-seed. Drop
  is necessary when the schema changes in a way that batch-write can't reconcile
  (e.g. attribute type `S` → `N`).

### 7.3 Deployment
- Prod seeding is a one-shot `make seed-sto ENV=prod` after deploy (CDK does
  not auto-seed).

## 8. Ops / Commits

- **Commit only when the user explicitly asks.** Treat progress signals
  ("let's wrap this up", "go ahead") as **not** the same as commit signals.
- Destructive operations like re-seeding or DB purges require explicit user
  approval before running.
- Do not add `Co-Authored-By: Claude` trailers to commit messages.

## 9. Console Parity (sto follows console patterns)

Business logic may differ, but the structure must mirror **console**. These are
the spots where this session drifted and should be checked first when writing
new code.

### 9.1 Controllers: no `#[server(...)]`. Use **by-macros `#[get]` / `#[post]` / `#[put]` / `#[patch]` / `#[delete]`**.
- File and function names use the `*_handler.rs` / `*_handler` pattern.
  e.g. `controllers/list_stos.rs` defines `list_stos_handler`.
- Return type is `Result<T>` (`crate::common::Result`), never
  `std::result::Result<T, ServerFnError>`.
- Public API URL prefix is `/v1/...`.
- Body wrap struct names should be user-facing (no `req`) — see
  server-functions.md.

### 9.2 `Error` type must implement `IntoResponse + AsStatusCode`
- For a controller to work under `#[get]`, `crate::common::Error` must
  implement axum's `IntoResponse` and dioxus's `AsStatusCode`.
- Copy console's `common/types/error.rs` pattern verbatim. When adding a new
  variant, update the `as_status_code` match as well.

### 9.3 Data loading: **`use_loader`**, never `use_server_future`
- Call result is a `Loader<T>` — `let snapshot = data();` pulls the inner value.
- Single-entity pages use `let r = data();`.
- Do not write 4-state matches like `match resp_ref { Some(Ok(_)) ... }` — `?`
  handles both Loading and Error.

### 9.4 DynamoDB table schema: **identical** to console
- GSI names are `gsi1-index` through `gsi6-index` plus `type-index` (the
  `-index` suffix is required).
- by-macros generated helpers expect `{idx}-index`. A wrong GSI name produces
  an "Index not found" 500 error.
- Attribute definitions must match console's `dynamodb-table.json` (all
  `gsi{1..6}_pk/sk`).
- Whatever the environment (localstack / CDK / new region), use console's
  standard format unchanged.

### 9.5 Prefer by-macros generated helpers
- ❌ Do not write new helper functions (e.g. another `common/dynamodb.rs`
  wrapper).
- ❌ Do not put raw `cli.query()...send().await` inside controllers.
- ✅ Use `Sto::get(cli, pk, sk)`, `Sto::query(cli, pk, opt)`,
  `Sto::find_by_*(cli, val, opt)`, etc.
- ✅ **Exception: when one PK mixes entity types** (`STO` + `STO_META#*` +
  `FILING#*`), raw query is required because by-macros does not provide a
  multi-type Query helper. Use `cli.query()` explicitly in that case only.

### 9.6 No bespoke `parse_*` helpers
- `FromStr` for enums is auto-generated by `DynamoEnum` —
  `"ISSUED".parse::<StoStatus>()` is enough.
- Do not build mapping dicts or separate parser functions.

### 9.7 Presentation vs. storage (do not store display strings)
- ❌ `country: "🇰🇷 한국"` — emoji/Korean strings in the DB.
- ✅ `country: Country::Kr` (stored as `"KR"`); view-side helper
  `country_display(c)` produces the display form.

## 10. Mismatches Surfaced This Session (regression checklist)

Compressed list of the core decisions, so we don't repeat the same mistakes.

1. **RSX child interpolation**: use `{var}` directly, not `"{var}"`
   (a CLAUDE.md rule we frequently violated).
2. **Do not keep a controlled set as `String`**: anything used for Korean
   display / search / filter must be an enum + `DynamoEnum`.
3. **Status value and free-form note must not be mixed**:
   `IssuerStatus::Operating` (domain state) and
   `status_note: "NXT consortium member"` (free-form note) are different
   fields.
4. **Time is epoch ms (i64)**: never store `"YYYY-MM-DD"` strings.
5. **Category meta rows use flat attributes**: do not nest as
   `meta = {kind: "Music", artist: ...}`. Row attributes themselves
   (`artist`, `rights_category`, ...) must be flat. Split structs into separate
   files (`sto_meta_music.rs`, `sto_meta_art.rs`, ...).
6. **GSI names end with `-index`**: match console's `dynamodb-table.json`. This
   is what by-macros expects.
7. **Controllers use by-macros (`#[get]` / `#[post]`)**: never dioxus
   `#[server]`.
8. **Data loading is `use_loader`**: `use_server_future` is no longer used
   anywhere in sto.
9. **Do not invent shared helpers**: by-macros / dioxus / aws-sdk usually
   already provide the function. Check before adding anything new to `common/`.
10. **No semantically duplicated enums like `Origin` vs `FilingSource`**: if the
    meaning is "data source", one enum is enough.
11. **No parallel `Country` and `Region` types**: country alone is sufficient
    (KR/US/SG/EU/Other). Region is derived view-side as a coarse grouping.
12. **No imaginary categories like `luxury / infra / content`**: if there is no
    real data, do not add the enum variant.
13. **Destructive ops like dropping a table are run by the user**: even with
    permissions, AI must get explicit approval before commands like
    `delete-table`.
14. **Commit only on request**: progress signals ("go ahead", "let's wrap up")
    are not commit signals.

## 11. Known Gaps / TODO

- Use of `Filing.filing_type::Other` variant (DART's non-standard filing types).
- `IssuerStatus` simplification dropped some information (series / approval
  stage) — currently parked in `status_note` free-form text. Enum may need
  expansion later.
- STOs missing category metadata (livestock 12, art 4) — DART PDF body parsing
  isn't in place yet, so there's nothing to populate. A PDF extraction pipeline
  is the long-term fix.
- Filing attachments are stored as URLs only; S3 mirroring is not done.
- Temporary Korean labels like `issuer_status_label` in
  `features/issuers/labels.rs` should move into i18n.
