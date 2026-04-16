use dioxus::prelude::*;

/// Tech integration section — shows what Biyard must provide per tech stack
/// since every company's app uses different tech
#[component]
pub(super) fn TechSection() -> Element {
    rsx! {
        section {
            class: "demo-section",
            style: "background: rgba(0,0,0,0.3);",
            div {
                class: "demo-container",

                div { style: "text-align: center; margin-bottom: 48px;",
                    p { style: "color: #00dfc0; font-size: 11px; font-weight: 900; letter-spacing: 0.4em; text-transform: uppercase;", "Technical Integration" }
                    h2 { class: "fandom-hero-text", style: "margin-top: 16px;",
                        "기업마다 "
                        span { class: "gradient-text-mint", "다른 기술 스택" }
                        "."
                        br {}
                        "그래서 Biyard가 제공해야 할 것."
                    }
                    p { style: "color: #94a3b8; font-size: 15px; line-height: 1.7; margin: 24px auto 0; max-width: 760px;",
                        "\"API 하나로 연동\"은 마케팅 문구. 실제로는 기업의 기존 앱 스택에 따라 "
                        span { style: "color: #e2e8f0; font-weight: 700;", "SDK · 플러그인 · 통합 지원" }
                        "을 패키지로 제공해야 합니다."
                    }
                }

                // Layer 1 — Language SDKs
                div { style: "margin-bottom: 40px;",
                    p { style: "font-size: 11px; color: #00dfc0; font-weight: 900; letter-spacing: 0.3em; margin-bottom: 8px;", "LAYER 1 \u{00B7} LANGUAGE-SPECIFIC SDK" }
                    h3 { style: "font-size: 22px; font-weight: 900; color: #fff; margin-bottom: 20px;", "기업 개발팀이 자기 스택에서 바로 쓰는 라이브러리" }
                    div { style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(240px, 1fr)); gap: 14px;",
                        for (name, stack, pkg, priority) in [
                            ("@biyard/js", "Node.js · Next.js 백엔드", "npm install", "P0"),
                            ("@biyard/react", "웹 프론트엔드 위젯", "npm install", "P0"),
                            ("biyard-android", "Kotlin · Java 네이티브", "Maven / Gradle", "P0"),
                            ("biyard-ios", "Swift 네이티브", "SPM / CocoaPods", "P0"),
                            ("@biyard/react-native", "크로스플랫폼 (르무통)", "npm install", "P0"),
                            ("biyard-python", "Python · Django · Flask", "pip install", "P1"),
                            ("biyard-php", "PHP · Laravel (구형 커머스)", "composer", "P1"),
                            ("biyard-go", "Go 백엔드", "go get", "P2"),
                        ] {
                            div { style: "padding: 16px; background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.06); border-radius: 12px;",
                                div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;",
                                    p { style: "font-family: monospace; font-size: 13px; font-weight: 700; color: #00dfc0;", "{name}" }
                                    span { style: "padding: 2px 6px; background: rgba(0,223,192,0.1); border-radius: 4px; font-size: 9px; color: #00dfc0; font-weight: 900;", "{priority}" }
                                }
                                p { style: "font-size: 12px; color: #cbd5e1; line-height: 1.5;", "{stack}" }
                                p { style: "font-family: monospace; font-size: 10px; color: #64748b; margin-top: 6px;", "{pkg}" }
                            }
                        }
                    }
                }

                // Layer 2 — Platform plugins
                div { style: "margin-bottom: 40px;",
                    p { style: "font-size: 11px; color: #00dfc0; font-weight: 900; letter-spacing: 0.3em; margin-bottom: 8px;", "LAYER 2 \u{00B7} PLATFORM PLUGINS" }
                    h3 { style: "font-size: 22px; font-weight: 900; color: #fff; margin-bottom: 20px;", "원클릭 설치 플러그인 (소형 브랜드 80% 커버)" }
                    div { style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 14px;",
                        for (name, market) in [
                            ("Cafe24 앱", "한국 1위 이커머스 호스팅"),
                            ("NHN Commerce", "NHN 고도몰 / 샵바이"),
                            ("Imweb", "Imweb 빌더 공식 연동"),
                            ("MakeShop", "MakeShop 스마트러닝"),
                            ("Shopify 앱", "글로벌 진출 대비 필수"),
                            ("WooCommerce", "WordPress 생태계"),
                        ] {
                            div { style: "padding: 14px 16px; background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.06); border-radius: 10px; display: flex; align-items: center; gap: 10px;",
                                div { style: "width: 32px; height: 32px; background: rgba(0,223,192,0.1); border: 1px solid rgba(0,223,192,0.2); border-radius: 8px; display: flex; align-items: center; justify-content: center; font-size: 14px;", "\u{26A1}" }
                                div {
                                    p { style: "font-size: 13px; font-weight: 700; color: #e2e8f0;", "{name}" }
                                    p { style: "font-size: 10px; color: #64748b;", "{market}" }
                                }
                            }
                        }
                    }
                }

                // Layer 3 — Integration patterns
                div { style: "margin-bottom: 40px;",
                    p { style: "font-size: 11px; color: #00dfc0; font-weight: 900; letter-spacing: 0.3em; margin-bottom: 8px;", "LAYER 3 \u{00B7} INTEGRATION PATTERNS" }
                    h3 { style: "font-size: 22px; font-weight: 900; color: #fff; margin-bottom: 20px;", "기업 상황에 맞는 연동 방식" }
                    div { style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(260px, 1fr)); gap: 14px;",
                        for (pattern, when, time, highlight) in [
                            ("Webhook 기반", "자체 앱 보유 (르무통 케이스)", "3~5일", true),
                            ("Embedded Widget", "기존 사이트에 <script> 한 줄 삽입", "1일", false),
                            ("Full SDK 통합", "앱 로그인 연동, 지갑 임베드", "1~2주", false),
                            ("API Polling", "레거시 시스템, webhook 불가능", "1~3일", false),
                        ] {
                            {
                                let (bg, border, tag_color) = if highlight {
                                    ("rgba(0,223,192,0.06)", "rgba(0,223,192,0.3)", "#00dfc0")
                                } else {
                                    ("rgba(255,255,255,0.03)", "rgba(255,255,255,0.08)", "#94a3b8")
                                };
                                rsx! {
                                    div { style: "padding: 18px; background: {bg}; border: 1px solid {border}; border-radius: 14px;",
                                        div { style: "display: flex; justify-content: space-between; margin-bottom: 8px;",
                                            p { style: "font-size: 14px; font-weight: 900; color: {tag_color};", "{pattern}" }
                                            if highlight {
                                                span { style: "font-size: 9px; padding: 2px 6px; background: {tag_color}; color: #020408; border-radius: 4px; font-weight: 900;", "LE MOUTON" }
                                            }
                                        }
                                        p { style: "font-size: 12px; color: #cbd5e1; line-height: 1.6; margin-bottom: 8px;", "{when}" }
                                        p { style: "font-size: 11px; color: #64748b;", "예상 연동 시간: " span { style: "color: {tag_color}; font-weight: 700;", "{time}" } }
                                    }
                                }
                            }
                        }
                    }
                }

                // Layer 4 — Support tier = revenue model
                div { style: "margin-bottom: 40px;",
                    p { style: "font-size: 11px; color: #00dfc0; font-weight: 900; letter-spacing: 0.3em; margin-bottom: 8px;", "LAYER 4 \u{00B7} SUPPORT TIER (= REVENUE)" }
                    h3 { style: "font-size: 22px; font-weight: 900; color: #fff; margin-bottom: 20px;", "통합 지원 티어 = Biyard의 실질 수익 구조" }
                    div { style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(260px, 1fr)); gap: 14px;",
                        for (tier, price, what, target) in [
                            ("Self-Serve", "Free / Starter", "SDK + 문서 + 커뮤니티", "소형 브랜드, 기술팀 있음"),
                            ("Assisted", "Business ($$/월)", "전담 엔지니어 월 20시간 + 슬랙 채널 + 코드 리뷰", "중형 브랜드 (르무통 같은)"),
                            ("White-glove", "Enterprise (SOW)", "1~3개월 커스텀 프로젝트 + 전담 PM + 엔지니어 2명", "대기업, 복잡한 레거시"),
                        ] {
                            div { style: "padding: 20px; background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.06); border-radius: 14px;",
                                p { style: "font-size: 11px; color: #00dfc0; font-weight: 900; letter-spacing: 0.2em; margin-bottom: 6px;", "{tier}" }
                                p { style: "font-size: 14px; font-weight: 700; color: #e2e8f0; margin-bottom: 12px;", "{price}" }
                                p { style: "font-size: 12px; color: #cbd5e1; line-height: 1.7; margin-bottom: 10px;", "{what}" }
                                p { style: "font-size: 10px; color: #64748b; padding-top: 10px; border-top: 1px solid rgba(255,255,255,0.05);", "\u{1F3AF} {target}" }
                            }
                        }
                    }
                }

                // Le Mouton specific integration playbook
                div { style: "padding: 32px; background: linear-gradient(135deg, rgba(212,197,176,0.05), rgba(0,223,192,0.04)); border: 1px solid rgba(212,197,176,0.2); border-radius: 20px;",
                    p { style: "color: #D4C5B0; font-size: 11px; font-weight: 900; letter-spacing: 0.3em; margin-bottom: 12px;", "\u{1F411} LE MOUTON PLAYBOOK" }
                    h3 { style: "font-size: 22px; font-weight: 900; color: #fff; margin-bottom: 16px;", "르무통 케이스에 맞춘 구체 연동 계획" }

                    p { style: "font-size: 13px; color: #cbd5e1; line-height: 1.8; margin-bottom: 14px;",
                        "1. " span { style: "color: #00dfc0; font-weight: 700;", "기술 스택 파악" } " \u{2014} 르무통 앱이 React Native / Flutter / 네이티브 중 무엇인지 확인 (대표 미팅 시 체크)"
                    }
                    p { style: "font-size: 13px; color: #cbd5e1; line-height: 1.8; margin-bottom: 14px;",
                        "2. " span { style: "color: #00dfc0; font-weight: 700;", "SDK 선정" } " \u{2014} 해당 스택용 SDK (@biyard/react-native 추정) 제공. 개발팀이 npm install 로 즉시 설치"
                    }
                    p { style: "font-size: 13px; color: #cbd5e1; line-height: 1.8; margin-bottom: 14px;",
                        "3. " span { style: "color: #00dfc0; font-weight: 700;", "Webhook 연결" } " \u{2014} 결제 완료 이벤트에 Biyard.recordPurchase() 훅 추가 (코드 2~3줄)"
                    }
                    p { style: "font-size: 13px; color: #cbd5e1; line-height: 1.8; margin-bottom: 14px;",
                        "4. " span { style: "color: #00dfc0; font-weight: 700;", "UI 임베드" } " \u{2014} 셰퍼드 대시보드, 팬덤 피드, DAO 화면은 Biyard Pre-built 컴포넌트로 제공 \u{2192} 르무통 마이페이지에 iframe 또는 컴포넌트로 삽입"
                    }
                    p { style: "font-size: 13px; color: #cbd5e1; line-height: 1.8; margin-bottom: 14px;",
                        "5. " span { style: "color: #00dfc0; font-weight: 700;", "적립금 마이그레이션" } " \u{2014} 기존 적립금 보유 고객 대상 옵트인 전환 이벤트 (포인트 \u{2192} 팬덤 지분, +20% 보너스)"
                    }
                    p { style: "font-size: 13px; color: #cbd5e1; line-height: 1.8;",
                        "6. " span { style: "color: #00dfc0; font-weight: 700;", "런치패드 파트너십" } " \u{2014} 르무통은 첫 레퍼런스 브랜드. 통합 비용 할인 / 공동 마케팅 / 케이스 스터디 공개 조건"
                    }

                    div { style: "margin-top: 20px; padding: 16px; background: rgba(0,223,192,0.05); border: 1px solid rgba(0,223,192,0.2); border-radius: 10px;",
                        p { style: "font-size: 11px; color: #00dfc0; font-weight: 900; letter-spacing: 0.2em; margin-bottom: 8px;", "\u{23F1} ESTIMATED TIMELINE" }
                        p { style: "font-size: 12px; color: #e2e8f0; line-height: 1.7;",
                            "르무통 개발자 1명 풀타임 + Biyard 엔지니어 페어 프로그래밍 \u{2192} "
                            span { style: "color: #00dfc0; font-weight: 900;", "1~2주" }
                            " 내 MVP 출시 가능"
                        }
                    }
                }
            }
        }
    }
}
