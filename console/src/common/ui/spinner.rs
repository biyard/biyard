use dioxus::prelude::*;

#[component]
pub fn Spinner(
    #[props(default = "mr-2 -ml-1 w-5 h-5 animate-spin")] class: &'static str,
) -> Element {
    rsx! {
        svg {
            class,
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            style: "color: var(--color-brand);",
            path { d: "M21 12a9 9 0 1 1-6.219-8.56" }
        }
    }
}
