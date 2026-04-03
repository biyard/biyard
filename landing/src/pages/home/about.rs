use dioxus::prelude::*;

#[component]
pub(super) fn AboutSection() -> Element {
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
                    span { class: "font-bold tracking-widest uppercase mb-4 block", style: "color: #ff4d4d; font-size: 10px;", "The Problem" }
                    h2 {
                        class: "text-4xl md:text-7xl font-black mb-8 leading-tight",
                        "토큰 시장의 "
                        span { class: "danger-text", "80%는 실패" }
                        "합니다."
                    }
                    div {
                        class: "inline-block px-10 py-4 glass-panel rounded-xl mb-8",
                        style: "border-color: rgba(255,77,77,0.3);",
                        div { class: "text-6xl font-black italic mb-2", style: "color: #ff4d4d;", "6개월" }
                        p { class: "text-xs uppercase tracking-widest", style: "color: #64748b; letter-spacing: 0.3em;", "기존 토큰의 평균 수명" }
                    }
                    p { class: "text-lg max-w-3xl mx-auto", style: "color: #94a3b8;", "실적 없는 토큰, 근거 없는 가격, 투명하지 않은 자금 흐름. 투자자는 반복적으로 피해를 입고, 시장 신뢰는 무너졌습니다." }
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
                            h3 { class: "text-2xl font-bold", "기존 거래소" }
                            span { class: "px-3 py-1 rounded text-red-400 font-bold uppercase", style: "background: rgba(255,77,77,0.2); font-size: 10px;", "Before" }
                        }
                        div {
                            class: "mb-10 flex justify-center",
                            div { class: "p-8 border-2 border-dashed rounded-full text-sm font-bold", style: "border-color: rgba(255,77,77,0.2); color: rgba(255,77,77,0.4);", "실적 없는 프로젝트" }
                        }
                        ul {
                            class: "space-y-6",
                            for text in [
                                "실적 없는 토큰 상장 \u{2192} 투자자 피해 반복",
                                "무분별한 물량 희석 \u{2192} 보유자 가치 훼손",
                                "정보 비대칭 \u{2192} 내부자만 유리한 구조",
                                "자금 흐름 불투명 \u{2192} 검증 불가능",
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
                            h3 { class: "text-2xl font-bold", "Biyard 런치패드" }
                            span { class: "px-3 py-1 rounded font-bold uppercase", style: "background: rgba(0,223,192,0.2); color: #00dfc0; font-size: 10px;", "Biyard" }
                        }
                        div {
                            class: "mb-10 flex justify-center",
                            div {
                                class: "p-8 glass-panel rounded-full font-bold text-sm",
                                style: "border-color: rgba(0,223,192,0.4); color: #00dfc0; box-shadow: 0 0 20px rgba(0,223,192,0.2);",
                                "실제 매출 기반 기업"
                            }
                        }
                        ul {
                            class: "space-y-6",
                            for text in [
                                "매출이 있는 기업만 토큰 발행 가능",
                                "트레저리 = 온체인 매출 증명. 가짜 불가",
                                "하한가 스마트 컨트랙트로 바닥 보장",
                                "모든 자금 흐름 온체인 공개",
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
                        span { style: "color: #00dfc0; font-size: 10px; font-weight: 900; letter-spacing: 0.4em; text-transform: uppercase;", "How It Works" }
                        h3 { class: "text-3xl md:text-5xl font-black mt-4", "Biyard는 어떻게 " span { class: "glow-text", "다를까요?" } }
                    }

                    div {
                        class: "glass-panel p-10 md:p-14 rounded-3xl",
                        style: "border-color: rgba(0,223,192,0.1);",

                        // 4-step flow
                        div {
                            class: "grid md:grid-cols-4 gap-8 mb-12",
                            for (i, (icon, title, desc)) in [
                                ("\u{1F6D2}", "고객이 구매", "기업의 상품/서비스를 구매하면 결제 금액의 2~4%가 자동으로 적립됩니다."),
                                ("\u{1F3E6}", "트레저리 적립", "적립된 금액이 온체인 트레저리에 누적됩니다. 이것이 토큰 가치의 근간이 됩니다."),
                                ("\u{2705}", "활동 리워드", "걷기, 매장 방문, SNS 공유 등 활동 인증 시 추가 토큰 리워드가 지급됩니다."),
                                ("\u{1F4C8}", "가치 상승", "매출이 늘수록 트레저리가 쌓이고, 하한가가 올라갑니다. 모든 홀더의 자산이 함께 성장합니다."),
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
                            p { class: "text-sm font-bold mb-3", style: "color: #94a3b8;", "핵심 공식" }
                            p {
                                class: "text-2xl md:text-3xl font-mono font-black",
                                style: "color: #00dfc0; text-shadow: 0 0 20px rgba(0,223,192,0.3);",
                                "Floor Price = Treasury \u{00F7} Supply"
                            }
                            p { class: "text-sm mt-4 max-w-xl mx-auto", style: "color: #64748b;", "매출이 지속되는 한 토큰 가치의 바닥이 수학적으로 보장됩니다. 누군가 하한가 이하로 매도하면 트레저리가 자동 매수 후 소각하여 하한가는 절대 하락하지 않습니다." }
                        }
                    }
                }
            }
        }
    }
}
