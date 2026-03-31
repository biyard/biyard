---
globs: ["app/**/*.rs", "app/**/Dioxus.toml", "app/**/*.css"]
---

# Dioxus Fullstack Frontend

## Overview

Frontend uses Dioxus 0.7 with fullstack rendering (SSR + client-side hydration). Single package with feature-gated modules.

## Dioxus.toml

```toml
[application]
tailwind_input = "tailwind.css"

[web.app]
title = "Biyard"
```

## Entry Point

```rust
// main.rs
fn main() {
    app_shell::common::run(app_shell::App);
}
```

`common::run()` dispatches between `dioxus::launch()` (web) and server mode with session management based on feature flags.

## Routing

```rust
#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Index {},
        #[route("/auth/:..rest")]
        Auth { rest: Vec<String> },
    #[end_layout]

    #[route("/:..rest")]
    PageNotFound { rest: Vec<String> },
}
```

Key patterns:
- `#[layout(Component)]` / `#[end_layout]` for nested layouts
- `#[nest("/path")]` / `#[end_nest]` for path nesting
- `ChildRouter` for feature-level sub-routing
- `#[cfg_attr(feature = "...", ...)]` for conditional routes

## Layout Pattern

```rust
#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div { class: "antialiased bg-bg",
            AppMenu {}
            Outlet::<Route> {}
        }
    }
}
```

## Feature Flags

| Flag       | Description                                    |
|:-----------|:-----------------------------------------------|
| `web`      | Dioxus web renderer, browser components        |
| `server`   | Dioxus server, DynamoDB, AWS SDK, handlers     |
| `lambda`   | AWS Lambda deployment (includes `server`)      |
| `full`     | All feature modules enabled (default)          |

## Feature Module Structure

```
app/src/features/<module>/
├── mod.rs            # Module exports
├── route.rs          # Feature-level router
├── layout.rs         # Feature layout wrapper
├── controllers/      # Server functions (#[server])
├── models/           # DynamoDB entities (feature: server)
├── components/       # UI components
├── views/            # Page-level views
├── hooks/            # Dioxus hooks
├── dto/              # Data transfer objects
├── types/            # Custom types
├── i18n.rs           # Translation strings (translate! macro)
└── interop/          # JS FFI bindings (wasm_bindgen)
```

## JavaScript Interop

3-layer pattern:

1. **JS source** — plain JS functions in `app/assets/` or `app/js/src/`
2. **Registration** — mount on `window.biyard.<namespace>` in index.js
3. **Rust FFI** — `#[wasm_bindgen(js_namespace = ["window", "biyard", "<ns>"])]`

```rust
#[wasm_bindgen(js_namespace = ["window", "biyard", "common", "theme"])]
extern "C" {
    pub fn load_theme() -> Option<String>;
    pub fn save_theme(theme: &str);
}
```

Rules:
- **Always guard JS calls** with `#[cfg(not(feature = "server"))]`
- Namespace must match exactly between JS registration and `js_namespace` array
- Load JS via `document::Script { src: asset!("/assets/file.js") }`

## TailwindCSS v4

- Input file: `app/tailwind.css`
- Source scanning: `@source "./src/**/*.{rs,css}"`
- Theme via `@theme static` blocks, dark/light via `[data-theme]` attribute
- Dioxus compiles Tailwind automatically from `Dioxus.toml` config

## Build Commands

```bash
# Development
dx serve --addr 0.0.0.0 --port 8000 --web

# Production build
dx build --release \
  @client --features web --platform web \
  @server --features server --platform server

# Lambda build
dx build --release \
  @client --features web --platform web \
  @server --features lambda --platform server
```

## Provider Pattern

Features use context providers stacked in the App component:

```rust
#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Provider {}
        AuthProvider {}
        Router::<Route> {}
    }
}
```

## Component Conventions

- Use `#[component]` attribute on all component functions
- Props via function parameters with `#[props(default)]` for optional values
- Server functions with `#[server]` macro for backend logic callable from client

## Clone Avoidance Rules

**Never `.clone()` Copy types.** These Dioxus types implement `Copy` — use direct assignment:
- `Signal`, `ReadSignal`, `Memo`, `Resource`, `UseNavigator`
- Example: `let x = my_signal;` NOT `let x = my_signal.clone();`

**Never `.clone()` Signal read results.** `signal()` returns an owned value:
- `let val = name();` NOT `let val = name().clone();`

**Minimize String clones in for-loops:**
- Clone once, derive others from it: `let delete_id = id.clone();` NOT `let delete_id = project.id.clone();`
- Use `"{project.field}"` in RSX for display instead of cloning into a separate variable
- When a value is used last, move it instead of cloning: `set(Some(key))` NOT `set(Some(key.clone()))`

**Prefer `Copy` for simple enums** with only unit variants — add `Copy` to derives.

## Data Loading

- **Always use `use_loader`** (from dioxus-fullstack-core), NOT `use_server_future`
- In **layouts**, do NOT use `use_loader()?` — wrap with `Ok()` and handle errors directly:
  ```rust
  let auth = use_loader(move || async move { Ok(get_me_handler().await.ok()) })?;
  ```
- Wrap `Outlet` with `SuspenseBoundary` when child components use `use_loader()?`

## Type Conventions

- Use `ProjectPartition` newtype instead of raw `String` for project IDs
- Component props: `ReadSignal<ProjectPartition>` for project_id
- Convert with `.into()`: `project_id: id.clone().into()`

## RSX Translation Usage

`translate!` macro fields are `&'static str`:
- **Text content**: `{t.xxx}` (expression block)
- **Attribute values / interpolation**: `"{t.xxx}"` or `"prefix {t.xxx} suffix"`

## Form Conventions

- SSR forms **must** include `method: "post"` — without it, defaults to GET and exposes form data in URL

## GET Handler Query Parameters

Query parameters must be declared in the `#[get]` macro URL — without curly braces:
```rust
// Correct: query params after ? without braces
#[get("/v1/projects?limit&bookmark", ...)]

// Wrong: missing query params (treated as body → error on GET)
#[get("/v1/projects", ...)]

// Wrong: braces around query params
#[get("/v1/projects?{limit}&{bookmark}", ...)]
```
- Path params use `:param`, query params use `?param1&param2`

## Reference Codebase

- Follow patterns from `ratel-new/app/ratel/src/` as reference implementation
