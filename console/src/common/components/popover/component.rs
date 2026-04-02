use dioxus::prelude::*;
use dioxus_primitives::popover::{
    self, PopoverContentProps, PopoverRootProps, PopoverTriggerProps,
};

#[component]
pub fn PopoverRoot(props: PopoverRootProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        popover::PopoverRoot {
            class: "popover relative inline-block",
            is_modal: props.is_modal,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn PopoverTrigger(props: PopoverTriggerProps) -> Element {
    rsx! {
        popover::PopoverTrigger { class: "popover-trigger px-[18px] py-2 border border-gray-200 dark:border-gray-700 rounded-lg bg-white dark:bg-gray-900 text-gray-700 dark:text-gray-300 cursor-pointer text-base transition-colors hover:bg-gray-200 dark:hover:bg-gray-700 focus-visible:shadow-[0_0_0_2px_#3b82f6]", attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn PopoverContent(props: PopoverContentProps) -> Element {
    rsx! {
        popover::PopoverContent {
            class: "popover-content fixed z-[1000] flex min-w-[200px] max-w-[calc(100%-2rem)] box-border flex-col p-1 rounded-lg mt-2 bg-white dark:bg-gray-100/10 text-center origin-top will-change-transform",
            id: props.id,
            side: props.side,
            align: props.align,
            attributes: props.attributes,
            {props.children}
        }
    }
}
