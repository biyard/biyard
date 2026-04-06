use dioxus::prelude::*;

#[derive(Copy, Clone, PartialEq, Default)]
pub enum AlertVariant {
    #[default]
    Info,
    Error,
    Success,
}

impl AlertVariant {
    fn outer_class(&self) -> &'static str {
        match self {
            AlertVariant::Info => "p-3 rounded-md bg-blue-50 text-blue-800 dark:bg-blue-900/30 dark:text-blue-200",
            AlertVariant::Error => "p-4 bg-red-50 rounded-md dark:bg-red-900/20",
            AlertVariant::Success => "p-3 rounded-md bg-green-50 text-green-800 dark:bg-green-900/30 dark:text-green-200",
        }
    }

    fn text_class(&self) -> &'static str {
        match self {
            AlertVariant::Info => "",
            AlertVariant::Error => "text-sm text-red-800 dark:text-red-400",
            AlertVariant::Success => "",
        }
    }
}

#[component]
pub fn AlertMessage(
    #[props(default)] variant: AlertVariant,
    children: Element,
) -> Element {
    let text_class = variant.text_class();
    rsx! {
        div { class: variant.outer_class(),
            if text_class.is_empty() {
                {children}
            } else {
                p { class: text_class, {children} }
            }
        }
    }
}
