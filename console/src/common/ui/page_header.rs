use dioxus::prelude::*;

#[component]
pub fn PageHeader(
    title: String,
    #[props(default)] subtitle: String,
    actions: Option<Element>,
) -> Element {
    rsx! {
        div { class: "mb-6 flex items-center justify-between",
            div {
                h1 { class: "text-3xl font-bold text-gray-900 dark:text-white",
                    "{title}"
                }
                if !subtitle.is_empty() {
                    p { class: "mt-1 text-sm text-gray-600 dark:text-gray-400",
                        "{subtitle}"
                    }
                }
            }
            if let Some(actions) = actions {
                {actions}
            }
        }
    }
}
