use dioxus::prelude::*;

#[derive(Copy, Clone, PartialEq, Default)]
pub enum BadgeColor {
    #[default]
    Green,
    Red,
    Yellow,
    Blue,
    Purple,
    Gray,
}

impl BadgeColor {
    fn classes(&self) -> &'static str {
        match self {
            BadgeColor::Green => "border-success bg-success-soft text-success",
            BadgeColor::Red => "border-danger bg-danger-soft text-danger",
            BadgeColor::Yellow => "border-warning bg-warning-soft text-warning",
            BadgeColor::Blue => "border-brand bg-brand-soft text-brand",
            BadgeColor::Purple => "border-purple bg-purple-soft text-purple",
            BadgeColor::Gray => "border-border-strong bg-panel-strong text-foreground-soft",
        }
    }
}

#[component]
pub fn StatusBadge(color: BadgeColor, children: Element) -> Element {
    rsx! {
        span {
            class: "inline-flex items-center rounded-full border px-3 py-1 text-[11px] font-semibold uppercase tracking-[0.12em] {color.classes()}",
            {children}
        }
    }
}
