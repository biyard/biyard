use dioxus::prelude::*;

/// Scope context badge shown above a page title, e.g. `WORKSPACE` or
/// `BRAND · <brand-name>`. Use this so operators always know which
/// context (enterprise workspace vs a specific brand) the current page
/// belongs to.
#[derive(Clone, PartialEq)]
pub enum PageScope {
    Workspace,
    Brand { name: String },
}

impl PageScope {
    fn label(&self, workspace_label: &str, brand_label: &str) -> String {
        match self {
            PageScope::Workspace => workspace_label.to_string(),
            PageScope::Brand { name } => {
                format!("{brand_label} · {name}")
            }
        }
    }

    fn accent_class(&self) -> &'static str {
        match self {
            PageScope::Workspace => "border-border bg-panel-muted text-foreground-muted",
            PageScope::Brand { .. } => "border-brand/30 bg-brand-soft text-brand",
        }
    }
}

#[component]
pub fn PageHeader(
    title: String,
    #[props(default)] subtitle: String,
    #[props(default)] scope: Option<PageScope>,
    #[props(default = "Workspace".to_string())] workspace_label: String,
    #[props(default = "Brand".to_string())] brand_label: String,
    actions: Option<Element>,
) -> Element {
    let scope_badge = scope.as_ref().map(|s| {
        let label = s.label(&workspace_label, &brand_label);
        let class = format!(
            "inline-flex items-center gap-1.5 rounded-full border px-3 py-1 text-[10px] font-semibold uppercase tracking-[0.16em] {}",
            s.accent_class()
        );
        rsx! {
            span { class: "{class}", "{label}" }
        }
    });

    rsx! {
        div { class: "mb-8 flex flex-col gap-4 xl:flex-row xl:items-end xl:justify-between",
            div { class: "max-w-3xl",
                if let Some(badge) = scope_badge {
                    div { class: "mb-3", {badge} }
                }
                h1 { class: "font-display text-2xl font-bold tracking-[-0.03em] text-foreground sm:text-3xl lg:text-[2.25rem]",
                    "{title}"
                }
                if !subtitle.is_empty() {
                    p { class: "mt-2 text-sm font-medium leading-6 text-foreground-muted",
                        "{subtitle}"
                    }
                }
            }
            if let Some(actions) = actions {
                // Always right-aligned, even on narrow viewports where the
                // header stacks (title on top, actions on the next row).
                div { class: "flex flex-wrap items-center justify-end gap-3", {actions} }
            }
        }
    }
}
