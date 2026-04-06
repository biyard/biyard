use dioxus::prelude::*;
use dioxus_primitives::toggle::{self, ToggleProps};

#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    rsx! {
        toggle::Toggle {
            class: "toggle inline-flex w-fit min-w-8 flex-row items-center justify-center px-2 border-none rounded-lg bg-transparent text-gray-700 dark:text-gray-300 text-sm outline-none hover:bg-gray-200 dark:hover:bg-gray-700 hover:cursor-pointer focus-visible:bg-gray-200 dark:focus-visible:bg-gray-700 focus-visible:cursor-pointer data-[state=on]:bg-gray-400 data-[state=on]:text-gray-900 dark:data-[state=on]:bg-gray-700 dark:data-[state=on]:text-white",
            pressed: props.pressed,
            default_pressed: props.default_pressed,
            disabled: props.disabled,
            on_pressed_change: props.on_pressed_change,
            onmounted: props.onmounted,
            onfocus: props.onfocus,
            onkeydown: props.onkeydown,
            attributes: props.attributes,
            {props.children}
        }
    }
}
