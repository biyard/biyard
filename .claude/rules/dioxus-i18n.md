---
globs: ["app/**/i18n.rs", "app/**/*.rs", "packages/dioxus-translate*/**/*.rs"]
---

# Translation System (dioxus-translate)

Compile-time i18n system with three packages:
- **dioxus-translate-types** — `Translator` trait (`en()`, `ko()`)
- **dioxus-translate-macro** — `translate!` macro + `#[derive(Translate)]`
- **dioxus-translate** — Runtime: `Language` enum, hooks, global signal

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

Enable Korean in `Cargo.toml`:
```toml
dioxus-translate = { version = "0.1.*", features = ["ko"] }
```

Without `ko` feature, only `Language::En` is available and `ko:` entries in `translate!` are compiled out.
