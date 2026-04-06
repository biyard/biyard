use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::dropdown_menu::{
    self, DropdownMenuContentProps, DropdownMenuItemProps, DropdownMenuProps,
    DropdownMenuTriggerProps,
};
use dioxus_primitives::merge_attributes;

#[component]
pub fn DropdownMenu(props: DropdownMenuProps) -> Element {
    let base = attributes!(div {
        class: "dropdown-menu relative inline-block"
    });
    let merged = merge_attributes(vec![base, props.attributes.clone()]);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        dropdown_menu::DropdownMenu {
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            disabled: props.disabled,
            roving_loop: props.roving_loop,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn DropdownMenuTrigger(props: DropdownMenuTriggerProps) -> Element {
    let base = attributes!(button {
        class: "dropdown-menu-trigger px-[18px] py-2 border-none rounded-lg bg-white dark:bg-gray-900 text-gray-700 dark:text-gray-300 cursor-pointer text-base transition-colors shadow-[inset_0_0_0_1px_#e5e7eb] dark:shadow-[inset_0_0_0_1px_#374151] hover:bg-gray-200 hover:text-gray-900 dark:hover:bg-gray-700 dark:hover:text-white focus-visible:shadow-[0_0_0_2px_#3b82f6]"
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        dropdown_menu::DropdownMenuTrigger { r#as: props.r#as, attributes: merged, {props.children} }
    }
}

#[component]
pub fn DropdownMenuContent(props: DropdownMenuContentProps) -> Element {
    let base = attributes!(div {
        class: "dropdown-menu-content absolute z-[1000] top-full left-0 min-w-[200px] p-1 rounded-lg mt-1 bg-white dark:bg-gray-100/10 opacity-0 will-change-[transform,opacity] shadow-[inset_0_0_0_1px_#e5e7eb] dark:shadow-[inset_0_0_0_1px_#4b5563]"
    });
    let merged = merge_attributes(vec![base, props.attributes.clone()]);

    rsx! {
        dropdown_menu::DropdownMenuContent { id: props.id, attributes: merged, {props.children} }
    }
}

#[component]
pub fn DropdownMenuItem<T: Clone + PartialEq + 'static>(
    props: DropdownMenuItemProps<T>,
) -> Element {
    let base = attributes!(div {
        class: "dropdown-menu-item flex items-center px-3 py-2 rounded-sm text-gray-700 dark:text-gray-300 cursor-pointer text-sm gap-2 outline-none select-none data-[disabled=true]:text-gray-500 data-[disabled=true]:cursor-not-allowed dark:data-[disabled=true]:text-gray-400 hover:not-data-[disabled=true]:bg-gray-200 hover:not-data-[disabled=true]:text-gray-900 focus-visible:bg-gray-200 focus-visible:text-gray-900 dark:hover:not-data-[disabled=true]:bg-gray-600 dark:hover:not-data-[disabled=true]:text-white dark:focus-visible:bg-gray-600 dark:focus-visible:text-white"
    });
    let merged = merge_attributes(vec![base, props.attributes.clone()]);

    rsx! {
        dropdown_menu::DropdownMenuItem {
            disabled: props.disabled,
            value: props.value,
            index: props.index,
            on_select: props.on_select,
            attributes: merged,
            {props.children}
        }
    }
}
