use dioxus::prelude::*;
use dioxus_primitives::tooltip::{self, TooltipContentProps, TooltipProps, TooltipTriggerProps};

#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        tooltip::Tooltip {
            class: "tooltip relative inline-block [&[data-disabled=true]_.tooltip-trigger]:cursor-default",
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn TooltipTrigger(props: TooltipTriggerProps) -> Element {
    rsx! {
        tooltip::TooltipTrigger {
            class: "tooltip-trigger inline-block",
            id: props.id,
            r#as: props.r#as,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn TooltipContent(props: TooltipContentProps) -> Element {
    rsx! {
        tooltip::TooltipContent {
            class: "tooltip-content absolute z-[1000] max-w-[250px] px-3 py-2 rounded-lg bg-gray-700 dark:bg-gray-300 text-white dark:text-gray-900 text-sm leading-relaxed",
            id: props.id,
            side: props.side,
            align: props.align,
            attributes: props.attributes,
            {props.children}
        }
    }
}
