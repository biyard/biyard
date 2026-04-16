use dioxus::prelude::*;

/// Top section — Paradigm shift explanation
/// "Why companies should abandon walled garden for open fandom"
#[component]
pub(super) fn ParadigmSection() -> Element {
    rsx! {
        section {
            class: "demo-section",
            style: "padding-top: 120px;",
            div {
                class: "demo-container",

                // Label
                p {
                    style: "color: #00dfc0; font-size: 11px; font-weight: 900; letter-spacing: 0.4em; text-transform: uppercase; text-align: center; margin-bottom: 24px;",
                    "Paradigm Shift"
                }

                // Headline
                h2 {
                    class: "fandom-hero-text",
                    style: "text-align: center; margin-bottom: 24px;",
                    "기업은 왜 "
                    span { style: "color: #ff9999; font-style: italic;", "Walled Garden" }
                    "을 포기하고"
                    br {}
                    span { class: "gradient-text-mint", "Open Fandom" }
                    "으로 가야 하는가?"
                }

                p {
                    style: "text-align: center; color: #94a3b8; font-size: 16px; line-height: 1.8; max-width: 760px; margin: 0 auto 64px;",
                    "지난 30년간 기업들은 포인트 제도로 고객을 "
                    span { style: "color: #fff; font-weight: 700;", "가두는 데 성공" }
                    "했습니다. 하지만 그들을 "
                    span { style: "color: #fff; font-weight: 700;", "팬으로 만드는 데는 실패" }
                    "했죠. 갇힌 고객은 이탈 기회가 생기면 떠나고, 팬은 경쟁자가 와도 남습니다."
                }

                // Comparison grid
                div {
                    class: "paradigm-grid",

                    // Before — Walled Garden
                    div {
                        class: "paradigm-card paradigm-before",
                        p {
                            style: "font-size: 10px; font-weight: 900; letter-spacing: 0.3em; color: #ff6b6b; text-transform: uppercase; margin-bottom: 8px;",
                            "Before"
                        }
                        h3 {
                            style: "font-size: 28px; font-weight: 900; color: #fff; margin-bottom: 20px;",
                            "Walled Garden"
                        }
                        p {
                            style: "color: #fca5a5; font-size: 14px; line-height: 1.7; margin-bottom: 24px;",
                            "\"고객을 가둬두자\" — 락인 전략"
                        }
                        div {
                            class: "space-y-3",
                            for line in [
                                "포인트는 기업만 사용 가능 (타 브랜드 무가치)",
                                "만료되면 사라지는 부채",
                                "고객이 떠나면 모든 축적이 0원",
                                "기업이 가치를 일방적으로 결정",
                                "고객은 소비자, 기업은 벤더",
                            ] {
                                p {
                                    style: "color: #cbd5e1; font-size: 13px; padding: 8px 0; border-bottom: 1px solid rgba(255,77,77,0.08);",
                                    "\u{2717} "
                                    span { style: "color: #94a3b8;", "{line}" }
                                }
                            }
                        }
                    }

                    // After — Open Fandom
                    div {
                        class: "paradigm-card paradigm-after",
                        p {
                            style: "font-size: 10px; font-weight: 900; letter-spacing: 0.3em; color: #00dfc0; text-transform: uppercase; margin-bottom: 8px;",
                            "Biyard Launchpad"
                        }
                        h3 {
                            style: "font-size: 28px; font-weight: 900; color: #fff; margin-bottom: 20px;",
                            "Open Fandom"
                        }
                        p {
                            style: "color: #00dfc0; font-size: 14px; line-height: 1.7; margin-bottom: 24px;",
                            "\"가두지 않고도 남게 하자\" — 공동 소유 전략"
                        }
                        div {
                            class: "space-y-3",
                            for line in [
                                "팬덤 지분이 매출과 함께 자산화",
                                "영구 보유 가능한 온체인 자산",
                                "팬은 스스로 브랜드를 홍보",
                                "가치가 공식(Treasury\u{00F7}Supply)으로 투명",
                                "고객은 팬, 기업은 팬덤의 중심",
                            ] {
                                p {
                                    style: "color: #cbd5e1; font-size: 13px; padding: 8px 0; border-bottom: 1px solid rgba(0,223,192,0.08);",
                                    "\u{2713} "
                                    span { style: "color: #e2e8f0;", "{line}" }
                                }
                            }
                        }
                    }
                }

                // Fandom paradigm insight box
                div {
                    style: "margin-top: 48px; padding: 32px 40px; background: linear-gradient(135deg, rgba(0,223,192,0.06), rgba(112,0,255,0.04)); border: 1px solid rgba(0,223,192,0.15); border-radius: 20px;",
                    p {
                        style: "font-size: 11px; font-weight: 900; letter-spacing: 0.3em; color: #00dfc0; text-transform: uppercase; text-align: center; margin-bottom: 16px;",
                        "Reference — Fandom Economy"
                    }
                    h3 {
                        style: "font-size: 22px; font-weight: 700; color: #fff; text-align: center; line-height: 1.5; margin-bottom: 16px;",
                        "BTS 아미, 블랙핑크 블링크, 뉴진스 버니즈 \u{2014} "
                        br {}
                        "엔터 업계는 이미 증명했습니다. "
                        span { class: "gradient-text-mint", "팬덤 = 조 단위 자산." }
                    }
                    p {
                        style: "text-align: center; color: #94a3b8; font-size: 14px; line-height: 1.7; max-width: 720px; margin: 0 auto;",
                        "Weverse는 엔터 기업만 입점 가능한 닫힌 생태계입니다. Biyard Launchpad는 "
                        span { style: "color: #e2e8f0; font-weight: 700;", "모든 브랜드가 자기만의 팬덤 경제를 만드는" }
                        " 범용 인프라를 목표합니다."
                    }
                }
            }
        }
    }
}
