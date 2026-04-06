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
- Run before committing:
  ```bash
  cargo fmt
  cargo clippy
  ```

## Feature-Driven Development

Organize code by business domain, not technical layers:
- Group related models, DTOs, types, and handlers by feature
- Each feature should be self-contained
- Prefer editing existing files over creating new ones
