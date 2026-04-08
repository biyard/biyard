use dioxus::prelude::*;
use dioxus_primitives::dialog::{
    self, DialogContentProps, DialogDescriptionProps, DialogRootProps, DialogTitleProps,
};

const BACKDROP_CLASS: &str = "fixed inset-0 z-50 bg-black/50 flex items-center justify-center p-4";
const CONTENT_CLASS: &str = "w-full max-w-2xl rounded-[28px] border border-border bg-panel p-6 shadow-[0_24px_60px_rgba(2,6,23,0.26)]";
const TITLE_CLASS: &str = "mb-3 font-display text-2xl font-bold tracking-tight text-foreground";
const DESCRIPTION_CLASS: &str = "mb-4 text-sm leading-6 text-foreground-muted";
const ACTIONS_CLASS: &str = "mt-6 flex justify-end gap-3";

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
