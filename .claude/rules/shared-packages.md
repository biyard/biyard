---
globs: ["packages/**/*.rs", "Cargo.toml"]
---

# Shared Workspace Packages

Custom Rust packages shared across the workspace. Currently sourced from the `ratel` git repository; will move to local `packages/` directory.

## btracing (`packages/btracing/`, v0.1.*)

Opinionated wrapper around the `tracing` crate. Provides structured logging and observability setup.

## by-axum (`packages/by-axum/`, v0.2.*)

Custom Axum web framework wrapper:
- Middleware configuration
- Routing utilities and handler patterns
- `by_axum::finishing()` for app setup
- Session management integration

## by-macros (`packages/by-macros/`, v0.6.*)

Procedural macros for code generation:
- `DynamoEntity` derive — generates DynamoDB CRUD functions (see `dynamodb-patterns` rule for details)
- `DynamoEnum` derive — enum serialization for DynamoDB fields
- Additional routing/schema macros

## by-types (`packages/by-types/`, v0.3.*)

Shared type definitions used across packages.

## dioxus-translate (`packages/dioxus-translate/`, v0.1.*)

Compile-time i18n system for Dioxus apps (see `dioxus-i18n` rule for full usage):
- `dioxus-translate-types` — `Translator` trait
- `dioxus-translate-macro` — `translate!` macro + `#[derive(Translate)]`
- `dioxus-translate` — Runtime hooks (`use_translate`, `use_language`), `Language` enum

## Dependency Management

- All dependency versions are defined at workspace level in root `Cargo.toml`
- Use `workspace = true` in member crates
- External packages referenced via git: `git = "https://github.com/biyard/ratel.git"`
