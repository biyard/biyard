use dioxus::prelude::*;

#[component]
pub fn SectionCard(#[props(default)] class: &'static str, children: Element) -> Element {
    rsx! {
        div { class: "rounded-2xl border border-border bg-panel p-5 {class}", {children} }
    }
}

#[component]
pub fn SectionTitle(#[props(default)] class: &'static str, children: Element) -> Element {
    rsx! {
        h3 { class: "font-display text-base font-semibold tracking-tight text-foreground gap-3 {class}",
            {children}
        }
    }
}

#[component]
pub fn DangerCard(children: Element) -> Element {
    rsx! {
        div { class: "rounded-2xl border border-danger bg-danger-soft p-5", {children} }
    }
}
