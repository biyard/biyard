use dioxus::prelude::*;

/// Centered empty state. Renders bare (no border, no background) so it
/// doesn't double-up borders when placed inside a `SectionCard`. The
/// outer card already provides the chrome.
#[component]
pub fn EmptyState(
    icon: Element,
    title: String,
    #[props(default)] description: String,
    actions: Option<Element>,
) -> Element {
    rsx! {
        div { class: "py-10 text-center",
            div { class: "mx-auto mb-4 flex h-14 w-14 items-center justify-center rounded-2xl bg-brand-soft text-brand [&_svg]:h-6 [&_svg]:w-6",
                {icon}
            }
            h3 { class: "font-display text-lg font-bold text-foreground",
                "{title}"
            }
            if !description.is_empty() {
                p { class: "mx-auto mt-2 max-w-md text-sm leading-6 text-foreground-muted",
                    "{description}"
                }
            }
            if let Some(actions) = actions {
                div { class: "mt-5 flex justify-center", {actions} }
            }
        }
    }
}
