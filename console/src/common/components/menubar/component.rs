use dioxus::prelude::*;
use dioxus_primitives::menubar::{
    self, MenubarContentProps, MenubarItemProps, MenubarMenuProps, MenubarProps,
    MenubarTriggerProps,
};

#[component]
pub fn Menubar(props: MenubarProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        menubar::Menubar {
            class: "menubar flex box-border p-1 border-none rounded-lg bg-white dark:bg-gray-800 gap-1 shadow-[inset_0_0_0_1px_#e5e7eb] dark:shadow-[inset_0_0_0_1px_#374151] [&_.menubar-menu[data-state=open]_.menubar-trigger]:bg-gray-200 [&_.menubar-menu[data-state=open]_.menubar-trigger]:text-gray-900 dark:[&_.menubar-menu[data-state=open]_.menubar-trigger]:bg-gray-600 dark:[&_.menubar-menu[data-state=open]_.menubar-trigger]:text-white [&_.menubar-menu:first-child_.menubar-content]:ml-[-0.25rem]",
            disabled: props.disabled,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn MenubarMenu(props: MenubarMenuProps) -> Element {
    rsx! {
        menubar::MenubarMenu {
            class: "menubar-menu relative",
            index: props.index,
            disabled: props.disabled,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn MenubarTrigger(props: MenubarTriggerProps) -> Element {
    rsx! {
        menubar::MenubarTrigger { class: "menubar-trigger px-3 py-2 border-none rounded-sm bg-transparent text-gray-700 dark:text-gray-300 cursor-pointer transition-colors data-[disabled=true]:text-gray-500 data-[disabled=true]:cursor-not-allowed dark:data-[disabled=true]:text-gray-400 hover:not-data-[disabled=true]:bg-gray-200 hover:not-data-[disabled=true]:text-gray-900 hover:not-data-[disabled=true]:outline-none focus-visible:bg-gray-200 focus-visible:text-gray-900 focus-visible:outline-none dark:hover:not-data-[disabled=true]:bg-gray-600 dark:hover:not-data-[disabled=true]:text-white dark:focus-visible:bg-gray-600 dark:focus-visible:text-white", attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn MenubarContent(props: MenubarContentProps) -> Element {
    rsx! {
        menubar::MenubarContent {
            class: "menubar-content absolute z-[1000] top-full left-0 min-w-[200px] p-1 rounded-lg mt-2 bg-white dark:bg-gray-100/10 opacity-0 pointer-events-none origin-top will-change-transform shadow-[inset_0_0_0_1px_#e5e7eb] dark:shadow-[inset_0_0_0_1px_#4b5563]",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn MenubarItem(props: MenubarItemProps) -> Element {
    rsx! {
        menubar::MenubarItem {
            class: "menubar-item block px-3 py-2 rounded-sm cursor-pointer text-sm data-[disabled=true]:text-gray-500 data-[disabled=true]:cursor-not-allowed dark:data-[disabled=true]:text-gray-400 hover:not-data-[disabled=true]:bg-gray-200 hover:not-data-[disabled=true]:text-gray-900 hover:not-data-[disabled=true]:outline-none focus-visible:bg-gray-200 focus-visible:text-gray-900 focus-visible:outline-none dark:hover:not-data-[disabled=true]:bg-gray-600 dark:hover:not-data-[disabled=true]:text-white dark:focus-visible:bg-gray-600 dark:focus-visible:text-white",
            index: props.index,
            value: props.value,
            disabled: props.disabled,
            on_select: props.on_select,
            attributes: props.attributes,
            {props.children}
        }
    }
}
