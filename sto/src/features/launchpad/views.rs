use dioxus::prelude::*;

use crate::features::catalog::views::Topbar;

#[component]
pub fn LaunchpadView() -> Element {
    rsx! {
        Topbar { active: "launchpad".to_string() }
        main { class: "max-w-7xl mx-auto px-6 py-8",
            section { class: "bg-panel border border-brand rounded-2xl p-14 mb-6",
                div { class: "text-xs font-bold text-brand tracking-widest mb-4", "BIYARD LAUNCHPAD" }
                h1 { class: "text-3xl md:text-4xl font-bold leading-tight mb-4 max-w-2xl",
                    "발행 자산과 함께 운영하는"
                    br {}
                    "브랜드 토큰 인프라"
                }
                p { class: "text-sm md:text-base text-foreground-soft leading-relaxed max-w-2xl",
                    "Biyard Launchpad는 발행사·증권사가 자체 "
                    strong { class: "text-foreground", "유틸리티 토큰" }
                    "을 발행·운영할 수 있도록 돕는 클라우드 인프라입니다. 2027년 1월 토큰증권 제도 시행 이후 "
                    strong { class: "text-foreground", "조각투자와 브랜드 혜택을 한 채널" }
                    "에서 제공할 수 있습니다."
                }
            }

            section { class: "grid grid-cols-1 md:grid-cols-3 gap-3 mb-12",
                Feat { icon: "🪙", title: "브랜드 토큰 발행",
                    body: "코드 작성 없이 표준 토큰을 발행하고 운영할 수 있는 API를 제공합니다. 발행사 사이트에 임베드해 자체 브랜드로 사용할 수 있습니다." }
                Feat { icon: "🔗", title: "온체인 인프라 위탁",
                    body: "지갑 생성·송수신·이력 조회 등 블록체인 운영을 표준 API로 위탁할 수 있습니다. 발행사는 토큰 정책 설계에만 집중하면 됩니다." }
                Feat { icon: "📊", title: "발행 자산 연계",
                    body: "Biyard STO 정보 채널과 SSO·데이터를 연결합니다. 자사 발행물을 보유한 투자자에게 브랜드 토큰을 자동으로 지급하는 시나리오를 구현할 수 있습니다." }
            }

            section { class: "bg-panel border border-brand rounded-2xl p-10 mb-12",
                h2 { class: "text-xl font-bold mb-3", "발행 자산 × 브랜드 토큰 결합 예시" }
                p { class: "text-sm text-foreground-soft leading-relaxed mb-6",
                    "발행 자산은 "
                    strong { class: "text-foreground", "수익에 대한 권리" }
                    "이고, 브랜드 토큰은 "
                    strong { class: "text-foreground", "경험과 혜택" }
                    "입니다. 두 가치를 같은 화면에서 제공할 수 있도록 연결하는 것이 Launchpad의 역할입니다."
                }
                div { class: "grid grid-cols-1 md:grid-cols-[1fr_auto_1fr_auto_1fr] gap-3 items-center",
                    Step { name: "청약 완료", body: "투자자가 발행 자산 청약 — 수익권 확보" }
                    Arrow {}
                    Step { name: "토큰 자동 지급", body: "청약 인증 즉시 발행사 브랜드 토큰 지급" }
                    Arrow {}
                    Step { name: "경험·혜택 사용", body: "전시 우선 관람·할인·커뮤니티 참여 등" }
                }
            }

            section { class: "bg-panel border border-dashed border-brand rounded-2xl p-10 text-center",
                h2 { class: "text-base font-bold mb-2", "도입 문의" }
                p { class: "text-xs text-foreground-muted mb-4", "발행과 브랜드 토큰 운영을 함께 검토 중이신 발행사·증권사라면 편하게 연락 주세요." }
                a { href: "mailto:hi@biyard.co?subject=Launchpad",
                    class: "bg-brand text-brand-contrast font-bold px-5 py-2.5 rounded-md text-sm inline-block hover:bg-brand-strong",
                    "문의하기"
                }
            }
        }
    }
}

#[component]
fn Feat(icon: &'static str, title: &'static str, body: &'static str) -> Element {
    rsx! {
        article { class: "bg-panel border border-border rounded-2xl p-6",
            div { class: "text-2xl mb-2", "{icon}" }
            h3 { class: "text-base font-bold mb-2", "{title}" }
            p { class: "text-sm text-foreground-muted leading-relaxed", "{body}" }
        }
    }
}

#[component]
fn Step(name: &'static str, body: &'static str) -> Element {
    rsx! {
        div { class: "bg-panel-muted border border-border rounded-md p-3 text-center",
            div { class: "text-xs font-bold text-brand mb-1", "{name}" }
            div { class: "text-[11px] text-foreground-muted leading-snug", "{body}" }
        }
    }
}

#[component]
fn Arrow() -> Element {
    rsx! {
        span { class: "text-brand font-bold text-lg text-center", "→" }
    }
}
