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
            BtnVariant::Primary => {
                "border-brand bg-brand text-brand-contrast hover:border-brand-strong hover:bg-brand-strong"
            }
            BtnVariant::Secondary => "border-border bg-panel text-foreground hover:bg-panel-strong",
            BtnVariant::Danger => "border-danger bg-danger text-white hover:opacity-90",
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
            class: "inline-flex items-center justify-center gap-2 rounded-2xl border px-4 py-2.5 text-sm font-semibold transition-colors duration-150 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-brand focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-55 {variant.classes()} {class}",
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
pub fn SubmitBtn(#[props(default)] disabled: bool, children: Element) -> Element {
    rsx! {
        button {
            r#type: "submit",
            disabled,
            class: "inline-flex w-full items-center justify-center gap-2 rounded-2xl border border-brand bg-brand px-4 py-3 text-sm font-semibold text-brand-contrast transition-colors duration-150 hover:border-brand-strong hover:bg-brand-strong focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-brand focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-55",
            {children}
        }
    }
}
