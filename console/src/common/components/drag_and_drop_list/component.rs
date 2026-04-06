use dioxus::prelude::*;
use dioxus_primitives::drag_and_drop_list::{
    self, DragAndDropContext, DragAndDropItemContext, DragAndDropListItemProps,
};
use dioxus_primitives::icon::Icon;

#[derive(Props, Clone, PartialEq)]
pub struct DragAndDropListProps {
    /// Items (labels) to be rendered.
    pub items: Vec<Element>,

    /// Set if the list items should be removable
    #[props(default)]
    pub is_removable: bool,

    /// Accessible label for the list
    #[props(default)]
    pub aria_label: Option<String>,

    /// Additional attributes to apply to the list element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the list component.
    pub children: Element,
}

#[component]
pub fn DragAndDropList(props: DragAndDropListProps) -> Element {
    let is_removable = props.is_removable;
    let items = props
        .items
        .iter()
        .map(|item| {
            rsx! {
                DragIcon {}
                div { class: "grow text-base font-normal", {item} }
                if is_removable {
                    RemoveButton {}
                }
            }
        })
        .collect();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "w-full [&_.dnd-list-ul]:pl-0 [&_.dnd-list-item]:flex [&_.dnd-list-item]:box-border [&_.dnd-list-item]:items-center [&_.dnd-list-item]:justify-between [&_.dnd-list-item]:p-4 [&_.dnd-list-item]:text-gray-700 [&_.dnd-list-item]:cursor-grab [&_.dnd-list-item]:list-none [&_.dnd-list-item]:outline-none [&_.dnd-list-item]:select-none dark:[&_.dnd-list-item]:text-gray-300 [&_.dnd-list-item[data-focus-visible=true]]:shadow-[0_0_0_2px_theme(colors.blue.500)] [&_.dnd-list-item:focus-visible]:shadow-[0_0_0_2px_theme(colors.blue.500)] [&_.dnd-list-item[data-is-grabbing=true]]:cursor-grabbing [&_.dnd-list-item[data-is-grabbing=true]]:opacity-60 [&_.dnd-list-item[data-is-grabbing=true]]:outline-2 [&_.dnd-list-item[data-is-grabbing=true]]:outline-dashed [&_.dnd-list-item[data-is-grabbing=true]]:outline-blue-500",
            drag_and_drop_list::DragAndDropList {
                items,
                aria_label: props.aria_label,
                attributes: props.attributes,
                {props.children}
            }
        }
    }
}

#[component]
pub fn DragAndDropListItem(props: DragAndDropListItemProps) -> Element {
    rsx! {
        drag_and_drop_list::DragAndDropListItem { index: props.index, attributes: props.attributes, {props.children} }
    }
}

#[component]
fn DragIcon() -> Element {
    rsx! {
        div { class: "flex w-6 items-center mr-4 text-gray-700 dark:text-gray-300", aria_hidden: "true",
            Icon {
                // equal icon from lucide https://lucide.dev/icons/equal
                stroke: "var(--secondary-color-4)",
                line {
                    x1: "5",
                    x2: "19",
                    y1: "9",
                    y2: "9",
                }
                line {
                    x1: "5",
                    x2: "19",
                    y1: "15",
                    y2: "15",
                }
            }
        }
    }
}

#[component]
pub fn RemoveButton(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut ctx: DragAndDropContext = use_context();
    let item_ctx: DragAndDropItemContext = use_context();
    let index = item_ctx.index();
    let label = format!("Remove item {}", index + 1);
    rsx! {
        button {
            class: "remove-button flex overflow-visible w-6 items-center p-0 border-none ml-4 bg-transparent cursor-pointer focus-visible:rounded-sm focus-visible:outline-2 focus-visible:outline-blue-500 focus-visible:outline-offset-2",
            aria_label: "{label}",
            onclick: move |_| ctx.remove(index),
            ..attributes,
            {children}
            Icon {
                // X icon from lucide https://lucide.dev/icons/x
                stroke: "var(--secondary-color-4)",
                path { d: "M18 6 6 18" }
                path { d: "m6 6 12 12" }
            }
        }
    }
}
