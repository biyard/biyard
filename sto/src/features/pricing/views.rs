use dioxus::prelude::*;

use crate::features::catalog::views::Topbar;

#[component]
pub fn PricingView() -> Element {
    rsx! {
        Topbar { active: "pricing".to_string() }
        main { class: "max-w-7xl mx-auto px-6 py-8",
            section { class: "text-center mb-10",
                h1 { class: "text-3xl md:text-4xl font-bold mb-3", "개인은 무료, 기업은 라이선스" }
                p { class: "text-foreground-muted text-sm md:text-base max-w-2xl mx-auto",
                    "투자자에게는 공시 기반 정보를 무료로 공개하고, 발행사·신탁사·증권사 등 기관 고객에게는 라이선스 형태로 데이터와 기능을 제공합니다."
                }
            }

            section { class: "grid grid-cols-1 md:grid-cols-2 gap-4 mb-12",
                Track {
                    badge: "개인",
                    badge_class: "bg-info-soft text-info",
                    title: "개인 투자자 · 무료",
                    desc: "공모 자산을 찾고 비교하는 데 필요한 정보는 모두 무료로 공개합니다. 별도 결제나 가입 없이 누구나 사용할 수 있습니다.",
                    features: vec![
                        "공모 자산 검색·카테고리·발행사별 필터",
                        "발행 구조·공시 자료 상세 보기",
                        "외부 원문 공시·발행사 페이지 바로가기",
                        "Biyard 신뢰지수 조회 (정식 출시 후)",
                    ],
                }
                Track {
                    badge: "기관",
                    badge_class: "bg-brand-soft text-brand",
                    title: "기관 라이선스 · 문의",
                    desc: "발행사·증권사·운용사를 위한 데이터·도구 묶음입니다. 발행 전 시장 반응을 미리 보고, 자체 채널에 정보를 임베드할 수 있습니다.",
                    features: vec![
                        "투자자 관심도·검색 트렌드 데이터",
                        "Biyard Launchpad 브랜드 토큰 API",
                        "Biyard 신뢰지수 평가 우선 적용",
                        "메인 화면 공모 노출 슬롯 (광고)",
                    ],
                }
            }

            section { class: "bg-panel border border-dashed border-border rounded-2xl p-6 text-center",
                p { class: "text-xs text-foreground-muted",
                    "본 서비스는 "
                    span { class: "text-brand font-semibold", "공시 자료 기반의 정보 제공" }
                    " 만 하며, 자본시장법상 신용평가업·투자자문업이 아닙니다. 투자 결정은 본인의 책임입니다."
                }
            }
        }
    }
}

#[component]
fn Track(
    badge: &'static str,
    badge_class: &'static str,
    title: &'static str,
    desc: &'static str,
    features: Vec<&'static str>,
) -> Element {
    rsx! {
        article { class: "bg-panel border border-border rounded-2xl p-8",
            span { class: "inline-block text-[10px] font-bold tracking-widest px-2 py-1 rounded mb-3 {badge_class}", "{badge}" }
            h2 { class: "text-xl font-bold mb-2", "{title}" }
            p { class: "text-foreground-muted text-sm leading-relaxed mb-5", "{desc}" }
            ul { class: "space-y-2",
                for feat in features.iter() {
                    li { class: "flex gap-2 items-start text-sm text-foreground-soft",
                        span { class: "text-brand font-bold shrink-0", "✓" }
                        span { "{feat}" }
                    }
                }
            }
        }
    }
}
