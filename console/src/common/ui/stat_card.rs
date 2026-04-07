use dioxus::prelude::*;

#[derive(Copy, Clone, PartialEq, Default)]
pub enum StatColor {
    #[default]
    Gray,
    Green,
    Red,
    Blue,
    Emerald,
    Indigo,
    Amber,
    Purple,
}

impl StatColor {
    fn card_class(&self) -> &'static str {
        match self {
            StatColor::Gray => "border-border bg-panel-muted",
            StatColor::Green | StatColor::Emerald => "border-success bg-success-soft",
            StatColor::Red => "border-danger bg-danger-soft",
            StatColor::Blue => "border-brand bg-brand-soft",
            StatColor::Indigo => "border-info bg-info-soft",
            StatColor::Amber => "border-warning bg-warning-soft",
            StatColor::Purple => "border-purple bg-purple-soft",
        }
    }

    fn label_class(&self) -> &'static str {
        match self {
            StatColor::Gray => "text-foreground-muted",
            StatColor::Green | StatColor::Emerald => "text-success",
            StatColor::Red => "text-danger",
            StatColor::Blue => "text-brand",
            StatColor::Indigo => "text-info",
            StatColor::Amber => "text-warning",
            StatColor::Purple => "text-purple",
        }
    }

    fn dot_class(&self) -> &'static str {
        match self {
            StatColor::Gray => "bg-border-strong",
            StatColor::Green | StatColor::Emerald => "bg-success",
            StatColor::Red => "bg-danger",
            StatColor::Blue => "bg-brand",
            StatColor::Indigo => "bg-info",
            StatColor::Amber => "bg-warning",
            StatColor::Purple => "bg-purple",
        }
    }
}

#[component]
pub fn StatCard(label: String, value: String, #[props(default)] color: StatColor) -> Element {
    rsx! {
        div { class: "rounded-[24px] border p-5 shadow-[inset_0_1px_0_rgba(255,255,255,0.05)] {color.card_class()}",
            div { class: "flex items-center gap-3",
                span { class: "h-2.5 w-2.5 rounded-full {color.dot_class()}" }
                dt { class: "text-[11px] font-semibold uppercase tracking-[0.14em] {color.label_class()}",
                    "{label}"
                }
            }
            dd { class: "mt-4 font-display text-[1.75rem] font-bold tracking-tight text-foreground",
                "{value}"
            }
        }
    }
}
