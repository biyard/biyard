use dioxus::prelude::*;
use crate::components::UserNav;

struct Holding {
    token: &'static str,
    brand: &'static str,
    price: f64,
    change: f64,
    amount: f64,
}

const HOLDINGS: &[Holding] = &[
    Holding { token: "LMT", brand: "Le Mouton", price: 0.0245, change: 3.2, amount: 8.5 },
    Holding { token: "CBT", brand: "Cafe Blossom", price: 0.0246, change: 1.8, amount: 3.24 },
    Holding { token: "RPT", brand: "RunPulse", price: 0.0179, change: 5.4, amount: 1.5 },
];

struct Activity {
    icon: &'static str,
    text: &'static str,
    time: &'static str,
}

const ACTIVITIES: &[Activity] = &[
    Activity { icon: "📈", text: "LMT 토큰 가치 +3.2% 상승", time: "1시간 전" },
    Activity { icon: "🛍️", text: "Le Mouton 구매 → 2,580원 적립", time: "3시간 전" },
    Activity { icon: "🏃", text: "걷기 챌린지 85 포인트 획득", time: "어제" },
    Activity { icon: "🔄", text: "200 포인트 → 2 LMT 전환", time: "2일 전" },
    Activity { icon: "🗳️", text: "'리워드 2배' 제안에 투표 완료", time: "3일 전" },
];

#[component]
pub fn Wallet() -> Element {
    let total_value: f64 = HOLDINGS.iter().map(|h| h.amount * h.price).sum();
    let total_krw = (total_value * 1200.0) as i64;
    let total_value_str = format!("${:.2}", total_value);
    let total_krw_str = format!("≈ ₩{}", total_krw);

    rsx! {
        div {
            style: "background: #0a0e17; color: #e8eefc; min-height: 100vh;",
            UserNav {}
            div {
                class: "max-w-5xl mx-auto px-4 py-10",
                // Portfolio Value
                div {
                    class: "rounded-2xl p-8 mb-8",
                    style: "background: linear-gradient(135deg, #141c2b, #1a2435); border: 1px solid rgba(0,212,170,0.12);",
                    p { class: "text-sm text-gray-400", "Total Portfolio Value" }
                    div {
                        class: "flex items-end gap-3 mt-1",
                        span { class: "text-5xl font-extrabold", "{total_value_str}" }
                        span { class: "text-lg text-gray-400 pb-1", "{total_krw_str}" }
                    }
                    div {
                        class: "mt-3",
                        span {
                            class: "inline-flex items-center px-2.5 py-1 rounded-full text-xs font-semibold",
                            style: "background: rgba(0,212,170,0.15); color: #00d4aa;",
                            "▲ 3.1% this month"
                        }
                    }
                }
                // Holdings Table
                div {
                    class: "rounded-2xl overflow-hidden mb-8",
                    style: "background: #141c2b; border: 1px solid rgba(0,212,170,0.12);",
                    div {
                        class: "px-6 py-4",
                        style: "border-bottom: 1px solid rgba(0,212,170,0.12);",
                        h2 { class: "font-semibold", "My Holdings" }
                    }
                    table {
                        class: "w-full text-sm",
                        thead {
                            tr {
                                class: "text-left text-xs text-gray-500",
                                style: "border-bottom: 1px solid rgba(0,212,170,0.12);",
                                th { class: "px-6 py-3", "Token" }
                                th { class: "px-6 py-3 text-right", "Price" }
                                th { class: "px-6 py-3 text-right", "24h" }
                                th { class: "px-6 py-3 text-right", "Holdings" }
                                th { class: "px-6 py-3 text-right", "Value" }
                            }
                        }
                        tbody {
                            for h in HOLDINGS.iter() {
                                {
                                    let price_str = format!("${:.4}", h.price);
                                    let change_str = format!("+{}%", h.change);
                                    let value_str = format!("${:.4}", h.amount * h.price);
                                    rsx! {
                                        tr {
                                            style: "border-bottom: 1px solid rgba(0,212,170,0.08);",
                                            td {
                                                class: "px-6 py-4",
                                                span { class: "font-semibold", "{h.token}" }
                                                span { class: "ml-2 text-xs text-gray-500", "{h.brand}" }
                                            }
                                            td { class: "px-6 py-4 text-right font-mono", "{price_str}" }
                                            td {
                                                class: "px-6 py-4 text-right font-medium",
                                                style: "color: #00d4aa;",
                                                "{change_str}"
                                            }
                                            td { class: "px-6 py-4 text-right", "{h.amount}" }
                                            td {
                                                class: "px-6 py-4 text-right font-semibold",
                                                "{value_str}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                // Recent Activity
                div {
                    class: "rounded-2xl p-6",
                    style: "background: #141c2b; border: 1px solid rgba(0,212,170,0.12);",
                    h3 { class: "font-semibold mb-4", "Recent Activity" }
                    div {
                        class: "space-y-3",
                        for a in ACTIVITIES.iter() {
                            div {
                                class: "flex items-start gap-3",
                                span { class: "text-lg leading-5", "{a.icon}" }
                                div {
                                    class: "flex-1",
                                    p { class: "text-sm", "{a.text}" }
                                    p { class: "text-xs text-gray-600", "{a.time}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
