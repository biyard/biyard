use dioxus::prelude::*;
use dioxus_primitives::dialog::{
    self, DialogContentProps, DialogDescriptionProps, DialogRootProps, DialogTitleProps,
};

const BACKDROP_CLASS: &str = "fixed inset-0 z-50 bg-black/50 flex items-center justify-center p-4";
const CONTENT_CLASS: &str = "bg-white dark:bg-gray-800 rounded-lg max-w-md w-full p-6";
const TITLE_CLASS: &str = "text-lg font-semibold text-gray-900 dark:text-white mb-4";
const DESCRIPTION_CLASS: &str = "text-sm text-gray-500 dark:text-gray-400 mb-4";
const ACTIONS_CLASS: &str = "flex justify-end space-x-3 mt-6";

#[component]
pub fn DialogRoot(props: DialogRootProps) -> Element {
    rsx! {
        dialog::DialogRoot {
            class: BACKDROP_CLASS,
            id: props.id,
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
pub fn DialogContent(props: DialogContentProps) -> Element {
    rsx! {
        dialog::DialogContent { class: CONTENT_CLASS.to_string(), id: props.id, attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
    rsx! {
        dialog::DialogTitle {
            class: TITLE_CLASS,
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn DialogDescription(props: DialogDescriptionProps) -> Element {
    rsx! {
        dialog::DialogDescription {
            class: DESCRIPTION_CLASS,
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn DialogActions(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { class: ACTIONS_CLASS, ..attributes, {children} }
    }
}
