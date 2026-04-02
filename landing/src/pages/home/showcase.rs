use dioxus::prelude::*;
use super::data::{format_number, format_usd, format_won, BRAND_SHOWCASES};

#[component]
pub(super) fn ShowcaseSection() -> Element {
    let accents: [&str; 3] = ["#00dfc0", "#a78bfa", "#60a5fa"];
    let icons: [&str; 3] = ["\u{1F45F}", "\u{2615}", "\u{1F454}"];
    let labels: [&str; 3] = ["Fashion & Walking", "F&B & Lifestyle", "Fashion & Retail"];
    // Colorful backgrounds for the visual card area (like Consensys product cards)
    let card_bgs: [&str; 3] = [
        "linear-gradient(135deg, #0d3b2e 0%, #00dfc0 50%, #064e3b 100%)",
        "linear-gradient(135deg, #2e1065 0%, #a78bfa 50%, #4c1d95 100%)",
        "linear-gradient(135deg, #172554 0%, #60a5fa 50%, #1e3a5f 100%)",
    ];
    // Stagger: 1st card left-aligned, 2nd center, 3rd right-aligned
    let offsets: [&str; 3] = ["md:mr-auto", "md:mx-auto", "md:ml-auto"];

    rsx! {
        section {
            id: "showcase",
            class: "ui-section px-6 md:px-24",
            div {
                class: "max-w-6xl w-full mx-auto",
                div {
                    class: "text-center mb-24 reveal",
                    span { style: "color: #00dfc0; font-size: 10px; font-weight: 900; letter-spacing: 0.4em; text-transform: uppercase;", "Use Cases" }
                    h2 { class: "text-4xl md:text-6xl font-black leading-tight mt-4", "다양한 브랜드," br {} "하나의 플랫폼." }
                }

                // Staggered card grid
                div {
                    class: "space-y-16 interactive",
                    for (bi, brand) in BRAND_SHOWCASES.iter().enumerate() {
                        {
                            let accent = accents[bi];
                            let icon = icons[bi];
                            let label = labels[bi];
                            let bg = card_bgs[bi];
                            let offset = offsets[bi];
                            let six_month = format_won(brand.scenario.six_month_total);
                            let treasury = format_usd(brand.stats.treasury);
                            let users = format_number(brand.stats.users);
                            let floor = format!("${:.4}", brand.stats.floor_price);
                            let delay = format!("transition-delay: {}s;", bi as f64 * 0.15);

                            rsx! {
                                div {
                                    class: "max-w-4xl {offset} reveal",
                                    style: "{delay}",

                                    // Visual card (colorful image area)
                                    div {
                                        class: "rounded-t-3xl overflow-hidden relative",
                                        style: "background: {bg}; height: 280px;",

                                        // Large centered icon
                                        div {
                                            class: "absolute inset-0 flex items-center justify-center",
                                            span { style: "font-size: 120px; opacity: 0.4; filter: drop-shadow(0 0 30px rgba(0,0,0,0.5));", "{icon}" }
                                        }

                                        // Link button (top-right)
                                        div {
                                            class: "absolute top-4 right-4 w-10 h-10 rounded-full flex items-center justify-center interactive",
                                            style: "background: rgba(255,255,255,0.9);",
                                            span { style: "color: #020408; font-size: 14px; font-weight: 900;", "\u{2197}" }
                                        }

                                        // Reward badge (bottom-right)
                                        div {
                                            class: "absolute bottom-4 right-4 px-5 py-3 rounded-2xl",
                                            style: "background: rgba(0,0,0,0.7); backdrop-filter: blur(12px);",
                                            div { style: "font-size: 9px; color: #94a3b8; text-transform: uppercase; letter-spacing: 0.2em; margin-bottom: 4px;", "6개월 리워드" }
                                            div { class: "text-2xl font-mono font-black", style: "color: {accent};", "{six_month}" }
                                        }
                                    }

                                    // Info card (below image)
                                    div {
                                        class: "glass-panel rounded-b-3xl p-8",
                                        style: "border-color: {accent}15; border-top: none;",

                                        // Brand header
                                        div {
                                            class: "flex items-center justify-between mb-4",
                                            div {
                                                span {
                                                    class: "font-black text-xs uppercase tracking-widest",
                                                    style: "color: {accent};",
                                                    "{brand.brand}"
                                                }
                                                span {
                                                    class: "ml-3 text-xs",
                                                    style: "color: #475569;",
                                                    "{label}"
                                                }
                                            }
                                            // Stats row
                                            div {
                                                class: "hidden md:flex gap-6",
                                                for (stat_label, stat_val) in [("Treasury", treasury.as_str()), ("Users", users.as_str()), ("Floor", floor.as_str())] {
                                                    div {
                                                        class: "text-right",
                                                        div { style: "font-size: 8px; color: #475569; text-transform: uppercase;", "{stat_label}" }
                                                        div { class: "text-xs font-mono font-bold", "{stat_val}" }
                                                    }
                                                }
                                            }
                                        }

                                        // Tagline + quote
                                        h3 {
                                            class: "text-xl md:text-2xl font-bold mb-4",
                                            "{brand.tagline}"
                                        }
                                        p {
                                            class: "text-sm leading-relaxed mb-6",
                                            style: "color: #94a3b8;",
                                            "\u{201C}{brand.customer_quote}\u{201D} \u{2014} "
                                            span { style: "color: #64748b;", "{brand.customer_name}" }
                                        }

                                        // Step pills
                                        div {
                                            class: "flex flex-wrap gap-2",
                                            for step in brand.steps.iter() {
                                                span {
                                                    class: "px-3 py-1.5 rounded-full font-bold uppercase",
                                                    style: "background: {accent}10; border: 1px solid {accent}20; color: {accent}; font-size: 9px;",
                                                    "{step.icon} {step.title}"
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
        }
    }
}
