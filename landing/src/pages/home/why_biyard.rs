use dioxus::prelude::*;
use dioxus_translate::use_translate;

use super::i18n::WhyBiyardTranslate;

#[component]
pub(super) fn WhyBiyardSection() -> Element {
    let t: WhyBiyardTranslate = use_translate();

    rsx! {
        section {
            class: "ui-section px-6 md:px-24",
            div {
                class: "max-w-6xl w-full mx-auto",
                div {
                    class: "text-center mb-20 reveal",
                    h2 {
                        class: "text-4xl md:text-6xl font-black mb-6",
                        "{t.heading_1}"
                        span { class: "glow-text tracking-tighter", "{t.heading_accent}" }
                        "{t.heading_2}"
                        br {}
                        "{t.heading_3}"
                    }
                }
                div {
                    class: "grid lg:grid-cols-2 gap-12 items-center",
                    // Before
                    div {
                        class: "glass-panel p-10 rounded-3xl reveal",
                        style: "border-color: rgba(255,77,77,0.15);",
                        div { class: "text-center mb-10", span { class: "font-bold uppercase tracking-widest text-xs", style: "color: #ff4d4d;", "{t.before_label}" } }
                        div {
                            class: "flex flex-col items-center gap-4",
                            div { class: "grid grid-cols-2 gap-4",
                                div { class: "p-4 border border-white/5 rounded-xl text-center text-sm font-semibold", style: "color: #94a3b8;", "{t.before_ad_spend}" br {} "{t.before_ad_amount}" }
                                div { class: "p-4 border border-white/5 rounded-xl text-center text-sm font-semibold", style: "color: #94a3b8;", "{t.before_acquisition}" br {} "{t.before_temporary}" }
                            }
                            div { class: "w-16 h-16 flex items-center justify-center text-3xl font-black", style: "color: #ff4d4d;", "\u{2193}" }
                            div { class: "p-8 glass-panel rounded-full font-black text-lg text-center", style: "border-color: rgba(255,77,77,0.2); color: #ff4d4d;", "{t.before_money_out}" }
                            div { class: "w-16 h-16 flex items-center justify-center text-3xl font-black", style: "color: #ff4d4d;", "\u{2193}" }
                            div { class: "grid grid-cols-2 gap-4",
                                div { class: "p-4 border border-white/5 rounded-xl text-center text-sm font-semibold", style: "color: #94a3b8;", "{t.before_churn}" br {} "{t.before_no_return}" }
                                div { class: "p-4 border border-white/5 rounded-xl text-center text-sm font-semibold", style: "color: #94a3b8;", "{t.before_repeat}" br {} "{t.before_repeat_spend}" }
                            }
                            p { class: "mt-8 text-xs text-center uppercase tracking-widest", style: "color: rgba(255,77,77,0.6);", "{t.before_summary}" }
                        }
                    }
                    // After
                    div {
                        class: "glass-panel p-10 rounded-3xl reveal",
                        style: "border-color: rgba(0,223,192,0.2); transition-delay: 0.1s;",
                        div { class: "text-center mb-10", span { class: "font-bold uppercase tracking-widest text-xs", style: "color: #00dfc0;", "{t.after_label}" } }
                        div {
                            class: "flex flex-col items-center gap-4",
                            div { class: "grid grid-cols-2 gap-4",
                                div { class: "p-4 rounded-xl text-center text-sm font-semibold", style: "border: 1px solid rgba(0,223,192,0.1); color: #e2e8f0;", "{t.after_revenue}" br {} "{t.after_deposit}" }
                                div { class: "p-4 rounded-xl text-center text-sm font-semibold", style: "border: 1px solid rgba(0,223,192,0.1); color: #e2e8f0;", "{t.after_value_up}" br {} "{t.after_auto_rise}" }
                            }
                            div { class: "w-16 h-16 flex items-center justify-center text-3xl font-black", style: "color: #00dfc0;", "\u{2193}" }
                            div { class: "p-8 glass-panel rounded-full font-black text-lg text-center", style: "border-color: rgba(0,223,192,0.5); color: #00dfc0; box-shadow: 0 0 25px rgba(0,223,192,0.3);", "{t.after_center}" }
                            div { class: "w-16 h-16 flex items-center justify-center text-3xl font-black", style: "color: #00dfc0;", "\u{2193}" }
                            div { class: "grid grid-cols-2 gap-4",
                                div { class: "p-4 rounded-xl text-center text-sm font-semibold", style: "border: 1px solid rgba(0,223,192,0.1); color: #e2e8f0;", "{t.after_advocacy}" br {} "{t.after_word_of_mouth}" }
                                div { class: "p-4 rounded-xl text-center text-sm font-semibold", style: "border: 1px solid rgba(0,223,192,0.1); color: #e2e8f0;", "{t.after_new_customers}" br {} "{t.after_organic}" }
                            }
                            p { class: "mt-8 text-xs text-center uppercase tracking-widest", style: "color: #00dfc0;", "{t.after_summary}" }
                        }
                    }
                }

                // Bridge text — connects cycle comparison to Core Innovation
                div {
                    class: "mt-24 max-w-3xl mx-auto text-center reveal",
                    p {
                        class: "text-lg md:text-xl leading-relaxed mb-6",
                        style: "color: #94a3b8;",
                        "{t.bridge_1}"
                    }
                    p {
                        class: "text-lg md:text-xl leading-relaxed mb-6",
                        style: "color: #cbd5e1;",
                        "{t.bridge_2}"
                    }
                    p {
                        class: "text-base",
                        style: "color: #64748b;",
                        "{t.bridge_3}"
                    }
                }
            }
        }
    }
}
