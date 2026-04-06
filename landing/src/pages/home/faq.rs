use dioxus::prelude::*;
use dioxus_translate::use_translate;

use super::i18n::FaqTranslate;

#[component]
pub(super) fn FaqSection() -> Element {
    let t: FaqTranslate = use_translate();

    rsx! {
        section {
            id: "faq",
            class: "ui-section px-6 md:px-24",
            div {
                class: "max-w-4xl w-full mx-auto",
                div {
                    class: "text-center mb-20 reveal",
                    span { style: "color: #00dfc0; font-size: 10px; font-weight: 900; letter-spacing: 0.4em; text-transform: uppercase;", "FAQ" }
                    h2 { class: "text-4xl md:text-6xl font-black mb-6 mt-4", "{t.section_title}" }
                }
                div {
                    class: "space-y-4 interactive",
                    for (i, (q, a)) in [
                        (t.q1, t.a1),
                        (t.q2, t.a2),
                        (t.q3, t.a3),
                        (t.q4, t.a4),
                        (t.q5, t.a5),
                        (t.q6, t.a6),
                    ].iter().enumerate() {
                        {
                            let delay = format!("transition-delay: {}s;", i as f64 * 0.1);
                            rsx! {
                                details {
                                    class: "faq-item glass-panel group reveal",
                                    style: "{delay}",
                                    summary {
                                        class: "list-none px-8 py-8 flex justify-between items-center cursor-pointer font-bold text-lg",
                                        style: "color: #e2e8f0;",
                                        span { "{q}" }
                                        span { class: "faq-icon font-bold text-2xl", style: "color: #00dfc0;", "+" }
                                    }
                                    div {
                                        class: "px-8 pb-8 text-sm leading-relaxed pt-8",
                                        style: "color: #94a3b8; border-top: 1px solid rgba(255,255,255,0.05);",
                                        "{a}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
