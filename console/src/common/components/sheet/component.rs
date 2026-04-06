use dioxus::prelude::*;
use dioxus_primitives::dialog::{
    self, DialogCtx, DialogDescriptionProps, DialogRootProps, DialogTitleProps,
};
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::icon;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SheetSide {
    Top,
    #[default]
    Right,
    Bottom,
    Left,
}

impl SheetSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            SheetSide::Top => "top",
            SheetSide::Right => "right",
            SheetSide::Bottom => "bottom",
            SheetSide::Left => "left",
        }
    }
}

#[component]
pub fn Sheet(props: DialogRootProps) -> Element {
    rsx! {
        SheetRoot {
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
fn SheetRoot(props: DialogRootProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        dialog::DialogRoot {
            class: "sheet-root fixed z-[1000] inset-0 opacity-0 will-change-[opacity] bg-black/50",
            "data-slot": "sheet-root",
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
pub fn SheetContent(
    #[props(default = ReadSignal::new(Signal::new(None)))] id: ReadSignal<Option<String>>,
    #[props(default)] side: SheetSide,
    #[props(default)] class: Option<String>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let class = class
        .map(|c| format!("sheet fixed z-[1001] flex box-border flex-col border-none bg-white text-gray-700 gap-4 shadow-[0_4px_20px_rgb(0_0_0/20%)] dark:bg-gray-800 dark:text-gray-300 will-change-transform {c}"))
        .unwrap_or("sheet fixed z-[1001] flex box-border flex-col border-none bg-white text-gray-700 gap-4 shadow-[0_4px_20px_rgb(0_0_0/20%)] dark:bg-gray-800 dark:text-gray-300 will-change-transform".to_string());

    rsx! {
        dialog::DialogContent {
            class,
            id,
            "data-slot": "sheet-content",
            "data-side": side.as_str(),
            attributes,
            {children}
            SheetClose { class: "sheet-close absolute top-4 right-4 flex w-6 h-6 items-center justify-center p-0 border-none rounded bg-transparent text-gray-300 cursor-pointer transition-colors hover:text-gray-700 dark:text-gray-600 dark:hover:text-gray-300",
                icon::Icon { width: "20px", height: "20px",
                    path { d: "M18 6 6 18" }
                    path { d: "m6 6 12 12" }
                }
            }
        }
    }
}

#[component]
pub fn SheetHeader(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "flex flex-col p-4 gap-1.5", "data-slot": "sheet-header", ..attributes, {children} }
    }
}

#[component]
pub fn SheetFooter(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "flex flex-col p-4 mt-auto gap-2", "data-slot": "sheet-footer", ..attributes, {children} }
    }
}

#[component]
pub fn SheetTitle(props: DialogTitleProps) -> Element {
    rsx! {
        dialog::DialogTitle {
            id: props.id,
            class: "m-0 text-gray-700 text-lg font-semibold dark:text-gray-300",
            "data-slot": "sheet-title",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SheetDescription(props: DialogDescriptionProps) -> Element {
    rsx! {
        dialog::DialogDescription {
            id: props.id,
            class: "m-0 text-gray-500 text-sm leading-5",
            "data-slot": "sheet-description",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SheetClose(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    children: Element,
) -> Element {
    let ctx: DialogCtx = use_context();

    let mut merged: Vec<Attribute> = attributes! {
        button {
            onclick: move |_| {
                ctx.set_open(false);
            }
        }
    };
    merged.extend(attributes);

    if let Some(dynamic) = r#as {
        dynamic.call(merged)
    } else {
        rsx! {
            button { ..merged,{children} }
        }
    }
}
