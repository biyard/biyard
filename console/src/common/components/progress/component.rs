use dioxus::prelude::*;
use dioxus_primitives::progress::{self, ProgressIndicatorProps, ProgressProps};

#[component]
pub fn Progress(props: ProgressProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        progress::Progress {
            class: "progress relative overflow-hidden w-[200px] h-2 box-border rounded-full bg-gray-100 dark:bg-gray-800",
            value: props.value,
            max: props.max,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ProgressIndicator(props: ProgressIndicatorProps) -> Element {
    rsx! {
        progress::ProgressIndicator { class: "progress-indicator h-full bg-gray-900 dark:bg-white transition-[width] duration-250 ease-in-out", attributes: props.attributes, {props.children} }
    }
}
