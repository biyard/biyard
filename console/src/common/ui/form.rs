use dioxus::prelude::*;

const INPUT_CLASS: &str = "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md dark:bg-gray-700 dark:text-white";
const INPUT_WITH_ICON_CLASS: &str = "block py-2 pr-3 pl-10 w-full placeholder-gray-400 rounded-md border border-gray-300 shadow-sm appearance-none dark:text-white dark:bg-gray-800 dark:border-gray-600 focus:border-blue-500 focus:ring-blue-500 focus:outline-none";

#[component]
pub fn FormLabel(children: Element) -> Element {
    rsx! {
        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
            {children}
        }
    }
}

#[component]
pub fn FormField(
    label: &'static str,
    #[props(default = "text")] r#type: &'static str,
    value: String,
    oninput: EventHandler<FormEvent>,
    #[props(default)] placeholder: String,
    #[props(default)] min: &'static str,
    #[props(default)] max: &'static str,
    #[props(default)] step: &'static str,
    #[props(default)] maxlength: &'static str,
    #[props(default)] required: bool,
    #[props(default)] disabled: bool,
) -> Element {
    rsx! {
        div {
            FormLabel { {label} }
            input {
                r#type,
                value,
                oninput: move |e| oninput.call(e),
                placeholder,
                min,
                max,
                step,
                maxlength,
                required,
                disabled,
                class: INPUT_CLASS,
            }
        }
    }
}

#[component]
pub fn FormFieldWithIcon(
    label: &'static str,
    id: &'static str,
    r#type: &'static str,
    value: String,
    oninput: EventHandler<FormEvent>,
    #[props(default)] placeholder: String,
    #[props(default)] autocomplete: &'static str,
    #[props(default = true)] required: bool,
    icon: Element,
) -> Element {
    rsx! {
        div {
            label {
                r#for: id,
                class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                {label}
            }
            div { class: "relative mt-1",
                div { class: "flex absolute inset-y-0 left-0 items-center pl-3 pointer-events-none",
                    {icon}
                }
                input {
                    id,
                    name: id,
                    r#type,
                    autocomplete,
                    required,
                    value,
                    oninput: move |e| oninput.call(e),
                    class: INPUT_WITH_ICON_CLASS,
                    placeholder,
                }
            }
        }
    }
}
