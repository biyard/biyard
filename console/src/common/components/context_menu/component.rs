use dioxus::prelude::*;
use dioxus_primitives::context_menu::{
    self, ContextMenuContentProps, ContextMenuItemProps, ContextMenuProps, ContextMenuTriggerProps,
};

#[component]
pub fn ContextMenu(props: ContextMenuProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        context_menu::ContextMenu {
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ContextMenuTrigger(props: ContextMenuTriggerProps) -> Element {
    rsx! {
        context_menu::ContextMenuTrigger {
            padding: "20px",
            background: "var(--primary-color)",
            border: "1px dashed var(--primary-color-6)",
            border_radius: ".5rem",
            cursor: "context-menu",
            user_select: "none",
            text_align: "center",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ContextMenuContent(props: ContextMenuContentProps) -> Element {
    rsx! {
        context_menu::ContextMenuContent {
            class: "context-menu-content z-[1000] min-w-[220px] p-1 rounded-lg bg-white dark:bg-gray-100/10 opacity-0 pointer-events-none will-change-transform shadow-[inset_0_0_0_1px_#e5e7eb] dark:shadow-[inset_0_0_0_1px_#4b5563]",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ContextMenuItem(props: ContextMenuItemProps) -> Element {
    rsx! {
        context_menu::ContextMenuItem {
            class: "context-menu-item flex items-center px-3 py-2 rounded-sm text-gray-700 dark:text-gray-300 cursor-pointer text-sm outline-none transition-colors select-none data-[disabled=true]:text-gray-500 data-[disabled=true]:cursor-not-allowed dark:data-[disabled=true]:text-gray-400 hover:not-data-[disabled=true]:bg-gray-200 hover:not-data-[disabled=true]:text-gray-900 focus-visible:bg-gray-200 focus-visible:text-gray-900 dark:hover:not-data-[disabled=true]:bg-gray-600 dark:hover:not-data-[disabled=true]:text-white dark:focus-visible:bg-gray-600 dark:focus-visible:text-white",
            disabled: props.disabled,
            value: props.value,
            index: props.index,
            on_select: props.on_select,
            attributes: props.attributes,
            {props.children}
        }
    }
}
