use dioxus::prelude::*;

use crate::common::ui::*;

#[component]
pub(super) fn InfoItem(
    label: String,
    value: String,
    code_like: bool,
    #[props(default)] copyable: bool,
    #[props(default)] explorer_url: Option<String>,
) -> Element {
    rsx! {
        div {
            p { class: "text-[11px] font-semibold uppercase tracking-[0.14em] text-foreground-muted",
                "{label}"
            }
            if code_like {
                div { class: "mt-2 flex items-center gap-2",
                    code { class: "block flex-1 break-all rounded-2xl border border-border bg-panel px-3 py-2 text-sm font-medium text-foreground",
                        "{value}"
                    }
                    if copyable {
                        CopyButton { value: value.clone() }
                    }
                    if let Some(ref url) = explorer_url {
                        a {
                            href: "{url}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "inline-flex shrink-0 items-center justify-center rounded-xl border border-border bg-panel px-2.5 py-2 text-xs font-medium text-foreground-muted hover:border-brand hover:text-brand transition-colors",
                            title: "View on block explorer",
                            "↗"
                        }
                    }
                }
            } else {
                p { class: "mt-2 text-sm font-semibold text-foreground",
                    "{value}"
                }
            }
        }
    }
}
