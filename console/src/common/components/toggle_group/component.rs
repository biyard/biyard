use dioxus::prelude::*;
use dioxus_primitives::toggle_group::{self, ToggleGroupProps, ToggleItemProps};

#[component]
pub fn ToggleGroup(props: ToggleGroupProps) -> Element {
    rsx! {
        toggle_group::ToggleGroup {
            class: "toggle-group w-fit [&_.toggle-item:first-child]:rounded-l-lg [&_.toggle-item:last-child]:rounded-r-lg [&[data-allow-multiple-pressed=true]_.toggle-item]:border-t [&[data-allow-multiple-pressed=true]_.toggle-item]:border-r [&[data-allow-multiple-pressed=true]_.toggle-item]:border-b [&[data-allow-multiple-pressed=true]_.toggle-item]:border-gray-200 dark:[&[data-allow-multiple-pressed=true]_.toggle-item]:border-gray-700 [&[data-allow-multiple-pressed=true]_.toggle-item:first-child]:border [&[data-allow-multiple-pressed=true]_.toggle-item:first-child]:border-gray-200 dark:[&[data-allow-multiple-pressed=true]_.toggle-item:first-child]:border-gray-700 [&[data-allow-multiple-pressed=true]_.toggle-item[data-state=on]]:border-gray-300 dark:[&[data-allow-multiple-pressed=true]_.toggle-item[data-state=on]]:border-gray-600 [&[data-allow-multiple-pressed=true]_.toggle-item:first-child[data-state=on]]:border [&[data-allow-multiple-pressed=true]_.toggle-item:first-child[data-state=on]]:border-gray-300 dark:[&[data-allow-multiple-pressed=true]_.toggle-item:first-child[data-state=on]]:border-gray-600",
            default_pressed: props.default_pressed,
            pressed: props.pressed,
            on_pressed_change: props.on_pressed_change,
            disabled: props.disabled,
            allow_multiple_pressed: props.allow_multiple_pressed,
            horizontal: props.horizontal,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ToggleItem(props: ToggleItemProps) -> Element {
    rsx! {
        toggle_group::ToggleItem {
            class: "toggle-item min-w-[35px] p-2.5 border-none rounded-none bg-transparent text-gray-700 dark:text-gray-300 text-sm leading-5 outline-none transition-[background-color,border] duration-200 ease-in-out hover:bg-gray-200 hover:cursor-pointer focus-visible:bg-gray-200 dark:hover:bg-gray-700 dark:focus-visible:bg-gray-700 data-[state=on]:bg-gray-400 data-[state=on]:text-gray-900 dark:data-[state=on]:bg-gray-700 dark:data-[state=on]:text-white",
            index: props.index,
            disabled: props.disabled,
            attributes: props.attributes,
            {props.children}
        }
    }
}
