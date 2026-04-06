use dioxus::prelude::*;

#[component]
pub fn EmptyState(
    icon: Element,
    title: String,
    #[props(default)] description: String,
    actions: Option<Element>,
) -> Element {
    rsx! {
        div { class: "bg-white dark:bg-gray-800 shadow rounded-lg p-12 text-center",
            {icon}
            h3 { class: "mt-2 text-sm font-medium text-gray-900 dark:text-white",
                "{title}"
            }
            if !description.is_empty() {
                p { class: "mt-1 text-sm text-gray-500 dark:text-gray-400",
                    "{description}"
                }
            }
            if let Some(actions) = actions {
                div { class: "mt-6", {actions} }
            }
        }
    }
}
