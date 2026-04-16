use dioxus::prelude::*;

/// Revenue model — THE MOST IMPORTANT SECTION
/// Shows clearly: What Biyard provides, and HOW we monetize each part.
#[component]
pub(super) fn RevenueSection() -> Element {
    rsx! {
        section {
            class: "demo-section",
            style: "background: linear-gradient(180deg, rgba(0,223,192,0.06), rgba(2,4,8,0) 40%); border-top: 2px solid rgba(0,223,192,0.3);",
            div {
                class: "demo-container",

                // Spotlight header
                div { style: "text-align: center; margin-bottom: 48px;",
                    p { style: "color: #00dfc0; font-size: 11px; font-weight: 900; letter-spacing: 0.4em; text-transform: uppercase;", "Revenue Model \u{00B7} THE ANSWER" }
                    h2 { style: "font-size: clamp(2rem, 5vw, 3.6rem); font-weight: 900; line-height: 1.1; margin-top: 16px; letter-spacing: -0.03em;",
                        "Biyard가 "
                        span { class: "gradient-text-mint", "어디서 돈을 버는가?" }
                    }
                    p { style: "color: #94a3b8; font-size: 16px; line-height: 1.7; margin: 20px auto 0; max-width: 720px;",
                        "앞의 데모에서 본 각 기능이 "
                        span { style: "color: #00dfc0; font-weight: 700;", "Biyard의 수익 지점" }
                        "입니다. 르무통을 온보딩할 때 "
                        span { style: "color: #fff; font-weight: 700;", "어떤 부분에 얼마를 청구" }
                        "하는지 한눈에."
                    }
                }

                // Revenue matrix — the core table
                div { style: "overflow-x: auto; margin-bottom: 48px;",
                    div { style: "min-width: 720px; background: rgba(0,0,0,0.4); border: 1px solid rgba(0,223,192,0.2); border-radius: 20px; overflow: hidden;",

                        // Table header
                        div { style: "display: grid; grid-template-columns: 2fr 1.2fr 1.5fr 1fr; gap: 0; background: rgba(0,223,192,0.08); padding: 16px 20px; border-bottom: 1px solid rgba(0,223,192,0.2); font-size: 10px; font-weight: 900; letter-spacing: 0.2em; color: #00dfc0; text-transform: uppercase;",
                            div { "Biyard 제공 파트" }
                            div { "수익화 방식" }
                            div { "요금 모델" }
                            div { style: "text-align: right;", "우선순위" }
                        }

                        // Rows
                        for (part, emoji, how, pricing, priority, hot) in [
                            (
                                "SDK + API 인프라",
                                "\u{1F527}",
                                "월 구독료",
                                "Starter: 무료 (월 1k API)\nBusiness: $399/월\nEnterprise: SOW",
                                "P0",
                                true,
                            ),
                            (
                                "토큰 발행 · 초기 설정",
                                "\u{1F680}",
                                "일회성 런칭 수수료",
                                "Launch Fee: $5,000\n(토큰 계약 배포 + 초기 설정)",
                                "P0",
                                true,
                            ),
                            (
                                "결제 webhook · 트레저리 적립",
                                "\u{1F4B8}",
                                "거래 수수료",
                                "거래당 0.5 ~ 1%\n(매출 규모에 따라 할인)",
                                "P0",
                                true,
                            ),
                            (
                                "Treasury 운용 · 검증",
                                "\u{1F3E6}",
                                "AUM 수수료",
                                "연 0.3 ~ 0.5% of AUM\n(자산 관리 운용료)",
                                "P0",
                                true,
                            ),
                            (
                                "셰퍼드 대시보드 · 지갑 UI",
                                "\u{1F4F1}",
                                "구독 포함",
                                "Business 플랜에 포함\n커스터마이징 옵션 별도",
                                "P0",
                                false,
                            ),
                            (
                                "팬덤 피드 · 커뮤니티 모듈",
                                "\u{1F465}",
                                "프리미엄 애드온",
                                "+$149/월\n(소셜 기능, 모더레이션 포함)",
                                "P1",
                                false,
                            ),
                            (
                                "DAO · 투표 모듈",
                                "\u{1F5F3}",
                                "프리미엄 애드온",
                                "+$199/월\n(투표 스마트컨트랙트 운영)",
                                "P1",
                                false,
                            ),
                            (
                                "활동 인증 · 리워드 룰 엔진",
                                "\u{1F3AF}",
                                "프리미엄 애드온",
                                "+$99/월\n(걷기, 체크인 등 룰 설정 UI)",
                                "P1",
                                false,
                            ),
                            (
                                "팬덤 애널리틱스 대시보드",
                                "\u{1F4CA}",
                                "프리미엄 애드온",
                                "+$249/월\n(LTV, 이탈률, 팬 세그먼트 분석)",
                                "P1",
                                false,
                            ),
                            (
                                "통합 지원 · 엔지니어 시간",
                                "\u{1F6E0}\u{FE0F}",
                                "시간당 또는 패키지",
                                "$200/hr\n또는 월 20hr 패키지 $3K",
                                "P0",
                                true,
                            ),
                            (
                                "커스텀 통합 · White-glove",
                                "\u{1F3D7}\u{FE0F}",
                                "SOW 프로젝트",
                                "$50K ~ $200K\n(1~3개월, 대기업용)",
                                "P2",
                                false,
                            ),
                            (
                                "포인트 \u{2192} 토큰 마이그레이션",
                                "\u{1F504}",
                                "일회성 서비스 피",
                                "$3,000 ~ $10,000\n(포인트 규모에 따라)",
                                "P1",
                                false,
                            ),
                        ] {
                            {
                                let (bg, left_accent) = if hot {
                                    ("rgba(0,223,192,0.04)", "border-left: 3px solid #00dfc0;")
                                } else {
                                    ("rgba(255,255,255,0.02)", "border-left: 3px solid rgba(255,255,255,0.08);")
                                };
                                rsx! {
                                    div { style: "display: grid; grid-template-columns: 2fr 1.2fr 1.5fr 1fr; gap: 0; padding: 16px 20px; background: {bg}; {left_accent} border-bottom: 1px solid rgba(255,255,255,0.04); align-items: start;",
                                        div { style: "display: flex; gap: 10px; align-items: start;",
                                            span { style: "font-size: 20px;", "{emoji}" }
                                            div {
                                                p { style: "font-size: 13px; font-weight: 700; color: #fff;", "{part}" }
                                                if hot {
                                                    p { style: "font-size: 9px; color: #00dfc0; font-weight: 900; letter-spacing: 0.2em; margin-top: 4px;", "\u{25CF} CORE REVENUE" }
                                                }
                                            }
                                        }
                                        div { style: "font-size: 12px; color: #cbd5e1; font-weight: 600;", "{how}" }
                                        div { style: "font-size: 11px; color: #94a3b8; line-height: 1.6; white-space: pre-line; font-family: monospace;", "{pricing}" }
                                        div { style: "text-align: right;",
                                            span { style: "font-size: 10px; font-weight: 900; padding: 3px 8px; background: rgba(0,223,192,0.1); border: 1px solid rgba(0,223,192,0.2); color: #00dfc0; border-radius: 4px;", "{priority}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Le Mouton revenue simulation
                div { style: "padding: 32px; background: linear-gradient(135deg, rgba(212,197,176,0.05), rgba(0,223,192,0.08)); border: 1px solid rgba(0,223,192,0.25); border-radius: 20px;",
                    p { style: "color: #D4C5B0; font-size: 11px; font-weight: 900; letter-spacing: 0.3em; margin-bottom: 10px;", "\u{1F411} LE MOUTON \u{00B7} 수익 시뮬레이션" }
                    h3 { style: "font-size: 22px; font-weight: 900; color: #fff; margin-bottom: 16px;", "르무통 1년차 Biyard 매출 추정" }

                    div { style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 12px; margin-bottom: 20px;",
                        for (label, val, note) in [
                            ("런칭 수수료 (1회)", "$5,000", "토큰 발행 + 초기 설정"),
                            ("월 구독료 (Business)", "$399 × 12", "기본 플랜"),
                            ("월 애드온 (DAO + 피드)", "$348 × 12", "+DAO $199 +피드 $149"),
                            ("거래 수수료", "≈ $12,000", "연 매출 20억 × 0.6%"),
                            ("통합 지원 패키지", "$3K × 3", "초반 3개월 집중 지원"),
                            ("AUM 운용료", "≈ $500", "트레저리 $200k × 0.3%"),
                        ] {
                            div { style: "padding: 14px; background: rgba(2,4,8,0.5); border: 1px solid rgba(0,223,192,0.12); border-radius: 10px;",
                                p { style: "font-size: 10px; color: #64748b; letter-spacing: 0.1em;", "{label}" }
                                p { style: "font-size: 18px; font-weight: 900; color: #00dfc0; margin-top: 4px;", "{val}" }
                                p { style: "font-size: 9px; color: #64748b; margin-top: 4px;", "{note}" }
                            }
                        }
                    }

                    div { style: "padding: 20px; background: #00dfc0; border-radius: 12px; text-align: center;",
                        p { style: "font-size: 11px; color: #020408; font-weight: 900; letter-spacing: 0.2em; opacity: 0.7;", "예상 연 매출 (르무통 1개 브랜드)" }
                        p { style: "font-size: 36px; font-weight: 900; color: #020408; margin-top: 6px;", "\u{2248} $35,000 / year" }
                        p { style: "font-size: 11px; color: #020408; opacity: 0.7; margin-top: 4px;", "약 5천만원 / 1개 브랜드 기준 \u{00B7} 50개 브랜드 = 연 25억 ARR" }
                    }
                }
            }
        }
    }
}
