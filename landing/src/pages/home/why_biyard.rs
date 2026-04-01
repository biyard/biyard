use dioxus::prelude::*;

use super::svgs::{AFTER_CYCLE_SVG, BEFORE_CYCLE_SVG};

#[component]
pub(super) fn WhyBiyardSection() -> Element {
    rsx! {
        // Why Biyard - Cycle comparison
        section {
            class: "py-20 px-4 relative overflow-hidden",
            style: "background: #0c1018;",
            div { class: "absolute", style: "top: -80px; left: 20%; width: 500px; height: 500px; background: radial-gradient(circle, rgba(239,68,68,0.04) 0%, transparent 65%); pointer-events: none;" }
            div { class: "absolute", style: "bottom: -80px; right: 20%; width: 500px; height: 500px; background: radial-gradient(circle, rgba(52,211,153,0.05) 0%, transparent 65%); pointer-events: none;" }
            div {
                class: "max-w-5xl mx-auto relative z-10",
                h2 {
                    class: "text-2xl md:text-3xl font-bold mb-16 text-center reveal-type",
                    style: "color: #e8eefc; font-family: 'Outfit', 'Noto Sans KR', sans-serif;",
                    "왜 Biyard를 선택해야 하나요?"
                }
                // Two cycle diagrams side by side
                div {
                    class: "grid grid-cols-1 lg:grid-cols-2 gap-10",

                    // LEFT: 기존 마케팅 악순환 (red tones)
                    div {
                        class: "reveal-bounce",
                        div {
                            class: "text-center mb-6",
                            span {
                                class: "text-xs font-bold tracking-widest px-3 py-1 rounded-full",
                                style: "background: rgba(239,68,68,0.1); color: #ef4444;",
                                "BEFORE"
                            }
                        }
                        // Cycle SVG
                        div {
                            class: "flex justify-center mb-6",
                            div {
                                style: "width: 280px; height: 280px; color: #ef4444;",
                                dangerous_inner_html: BEFORE_CYCLE_SVG,
                            }
                        }
                        p {
                            class: "text-center text-sm",
                            style: "color: #ef4444; opacity: 0.7;",
                            "광고비 → 일시적 유치 → 이탈 → 또 광고... 끝없는 악순환"
                        }
                    }

                    // RIGHT: Biyard 선순환 (green tones)
                    div {
                        class: "reveal-bounce",
                        div {
                            class: "text-center mb-6",
                            span {
                                class: "text-xs font-bold tracking-widest px-3 py-1 rounded-full",
                                style: "background: rgba(0,212,170,0.1); color: #00d4aa;",
                                "WITH BIYARD"
                            }
                        }
                        // Cycle SVG
                        div {
                            class: "flex justify-center mb-6",
                            div {
                                style: "width: 280px; height: 280px; color: #00d4aa;",
                                dangerous_inner_html: AFTER_CYCLE_SVG,
                            }
                        }
                        p {
                            class: "text-center text-sm",
                            style: "color: #00d4aa; opacity: 0.8;",
                            "매출 → 가치 상승 → 고객 홍보 → 신규 유입 → 매출... 선순환"
                        }
                    }
                }

                // Strong closing statement
                div {
                    class: "mt-16 text-center max-w-3xl mx-auto reveal-fade",
                    div {
                        class: "rounded-2xl py-10 px-8 relative overflow-hidden",
                        style: "background: rgba(0,212,170,0.04); border: 1px solid rgba(0,212,170,0.12);",
                        // Top glow line
                        div {
                            class: "absolute top-0 left-[15%] right-[15%] h-[2px]",
                            style: "background: linear-gradient(90deg, transparent, rgba(0,212,170,0.4), transparent);",
                        }
                        p {
                            class: "text-2xl md:text-3xl leading-snug",
                            style: "font-family: 'Outfit', 'Noto Sans KR', sans-serif; font-weight: 600;",
                            span {
                                style: "background-image: linear-gradient(to right, #60a5fa, #a78bfa); -webkit-background-clip: text; background-clip: text; color: transparent;",
                                "투자자가 피해를 보는 시장은"
                            }
                            br {}
                            span {
                                style: "background-image: linear-gradient(to right, #a78bfa, #f472b6); -webkit-background-clip: text; background-clip: text; color: transparent;",
                                "이제 끝나야 합니다."
                            }
                        }
                        p {
                            class: "mt-5 text-lg",
                            style: "color: #7a8ba6; font-weight: 400;",
                            "매출이 가치를 만들고, 투명성이 신뢰를 만드는 시장."
                        }
                        p {
                            class: "mt-3 text-xl font-bold",
                            style: "background-image: linear-gradient(to right, #60a5fa, #00d4aa); -webkit-background-clip: text; background-clip: text; color: transparent;",
                            "Biyard Launchpad가 그 시작입니다."
                        }
                    }
                }
            }
        }
    }
}
