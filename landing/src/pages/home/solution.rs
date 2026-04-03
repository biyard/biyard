use dioxus::prelude::*;
use dioxus_translate::use_translate;

use super::i18n::SolutionTranslate;

#[component]
pub(super) fn SolutionSection() -> Element {
    let t: SolutionTranslate = use_translate();

    rsx! {
        section {
            id: "solution",
            class: "ui-section px-6 md:px-24",
            style: "background: rgba(0,0,0,0.2);",
            div {
                class: "max-w-6xl w-full mx-auto",
                div {
                    class: "text-center mb-24 reveal",
                    span { style: "color: #00dfc0; font-size: 10px; font-weight: 900; letter-spacing: 0.4em; text-transform: uppercase;", "{t.section_label}" }
                    h2 { class: "text-4xl md:text-7xl font-black mt-4", "{t.heading_1}" span { class: "glow-text", "{t.heading_accent}" } "{t.heading_2}" }
                }
                div {
                    class: "grid lg:grid-cols-3 gap-8 interactive",
                    // Card 1
                    div {
                        class: "glass-panel p-10 rounded-3xl reveal h-full",
                        style: "border-top: 2px solid rgba(0,223,192,0.3);",
                        h4 { style: "color: #00dfc0; font-size: 10px; font-weight: 900; letter-spacing: 0.3em; text-transform: uppercase; margin-bottom: 24px;", "{t.card_1_label}" }
                        h3 { class: "text-3xl font-bold mb-6 italic", "{t.card_1_title}" }
                        div { class: "text-5xl font-mono font-black mb-8", "2-4%" }
                        p { class: "text-sm leading-relaxed", style: "color: #94a3b8;", "{t.card_1_desc}" }
                    }
                    // Card 2
                    div {
                        class: "glass-panel p-10 rounded-3xl reveal h-full",
                        style: "border-top: 2px solid rgba(167,139,250,0.3); transition-delay: 0.1s;",
                        h4 { style: "color: #a78bfa; font-size: 10px; font-weight: 900; letter-spacing: 0.3em; text-transform: uppercase; margin-bottom: 24px;", "{t.card_2_label}" }
                        h3 { class: "text-3xl font-bold mb-6 italic", "{t.card_2_title}" }
                        div { class: "text-5xl font-mono font-black mb-8 tracking-tighter", style: "color: #a78bfa;", "Floor " span { class: "text-sm", "Price" } }
                        p { class: "text-sm leading-relaxed", style: "color: #94a3b8;", "{t.card_2_desc}" }
                    }
                    // Card 3
                    div {
                        class: "glass-panel p-10 rounded-3xl reveal h-full",
                        style: "border-top: 2px solid rgba(96,165,250,0.3); transition-delay: 0.2s;",
                        h4 { style: "color: #60a5fa; font-size: 10px; font-weight: 900; letter-spacing: 0.3em; text-transform: uppercase; margin-bottom: 24px;", "{t.card_3_label}" }
                        h3 { class: "text-3xl font-bold mb-6 italic", "{t.card_3_title}" }
                        div { class: "text-6xl font-mono font-black mb-8", style: "color: #60a5fa;", "\u{221E}" }
                        p { class: "text-sm leading-relaxed", style: "color: #94a3b8;", "{t.card_3_desc}" }
                    }
                }
            }
        }
    }
}
