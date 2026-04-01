use dioxus::prelude::*;

use super::data::{format_number, format_usd, format_won, BRAND_SHOWCASES};

#[component]
pub(super) fn ShowcaseSection() -> Element {
    let brand_accents: [&str; 3] = ["#60a5fa", "#f472b6", "#34d399"];
    let brand_rgbs: [&str; 3] = ["96,165,250", "244,114,182", "52,211,153"];

    rsx! {
        section {
            id: "showcase",
            class: "py-20 px-4 relative overflow-hidden",
            style: "background: #0c1018;",
            div {
                class: "max-w-6xl mx-auto relative z-10",
                // Section header
                div {
                    class: "text-center mb-12 reveal-fade",
                    p {
                        class: "text-sm font-semibold tracking-widest uppercase mb-3",
                        style: "color: #00d4aa;",
                        "USE CASES"
                    }
                    h2 {
                        class: "text-3xl md:text-4xl font-bold mb-2 reveal-type",
                        style: "color: #e8eefc;",
                        "\u{B2E4}\u{C591}\u{D55C} \u{BE0C}\u{B79C}\u{B4DC}, \u{D558}\u{B098}\u{C758} \u{D50C}\u{B7AB}\u{D3FC}"
                    }
                    p {
                        class: "text-base",
                        style: "color: #7a8ba6;",
                        "\u{C5B4}\u{B5A4} \u{C0B0}\u{C5C5}\u{C774}\u{B4E0} \u{B9E4}\u{CD9C} \u{AE30}\u{BC18} \u{D1A0}\u{D070} \u{C774}\u{CF54}\u{B178}\u{BBF8}\u{B97C} \u{AD6C}\u{CD95}\u{D560} \u{C218} \u{C788}\u{C2B5}\u{B2C8}\u{B2E4}"
                    }
                }
                // 3-column card grid
                div {
                    class: "grid grid-cols-1 md:grid-cols-3 gap-6",
                    for (bi, brand) in BRAND_SHOWCASES.iter().enumerate() {
                        {
                            let accent = brand_accents[bi];
                            let rgb = brand_rgbs[bi];
                            let card_style = format!(
                                "background: rgba(10,16,26,0.6); backdrop-filter: blur(16px); -webkit-backdrop-filter: blur(16px); border: 1px solid rgba({},0.2); box-shadow: 0 12px 40px rgba(0,0,0,0.3), inset 0 1px 0 rgba(255,255,255,0.04);",
                                rgb
                            );
                            let badge_style = format!("background: rgba({},0.15); color: {};", rgb, accent);
                            let tagline_color = format!("color: {};", accent);
                            let line_style = format!("background: linear-gradient(90deg, transparent, rgba({},0.5), transparent);", rgb);
                            let quote_border = format!("border-left: 2px solid {};", accent);
                            let stats_border = format!("border: 1px solid rgba({},0.15);", rgb);
                            let detail_label_color = format!("color: {};", accent);

                            // Pre-compute strings
                            let purchase_price_str = format_won(brand.scenario.purchase_price);
                            let reward_amount_str = format_won(brand.scenario.reward_amount);
                            let reward_rate_str = format!("{}%", brand.scenario.reward_rate);
                            let activity_reward_str = format_won(brand.scenario.activity_reward);
                            let monthly_reward_str = format_won(brand.scenario.monthly_reward);
                            let six_month_str = format_won(brand.scenario.six_month_total);
                            let treasury_str = format_usd(brand.stats.treasury);
                            let users_str = format_number(brand.stats.users);
                            let floor_str = format!("{:.4}", brand.stats.floor_price);
                            let scenario_summary = format!("{} {} -> {} reward", brand.scenario.purchase_item, purchase_price_str, reward_amount_str);

                            rsx! {
                                div {
                                    class: "rounded-2xl relative overflow-hidden reveal-bounce flex flex-col",
                                    style: "{card_style}",
                                    // Top accent line
                                    div {
                                        class: "absolute top-0 left-[10%] right-[10%] h-[2px]",
                                        style: "{line_style}",
                                    }
                                    // Card content
                                    div {
                                        class: "p-6 flex flex-col flex-1",
                                        // Brand name + segment badge
                                        div {
                                            class: "flex items-center gap-2 mb-3",
                                            h3 {
                                                class: "text-lg font-bold",
                                                style: "color: #e8eefc;",
                                                "{brand.brand}"
                                            }
                                            span {
                                                class: "text-xs font-medium px-2 py-0.5 rounded-full",
                                                style: "{badge_style}",
                                                "{brand.segment}"
                                            }
                                        }
                                        // Tagline
                                        p {
                                            class: "text-sm font-semibold mb-2",
                                            style: "{tagline_color}",
                                            "{brand.tagline}"
                                        }
                                        // Scenario summary
                                        p {
                                            class: "text-xs mb-4",
                                            style: "color: #7a8ba6;",
                                            "{scenario_summary}"
                                        }
                                        // Customer quote
                                        div {
                                            class: "pl-3 mb-4",
                                            style: "{quote_border}",
                                            p {
                                                class: "text-xs leading-relaxed italic",
                                                style: "color: #c8d4e8;",
                                                "\"{brand.customer_quote}\""
                                            }
                                            p {
                                                class: "text-xs mt-1 text-right",
                                                style: "color: #7a8ba6;",
                                                "- {brand.customer_name}"
                                            }
                                        }
                                        // Stats row
                                        div {
                                            class: "grid grid-cols-3 gap-2 mb-4",
                                            div {
                                                class: "text-center rounded-lg py-2",
                                                style: "{stats_border} background: rgba(10,16,26,0.5);",
                                                p { class: "text-xs", style: "color: #7a8ba6;", "Treasury" }
                                                p { class: "text-sm font-bold", style: "color: #e8eefc;", "{treasury_str}" }
                                            }
                                            div {
                                                class: "text-center rounded-lg py-2",
                                                style: "{stats_border} background: rgba(10,16,26,0.5);",
                                                p { class: "text-xs", style: "color: #7a8ba6;", "Users" }
                                                p { class: "text-sm font-bold", style: "color: #e8eefc;", "{users_str}" }
                                            }
                                            div {
                                                class: "text-center rounded-lg py-2",
                                                style: "{stats_border} background: rgba(10,16,26,0.5);",
                                                p { class: "text-xs", style: "color: #7a8ba6;", "Floor" }
                                                p { class: "text-sm font-bold", style: "{detail_label_color}", "{floor_str}" }
                                            }
                                        }
                                        // Expandable details
                                        details {
                                            class: "rounded-lg mt-auto",
                                            style: "background: rgba(10,16,26,0.4); border: 1px solid rgba(255,255,255,0.06);",
                                            summary {
                                                class: "cursor-pointer px-4 py-2 text-xs font-semibold list-none text-center",
                                                style: "{detail_label_color}",
                                                "\u{C0C1}\u{C138} \u{BCF4}\u{AE30} \u{25BE}"
                                            }
                                            div {
                                                class: "px-4 pb-4",
                                                // 4-step flow
                                                p {
                                                    class: "text-xs font-bold mb-2 mt-2",
                                                    style: "color: #e8eefc;",
                                                    "\u{B9AC}\u{C6CC}\u{B4DC} \u{D50C}\u{B85C}\u{C6B0}"
                                                }
                                                div {
                                                    class: "space-y-1 mb-4",
                                                    for (si, step) in brand.steps.iter().enumerate() {
                                                        {
                                                            let step_num = si + 1;
                                                            let step_label = format!("{}. {} - {}", step_num, step.title, step.desc);
                                                            rsx! {
                                                                p {
                                                                    class: "text-xs",
                                                                    style: "color: #7a8ba6;",
                                                                    "{step.icon} {step_label}"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                // Scenario details
                                                p {
                                                    class: "text-xs font-bold mb-2",
                                                    style: "color: #e8eefc;",
                                                    "\u{C2DC}\u{B098}\u{B9AC}\u{C624} \u{C0C1}\u{C138}"
                                                }
                                                div {
                                                    class: "space-y-1",
                                                    div {
                                                        class: "flex justify-between",
                                                        span { class: "text-xs", style: "color: #7a8ba6;", "\u{AD6C}\u{B9E4} \u{C0C1}\u{D488}" }
                                                        span { class: "text-xs font-semibold", style: "color: #e8eefc;", "{brand.scenario.purchase_item}" }
                                                    }
                                                    div {
                                                        class: "flex justify-between",
                                                        span { class: "text-xs", style: "color: #7a8ba6;", "\u{AD6C}\u{B9E4} \u{AC00}\u{ACA9}" }
                                                        span { class: "text-xs font-semibold", style: "color: #e8eefc;", "{purchase_price_str}" }
                                                    }
                                                    div {
                                                        class: "flex justify-between",
                                                        span { class: "text-xs", style: "color: #7a8ba6;", "\u{B9AC}\u{C6CC}\u{B4DC}\u{C728}" }
                                                        span { class: "text-xs font-bold", style: "{detail_label_color}", "{reward_rate_str}" }
                                                    }
                                                    div {
                                                        class: "flex justify-between",
                                                        span { class: "text-xs", style: "color: #7a8ba6;", "\u{AD6C}\u{B9E4} \u{B9AC}\u{C6CC}\u{B4DC}" }
                                                        span { class: "text-xs font-semibold", style: "color: #e8eefc;", "{reward_amount_str}" }
                                                    }
                                                    div {
                                                        class: "flex justify-between",
                                                        span { class: "text-xs", style: "color: #7a8ba6;", "\u{D65C}\u{B3D9} ({brand.scenario.activity_type})" }
                                                        span { class: "text-xs font-semibold", style: "color: #e8eefc;", "{brand.scenario.activity_detail}" }
                                                    }
                                                    div {
                                                        class: "flex justify-between",
                                                        span { class: "text-xs", style: "color: #7a8ba6;", "\u{D65C}\u{B3D9} \u{B9AC}\u{C6CC}\u{B4DC}" }
                                                        span { class: "text-xs font-semibold", style: "color: #e8eefc;", "{activity_reward_str}" }
                                                    }
                                                    div {
                                                        class: "flex justify-between",
                                                        span { class: "text-xs", style: "color: #7a8ba6;", "\u{C6D4}\u{AC04} \u{B9AC}\u{C6CC}\u{B4DC}" }
                                                        span { class: "text-xs font-bold", style: "{detail_label_color}", "{monthly_reward_str}" }
                                                    }
                                                    div {
                                                        class: "flex justify-between pt-1",
                                                        style: "border-top: 1px solid rgba(255,255,255,0.06);",
                                                        span { class: "text-xs font-bold", style: "color: #e8eefc;", "6\u{AC1C}\u{C6D4} \u{CD1D} \u{B9AC}\u{C6CC}\u{B4DC}" }
                                                        span { class: "text-sm font-extrabold", style: "{detail_label_color}", "{six_month_str}" }
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
