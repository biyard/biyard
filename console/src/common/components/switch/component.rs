use dioxus::prelude::*;
use dioxus_primitives::switch::{self, SwitchProps, SwitchThumbProps};

#[component]
pub fn Switch(props: SwitchProps) -> Element {
    rsx! {
        switch::Switch {
            class: "switch relative w-8 h-[1.15rem] rounded-full bg-gray-200 dark:bg-gray-700 cursor-pointer transition-colors duration-150 data-[state=checked]:bg-blue-600 [&[data-state=checked]_.switch-thumb]:bg-white [&[data-state=checked]_.switch-thumb]:translate-x-[calc(2rem-1px-(1.15rem-2px))] dark:[&[data-state=checked]_.switch-thumb]:bg-gray-900 data-[disabled=true]:cursor-not-allowed data-[disabled=true]:opacity-50",
            checked: props.checked,
            default_checked: props.default_checked,
            disabled: props.disabled,
            required: props.required,
            name: props.name,
            value: props.value,
            on_checked_change: props.on_checked_change,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SwitchThumb(props: SwitchThumbProps) -> Element {
    rsx! {
        switch::SwitchThumb { class: "switch-thumb block rounded-full bg-white dark:bg-blue-600 transition-transform duration-150 will-change-transform w-[calc(1.15rem-2px)] h-[calc(1.15rem-2px)] translate-x-px", attributes: props.attributes, {props.children} }
    }
}
