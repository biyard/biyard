---
globs: ["console/Cargo.toml", "landing/Cargo.toml", "Cargo.toml"]
---

# Shared Dependencies

Shared Rust crates used by `console/` and `landing/`. None of the non-trivial
shared code currently lives in this repo's `packages/` directory — it comes
from either crates.io or the `biyard/ratel` git repository.

## External crates

### `btracing` (crates.io v0.1.6)

Opinionated wrapper around the `tracing` crate. Direct crates.io dependency,
not sourced from ratel git.

### `by-macros` (ratel git pin)

Procedural macros for code generation. Pinned to `biyard/ratel.git` at rev
`867cb9be7b173c92acdeefe237dd87b4adcaca1f`:

- **`DynamoEntity`** derive — generates DynamoDB CRUD functions. See
  [dynamodb-patterns.md](dynamodb-patterns.md).
- **`DynamoEnum`** derive — enum serialization for DynamoDB fields.
- **`#[get]` / `#[post]` / `#[put]` / `#[patch]` / `#[delete]`** attribute
  macros — generate Dioxus fullstack handlers. See
  [server-functions.md](server-functions.md).

### `dioxus-translate` (ratel git pin)

Compile-time i18n system, pinned to the same `biyard/ratel.git` rev. Feature
`ko` enabled at workspace level. See [dioxus-i18n.md](dioxus-i18n.md).

## Local `packages/`

Only `packages/console-interop/` exists locally. It is **not** in the workspace
`[workspace] members` list — it's referenced via path dependency from
`console/Cargo.toml` when needed. Do not add unrelated shared code here without
discussion.

## Packages that are NOT used

Earlier docs mentioned `by-axum` and `by-types` — these are **not** dependencies
of `console/` or `landing/` and should not be added back without explicit
justification. Backend work goes through `by-macros`' handler macros, not a
standalone Axum wrapper.

## Dependency management rules

- Define versions at the workspace root `Cargo.toml`, use `workspace = true`
  in member crates.
- External packages are referenced via git with a pinned `rev`
  (`git = "https://github.com/biyard/ratel.git", rev = "..."`). Keep the rev
  in sync across all ratel-sourced crates (`by-macros`, `dioxus-translate`).
- Do not bump the ratel rev casually — it's a cross-crate ABI surface for the
  macros. Verify both `console` and `landing` build before committing.
