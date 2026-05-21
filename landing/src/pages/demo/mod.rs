mod biyard_screens;
mod phone_demo;
mod screens;

use dioxus::prelude::*;

use crate::Route;

/// Le Mouton Demo — focused ONLY on the interactive phone demo.
/// Each step shows: provided tech, revenue point, competitive edge, integration code.
#[component]
pub fn LemoutonDemo() -> Element {
    rsx! {
        div {
            class: "demo-page",
            document::Link { rel: "stylesheet", href: asset!("/assets/demo-lemouton.css") }

            // Minimal nav
            nav {
                class: "demo-nav",
                div { style: "max-width: 1400px; margin: 0 auto; display: flex; align-items: center; justify-content: space-between;",
                    Link {
                        to: Route::Home {},
                        style: "color: #64748b; font-size: 12px; font-weight: 700; letter-spacing: 0.15em; text-transform: uppercase;",
                        "\u{2190} 랜딩"
                    }
                    div { style: "display: flex; align-items: center; gap: 10px;",
                        span { style: "color: #00dfc0; font-size: 16px; font-weight: 900; letter-spacing: 0.05em; font-style: italic;", "BIYARD" }
                        span { style: "color: #475569; font-size: 11px;", "\u{00D7}" }
                        span { style: "color: #D4C5B0; font-size: 13px; font-family: Georgia, serif;", "Le Mouton" }
                    }
                    span { style: "color: #64748b; font-size: 10px; font-weight: 700; letter-spacing: 0.2em;", "DEMO #1" }
                }
            }

            // Demo
            phone_demo::PhoneDemo {}
        }
    }
}
