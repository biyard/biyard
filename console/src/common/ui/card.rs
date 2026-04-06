use dioxus::prelude::*;

#[component]
pub fn SectionCard(
    #[props(default)] class: &'static str,
    children: Element,
) -> Element {
    rsx! {
        div { class: "bg-white dark:bg-gray-800 shadow rounded-lg p-6 {class}",
            {children}
        }
    }
}

#[component]
pub fn SectionTitle(children: Element) -> Element {
    rsx! {
        h3 { class: "text-lg font-medium text-gray-900 dark:text-white mb-4",
            {children}
        }
    }
}

#[component]
pub fn DangerCard(children: Element) -> Element {
    rsx! {
        div { class: "bg-white dark:bg-gray-800 shadow rounded-lg p-6 border-2 border-red-200 dark:border-red-900",
            {children}
        }
    }
}
