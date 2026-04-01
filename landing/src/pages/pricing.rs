use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Pricing() -> Element {
    rsx! {
        div {
            style: "min-height: 100vh; background: #0c1018; color: #e8eefc; font-family: 'Outfit', 'Noto Sans KR', sans-serif;",

            // Header
            div {
                style: "background: rgba(12,16,24,0.85); backdrop-filter: blur(16px); border-bottom: 1px solid rgba(0,212,170,0.08); padding: 14px 24px;",
                div {
                    class: "max-w-6xl mx-auto flex items-center justify-between",
                    Link {
                        to: Route::Home {},
                        class: "flex items-center gap-2",
                        img {
                            src: asset!("/assets/biyard-logo.png"),
                            alt: "Biyard",
                            style: "width: 28px; height: 28px;",
                        }
                        span {
                            class: "text-xl font-extrabold",
                            style: "color: #00d4aa;",
                            "Biyard"
                        }
                    }
                    Link {
                        to: Route::Home {},
                        class: "text-sm",
                        style: "color: #7a8ba6;",
                        "\u{2190} Back"
                    }
                }
            }

            // Content
            div {
                class: "max-w-5xl mx-auto px-4 py-20",

                // Header
                div {
                    class: "text-center mb-16",
                    p {
                        class: "text-sm font-semibold tracking-widest uppercase mb-3",
                        style: "color: #00d4aa;",
                        "PRICING"
                    }
                    h1 {
                        class: "text-4xl md:text-5xl mb-4",
                        style: "font-weight: 300; letter-spacing: -0.02em;",
                        "우리 브랜드에 맞는 "
                        span {
                            style: "background-image: linear-gradient(to right, #60a5fa, #a78bfa); -webkit-background-clip: text; background-clip: text; color: transparent; font-weight: 600;",
                            "요금제를 선택하세요"
                        }
                    }
                    p {
                        class: "text-lg max-w-xl mx-auto",
                        style: "color: #7a8ba6;",
                        "블록체인 지식 없이도 시작할 수 있습니다. 규모에 맞게 성장하세요."
                    }
                }

                // Pricing cards
                div {
                    class: "grid grid-cols-1 md:grid-cols-3 gap-6 mb-16",

                    // Starter
                    div {
                        class: "rounded-2xl p-8 relative overflow-hidden",
                        style: "background: rgba(10,16,26,0.5); backdrop-filter: blur(16px); border: 1px solid rgba(255,255,255,0.06);",
                        div {
                            class: "absolute top-0 left-[10%] right-[10%] h-[1px]",
                            style: "background: linear-gradient(90deg, transparent, rgba(96,165,250,0.3), transparent);",
                        }
                        p {
                            class: "text-sm font-bold tracking-widest mb-2",
                            style: "color: #60a5fa;",
                            "STARTER"
                        }
                        div {
                            class: "mb-6",
                            span {
                                class: "text-4xl font-extrabold",
                                style: "color: #e8eefc;",
                                "무료"
                            }
                        }
                        p {
                            class: "text-sm mb-8",
                            style: "color: #7a8ba6;",
                            "토큰 이코노미를 처음 시작하는 브랜드를 위한 플랜"
                        }
                        div {
                            class: "space-y-3 mb-8",
                            PricingItem { text: "월 1,000건 거래", included: true }
                            PricingItem { text: "토큰 보유자 500명", included: true }
                            PricingItem { text: "거래 수수료 3%", included: true }
                            PricingItem { text: "기본 대시보드", included: true }
                            PricingItem { text: "챌린지 템플릿 1개", included: true }
                            PricingItem { text: "이메일 지원", included: true }
                            PricingItem { text: "고급 분석", included: false }
                            PricingItem { text: "전담 담당자", included: false }
                        }
                        a {
                            href: "#",
                            class: "block w-full text-center py-3 rounded-xl text-sm font-bold",
                            style: "border: 1px solid rgba(96,165,250,0.3); color: #60a5fa;",
                            "무료로 시작하기"
                        }
                    }

                    // Business (recommended)
                    div {
                        class: "rounded-2xl p-8 relative overflow-hidden",
                        style: "background: rgba(10,16,26,0.5); backdrop-filter: blur(16px); border: 1px solid rgba(0,212,170,0.25); box-shadow: 0 0 30px rgba(0,212,170,0.08);",
                        div {
                            class: "absolute top-0 left-[5%] right-[5%] h-[2px]",
                            style: "background: linear-gradient(90deg, transparent, #00d4aa, transparent);",
                        }
                        // Recommended badge
                        div {
                            class: "absolute top-4 right-4",
                            span {
                                class: "text-xs font-bold px-3 py-1 rounded-full",
                                style: "background: rgba(0,212,170,0.15); color: #00d4aa;",
                                "추천"
                            }
                        }
                        p {
                            class: "text-sm font-bold tracking-widest mb-2",
                            style: "color: #00d4aa;",
                            "BUSINESS"
                        }
                        div {
                            class: "mb-1",
                            span {
                                class: "text-4xl font-extrabold",
                                style: "color: #e8eefc;",
                                "49만원"
                            }
                            span {
                                class: "text-sm ml-1",
                                style: "color: #7a8ba6;",
                                "/월"
                            }
                        }
                        p {
                            class: "text-xs mb-6",
                            style: "color: #4a5568;",
                            "연간 결제 시 월 39만원"
                        }
                        p {
                            class: "text-sm mb-8",
                            style: "color: #7a8ba6;",
                            "본격적으로 토큰 이코노미를 운영하는 성장 브랜드"
                        }
                        div {
                            class: "space-y-3 mb-8",
                            PricingItem { text: "월 100,000건 거래", included: true }
                            PricingItem { text: "토큰 보유자 50,000명", included: true }
                            PricingItem { text: "거래 수수료 1.5%", included: true }
                            PricingItem { text: "고급 분석 대시보드", included: true }
                            PricingItem { text: "챌린지 템플릿 무제한", included: true }
                            PricingItem { text: "전담 담당자 배정", included: true }
                            PricingItem { text: "API 우선 지원", included: true }
                            PricingItem { text: "맞춤 제작", included: false }
                        }
                        a {
                            href: "#",
                            class: "block w-full text-center py-3 rounded-xl text-sm font-bold",
                            style: "background: #00d4aa; color: #0c1018;",
                            "시작하기"
                        }
                    }

                    // Enterprise
                    div {
                        class: "rounded-2xl p-8 relative overflow-hidden",
                        style: "background: rgba(10,16,26,0.5); backdrop-filter: blur(16px); border: 1px solid rgba(255,255,255,0.06);",
                        div {
                            class: "absolute top-0 left-[10%] right-[10%] h-[1px]",
                            style: "background: linear-gradient(90deg, transparent, rgba(167,139,250,0.3), transparent);",
                        }
                        p {
                            class: "text-sm font-bold tracking-widest mb-2",
                            style: "color: #a78bfa;",
                            "ENTERPRISE"
                        }
                        div {
                            class: "mb-6",
                            span {
                                class: "text-4xl font-extrabold",
                                style: "color: #e8eefc;",
                                "별도 협의"
                            }
                        }
                        p {
                            class: "text-sm mb-8",
                            style: "color: #7a8ba6;",
                            "대규모 브랜드를 위한 맞춤형 솔루션"
                        }
                        div {
                            class: "space-y-3 mb-8",
                            PricingItem { text: "거래 건수 무제한", included: true }
                            PricingItem { text: "토큰 보유자 무제한", included: true }
                            PricingItem { text: "거래 수수료 협의", included: true }
                            PricingItem { text: "맞춤 대시보드 제작", included: true }
                            PricingItem { text: "챌린지 템플릿 무제한", included: true }
                            PricingItem { text: "전담 팀 배정", included: true }
                            PricingItem { text: "SLA 99.99% 보장", included: true }
                            PricingItem { text: "화이트라벨 지원", included: true }
                        }
                        a {
                            href: "mailto:finance@biyard.co",
                            class: "block w-full text-center py-3 rounded-xl text-sm font-bold",
                            style: "border: 1px solid rgba(167,139,250,0.3); color: #a78bfa;",
                            "문의하기"
                        }
                    }
                }

                // Bottom note
                div {
                    class: "text-center",
                    style: "color: #4a5568;",
                    p {
                        class: "text-sm",
                        "모든 요금제는 부가세 별도입니다. 연간 결제 시 20% 할인이 적용됩니다."
                    }
                    p {
                        class: "text-sm mt-1",
                        "궁금한 점이 있으시면 "
                        a {
                            href: "mailto:finance@biyard.co",
                            style: "color: #00d4aa;",
                            "finance@biyard.co"
                        }
                        "로 문의해주세요."
                    }
                }
            }
        }
    }
}

#[component]
fn PricingItem(text: &'static str, included: bool) -> Element {
    rsx! {
        div {
            class: "flex items-center gap-2",
            if included {
                span { style: "color: #00d4aa; font-size: 14px;", "\u{2713}" }
            } else {
                span { style: "color: #4a5568; font-size: 14px;", "\u{2717}" }
            }
            span {
                class: "text-sm",
                style: if included { "color: #e8eefc;" } else { "color: #4a5568;" },
                "{text}"
            }
        }
    }
}
