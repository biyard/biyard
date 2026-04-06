use dioxus::prelude::*;
use dioxus_primitives::toolbar::{self, ToolbarButtonProps, ToolbarProps, ToolbarSeparatorProps};

#[component]
pub fn Toolbar(props: ToolbarProps) -> Element {
    rsx! {
        toolbar::Toolbar {
            class: "toolbar flex flex-row flex-wrap items-center justify-between p-1 border-none rounded-lg gap-1 shadow-[inset_0_0_0_1px_theme(colors.gray.200)] dark:shadow-[inset_0_0_0_1px_theme(colors.gray.700)] [&_button:hover:not(:disabled)]:bg-gray-200 [&_button:hover:not(:disabled)]:text-gray-900 [&_button:focus-visible]:bg-gray-200 [&_button:focus-visible]:text-gray-900 dark:[&_button:hover:not(:disabled)]:bg-gray-700 dark:[&_button:hover:not(:disabled)]:text-white dark:[&_button:focus-visible]:bg-gray-700 dark:[&_button:focus-visible]:text-white [&_button:disabled]:text-gray-500 dark:[&_button:disabled]:text-gray-400 [&_button:disabled]:cursor-not-allowed [&_.toolbar-content_p]:p-0 [&_.toolbar-content_p]:m-0",
            aria_label: props.aria_label,
            disabled: props.disabled,
            horizontal: props.horizontal,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ToolbarButton(props: ToolbarButtonProps) -> Element {
    rsx! {
        toolbar::ToolbarButton {
            class: "toolbar-button py-2 px-3 border-none rounded bg-transparent text-gray-700 dark:text-gray-300 cursor-pointer",
            index: props.index,
            disabled: props.disabled,
            on_click: props.on_click,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ToolbarSeparator(props: ToolbarSeparatorProps) -> Element {
    rsx! {
        toolbar::ToolbarSeparator {
            class: "w-px h-6 mx-[5px] bg-gray-200 dark:bg-gray-700",
            decorative: props.decorative,
            horizontal: props.horizontal,
            attributes: props.attributes,
        }
    }
}

#[component]
pub fn ToolbarGroup(
    #[props(extends = GlobalAttributes)]
    #[props(extends = div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "flex flex-row gap-[5px]", ..attributes, {children} }
    }
}
