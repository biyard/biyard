use dioxus::prelude::*;
use dioxus_translate::use_translate;

use super::i18n::AboutTranslate;

#[component]
pub(super) fn AboutSection() -> Element {
    let t: AboutTranslate = use_translate();

    rsx! {
        section {
            id: "about",
            class: "ui-section px-6 md:px-24",
            style: "background: rgba(0,0,0,0.4);",
            div {
                class: "max-w-6xl w-full mx-auto",

                // Header
                div {
                    class: "text-center mb-24 reveal",
                    span { class: "font-bold tracking-widest uppercase mb-4 block", style: "color: #ff4d4d; font-size: 10px;", "{t.section_label}" }
                    h2 {
                        class: "text-4xl md:text-7xl font-black mb-8 leading-tight",
                        "{t.heading_1}"
                        span { class: "danger-text", "{t.heading_accent}" }
                        "{t.heading_2}"
                    }
                    div {
                        class: "inline-block px-10 py-4 glass-panel rounded-xl mb-8",
                        style: "border-color: rgba(255,77,77,0.3);",
                        div { class: "text-6xl font-black italic mb-2", style: "color: #ff4d4d;", "6개월" }
                        p { class: "text-xs uppercase tracking-widest", style: "color: #64748b; letter-spacing: 0.3em;", "{t.avg_token_life}" }
                    }
                    p { class: "text-lg max-w-3xl mx-auto", style: "color: #94a3b8;", "{t.problem_desc}" }
                }

                // Before / After cards
                div {
                    class: "grid md:grid-cols-2 gap-10 interactive",

                    // Before
                    div {
                        class: "glass-panel p-10 rounded-2xl reveal",
                        style: "border-color: rgba(255,77,77,0.1);",
                        div {
                            class: "flex justify-between items-start mb-8",
                            h3 { class: "text-2xl font-bold", "{t.before_title}" }
                            span { class: "px-3 py-1 rounded text-red-400 font-bold uppercase", style: "background: rgba(255,77,77,0.2); font-size: 10px;", "Before" }
                        }
                        div {
                            class: "mb-10 flex justify-center",
                            div { class: "p-8 border-2 border-dashed rounded-full text-sm font-bold", style: "border-color: rgba(255,77,77,0.2); color: rgba(255,77,77,0.4);", "{t.before_diagram}" }
                        }
                        ul {
                            class: "space-y-6",
                            for text in [
                                t.before_1,
                                t.before_2,
                                t.before_3,
                                t.before_4,
                            ] {
                                li {
                                    class: "flex items-start gap-4",
                                    style: "color: #94a3b8;",
                                    span { class: "font-bold", style: "color: #ff4d4d;", "\u{2717}" }
                                    span { "{text}" }
                                }
                            }
                        }
                    }

                    // Biyard
                    div {
                        class: "glass-panel p-10 rounded-2xl reveal",
                        style: "border-color: rgba(0,223,192,0.2); transition-delay: 0.1s;",
                        div {
                            class: "flex justify-between items-start mb-8",
                            h3 { class: "text-2xl font-bold", "{t.after_title}" }
                            span { class: "px-3 py-1 rounded font-bold uppercase", style: "background: rgba(0,223,192,0.2); color: #00dfc0; font-size: 10px;", "Biyard" }
                        }
                        div {
                            class: "mb-10 flex justify-center",
                            div {
                                class: "p-8 glass-panel rounded-full font-bold text-sm",
                                style: "border-color: rgba(0,223,192,0.4); color: #00dfc0; box-shadow: 0 0 20px rgba(0,223,192,0.2);",
                                "{t.after_diagram}"
                            }
                        }
                        ul {
                            class: "space-y-6",
                            for text in [
                                t.after_1,
                                t.after_2,
                                t.after_3,
                                t.after_4,
                            ] {
                                li {
                                    class: "flex items-start gap-4",
                                    style: "color: #cbd5e1;",
                                    span { class: "font-bold", style: "color: #00dfc0;", "\u{2713}" }
                                    span { "{text}" }
                                }
                            }
                        }
                    }
                }

                // Platform mechanism — HOW Biyard solves it
                div {
                    class: "mt-24 reveal",
                    div {
                        class: "text-center mb-16",
                        span { style: "color: #00dfc0; font-size: 10px; font-weight: 900; letter-spacing: 0.4em; text-transform: uppercase;", "{t.how_it_works}" }
                        h3 { class: "text-3xl md:text-5xl font-black mt-4", "{t.how_heading_1}" span { class: "glow-text", "{t.how_heading_accent}" } }
                    }

                    div {
                        class: "glass-panel p-10 md:p-14 rounded-3xl",
                        style: "border-color: rgba(0,223,192,0.1);",

                        // 4-step flow
                        div {
                            class: "grid md:grid-cols-4 gap-8 mb-12",
                            for (i, (icon, title, desc)) in [
                                (t.step_1_icon, t.step_1_title, t.step_1_desc),
                                ("\u{1F3E6}", t.step_2_title, t.step_2_desc),
                                ("\u{2705}", t.step_3_title, t.step_3_desc),
                                ("\u{1F4C8}", t.step_4_title, t.step_4_desc),
                            ].iter().enumerate() {
                                {
                                    let delay = format!("transition-delay: {}s;", i as f64 * 0.1);
                                    rsx! {
                                        div {
                                            class: "text-center reveal",
                                            style: "{delay}",
                                            div {
                                                class: "w-16 h-16 mx-auto mb-4 rounded-2xl flex items-center justify-center text-2xl",
                                                style: "background: rgba(0,223,192,0.08); border: 1px solid rgba(0,223,192,0.15);",
                                                "{icon}"
                                            }
                                            div {
                                                class: "text-xs font-black uppercase tracking-widest mb-2",
                                                style: "color: #00dfc0;",
                                                "Step {i}"
                                            }
                                            h4 { class: "text-lg font-bold mb-3", "{title}" }
                                            p { class: "text-sm leading-relaxed", style: "color: #94a3b8;", "{desc}" }
                                        }
                                    }
                                }
                            }
                        }

                        // Key formula
                        div {
                            class: "text-center pt-10 reveal",
                            style: "border-top: 1px solid rgba(255,255,255,0.05);",
                            p { class: "text-sm font-bold mb-3", style: "color: #94a3b8;", "{t.formula_label}" }
                            p {
                                class: "text-2xl md:text-3xl font-mono font-black",
                                style: "color: #00dfc0; text-shadow: 0 0 20px rgba(0,223,192,0.3);",
                                "Floor Price = Treasury \u{00F7} Supply"
                            }
                            p { class: "text-sm mt-4 max-w-xl mx-auto", style: "color: #64748b;", "{t.formula_desc}" }
                        }
                    }
                }
            }
        }
    }
}
