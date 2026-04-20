---
description: Universal code quality standards
---

# Code Quality Standards

## Rust

- Use workspace-level dependency versions (`workspace = true`)
- Edition 2024 features available
- Prefer `async/await` with Tokio runtime
- Avoid `unwrap()` in production code — use proper error handling
- Add doc comments for public APIs and complex logic
- **Build with warnings as errors:** always use `RUSTFLAGS="-D warnings"` when running `cargo build` checks. Example: `DYNAMO_TABLE_PREFIX=biyard-dev RUSTFLAGS="-D warnings" cargo build -p console --features server`
- **Prefer typed request bodies:** use named `#[derive(Serialize)]` structs over `serde_json::json!()` for HTTP request bodies.
- Run before committing:
  ```bash
  cargo fmt
  cargo clippy
  ```

## Code Style

- **No excessive comments.** Do not add explanatory block comments, section dividers (`// --- HOOKS ---`), or narrative comments that restate what the code does. Prefer renaming variables or extracting functions over adding comments.
- **No dead code.** Remove unused imports, functions, structs, and constants immediately.

## Protected Modules

- **Do not delete `common/components/` design system modules** even if they appear unused. These are reusable design system building blocks. Only delete dead code from controllers, DTOs, and other non-component code.

## Feature-Driven Development

Organize code by business domain, not technical layers:
- Group related models, DTOs, types, and handlers by feature
- Each feature should be self-contained
- Prefer editing existing files over creating new ones
