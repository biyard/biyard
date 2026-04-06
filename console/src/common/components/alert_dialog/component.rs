use dioxus::prelude::*;
use dioxus_primitives::alert_dialog::{
    self, AlertDialogActionProps, AlertDialogActionsProps, AlertDialogCancelProps,
    AlertDialogContentProps, AlertDialogDescriptionProps, AlertDialogRootProps,
    AlertDialogTitleProps,
};

#[component]
pub fn AlertDialogRoot(props: AlertDialogRootProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        alert_dialog::AlertDialogRoot {
            class: "alert-dialog-backdrop fixed z-[1000] inset-0 bg-black/30",
            id: props.id,
            default_open: props.default_open,
            open: props.open,
            on_open_change: props.on_open_change,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogContent(props: AlertDialogContentProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogContent {
            id: props.id,
            class: props.class.unwrap_or_default() + " alert-dialog fixed z-[1001] top-1/2 left-1/2 flex w-full box-border flex-col m-0 rounded-lg bg-white text-gray-700 text-center border border-gray-200 max-w-[calc(100%-2rem)] pt-8 px-6 pb-6 shadow-[0_2px_10px_rgb(0_0_0/18%)] font-sans gap-4 -translate-x-1/2 -translate-y-1/2 dark:bg-gray-800 dark:text-gray-300 dark:border-gray-700 sm:max-w-lg sm:text-left [&_.alert-dialog-title]:m-0 [&_.alert-dialog-title]:text-gray-700 [&_.alert-dialog-title]:text-xl [&_.alert-dialog-title]:leading-7 [&_.alert-dialog-title]:font-bold dark:[&_.alert-dialog-title]:text-gray-300 [&_.alert-dialog-description]:m-0 [&_.alert-dialog-description]:text-gray-500 [&_.alert-dialog-description]:text-base [&_.alert-dialog-description]:leading-6 dark:[&_.alert-dialog-description]:text-gray-400",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogTitle(props: AlertDialogTitleProps) -> Element {
    alert_dialog::AlertDialogTitle(props)
}

#[component]
pub fn AlertDialogDescription(props: AlertDialogDescriptionProps) -> Element {
    alert_dialog::AlertDialogDescription(props)
}

#[component]
pub fn AlertDialogActions(props: AlertDialogActionsProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogActions { class: "flex flex-col-reverse gap-3 sm:flex-row sm:justify-end", attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn AlertDialogCancel(props: AlertDialogCancelProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogCancel {
            on_click: props.on_click,
            class: "px-[18px] py-2 border border-gray-200 rounded-lg bg-white text-gray-700 cursor-pointer text-base transition-colors duration-200 ease-in-out dark:border-gray-700 dark:bg-gray-900 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700 focus-visible:shadow-[0_0_0_2px_#3b82f6]",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AlertDialogAction(props: AlertDialogActionProps) -> Element {
    rsx! {
        alert_dialog::AlertDialogAction {
            class: "px-[18px] py-2 border border-red-600 rounded-lg bg-red-600 text-white cursor-pointer text-base transition-colors duration-200 ease-in-out hover:bg-red-700 focus-visible:shadow-[0_0_0_2px_#3b82f6]",
            on_click: props.on_click,
            attributes: props.attributes,
            {props.children}
        }
    }
}
