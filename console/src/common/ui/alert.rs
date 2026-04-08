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
            AlertVariant::Info => {
                "rounded-2xl border border-brand bg-brand-soft px-4 py-3 text-brand"
            }
            AlertVariant::Error => {
                "rounded-2xl border border-danger bg-danger-soft px-4 py-3 text-danger"
            }
            AlertVariant::Success => {
                "rounded-2xl border border-success bg-success-soft px-4 py-3 text-success"
            }
        }
    }

    fn text_class(&self) -> &'static str {
        match self {
            AlertVariant::Info | AlertVariant::Success => "text-sm font-medium",
            AlertVariant::Error => "text-sm font-medium",
        }
    }
}

#[component]
pub fn AlertMessage(#[props(default)] variant: AlertVariant, children: Element) -> Element {
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
