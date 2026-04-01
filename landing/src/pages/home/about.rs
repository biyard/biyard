use dioxus::prelude::*;

use super::svgs::{TransformArrow, BEFORE_DIAGRAM_SVG, AFTER_DIAGRAM_SVG};

#[component]
pub(super) fn AboutSection() -> Element {
    let left_svg = BEFORE_DIAGRAM_SVG;
    let right_svg = AFTER_DIAGRAM_SVG;

    rsx! {
        section {
            id: "about",
            class: "py-20 px-4 relative overflow-hidden",
            style: "background: #0c1018;",
            // Amber/orange glow from top-right
            div { class: "absolute", style: "top: -100px; right: -100px; width: 500px; height: 500px; background: radial-gradient(circle, rgba(251,191,36,0.08) 0%, transparent 70%); pointer-events: none;" }
            div { class: "absolute", style: "bottom: -50px; left: -100px; width: 400px; height: 400px; background: radial-gradient(circle, rgba(244,114,182,0.06) 0%, transparent 70%); pointer-events: none;" }
            div {
                class: "max-w-6xl mx-auto relative z-10",
                div {
                    class: "text-center mb-16 reveal-fade",
                    p {
                        class: "text-sm font-semibold tracking-widest uppercase mb-4",
                        style: "color: #00d4aa;",
                        "WHY BIYARD?"
                    }
                    h2 {
                        class: "text-3xl md:text-4xl font-bold reveal-type",
                        style: "color: #e8eefc;",
                        "기존 거래소/증시의 문제, Biyard가 해결합니다"
                    }
                }
                div {
                    class: "grid grid-cols-1 lg:grid-cols-[1fr_auto_1fr] gap-6 items-center",

                    // LEFT PANEL: 기존 거래소
                    div {
                        class: "rounded-2xl p-8 relative overflow-hidden reveal-bounce",
                        style: "background: rgba(30,30,40,0.6); backdrop-filter: blur(12px); -webkit-backdrop-filter: blur(12px); border: 1px solid rgba(239,68,68,0.15); box-shadow: 0 8px 32px rgba(0,0,0,0.3);",
                        div {
                            class: "absolute top-4 right-4 px-3 py-1 rounded-full text-xs font-bold",
                            style: "background: rgba(239,68,68,0.1); color: #ef4444;",
                            "BEFORE"
                        }
                        h3 {
                            class: "text-xl font-bold mb-6",
                            style: "color: #7a8ba6;",
                            "기존 거래소"
                        }
                        // SVG Diagram
                        div {
                            class: "flex justify-center mb-6 h-64",
                            div {
                                dangerous_inner_html: "{left_svg}",
                            }
                        }
                        // Bullet points
                        div {
                            class: "space-y-2",
                            for text in [
                                "실적 없는 토큰이 상장되어 투자자 피해 반복",
                                "무분별한 물량 희석으로 보유자 가치 훼손",
                                "근거 없는 가격 변동으로 시장 신뢰 상실",
                                "정보 비대칭 \u{2014} 내부자만 유리한 구조",
                                "자금 흐름 불투명 \u{2014} 검증 불가능",
                            ] {
                                div {
                                    class: "flex items-start gap-2",
                                    div {
                                        class: "w-1.5 h-1.5 rounded-full flex-shrink-0 mt-1.5",
                                        style: "background: #ef4444; opacity: 0.6;",
                                    }
                                    p { class: "text-sm", style: "color: #7a8ba6;", "{text}" }
                                }
                            }
                        }
                        // Stats comparison box
                        div {
                            class: "mt-4 rounded-lg p-3 flex justify-between text-center",
                            style: "background: rgba(239,68,68,0.06); border: 1px solid rgba(239,68,68,0.12);",
                            div {
                                p { class: "text-xs", style: "color: #7a8ba6;", "평균 토큰 수명" }
                                p { class: "text-sm font-bold", style: "color: #ef4444;", "6개월" }
                            }
                            div {
                                p { class: "text-xs", style: "color: #7a8ba6;", "투자자 손실률" }
                                p { class: "text-sm font-bold", style: "color: #ef4444;", "80%+" }
                            }
                        }
                    }

                    // CENTER: Transform arrow
                    div {
                        class: "hidden lg:flex items-center justify-center self-center",
                        div {
                            class: "flex flex-col items-center gap-2",
                            div {
                                style: "display: flex; align-items: center; justify-content: center;",
                                TransformArrow {}
                            }
                            p {
                                class: "text-xs font-bold tracking-widest",
                                style: "color: #00d4aa;",
                                "TRANSFORM"
                            }
                            p {
                                class: "text-xs font-bold text-center",
                                style: "color: #00d4aa;",
                                "Biyard가 해결합니다"
                            }
                        }
                    }

                    // RIGHT PANEL: Biyard 런치패드
                    div {
                        class: "rounded-2xl p-8 relative overflow-hidden reveal-bounce",
                        style: "background: rgba(0,212,170,0.05); backdrop-filter: blur(12px); -webkit-backdrop-filter: blur(12px); border: 1px solid rgba(0,212,170,0.2); box-shadow: 0 8px 32px rgba(0,212,170,0.1), inset 0 1px 0 rgba(0,212,170,0.1);",
                        div {
                            class: "absolute top-4 right-4 px-3 py-1 rounded-full text-xs font-bold",
                            style: "background: rgba(0,212,170,0.15); color: #00d4aa;",
                            "AFTER"
                        }
                        h3 {
                            class: "text-xl font-bold mb-6",
                            style: "color: #00d4aa;",
                            "Biyard 런치패드"
                        }
                        // SVG Diagram
                        div {
                            class: "flex justify-center mb-6 h-64",
                            div {
                                dangerous_inner_html: "{right_svg}",
                            }
                        }
                        // Bullet points
                        div {
                            class: "space-y-2",
                            for text in [
                                "실제 매출이 있는 기업만 토큰 발행 가능",
                                "트레저리 = 매출 온체인 증명 \u{2014} 가짜 불가",
                                "하한가 스마트 컨트랙트 \u{2014} 덤핑해도 바닥 보장",
                                "매출이 늘면 모든 홀더 자산 가치 자동 상승",
                                "모든 자금 흐름 온체인 공개 \u{2014} 완전한 투명성",
                            ] {
                                div {
                                    class: "flex items-start gap-2",
                                    div {
                                        class: "w-1.5 h-1.5 rounded-full flex-shrink-0 mt-1.5",
                                        style: "background: #00d4aa;",
                                    }
                                    p { class: "text-sm", style: "color: #e8eefc;", "{text}" }
                                }
                            }
                        }
                        // Stats comparison box
                        div {
                            class: "mt-4 rounded-lg p-3 flex justify-between text-center",
                            style: "background: rgba(0,212,170,0.06); border: 1px solid rgba(0,212,170,0.12);",
                            div {
                                p { class: "text-xs", style: "color: #7a8ba6;", "매출 기반 가치 보장" }
                                p { class: "text-sm font-bold", style: "color: #00d4aa;", "100%" }
                            }
                            div {
                                p { class: "text-xs", style: "color: #7a8ba6;", "온체인 투명성" }
                                p { class: "text-sm font-bold", style: "color: #00d4aa;", "24/7" }
                            }
                        }
                    }
                }

                // Bottom summary stats (compact)
                div {
                    class: "grid grid-cols-1 sm:grid-cols-3 gap-4 mt-10",
                    for (value, label) in [
                        ("100%", "매출 기반 가치 보장"),
                        ("0%", "스캠 토큰 상장 가능성"),
                        ("24/7", "온체인 투명성 공개"),
                    ] {
                        div {
                            class: "text-center rounded-xl py-4 backdrop-blur-md",
                            style: "background: rgba(20,28,43,0.5); border: 1px solid rgba(0,212,170,0.15); box-shadow: 0 4px 16px rgba(0,0,0,0.2);",
                            p {
                                class: "text-2xl font-extrabold",
                                style: "color: #00d4aa;",
                                "{value}"
                            }
                            p {
                                class: "text-xs mt-1",
                                style: "color: #7a8ba6;",
                                "{label}"
                            }
                        }
                    }
                }
            }
        }
    }
}
