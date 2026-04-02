use dioxus::prelude::*;
use super::data::{format_number, format_usd, format_won, BRAND_SHOWCASES};

#[component]
pub(super) fn ShowcaseSection() -> Element {
    let accents: [&str; 3] = ["#00dfc0", "#a78bfa", "#60a5fa"];
    let icons: [&str; 3] = ["\u{1F45F}", "\u{2615}", "\u{1F454}"];
    let labels: [&str; 3] = ["Fashion & Walking", "F&B & Lifestyle", "Fashion & Retail"];

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
                div {
                    class: "space-y-12 interactive",
                    for (bi, brand) in BRAND_SHOWCASES.iter().enumerate() {
                        {
                            let accent = accents[bi];
                            let icon = icons[bi];
                            let label = labels[bi];
                            let six_month = format_won(brand.scenario.six_month_total);
                            let treasury = format_usd(brand.stats.treasury);
                            let users = format_number(brand.stats.users);
                            let floor = format!("${:.4}", brand.stats.floor_price);
                            let delay = format!("transition-delay: {}s;", bi as f64 * 0.1);

                            rsx! {
                                div {
                                    class: "glass-panel p-8 md:p-14 rounded-3xl grid lg:grid-cols-2 gap-16 items-center reveal",
                                    style: "border-color: {accent}15; {delay}",

                                    // Left: brand story
                                    div {
                                        class: "order-2 lg:order-1",
                                        div {
                                            class: "flex items-center gap-4 mb-8",
                                            div { class: "w-12 h-12 rounded-xl flex items-center justify-center text-2xl shadow-xl", style: "background: rgba(255,255,255,0.05);", "{icon}" }
                                            div {
                                                h4 { class: "font-bold text-xl uppercase tracking-tighter", "{brand.brand}" }
                                                p { style: "color: {accent}; font-size: 10px; font-weight: 900; letter-spacing: 0.3em; text-transform: uppercase;", "{label}" }
                                            }
                                        }
                                        h3 {
                                            class: "text-3xl md:text-5xl font-bold mb-8 italic",
                                            style: "color: {accent};",
                                            "\"{brand.tagline}\""
                                        }
                                        blockquote {
                                            class: "pl-6 mb-10",
                                            style: "border-left: 4px solid {accent};",
                                            p { class: "italic text-sm leading-relaxed", style: "color: #cbd5e1;", "\"{brand.customer_quote}\"" }
                                            cite { class: "text-xs mt-2 block", style: "color: #64748b;", "\u{2014} {brand.customer_name}" }
                                        }
                                        div {
                                            class: "flex flex-wrap gap-2",
                                            for step in brand.steps.iter() {
                                                span {
                                                    class: "px-4 py-2 rounded-full border font-bold uppercase",
                                                    style: "border-color: {accent}30; color: {accent}; font-size: 10px;",
                                                    "{step.icon} {step.title}"
                                                }
                                            }
                                        }
                                    }

                                    // Right: metrics + large icon
                                    div {
                                        class: "order-1 lg:order-2 glass-panel p-10 rounded-3xl relative overflow-hidden",
                                        style: "background: linear-gradient(to bottom right, {accent}08, transparent);",
                                        // Large decorative icon
                                        div {
                                            style: "position: absolute; top: -20px; right: -20px; font-size: 120px; opacity: 0.08; pointer-events: none;",
                                            "{icon}"
                                        }
                                        div { style: "font-size: 10px; color: #64748b; text-transform: uppercase; letter-spacing: 0.3em; font-weight: 900; margin-bottom: 16px;", "6개월 누적 리워드" }
                                        div { class: "text-5xl font-mono font-black mb-10", style: "color: {accent}; text-shadow: 0 0 20px {accent}40;", "{six_month}" }
                                        div {
                                            class: "grid grid-cols-3 gap-4 pt-10",
                                            style: "border-top: 1px solid rgba(255,255,255,0.05);",
                                            for (stat_label, stat_val, is_accent) in [("Treasury", treasury.as_str(), false), ("Users", users.as_str(), false), ("Floor", floor.as_str(), true)] {
                                                div {
                                                    class: "text-center",
                                                    div { style: "font-size: 9px; color: #64748b; text-transform: uppercase; margin-bottom: 8px;", "{stat_label}" }
                                                    div {
                                                        class: "text-sm font-bold font-mono",
                                                        style: if is_accent { format!("color: {};", accent) } else { "color: white;".to_string() },
                                                        "{stat_val}"
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
}
