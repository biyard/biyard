use dioxus::prelude::*;

/// Category indicator on a stat card. Only the small leading dot and the
/// label tone change between variants — the card background and border
/// stay neutral so a row of stat cards reads as a calm row of numbers
/// instead of a colorful "0 = warning" alarm.
///
/// Reserve `Red` for actual errors and `Amber` for actual warnings; for
/// counters that just happen to be zero, prefer `Gray`.
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
        // Unified neutral card chrome — same border + background regardless
        // of category. The category dot + label color carry the meaning.
        div { class: "rounded-2xl border border-border bg-panel p-5",
            div { class: "flex items-center gap-3",
                span { class: "h-2 w-2 rounded-full {color.dot_class()}" }
                dt { class: "text-[11px] font-semibold uppercase tracking-[0.14em] {color.label_class()}",
                    "{label}"
                }
            }
            dd { class: "mt-3 font-display text-2xl font-bold tracking-tight text-foreground",
                "{value}"
            }
        }
    }
}
