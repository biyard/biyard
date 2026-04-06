use dioxus::prelude::*;

#[derive(Copy, Clone, PartialEq, Default)]
pub enum BtnVariant {
    #[default]
    Primary,
    Secondary,
    Danger,
}

impl BtnVariant {
    fn classes(&self) -> &'static str {
        match self {
            BtnVariant::Primary => "text-white bg-blue-600 rounded-md hover:bg-blue-700 disabled:opacity-50",
            BtnVariant::Secondary => "text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-700 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600",
            BtnVariant::Danger => "text-white bg-red-600 rounded-md hover:bg-red-700",
        }
    }
}

#[component]
pub fn Btn(
    #[props(default)] variant: BtnVariant,
    #[props(default)] disabled: bool,
    #[props(default)] class: &'static str,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    rsx! {
        button {
            class: "px-4 py-2 text-sm font-medium {variant.classes()} {class}",
            disabled,
            onclick: move |e| {
                if let Some(f) = &onclick {
                    f.call(e);
                }
            },
            {children}
        }
    }
}

#[component]
pub fn SubmitBtn(
    #[props(default)] disabled: bool,
    children: Element,
) -> Element {
    rsx! {
        button {
            r#type: "submit",
            disabled,
            class: "flex justify-center py-2 px-4 w-full text-sm font-medium text-white bg-blue-600 rounded-md border border-transparent shadow-sm hover:bg-blue-700 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:outline-none disabled:opacity-50 disabled:cursor-not-allowed",
            {children}
        }
    }
}
