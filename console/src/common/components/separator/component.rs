use dioxus::prelude::*;
use dioxus_primitives::separator::{self, SeparatorProps};

#[component]
pub fn Separator(props: SeparatorProps) -> Element {
    let orientation_class = if props.horizontal {
        "w-full h-px"
    } else {
        "w-px h-full"
    };

    rsx! {
        separator::Separator {
            class: "bg-gray-200 dark:bg-gray-600 {orientation_class}",
            horizontal: props.horizontal,
            decorative: props.decorative,
            attributes: props.attributes,
            {props.children}
        }
    }
}
