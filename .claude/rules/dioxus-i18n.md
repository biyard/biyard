---
globs: ["console/**/i18n.rs", "landing/**/i18n.rs", "console/**/*.rs", "landing/**/*.rs"]
---

# Translation System (dioxus-translate)

Compile-time i18n system sourced from `dioxus-translate` (pinned to
`biyard/ratel.git`), providing:
- **`Translator` trait** — `en()`, `ko()`
- **`translate!` macro + `#[derive(Translate)]`** — compile-time string generation
- **Runtime** — `Language` enum, `use_translate()` / `use_language()` hooks, global signal

## `translate!` Macro

Define translations in each feature's `i18n.rs`:

```rust
// features/<module>/i18n.rs
translate! {
    MyTranslate;

    edit: {
        en: "Edit",
        ko: "편집하기",
    },
    delete: {
        en: "Delete",
        ko: "삭제",
    },
    save_changes: {
        en: "Save Changes",
        ko: "변경 저장",
    },
}
```

This generates:
- `pub struct MyTranslate` with fields as `&'static str`
- `Translator` trait impl with `en()` and `ko()` methods

## Usage in Components

```rust
use super::MyTranslate;

#[component]
fn MyComponent() -> Element {
    let t: MyTranslate = use_translate();

    rsx! {
        button { onclick: handle_edit, "{t.edit}" }
        button { onclick: handle_delete, "{t.delete}" }
    }
}
```

`use_translate()` automatically returns the correct language based on the global signal.

## Enum Translation

```rust
#[derive(Translate)]
enum Status {
    #[translate(en = "Active", ko = "활성")]
    Active,
    #[translate(en = "Inactive", ko = "비활성")]
    Inactive,
}
```

Generates `translate(&self, lang: &Language) -> &'static str` and `variants() -> Vec<String>`.

## Language Management

```rust
// Read current language
let lang = use_language();

// Switch language
let mut lang = use_language();
lang.set(Language::Ko);
```

Detection priority:
- **Web**: `localStorage("language")` → `navigator.language` → `Language::En`
- **SSR**: `language` cookie from request headers → `Language::En`

Storage key: `dioxus_translate::STORAGE_KEY`

## File Organization

**Always place `translate!` macro definitions in a dedicated `i18n.rs` file.** Never inline `translate!` blocks in page or component files.

Each feature module has its own `i18n.rs`:

```
features/<module>/
├── mod.rs          # mod i18n; pub use i18n::*;
├── i18n.rs         # translate! { ... }
├── components/
│   └── header.rs   # use super::MyTranslate; let t = use_translate();
└── views/
```

## Feature Flags

Korean is already enabled at the workspace level in the root `Cargo.toml`:
```toml
dioxus-translate = { version = "0.1", git = "https://github.com/biyard/ratel.git", rev = "...", features = ["ko"] }
```

Without the `ko` feature, only `Language::En` is available and `ko:` entries
in `translate!` are compiled out.
