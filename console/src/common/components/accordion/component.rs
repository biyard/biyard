use dioxus::prelude::*;
use dioxus_primitives::accordion::{
    self, AccordionContentProps, AccordionItemProps, AccordionProps, AccordionTriggerProps,
};
use dioxus_primitives::icon;

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        accordion::Accordion {
            class: "[contain:inline-size]",
            width: "15rem",
            id: props.id,
            allow_multiple_open: props.allow_multiple_open,
            disabled: props.disabled,
            collapsible: props.collapsible,
            horizontal: props.horizontal,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element {
    rsx! {
        accordion::AccordionItem {
            class: "accordion-item overflow-hidden box-border border-b border-gray-200 dark:border-gray-700 mt-px first:mt-0 last:border-b-0",
            disabled: props.disabled,
            default_open: props.default_open,
            on_change: props.on_change,
            on_trigger_click: props.on_trigger_click,
            index: props.index,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AccordionTrigger(props: AccordionTriggerProps) -> Element {
    rsx! {
        accordion::AccordionTrigger {
            class: "accordion-trigger flex w-full box-border flex-row items-center justify-between p-0 py-4 border-none bg-transparent text-gray-700 dark:text-gray-300 outline-none text-left hover:cursor-pointer hover:underline focus-visible:border-none focus-visible:shadow-[inset_0_0_0_2px_#3b82f6]",
            id: props.id,
            attributes: props.attributes,
            {props.children}
            icon::Icon {
                class: "accordion-expand-icon transition-[rotate] duration-300 ease-[cubic-bezier(0.4,0,0.2,1)]",
                width: "20px",
                height: "20px",
                stroke: "var(--secondary-color-4)",
                polyline { points: "6 9 12 15 18 9" }
            }
        }
    }
}

#[component]
pub fn AccordionContent(props: AccordionContentProps) -> Element {
    rsx! {
        accordion::AccordionContent {
            class: "accordion-content grid",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}
