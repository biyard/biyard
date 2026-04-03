use dioxus::prelude::*;
use dioxus_translate::use_translate;

use super::data::console_url;
use super::hero_cube::HeroCubeGroup;
use super::i18n::HeroTranslate;

#[component]
pub(super) fn HeroSection() -> Element {
    let console_href = console_url();
    let t: HeroTranslate = use_translate();
    rsx! {
        // Three.js canvas — fixed behind everything
        div {
            style: "position: fixed; top: 0; left: 0; width: 100%; height: 100%; z-index: 0; pointer-events: none;",
            HeroCubeGroup {}
        }

        section {
            class: "ui-section px-6 md:px-24",
            div {
                class: "max-w-5xl w-full",

                // Status badge
                div {
                    class: "inline-flex items-center gap-3 mb-8 px-5 py-2 rounded-full glass-panel reveal active interactive",
                    style: "border-color: rgba(0,223,192,0.2);",
                    span {
                        class: "relative flex h-2 w-2",
                        span { class: "animate-ping absolute inline-flex h-full w-full rounded-full opacity-75", style: "background: #00dfc0;" }
                        span { class: "relative inline-flex rounded-full h-2 w-2", style: "background: #00dfc0;" }
                    }
                    span {
                        style: "color: #00dfc0; font-size: 10px; font-weight: 900; letter-spacing: 0.4em; text-transform: uppercase;",
                        "{t.badge}"
                    }
                }

                // Main title
                h1 {
                    class: "text-5xl md:text-8xl font-black mb-10 reveal active",
                    style: "line-height: 1.1;",
                    "{t.headline_1}"
                    span { class: "glow-text italic", "{t.headline_accent}" }
                    br {}
                    "{t.headline_2}"
                }

                // Description
                p {
                    class: "text-lg md:text-2xl mb-12 leading-relaxed max-w-3xl reveal active",
                    style: "color: #94a3b8; transition-delay: 0.2s;",
                    "{t.subtitle}"
                }

                // Buttons
                div {
                    class: "flex flex-col sm:flex-row gap-6 interactive reveal active",
                    style: "transition-delay: 0.4s;",
                    a {
                        href: "{console_href}",
                        class: "btn-hyper px-12 py-5 rounded-sm font-black text-sm uppercase tracking-widest text-center",
                        "{t.cta_console}"
                    }
                    a {
                        href: "#about",
                        class: "glass-panel px-12 py-5 rounded-sm font-bold text-sm text-center",
                        "{t.cta_learn}"
                    }
                }
            }
        }
    }
}
