use dioxus::prelude::*;

#[component]
pub fn EmptyState(
    icon: Element,
    title: String,
    #[props(default)] description: String,
    actions: Option<Element>,
) -> Element {
    rsx! {
        div { class: "rounded-[28px] border border-border bg-panel p-12 text-center shadow-[0_18px_40px_rgba(15,23,42,0.05)]",
            div { class: "mx-auto mb-5 flex h-16 w-16 items-center justify-center rounded-[20px] bg-brand-soft text-brand [&_svg]:h-7 [&_svg]:w-7",
                {icon}
            }
            h3 { class: "font-display text-xl font-bold text-foreground",
                "{title}"
            }
            if !description.is_empty() {
                p { class: "mx-auto mt-2 max-w-md text-sm leading-6 text-foreground-muted",
                    "{description}"
                }
            }
            if let Some(actions) = actions {
                div { class: "mt-6 flex justify-center", {actions} }
            }
        }
    }
}
