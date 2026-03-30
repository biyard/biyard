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
