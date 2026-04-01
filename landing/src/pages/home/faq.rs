use dioxus::prelude::*;

#[component]
pub(super) fn FaqSection() -> Element {
    rsx! {
        section {
            id: "faq",
            class: "py-20 px-4 relative overflow-hidden",
            style: "background: #141c2b;",
            div { class: "absolute", style: "bottom: -80px; left: 10%; width: 400px; height: 400px; background: radial-gradient(circle, rgba(96,165,250,0.05) 0%, transparent 65%); pointer-events: none;" }
            div {
                class: "max-w-3xl mx-auto relative z-10",
                div {
                    class: "text-center mb-12 reveal-fade",
                    p {
                        class: "text-sm font-semibold tracking-widest uppercase mb-3",
                        style: "color: #00d4aa;",
                        "FAQ"
                    }
                    h2 {
                        class: "text-3xl md:text-4xl font-bold reveal-type",
                        style: "color: #e8eefc; font-family: 'Outfit', 'Noto Sans KR', sans-serif;",
                        "자주 묻는 질문"
                    }
                }
                div {
                    class: "space-y-4",
                    for (q, a) in [
                        (
                            "토큰 가치는 어떻게 보장되나요?",
                            "모든 토큰은 실제 기업의 매출에 연동됩니다. 기업의 매출 일부가 트레저리에 적립되고, 이 트레저리가 토큰 가치의 하한선을 형성합니다. 매출이 없는 프로젝트는 토큰을 발행할 수 없으며, 매출이 지속되는 한 토큰 가치의 바닥이 보장됩니다."
                        ),
                        (
                            "하한가(Floor Price)는 어떻게 유지되나요?",
                            "스마트 컨트랙트가 자동으로 하한가를 방어합니다. 누군가 토큰을 하한가 이하로 매도하면, 트레저리가 자동으로 해당 토큰을 하한가에 매수(Buyback)하고 소각(Burn)합니다. 수학적으로 Floor Price = Treasury / Circulating Supply이므로, 매수 후 소각하면 공급량이 줄어 하한가는 절대 하락하지 않습니다."
                        ),
                        (
                            "대량 매도(덤핑)가 발생하면 어떻게 되나요?",
                            "전체 물량의 90%가 한번에 매도되어도 하한가는 유지됩니다. 트레저리가 모든 매도 물량을 하한가에 매수하고 소각하기 때문입니다. 오히려 소각으로 유통량이 줄어, 이후 매출이 계속 유입되면 남은 토큰의 하한가는 더 올라갑니다."
                        ),
                        (
                            "기존 토큰 거래소와 무엇이 다른가요?",
                            "기존 거래소는 실적 없는 토큰도 상장이 가능하고, 가치 근거 없는 가격 변동이 발생합니다. Biyard는 실제 매출이 있는 기업만 토큰을 발행할 수 있고, 모든 자금 흐름이 온체인에 공개되며, 스마트 컨트랙트가 하한가를 자동 방어합니다."
                        ),
                        (
                            "어떤 기업이 입점할 수 있나요?",
                            "매출이 발생하는 모든 기업이 대상입니다. 신발, 커피, 의류, 식품, 헬스케어 등 업종에 제한이 없습니다. API 연동만으로 기존 POS나 앱에 토큰 이코노미를 붙일 수 있어, 블록체인 전문 인력 없이도 시작할 수 있습니다."
                        ),
                        (
                            "토큰 보유자는 어떤 혜택이 있나요?",
                            "토큰 보유자는 사실상 해당 브랜드의 주주와 같습니다. 매출이 늘면 트레저리가 쌓이고 토큰 가치가 올라갑니다. 또한 DAO를 통해 브랜드의 리워드 정책, 이벤트 등 의사결정에 직접 참여할 수 있습니다."
                        ),
                    ] {
                        details {
                            class: "rounded-xl reveal-bounce",
                            style: "background: rgba(10,16,26,0.5); backdrop-filter: blur(16px); -webkit-backdrop-filter: blur(16px); border: 1px solid rgba(255,255,255,0.06);",
                            summary {
                                class: "cursor-pointer px-6 py-4 text-sm font-semibold flex items-center justify-between list-none",
                                style: "color: #e8eefc;",
                                span { "{q}" }
                                span { style: "color: #00d4aa; font-size: 18px;", "+" }
                            }
                            p {
                                class: "px-6 pb-5 text-sm leading-relaxed",
                                style: "color: #7a8ba6;",
                                "{a}"
                            }
                        }
                    }
                }
            }
        }
    }
}
