use dioxus::prelude::*;

use crate::features::catalog::views::Topbar;

#[component]
pub fn PricingView() -> Element {
    rsx! {
        Topbar { active: "pricing".to_string() }
        main { class: "max-w-7xl mx-auto px-6 py-8",
            section { class: "text-center mb-10",
                h1 { class: "text-3xl md:text-4xl font-bold mb-3", "두 트랙으로 운영합니다" }
                p { class: "text-foreground-muted text-sm md:text-base max-w-2xl mx-auto",
                    "B2C 는 무료로 진입장벽을 없애고, B2B 는 발행사·신탁사·증권사·투자자문사 대상 라이선스 형태로 데이터·기능을 제공합니다."
                }
            }

            section { class: "grid grid-cols-1 md:grid-cols-2 gap-4 mb-12",
                Track {
                    badge: "B2C",
                    badge_class: "bg-info-soft text-info",
                    title: "개인 투자자 — 무료",
                    desc: "카탈로그·상세·청산 결과 비교 등 데이터 수집·가공 결과를 그대로 공개합니다. 진입장벽을 없애 트래픽을 확보하고, 데이터 신뢰도·플랫폼 레퍼런스를 쌓는 채널.",
                    features: vec![
                        "전체 STO 카탈로그·검색·필터",
                        "발행 구조·청산 결과 상세",
                        "공시·외부 링크",
                        "Biyard Index 등급 조회 (정식 출시 후)",
                    ],
                }
                Track {
                    badge: "B2B",
                    badge_class: "bg-brand-soft text-brand",
                    title: "발행사·증권사 — 라이선스",
                    desc: "Biyard Launchpad 발행 토큰 정보, 유저 관심도·트래픽 기반 데이터, 발행 전 기초자산 수요 파악 기능을 묶음 라이선스로 제공.",
                    features: vec![
                        "관심도·트래픽 기반 수요 파악 데이터",
                        "Launchpad 발행 토큰 운영 API",
                        "Biyard Index 평가 신청 우선권",
                        "발행사 노출 슬롯 (메인 히어로)",
                    ],
                }
            }

            section { class: "bg-panel border border-dashed border-border rounded-2xl p-6 text-center",
                p { class: "text-xs text-foreground-muted",
                    "차별화 메시지: "
                    span { class: "text-brand font-semibold", "\"등급·매수추천 X, 정량 비교·유사군 매칭 O\"" }
                    " — 신용평가업·투자자문업 인가 영역은 피하면서 의사결정에 도움이 되는 정보만 제공합니다."
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
