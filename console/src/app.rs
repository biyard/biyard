use dioxus::prelude::*;

use crate::features::accounts::AccountResponse;

#[component]
pub fn App() -> Element {
    let _ = crate::features::accounts::context::Context::init()?;
    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/dx-components-theme.css"),
        }
        SuspenseBoundary {
            fallback: move |_| rsx! {
                div { class: "min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900",
                    svg {
                        class: "h-12 w-12 text-gray-400 animate-spin",
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 24 24",
                        circle {
                            class: "opacity-25",
                            cx: "12",
                            cy: "12",
                            r: "10",
                            stroke: "currentColor",
                            stroke_width: "4",
                        }
                        path {
                            class: "opacity-75",
                            fill: "currentColor",
                            d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z",
                        }
                    }
                }
            },
            Router::<crate::Route> {}
        }
    }
}
