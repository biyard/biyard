---
globs: ["console/**/*.rs", "landing/**/*.rs", "console/**/*.css", "landing/**/*.css", "**/Dioxus.toml"]
---

# Dioxus Fullstack Frontend

## Overview

Frontend uses Dioxus 0.7 with fullstack rendering (SSR + client-side hydration). Two apps: `console/` (authenticated product) and `landing/` (marketing site). Each is a single package with feature-gated modules.

## Dioxus.toml

```toml
[application]
tailwind_input = "tailwind.css"

[web.app]
title = "Biyard"
```

## Entry Point

`console/` uses a shared `run()` that dispatches between web and server mode with session management:

```rust
// console/src/main.rs
fn main() {
    console::common::run(console::App);
}
```

`landing/` calls `dioxus::launch` directly (no shared `run()`, no session management):

```rust
// landing/src/main.rs
fn main() {
    dioxus::launch(App);
}
```

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
console/src/features/<module>/
├── mod.rs            # Module exports
├── route.rs          # Feature-level router
├── layout.rs         # Feature layout wrapper
├── controllers/      # Server handlers (#[get]/#[post]/... from by-macros) — see server-functions.md
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

1. **JS source** — plain JS functions in `console/assets/` or `console/js/src/`
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

- Input file: `console/tailwind.css` (and `landing/tailwind.css`)
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
- Backend handlers are written with `#[get/post/...]` from `by-macros` (not Dioxus `#[server]`) — see [server-functions.md](server-functions.md)

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

## Hook Call Order — Hooks Must Always Run

**Hooks must be called unconditionally on every render, in the same
order.** Dioxus tracks hooks by index, so a hook that is sometimes
skipped and sometimes called shifts every later hook by one slot. The
visible failure modes are `cannot reclaim ElementId(N)`, `replaceWith:
new child contains the parent`, and stale signal state.

This means **every** Dioxus hook (`use_signal`, `use_loader`,
`use_context_provider`, `use_effect`, `use_memo`, `use_resource`,
`use_navigator`, `use_translate`, `use_hook`, …) must be reached on
every render. The two patterns that violate this most often:

**1. `use_loader(...)?` followed by more hooks.** The `?` propagates
`Loading` and returns from the component early, so any hook below the
`?` is skipped on the first render and only registered once the loader
resolves — that's a hook count change between renders.

```rust
// WRONG — `use_signal` is skipped while the loader is pending
let project = use_loader(...)?;
let mut show_dialog = use_signal(|| false);

// WRONG — second `use_loader` is skipped while the first is pending
let a = use_loader(...)?;
let b = use_loader(...)?;

// CORRECT — register every hook first, then propagate Loading
let project_result = use_loader(...);
let status_result = use_loader(...);
let mut show_dialog = use_signal(|| false);
let project = project_result?;
let status = status_result?;
```

For `use_effect` that depends on a loader value, capture the loader as
`Option<Loader<T>>` (it is `Copy`) so the effect is registered
unconditionally and no-ops while the loader is pending:

```rust
let loaded_result = use_loader(...);
let loaded_for_effect: Option<Loader<_>> = loaded_result.as_ref().ok().copied();
use_effect(move || {
    if let Some(loader) = loaded_for_effect {
        // ... read loader() and write to a store ...
    }
});
let _loaded = loaded_result?;
```

**2. `return rsx! { ... }` (or `return rsx! {};`) before all hooks are
registered.** Even a "redirect to sign-in" early return must come
*after* every hook call, otherwise the redirect branch and the normal
branch register a different number of hooks.

```rust
// WRONG — context providers are skipped on the redirect branch
let account_ctx = use_account_context();
if !account_ctx().is_logged_in() {
    nav.push(Route::SignIn {});
    return rsx! {};
}
let _ = use_context_provider(|| Signal::new(SidebarOpen(false)));

// CORRECT — register every hook first, then branch
let account_ctx = use_account_context();
let _ = use_context_provider(|| Signal::new(SidebarOpen(false)));
if !account_ctx().is_logged_in() {
    nav.push(Route::SignIn {});
    return rsx! {};
}
```

### Understanding `cannot reclaim ElementId(N)`

`arena.rs` is Dioxus's mounted element-id arena, not its hook store.
Hooks are tracked separately in `scope_context.rs` using `hook_index`.

So `cannot reclaim ElementId(N)` should be read as:

- Dioxus tried to reclaim/unmount the same mounted node twice
- which means mount bookkeeping or fallback ownership got out of sync
- the numeric `ElementId(N)` is not stable across runs and can be reused
  over time, so the number itself is usually not the useful clue

Hook-order bugs are one common way to get there, but not the only way.
Another important case to inspect is **nested suspense boundary
interaction**:

- which boundary owns the persistent shell
- which boundary owns the route body
- whether both can suspend during the same navigation

In this repo, inspect shared chrome (`sidebar`, `topbar`, root
providers/layout wrappers) for `use_loader(...)?` and compare that with
nested route-level `SuspenseBoundary` usage.

Practical debugging checklist:

- check shell components for suspending loaders
- check route bodies for nested suspense during the same transition
- check for hooks registered after `?` or after an early `return`

**Rule of thumb:** every `use_*` call belongs in the top section of the
component, above any `?`, any `return`, any `if` that early-returns,
and any `let Some(...) = ... else { return ... }`.

## Data Loading

- **Always use `use_loader`** (from dioxus-fullstack-core), NOT `use_server_future`
- In **layouts**, do NOT use `use_loader()?` — wrap with `Ok()` and handle errors directly:
  ```rust
  let auth = use_loader(move || async move { Ok(get_me_handler().await.ok()) })?;
  ```
- Wrap `Outlet` with `SuspenseBoundary` when child components use `use_loader()?`
- See **Hook Call Order** above for the chained-`?` pitfall.

## Keep RSX Thin — Extract Handler Bodies

**Do not inline handler logic inside `rsx!`.** Event handler closures
(`onclick`, `onsubmit`, `oninput`, …) and computed values used by RSX
should be defined as `let` bindings *above* the `rsx!` block, not
written inline in the markup.

Why:
- Inline closures with `spawn(async move { ... })` and multi-line
  bodies bury logic inside template noise, making it hard to read,
  search, and review.
- Cloning, error handling, and `match` arms in inline closures tend to
  drift into ad-hoc patterns that are hard to keep consistent.
- Hoisting handlers makes it obvious which signals/state each one
  captures, and lets the same handler be reused by multiple buttons or
  bound through `EventHandler` props.

```rust
// WRONG — handler body is buried inside rsx!
rsx! {
    Btn {
        onclick: move |_| {
            let pid = project_id();
            let nav = nav.clone();
            spawn(async move {
                let res = create_token_handler(pid.clone(), ...).await;
                match res {
                    Ok(_) => nav.push(Route::ProjectDetail { project_id: pid }),
                    Err(e) => message.set(Some((AlertVariant::Error, e.to_string()))),
                }
            });
        },
        {t.create_token}
    }
}

// CORRECT — handler hoisted out of rsx!
let on_create = move |_| {
    let pid = project_id();
    let nav = nav.clone();
    spawn(async move {
        match create_token_handler(pid.clone(), ...).await {
            Ok(_) => nav.push(Route::ProjectDetail { project_id: pid }),
            Err(e) => message.set(Some((AlertVariant::Error, e.to_string()))),
        }
    });
};

rsx! {
    Btn { onclick: on_create, {t.create_token} }
}
```

**Allowed exceptions:**
- One-line trivial closures that just toggle a signal:
  `onclick: move |_| show_dialog.set(true)`.
- Pure-display formatting (`format!`, `replace`) when it's a single
  expression directly bound into an attribute.

For everything else — async work, branching, multi-step state updates,
error handling — bind the closure to a `let` above `rsx!` and reference
it by name inside the markup.

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

## Backend Handler Conventions

See [server-functions.md](server-functions.md) for the full rules on
`#[get/post/put/patch/delete]` handlers under
`console/src/features/*/controllers/*.rs` — including path/query params, body
parameter naming, auth extractors, and DynamoDB access.
