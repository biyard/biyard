use dioxus::prelude::*;
use dioxus_primitives::radio_group::{self, RadioGroupProps, RadioItemProps};

#[component]
pub fn RadioGroup(props: RadioGroupProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        radio_group::RadioGroup {
            class: "radio-group flex flex-col gap-3",
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            required: props.required,
            name: props.name,
            horizontal: props.horizontal,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn RadioItem(props: RadioItemProps) -> Element {
    rsx! {
        radio_group::RadioItem {
            class: "radio-item flex flex-row items-center p-0 border-none bg-transparent text-gray-900 dark:text-gray-300 text-sm leading-5 gap-3",
            value: props.value,
            index: props.index,
            disabled: props.disabled,
            attributes: props.attributes,
            {props.children}
        }
    }
}
