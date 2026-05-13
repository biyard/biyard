use dioxus::prelude::*;

use crate::features::catalog::views::Topbar;

#[component]
pub fn WhitepaperView() -> Element {
    rsx! {
        Topbar { active: "index".to_string() }
        main { class: "max-w-5xl mx-auto px-6 py-8",
            div { class: "flex items-center gap-2 text-xs text-foreground-muted mb-2",
                a { href: "/biyard-index", class: "hover:text-brand", "Biyard Index" }
                span { "/" }
                span { class: "text-foreground-soft", "Whitepaper" }
            }
            article { class: "bg-panel border border-border rounded-2xl p-12 md:p-14",
                div { class: "flex gap-2 mb-3",
                    span { class: "bg-brand-soft text-brand text-[10px] font-bold tracking-wider px-2 py-1 rounded", "v0.1 · DRAFT" }
                    span { class: "text-[11px] text-foreground-muted self-center", "작성 중 · 2026.05" }
                }
                h1 { class: "text-2xl md:text-3xl font-bold mb-3", "Biyard Index 백서" }
                p { class: "text-foreground-soft leading-relaxed mb-6 pb-6 border-b border-border",
                    "본 문서는 Biyard Index 의 설계 원칙·축 정의·산출식·검증 절차를 정리한 기술 문서입니다. Biyard Index 는 토큰증권 (Security Token) 을 평가하기 위해 설계된 Web3 전용 지표이며, 기존 신용평가가 다루지 못하는 온체인 발행 무결성·스마트컨트랙트 보안·지갑 분포·거버넌스 등을 정량 축으로 환산합니다."
                }
                Section { title: "1. 개요",
                    p { class: "text-foreground-soft leading-relaxed mb-3",
                        "Biyard Index 는 분산원장 기반 토큰증권의 신뢰도를 정량적으로 평가하는 표준 지표입니다. 발행사 재무제표·증권신고서 같은 전통 정보는 기존 신용평가의 영역이며, Biyard Index 는 이들이 다루지 못하는 "
                        strong { class: "text-foreground", "Web3 고유 신뢰 신호" }
                        " 를 측정합니다."
                    }
                }
                Section { title: "2. 6개 평가 축",
                    ol { class: "list-decimal list-inside space-y-2 text-foreground-soft text-sm leading-relaxed",
                        li { strong { class: "text-foreground", "컨트랙트 보안" } " — 토큰 컨트랙트 감사 이력·취약점 발견·업그레이드 권한·키 관리" }
                        li { strong { class: "text-foreground", "온체인 발행 무결성" } " — 발행 수량 = 신탁 자산 1:1 일치, 임의 발행·소각 권한, 이벤트 로그 일관성" }
                        li { strong { class: "text-foreground", "지갑 분포·집중도" } " — 상위 N 지갑 비중, 발행사·운영사 자체 보유 비중" }
                        li { strong { class: "text-foreground", "유통 신뢰성" } " — 거래소 거래량·호가 스프레드·Wash trading 시그널" }
                        li { strong { class: "text-foreground", "거버넌스" } " — 업그레이드 절차·멀티시그·타임락·운영 주체 독립성" }
                        li { strong { class: "text-foreground", "정보 신뢰성" } " — 정보 갱신 적시성·이의 제기 응답·외부 채널 일관성" }
                    }
                }
                Section { title: "3. 등급 환산",
                    p { class: "text-foreground-soft mb-3 leading-relaxed",
                        "종합 점수는 유형 내 백분위를 기준으로 다섯 단계 등급으로 환산됩니다."
                    }
                    table { class: "w-full text-sm",
                        thead { tr { class: "text-foreground-muted text-xs uppercase",
                            th { class: "text-left py-2 px-3 bg-panel-muted", "등급" }
                            th { class: "text-left py-2 px-3 bg-panel-muted", "백분위" }
                            th { class: "text-left py-2 px-3 bg-panel-muted", "설명" }
                        } }
                        tbody {
                            GradeRow { letter: "S", pct: "상위 5%", desc: "모든 축에서 평균을 크게 상회" }
                            GradeRow { letter: "A", pct: "상위 5~25%", desc: "대부분 축에서 평균 상회" }
                            GradeRow { letter: "B", pct: "상위 25~60%", desc: "유형 평균 수준" }
                            GradeRow { letter: "C", pct: "상위 60~85%", desc: "일부 축에서 평균 미달" }
                            GradeRow { letter: "D", pct: "하위 15%", desc: "다수 축에서 평균 크게 미달" }
                        }
                    }
                }
                Section { title: "4. 면책 조항",
                    p { class: "text-foreground-soft leading-relaxed",
                        "Biyard Index 는 자본시장법상 "
                        strong { class: "text-foreground", "신용평가업·투자자문업·금융투자업이 아닙니다" }
                        ". 본 지표의 등급·점수는 온체인 데이터 및 공시 자료 기반 정보 제공 목적이며, 특정 토큰증권에 대한 투자 권유·매수 추천·목표가 제시가 아닙니다. 투자 결정은 사용자 본인의 책임입니다."
                    }
                }

                div { class: "flex gap-2 mt-10",
                    a { href: "/biyard-index", class: "bg-brand text-brand-contrast font-bold px-4 py-2 rounded text-sm",
                        "← 제품 페이지로"
                    }
                }
            }
        }
    }
}

#[component]
fn Section(title: &'static str, children: Element) -> Element {
    rsx! {
        section { class: "mb-8",
            h2 { class: "text-xl font-bold mb-3", "{title}" }
            {children}
        }
    }
}

#[component]
fn GradeRow(letter: &'static str, pct: &'static str, desc: &'static str) -> Element {
    rsx! {
        tr { class: "border-b border-border",
            td { class: "py-2 px-3 font-mono font-bold text-brand", "{letter}" }
            td { class: "py-2 px-3 text-foreground-soft", "{pct}" }
            td { class: "py-2 px-3 text-foreground-muted text-xs", "{desc}" }
        }
    }
}
