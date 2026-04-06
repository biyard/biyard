use dioxus::prelude::*;

#[component]
pub fn Skeleton(#[props(extends=GlobalAttributes)] attributes: Vec<Attribute>) -> Element {
    rsx! {
        div { class: "animate-pulse rounded-md bg-gray-100 dark:bg-gray-800", ..attributes }
    }
}
