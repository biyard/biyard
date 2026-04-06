use dioxus::prelude::*;
use dioxus_primitives::icon;
use dioxus_primitives::navbar::{
    self, NavbarContentProps, NavbarItemProps, NavbarNavProps, NavbarProps, NavbarTriggerProps,
};

#[component]
pub fn Navbar(props: NavbarProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        navbar::Navbar {
            class: "navbar flex box-border p-1 border-none rounded-lg gap-1 [&_.navbar-nav[data-state=open]_.navbar-trigger]:bg-gray-200 [&_.navbar-nav[data-state=open]_.navbar-trigger]:text-gray-900 dark:[&_.navbar-nav[data-state=open]_.navbar-trigger]:bg-gray-700 dark:[&_.navbar-nav[data-state=open]_.navbar-trigger]:text-white [&_.navbar-expand-icon]:transition-[rotate] [&_.navbar-expand-icon]:duration-150 [&_.navbar-expand-icon]:ease-[cubic-bezier(0.4,0,0.2,1)] [&_.navbar-nav[data-state=open]_.navbar-expand-icon]:rotate-180",
            disabled: props.disabled,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn NavbarNav(props: NavbarNavProps) -> Element {
    rsx! {
        navbar::NavbarNav {
            class: "navbar-nav relative",
            index: props.index,
            disabled: props.disabled,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn NavbarTrigger(props: NavbarTriggerProps) -> Element {
    rsx! {
        navbar::NavbarTrigger { class: "navbar-trigger flex flex-row items-center justify-center px-3 py-2 border-none rounded bg-transparent text-gray-700 cursor-pointer transition-colors dark:text-gray-300 data-[disabled=true]:text-gray-500 data-[disabled=true]:cursor-not-allowed dark:data-[disabled=true]:text-gray-400 hover:not-data-[disabled=true]:bg-gray-200 hover:not-data-[disabled=true]:text-gray-900 hover:not-data-[disabled=true]:outline-none focus-visible:bg-gray-200 focus-visible:text-gray-900 focus-visible:outline-none dark:hover:not-data-[disabled=true]:bg-gray-700 dark:hover:not-data-[disabled=true]:text-white dark:focus-visible:bg-gray-700 dark:focus-visible:text-white", attributes: props.attributes,
            {props.children}
            icon::Icon {
                class: "navbar-expand-icon",
                width: "20px",
                height: "20px",
                stroke: "var(--secondary-color-4)",
                polyline { points: "6 9 12 15 18 9" }
            }
        }
    }
}

#[component]
pub fn NavbarContent(props: NavbarContentProps) -> Element {
    rsx! {
        navbar::NavbarContent {
            class: "navbar-content absolute z-[1000] top-full left-0 min-w-[200px] p-1 rounded-lg mt-2 bg-white opacity-0 pointer-events-none shadow-[inset_0_0_0_1px_theme(colors.gray.200)] dark:bg-gray-800 dark:shadow-[inset_0_0_0_1px_theme(colors.gray.600)]",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn NavbarItem(props: NavbarItemProps) -> Element {
    rsx! {
        navbar::NavbarItem {
            class: "navbar-item block px-3 py-2 rounded text-gray-700 cursor-pointer text-sm no-underline dark:text-gray-300 data-[disabled=true]:text-gray-500 data-[disabled=true]:cursor-not-allowed dark:data-[disabled=true]:text-gray-400 hover:not-data-[disabled=true]:bg-gray-200 hover:not-data-[disabled=true]:text-gray-900 hover:not-data-[disabled=true]:outline-none focus-visible:bg-gray-200 focus-visible:text-gray-900 focus-visible:outline-none dark:hover:not-data-[disabled=true]:bg-gray-700 dark:hover:not-data-[disabled=true]:text-white dark:focus-visible:bg-gray-700 dark:focus-visible:text-white",
            index: props.index,
            value: props.value,
            disabled: props.disabled,
            new_tab: props.new_tab,
            to: props.to,
            active_class: props.active_class,
            attributes: props.attributes,
            on_select: props.on_select,
            onclick: props.onclick,
            onmounted: props.onmounted,
            {props.children}
        }
    }
}
