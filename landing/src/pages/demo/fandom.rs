use dioxus::prelude::*;

/// Deep-dive on why fandom is the right frame + what fandoms want
#[component]
pub(super) fn FandomSection() -> Element {
    rsx! {
        section {
            class: "demo-section",
            div {
                class: "demo-container",
                div { style: "text-align: center; margin-bottom: 48px;",
                    p { style: "color: #00dfc0; font-size: 11px; font-weight: 900; letter-spacing: 0.4em; text-transform: uppercase;", "The Fandom Playbook" }
                    h2 { class: "fandom-hero-text", style: "margin-top: 16px;",
                        "팬덤은 "
                        span { class: "gradient-text-mint", "무엇을 원하는가?" }
                    }
                    p { style: "color: #94a3b8; font-size: 15px; line-height: 1.7; margin: 24px auto 0; max-width: 720px;",
                        "엔터 업계는 이미 답을 찾았습니다. 르무통 같은 D2C 브랜드에게 이 공식을 복제해주는 것이 Biyard의 역할입니다."
                    }
                }

                // 4 cards
                div { style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(260px, 1fr)); gap: 20px; margin-bottom: 64px;",
                    for (emoji, title, desc, ex) in [
                        ("\u{1F451}", "소속감", "나는 이 브랜드의 '시민'이다. 나만 아는 코드와 문화가 있다.", "ARMY \u{00B7} 셰퍼드 Lv.5"),
                        ("\u{1F4B0}", "경제적 공유", "브랜드 성장의 과실을 내가 일부 받는다. 단순 소비자가 아니다.", "매출 \u{2191} \u{2192} 팬덤 지분 \u{2191}"),
                        ("\u{1F5F3}", "영향력", "브랜드의 결정에 내 목소리가 반영된다. 팬이 방향을 만든다.", "DAO \u{00B7} 신상 컬러 투표"),
                        ("\u{1F3C6}", "지위", "팬덤 내 랭킹과 배지. 오래 깊게 팬인 사람이 인정받는다.", "Shepherd Lv.1\u{2192}5"),
                    ] {
                        div { class: "fandom-card",
                            p { style: "font-size: 40px; margin-bottom: 12px;", "{emoji}" }
                            h3 { style: "font-size: 18px; font-weight: 900; color: #fff; margin-bottom: 8px;", "{title}" }
                            p { style: "font-size: 13px; color: #94a3b8; line-height: 1.7; margin-bottom: 12px;", "{desc}" }
                            p { style: "font-size: 11px; color: #00dfc0; font-family: monospace;", "{ex}" }
                        }
                    }
                }

                // Case application to Le Mouton
                div { style: "padding: 40px; background: linear-gradient(135deg, rgba(139,115,85,0.08), rgba(0,223,192,0.04)); border: 1px solid rgba(212,197,176,0.15); border-radius: 24px;",
                    p { style: "color: #D4C5B0; font-size: 11px; font-weight: 900; letter-spacing: 0.3em; text-transform: uppercase; margin-bottom: 12px;", "\u{1F411} Le Mouton Shepherd" }
                    h3 { style: "font-size: clamp(1.6rem, 3vw, 2.2rem); font-weight: 900; color: #fff; line-height: 1.3; margin-bottom: 20px;",
                        "르무통 팬덤을 "
                        span { class: "gradient-text-mint", "\"셰퍼드(Shepherd)\"" }
                        "로 명명합니다."
                    }
                    p { style: "color: #cbd5e1; font-size: 14px; line-height: 1.8; margin-bottom: 16px;",
                        "양털(merino wool)을 키우는 목동에서 따온 이름. 르무통 신발을 사랑하고, 걷고, 이야기하는 사람들의 정체성입니다."
                    }
                    div { style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(160px, 1fr)); gap: 12px; margin-top: 20px;",
                        for (lvl, name, threshold) in [
                            ("Lv.1", "Newborn", "가입"),
                            ("Lv.2", "Wool Walker", "첫 구매"),
                            ("Lv.3", "Shepherd", "2회 구매 + 인증"),
                            ("Lv.4", "Master", "5회 구매 + DAO"),
                            ("Lv.5", "Legend", "10회 + 커뮤니티 기여"),
                        ] {
                            div { style: "padding: 14px; background: rgba(2,4,8,0.4); border: 1px solid rgba(0,223,192,0.15); border-radius: 12px; text-align: center;",
                                p { style: "font-size: 10px; color: #00dfc0; font-weight: 900; letter-spacing: 0.2em;", "{lvl}" }
                                p { style: "font-size: 14px; font-weight: 700; color: #fff; margin: 4px 0;", "{name}" }
                                p { style: "font-size: 10px; color: #64748b;", "{threshold}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
