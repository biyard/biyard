use dioxus::prelude::*;

// ─────────────────────────────────────────────────────────
// Biyard Launchpad overlay screens — ALL marked with badge
// These are what Biyard provides on top of Le Mouton's app
// ─────────────────────────────────────────────────────────

/// After-purchase Fandom level-up modal (Biyard provides)
#[component]
pub(super) fn BiyardFandomReward() -> Element {
    rsx! {
        div { style: "height: 100%; display: flex; flex-direction: column; background: #FAF7F2; position: relative;",
            // Faint underlying Le Mouton confirmation
            div { class: "lm-header",
                span { style: "font-size: 14px; color: #2C2420;", "결제 완료" }
                span {}
                span {}
            }
            div { style: "padding: 20px; opacity: 0.35;",
                p { style: "font-size: 13px; color: #2C2420;", "결제가 완료되었습니다" }
                p { style: "font-size: 11px; color: #6B5D52; margin-top: 4px;", "주문번호 LM-2404-087654" }
            }

            // Biyard overlay — fandom level up
            div {
                class: "biyard-zone",
                style: "position: absolute; inset: 40px 16px; padding: 20px; display: flex; flex-direction: column; gap: 14px;",
                div { class: "biyard-badge", "\u{2B21} BIYARD LAUNCHPAD" }

                div { style: "text-align: center; padding-top: 8px;",
                    p { style: "font-size: 56px;", "\u{1F411}" }
                    p { style: "color: #00dfc0; font-size: 11px; font-weight: 900; letter-spacing: 0.3em; margin: 8px 0 4px;", "SHEPHERD LEVEL UP" }
                    h3 { style: "font-size: 24px; font-weight: 900; color: #fff; margin-bottom: 6px;",
                        "Lv.2 " span { class: "gradient-text-mint", "Wool Walker" } " 등극!"
                    }
                    p { style: "color: #94a3b8; font-size: 12px; line-height: 1.6;",
                        "르무통 셰퍼드 커뮤니티에"
                        br {}
                        "1,251번째로 합류하셨어요"
                    }
                }

                div { style: "background: rgba(0,223,192,0.05); border: 1px solid rgba(0,223,192,0.2); border-radius: 12px; padding: 14px;",
                    div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px;",
                        span { style: "color: #94a3b8; font-size: 10px; letter-spacing: 0.1em;", "팬덤 지분 획득" }
                        span { style: "color: #00dfc0; font-size: 18px; font-weight: 900;", "+ 3,870" }
                    }
                    div { style: "height: 6px; background: rgba(255,255,255,0.05); border-radius: 3px; overflow: hidden;",
                        div { style: "height: 100%; width: 62%; background: linear-gradient(90deg, #00dfc0, #60a5fa); border-radius: 3px;" }
                    }
                    p { style: "font-size: 10px; color: #64748b; margin-top: 8px;", "Lv.3 Shepherd까지 3,820 남음" }
                }

                div { style: "display: flex; gap: 8px;",
                    div { style: "flex: 1; padding: 12px; background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.06); border-radius: 10px; text-align: center;",
                        p { style: "font-size: 10px; color: #64748b;", "팬덤 규모" }
                        p { style: "font-size: 14px; font-weight: 800; color: #e2e8f0; margin-top: 4px;", "1,251명" }
                    }
                    div { style: "flex: 1; padding: 12px; background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.06); border-radius: 10px; text-align: center;",
                        p { style: "font-size: 10px; color: #64748b;", "팬덤 가치" }
                        p { style: "font-size: 14px; font-weight: 800; color: #00dfc0; margin-top: 4px;", "$24,500" }
                    }
                }

                div { style: "padding: 12px; background: #00dfc0; color: #020408; border-radius: 10px; text-align: center; font-size: 13px; font-weight: 900; letter-spacing: 0.05em; margin-top: auto;",
                    "나의 셰퍼드 대시보드 보기 \u{2192}"
                }
            }
        }
    }
}

/// Shepherd Dashboard — user's fandom home (Biyard provides)
#[component]
pub(super) fn BiyardDashboard() -> Element {
    rsx! {
        div { style: "height: 100%; display: flex; flex-direction: column; background: #020408; position: relative;",
            div { class: "biyard-badge", style: "top: 14px; right: 14px; position: absolute; z-index: 20;", "\u{2B21} BIYARD" }

            // Header
            div { style: "padding: 20px 20px 16px; background: linear-gradient(180deg, rgba(0,223,192,0.08), transparent);",
                p { style: "font-size: 10px; color: #00dfc0; font-weight: 900; letter-spacing: 0.3em;", "LE MOUTON SHEPHERD" }
                p { style: "font-size: 22px; font-weight: 900; color: #fff; margin-top: 4px;", "김서연" span { style: "color: #00dfc0;", "님" } }
                p { style: "font-size: 11px; color: #94a3b8; margin-top: 4px;", "Lv.2 Wool Walker \u{00B7} 상위 15%" }
            }

            // My stake
            div { style: "padding: 0 20px;",
                div { style: "padding: 20px; background: linear-gradient(135deg, rgba(0,223,192,0.08), rgba(112,0,255,0.04)); border: 1px solid rgba(0,223,192,0.2); border-radius: 16px;",
                    p { style: "font-size: 10px; color: #94a3b8; letter-spacing: 0.2em;", "나의 팬덤 지분" }
                    p { style: "font-size: 34px; font-weight: 900; color: #00dfc0; margin: 4px 0; text-shadow: 0 0 20px rgba(0,223,192,0.4);", "12,180원" }
                    p { style: "font-size: 11px; color: #00dfc0;", "\u{2191} +18.2% (지난 30일)" }
                    // Mini chart
                    div { style: "margin-top: 12px; height: 40px; display: flex; align-items: flex-end; gap: 3px;",
                        for h in [30, 40, 35, 50, 45, 60, 55, 70, 65, 80, 75, 90_i32] {
                            div { style: "flex: 1; height: {h}%; background: linear-gradient(180deg, #00dfc0, rgba(0,223,192,0.3)); border-radius: 2px;" }
                        }
                    }
                }
            }

            // Fandom pulse
            div { style: "padding: 16px 20px; display: grid; grid-template-columns: 1fr 1fr; gap: 10px;",
                for (label, val, color) in [
                    ("르무통 팬덤", "1,251명", "#a78bfa"),
                    ("팬덤 총 가치", "$24,500", "#00dfc0"),
                    ("이번 달 성장", "+8.4%", "#34d399"),
                    ("나의 랭킹", "#187", "#fbbf24"),
                ] {
                    div { style: "padding: 14px; background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.06); border-radius: 12px;",
                        p { style: "font-size: 10px; color: #64748b; letter-spacing: 0.1em;", "{label}" }
                        p { style: "font-size: 18px; font-weight: 900; color: {color}; margin-top: 4px;", "{val}" }
                    }
                }
            }

            // Quick actions
            div { style: "padding: 4px 20px; display: flex; gap: 8px; overflow-x: auto;",
                for (icon, label) in [("\u{1F465}", "팬덤 피드"), ("\u{1F5F3}", "DAO 투표"), ("\u{1F4AA}", "걷기 인증"), ("\u{1F3AF}", "셰퍼드 존")] {
                    div { style: "flex-shrink: 0; padding: 10px 14px; background: rgba(0,223,192,0.06); border: 1px solid rgba(0,223,192,0.15); border-radius: 20px; font-size: 11px; color: #00dfc0; font-weight: 700;",
                        "{icon} {label}"
                    }
                }
            }

            // Recent activity
            div { style: "flex: 1; padding: 20px; overflow: auto;",
                p { style: "font-size: 11px; color: #64748b; letter-spacing: 0.2em; margin-bottom: 10px;", "최근 활동" }
                for (emoji, text, val, time) in [
                    ("\u{1F6D2}", "르무통 메이트 구매", "+3,870", "방금"),
                    ("\u{1F6B6}", "오늘 8,412보 달성", "+80", "2시간 전"),
                    ("\u{1F5F3}", "DAO 제안에 찬성", "+20", "1일 전"),
                    ("\u{1F4E2}", "리뷰 작성 \u{00B7} 5점", "+200", "3일 전"),
                ] {
                    div { style: "display: flex; gap: 10px; padding: 10px 0; border-bottom: 1px solid rgba(255,255,255,0.04);",
                        span { style: "font-size: 20px;", "{emoji}" }
                        div { style: "flex: 1;",
                            p { style: "font-size: 12px; color: #e2e8f0;", "{text}" }
                            p { style: "font-size: 10px; color: #64748b; margin-top: 2px;", "{time}" }
                        }
                        span { style: "font-size: 13px; font-weight: 700; color: #00dfc0;", "{val}" }
                    }
                }
            }
        }
    }
}

/// Fandom community feed (Biyard provides)
#[component]
pub(super) fn BiyardFandomFeed() -> Element {
    rsx! {
        div { style: "height: 100%; display: flex; flex-direction: column; background: #020408; position: relative;",
            div { class: "biyard-badge", style: "top: 14px; right: 14px; position: absolute; z-index: 20;", "\u{2B21} BIYARD" }

            div { style: "padding: 20px 20px 16px;",
                p { style: "font-size: 10px; color: #00dfc0; font-weight: 900; letter-spacing: 0.3em;", "FANDOM FEED" }
                p { style: "font-size: 22px; font-weight: 900; color: #fff; margin-top: 4px;", "셰퍼드들의 이야기" }
            }

            div { style: "flex: 1; padding: 0 16px 20px; overflow: auto; display: flex; flex-direction: column; gap: 12px;",
                for (user, lvl, text, emoji, likes) in [
                    ("\u{1F468} 준혁", "Lv.4 Shepherd", "메이트 구매 2개월차! 발 편하고 토큰 가치도 +23% 올랐네요. 지금이 찐팬의 시대 \u{1F525}", "\u{1F45F}", "156"),
                    ("\u{1F469} 민지", "Lv.3 Walker", "오늘 10,000보 걷기 인증 성공! 출근길에 신고 다녀서 3개월만에 1.2km 기록", "\u{1F6B6}", "98"),
                    ("\u{1F468} 현우", "Lv.5 Master", "DAO 투표 제안했어요 \u{2014} '친환경 패키지로 바꾸자'. 셰퍼드님들 투표 부탁 \u{1F64F}", "\u{1F5F3}", "234"),
                ] {
                    div { style: "padding: 14px; background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.06); border-radius: 14px;",
                        div { style: "display: flex; gap: 10px; align-items: center; margin-bottom: 8px;",
                            div { style: "width: 32px; height: 32px; border-radius: 50%; background: rgba(0,223,192,0.15); display: flex; align-items: center; justify-content: center;", "{emoji}" }
                            div { style: "flex: 1;",
                                p { style: "font-size: 12px; color: #e2e8f0; font-weight: 700;", "{user}" }
                                p { style: "font-size: 10px; color: #00dfc0;", "{lvl}" }
                            }
                        }
                        p { style: "font-size: 12px; color: #cbd5e1; line-height: 1.6;", "{text}" }
                        div { style: "display: flex; gap: 14px; margin-top: 10px; font-size: 11px; color: #64748b;",
                            span { "\u{2665}\u{FE0F} {likes}" }
                            span { "\u{1F4AC} 12" }
                            span { "\u{1F501} 공유" }
                        }
                    }
                }
            }
        }
    }
}

/// DAO voting (Biyard provides)
#[component]
pub(super) fn BiyardDao() -> Element {
    rsx! {
        div { style: "height: 100%; display: flex; flex-direction: column; background: #020408; position: relative;",
            div { class: "biyard-badge", style: "top: 14px; right: 14px; position: absolute; z-index: 20;", "\u{2B21} BIYARD" }

            div { style: "padding: 20px 20px 16px;",
                p { style: "font-size: 10px; color: #00dfc0; font-weight: 900; letter-spacing: 0.3em;", "SHEPHERD DAO" }
                p { style: "font-size: 22px; font-weight: 900; color: #fff; margin-top: 4px;", "팬덤 투표" }
                p { style: "font-size: 12px; color: #94a3b8; margin-top: 6px;", "Lv.2 이상 셰퍼드만 참여 가능" }
            }

            div { style: "flex: 1; padding: 0 16px 20px; overflow: auto; display: flex; flex-direction: column; gap: 12px;",
                // Active vote
                div { style: "padding: 16px; background: linear-gradient(135deg, rgba(0,223,192,0.08), rgba(112,0,255,0.03)); border: 1px solid rgba(0,223,192,0.3); border-radius: 14px;",
                    p { style: "font-size: 10px; color: #00dfc0; letter-spacing: 0.2em; margin-bottom: 8px;", "\u{25CF} 진행 중" }
                    p { style: "font-size: 15px; font-weight: 700; color: #fff; margin-bottom: 6px;", "2025 F/W 신상 컬러 선택" }
                    p { style: "font-size: 11px; color: #94a3b8; line-height: 1.6; margin-bottom: 14px;", "다음 시즌 르무통 메이트의 메인 컬러를 셰퍼드가 직접 결정합니다." }

                    for (opt, pct, n, selected) in [("샌드 베이지", 52, 651, true), ("오크 브라운", 31, 388, false), ("올리브 그린", 17, 212, false)] {
                        {
                            let border = if selected { "border: 1.5px solid #00dfc0;" } else { "border: 1px solid rgba(255,255,255,0.06);" };
                            let color = if selected { "#00dfc0" } else { "#cbd5e1" };
                            rsx! {
                                div { style: "padding: 10px 12px; background: rgba(0,0,0,0.3); {border} border-radius: 10px; margin-bottom: 6px;",
                                    div { style: "display: flex; justify-content: space-between; font-size: 12px; margin-bottom: 6px;",
                                        span { style: "color: {color}; font-weight: 700;", "{opt}" }
                                        span { style: "color: #94a3b8;", "{pct}% ({n})" }
                                    }
                                    div { style: "height: 4px; background: rgba(255,255,255,0.05); border-radius: 2px; overflow: hidden;",
                                        div { style: "height: 100%; width: {pct}%; background: #00dfc0; border-radius: 2px;" }
                                    }
                                }
                            }
                        }
                    }
                    p { style: "font-size: 10px; color: #64748b; margin-top: 8px;", "2025-04-10 마감 \u{00B7} 총 1,251명 참여" }
                }

                // Past vote
                div { style: "padding: 14px; background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.05); border-radius: 14px; opacity: 0.7;",
                    p { style: "font-size: 10px; color: #64748b; letter-spacing: 0.2em; margin-bottom: 6px;", "\u{2713} 종료" }
                    p { style: "font-size: 13px; font-weight: 600; color: #cbd5e1; margin-bottom: 4px;", "친환경 패키지 도입 여부" }
                    p { style: "font-size: 11px; color: #00dfc0;", "찬성 87% \u{00B7} 가결" }
                }
            }
        }
    }
}

/// Activity verification — walking (Biyard provides)
#[component]
pub(super) fn BiyardActivity() -> Element {
    rsx! {
        div { style: "height: 100%; display: flex; flex-direction: column; background: #020408; position: relative;",
            div { class: "biyard-badge", style: "top: 14px; right: 14px; position: absolute; z-index: 20;", "\u{2B21} BIYARD" }

            div { style: "padding: 20px 20px 16px;",
                p { style: "font-size: 10px; color: #00dfc0; font-weight: 900; letter-spacing: 0.3em;", "ACTIVITY VERIFICATION" }
                p { style: "font-size: 22px; font-weight: 900; color: #fff; margin-top: 4px;", "걷기 인증" }
                p { style: "font-size: 12px; color: #94a3b8; margin-top: 6px;", "신발을 신고 걸을수록 팬덤 지분이 자랍니다" }
            }

            div { style: "flex: 1; padding: 16px; display: flex; flex-direction: column; gap: 14px;",
                // Today
                div { style: "padding: 24px; background: linear-gradient(135deg, rgba(0,223,192,0.1), rgba(0,223,192,0.02)); border: 1px solid rgba(0,223,192,0.25); border-radius: 18px; text-align: center;",
                    p { style: "font-size: 10px; color: #94a3b8; letter-spacing: 0.2em;", "오늘" }
                    p { style: "font-size: 42px; font-weight: 900; color: #00dfc0; text-shadow: 0 0 20px rgba(0,223,192,0.4); margin: 4px 0;", "8,412보" }
                    p { style: "font-size: 12px; color: #94a3b8;", "목표 10,000보까지 1,588보 남음" }
                    div { style: "margin-top: 14px; height: 8px; background: rgba(255,255,255,0.05); border-radius: 4px; overflow: hidden;",
                        div { style: "height: 100%; width: 84%; background: linear-gradient(90deg, #00dfc0, #60a5fa); border-radius: 4px;" }
                    }
                }

                // Reward preview
                div { style: "padding: 16px; background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.06); border-radius: 14px;",
                    div { style: "display: flex; justify-content: space-between; align-items: center;",
                        div {
                            p { style: "font-size: 11px; color: #64748b;", "10,000보 달성 시" }
                            p { style: "font-size: 20px; font-weight: 900; color: #00dfc0; margin-top: 4px;", "+ 80 팬덤 지분" }
                        }
                        span { style: "font-size: 32px;", "\u{1F45F}" }
                    }
                }

                // Weekly
                div { style: "padding: 16px; background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.06); border-radius: 14px;",
                    p { style: "font-size: 11px; color: #64748b; letter-spacing: 0.1em; margin-bottom: 10px;", "이번 주" }
                    div { style: "display: flex; gap: 4px; align-items: flex-end; height: 60px;",
                        for (day, h) in [("월",55_i32), ("화",78), ("수",90), ("목",45), ("금",88), ("토",95), ("일",62)] {
                            div { style: "flex: 1; display: flex; flex-direction: column; align-items: center; gap: 4px;",
                                div { style: "width: 100%; height: {h}%; background: linear-gradient(180deg, #00dfc0, rgba(0,223,192,0.3)); border-radius: 3px;" }
                                p { style: "font-size: 9px; color: #64748b;", "{day}" }
                            }
                        }
                    }
                    p { style: "margin-top: 10px; font-size: 11px; color: #00dfc0; text-align: center;", "주간 누적 +420 팬덤 지분" }
                }
            }
        }
    }
}
