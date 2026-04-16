mod biyard_screens;
mod fandom;
mod paradigm;
mod phone_demo;
mod revenue;
mod screens;
mod tech;

use dioxus::prelude::*;

use crate::Route;

/// Main Le Mouton Demo page — accessible at /demo/lemouton
///
/// Structure (top to bottom):
/// 1. Hero / intro
/// 2. Paradigm Shift — Why Walled Garden → Open Fandom
/// 3. Interactive Phone Demo — Le Mouton app with Biyard parts highlighted
/// 4. Revenue Model — How Biyard monetizes each piece (MOST IMPORTANT)
/// 5. Technical Integration — SDK/plugin/support per tech stack
/// 6. Fandom Playbook — Why fandom, and how to apply to Le Mouton
#[component]
pub fn LemoutonDemo() -> Element {
    rsx! {
        div {
            class: "demo-page",
            document::Link { rel: "stylesheet", href: asset!("/assets/demo-lemouton.css") }

            // Nav
            nav {
                class: "demo-nav",
                div { style: "max-width: 1200px; margin: 0 auto; display: flex; align-items: center; justify-content: space-between;",
                    Link {
                        to: Route::Home {},
                        style: "color: #64748b; font-size: 12px; font-weight: 700; letter-spacing: 0.15em; text-transform: uppercase;",
                        "\u{2190} 랜딩으로 돌아가기"
                    }
                    div { style: "display: flex; align-items: center; gap: 10px;",
                        span { style: "color: #00dfc0; font-size: 18px; font-weight: 900; letter-spacing: 0.05em; font-style: italic;", "BIYARD" }
                        span { style: "color: #64748b; font-size: 11px; font-weight: 700;", "\u{00D7}" }
                        span { style: "color: #D4C5B0; font-size: 14px; font-weight: 500; font-family: Georgia, serif;", "Le Mouton" }
                    }
                    span { style: "color: #64748b; font-size: 10px; font-weight: 700; letter-spacing: 0.2em;", "DEMO \u{00B7} 내부용" }
                }
            }

            // Hero intro
            section {
                class: "demo-section",
                style: "padding-top: 100px; padding-bottom: 40px;",
                div { class: "demo-container", style: "text-align: center;",
                    p { style: "color: #00dfc0; font-size: 11px; font-weight: 900; letter-spacing: 0.4em; text-transform: uppercase; margin-bottom: 16px;", "Use Case #1 \u{00B7} Le Mouton" }
                    h1 { style: "font-size: clamp(2.4rem, 6vw, 4.8rem); font-weight: 900; line-height: 1.05; letter-spacing: -0.03em;",
                        "르무통에 "
                        span { class: "gradient-text-mint", "Biyard Launchpad" }
                        "를"
                        br {}
                        "어떻게 적용하는가?"
                    }
                    p { style: "color: #94a3b8; font-size: 16px; line-height: 1.8; margin: 28px auto 0; max-width: 720px;",
                        "르무통은 "
                        span { style: "color: #D4C5B0; font-weight: 700;", "자체 앱 개발을 거의 완료" }
                        "한 상태입니다. Biyard는 앱을 새로 만들지 않고, "
                        span { style: "color: #00dfc0; font-weight: 700;", "팬덤 경제 기능만 SDK/API로 얹어줍니다." }
                    }
                }
            }

            // Sections
            paradigm::ParadigmSection {}
            phone_demo::PhoneDemo {}
            revenue::RevenueSection {}
            tech::TechSection {}
            fandom::FandomSection {}

            // Footer
            footer {
                style: "padding: 60px 24px 40px; border-top: 1px solid rgba(255,255,255,0.05);",
                div { style: "max-width: 1200px; margin: 0 auto; text-align: center;",
                    p { style: "color: #64748b; font-size: 11px; font-weight: 700; letter-spacing: 0.2em; text-transform: uppercase;",
                        "Biyard Launchpad \u{00D7} Le Mouton \u{00B7} Internal Demo"
                    }
                    p { style: "color: #334155; font-size: 10px; font-family: monospace; margin-top: 10px;",
                        "Demo #1 of 4 \u{00B7} 2025 \u{00B7} BIYARD CORP"
                    }
                }
            }
        }
    }
}
