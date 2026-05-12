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
                    "STO 와 결합 가능한"
                    br {}
                    "브랜드 토큰 PaaS"
                }
                p { class: "text-sm md:text-base text-foreground-soft leading-relaxed max-w-2xl",
                    "Biyard Launchpad 는 발행사·증권사·브랜드가 자체 "
                    strong { class: "text-foreground", "유틸리티 토큰" }
                    " 을 손쉽게 발행·운영할 수 있는 PaaS 인프라입니다. 토큰증권 (STO) 이 본격 시행되는 2027.01 이후, "
                    strong { class: "text-foreground", "STO + 유틸리티 토큰 결합 모델" }
                    " 을 통해 발행사·투자자에게 새로운 가치 경험을 제공할 수 있습니다."
                }
            }

            section { class: "grid grid-cols-1 md:grid-cols-3 gap-3 mb-12",
                Feat { icon: "🪙", title: "브랜드 토큰 발행",
                    body: "코드 작성 없이 ERC 표준 기반 브랜드 토큰을 발행할 수 있는 PaaS API. 화이트라벨 형태로 발행사 사이트에 임베드 가능." }
                Feat { icon: "🔗", title: "온체인 인프라",
                    body: "지갑 생성·전송·이력 조회까지 표준 API 로 제공. 발행사는 블록체인 운영 없이 토큰 비즈니스만 설계." }
                Feat { icon: "📊", title: "STO 정보 연계",
                    body: "Biyard STO 정보 플랫폼과 SSO·데이터 연동. 자체 STO 보유 투자자에게 유틸리티 토큰을 자동 분배하는 시나리오 구현 가능." }
            }

            section { class: "bg-panel border border-brand rounded-2xl p-10 mb-12",
                h2 { class: "text-xl font-bold mb-3", "STO + 유틸리티 토큰 결합 시나리오" }
                p { class: "text-sm text-foreground-soft leading-relaxed mb-6",
                    "STO 는 "
                    strong { class: "text-foreground", "증권 (수익 권리)" }
                    " 이고, 유틸리티 토큰은 "
                    strong { class: "text-foreground", "경험·혜택" }
                    " 입니다. 하나로 묶을 수 없는 두 가치를 발행사가 동시에 제공할 수 있게 하는 것이 Biyard Launchpad 의 역할입니다."
                }
                div { class: "grid grid-cols-1 md:grid-cols-[1fr_auto_1fr_auto_1fr] gap-3 items-center",
                    Step { name: "STO 청약", body: "투자자가 토큰증권 청약 — 수익 권리 확보" }
                    Arrow {}
                    Step { name: "브랜드 토큰 자동 분배", body: "청약 인증 → 발행사 브랜드 토큰 지급" }
                    Arrow {}
                    Step { name: "경험·혜택 사용", body: "전시 우선 관람·상품 할인·커뮤니티 참여 등" }
                }
            }

            section { class: "bg-panel border border-dashed border-brand rounded-2xl p-10 text-center",
                h2 { class: "text-base font-bold mb-2", "Launchpad 도입 문의" }
                p { class: "text-xs text-foreground-muted mb-4", "STO 발행과 브랜드 토큰을 함께 운영하고 싶으신 발행사·증권사·브랜드는 문의해주세요." }
                a { href: "mailto:hi@biyard.co?subject=Launchpad",
                    class: "bg-brand text-brand-contrast font-bold px-5 py-2.5 rounded-md text-sm inline-block hover:bg-brand-strong",
                    "문의하기 →"
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
