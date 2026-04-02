use dioxus::prelude::*;
use dioxus_primitives::checkbox::{self, CheckboxProps};
use dioxus_primitives::icon;

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    rsx! {
        checkbox::Checkbox {
            class: "checkbox w-4 h-4 box-border p-0 border-none rounded m-0 bg-gray-900 text-gray-700 dark:text-gray-300 cursor-pointer shadow-[inset_0_0_0_1px_#9ca3af] dark:shadow-[inset_0_0_0_1px_#4b5563] focus-visible:shadow-[0_0_0_2px_#3b82f6] data-[state=checked]:bg-blue-600 data-[state=checked]:text-white data-[state=checked]:shadow-none",
            checked: props.checked,
            default_checked: props.default_checked,
            required: props.required,
            disabled: props.disabled,
            name: props.name,
            value: props.value,
            on_checked_change: props.on_checked_change,
            attributes: props.attributes,
            checkbox::CheckboxIndicator { class: "flex items-center justify-center",
                icon::Icon { width: "1rem", height: "1rem",
                    path { d: "M5 13l4 4L19 7" }
                }
            }
        }
    }
}
