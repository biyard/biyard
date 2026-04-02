use dioxus::prelude::*;

#[component]
pub(super) fn FaqSection() -> Element {
    rsx! {
        section {
            id: "faq",
            class: "ui-section px-6 md:px-24",
            div {
                class: "max-w-4xl w-full mx-auto",
                div {
                    class: "text-center mb-20 reveal",
                    span { style: "color: #00dfc0; font-size: 10px; font-weight: 900; letter-spacing: 0.4em; text-transform: uppercase;", "FAQ" }
                    h2 { class: "text-4xl md:text-6xl font-black mb-6 mt-4", "자주 묻는 질문" }
                }
                div {
                    class: "space-y-4 interactive",
                    for (i, (q, a)) in [
                        ("토큰 가치는 어떻게 보장되나요?", "모든 토큰은 실제 기업의 매출에 연동됩니다. 기업의 매출 일부가 트레저리에 적립되고, 이 트레저리가 토큰 가치의 하한선을 형성합니다. 매출이 없는 프로젝트는 토큰을 발행할 수 없으며, 매출이 지속되는 한 토큰 가치의 바닥이 보장됩니다."),
                        ("하한가(Floor Price)는 어떻게 유지되나요?", "스마트 컨트랙트가 자동으로 하한가를 방어합니다. 누군가 토큰을 하한가 이하로 매도하면, 트레저리가 자동으로 해당 토큰을 하한가에 매수(Buyback)하고 소각(Burn)합니다."),
                        ("대량 매도(덤핑)가 발생하면 어떻게 되나요?", "전체 물량의 90%가 한번에 매도되어도 하한가는 유지됩니다. 트레저리가 모든 매도 물량을 하한가에 매수하고 소각하기 때문입니다."),
                        ("기존 토큰 거래소와 무엇이 다른가요?", "기존 거래소는 실적 없는 토큰도 상장이 가능하고, 가치 근거 없는 가격 변동이 발생합니다. Biyard는 실제 매출이 있는 기업만 토큰을 발행할 수 있고, 모든 자금 흐름이 온체인에 공개됩니다."),
                        ("어떤 기업이 입점할 수 있나요?", "매출이 발생하는 모든 기업이 대상입니다. 신발, 커피, 의류, 식품, 헬스케어 등 업종에 제한이 없습니다. API 연동만으로 기존 POS나 앱에 토큰 이코노미를 붙일 수 있어, 블록체인 전문 인력 없이도 시작할 수 있습니다."),
                        ("토큰 보유자는 어떤 혜택이 있나요?", "토큰 보유자는 사실상 해당 브랜드의 주주와 같습니다. 매출이 늘면 트레저리가 쌓이고 토큰 가치가 올라갑니다. 또한 DAO를 통해 브랜드의 리워드 정책, 이벤트 등 의사결정에 직접 참여할 수 있습니다."),
                    ].iter().enumerate() {
                        {
                            let delay = format!("transition-delay: {}s;", i as f64 * 0.1);
                            rsx! {
                                details {
                                    class: "faq-item glass-panel group reveal",
                                    style: "{delay}",
                                    summary {
                                        class: "list-none px-8 py-8 flex justify-between items-center cursor-pointer font-bold text-lg",
                                        style: "color: #e2e8f0;",
                                        span { "{q}" }
                                        span { class: "faq-icon font-bold text-2xl", style: "color: #00dfc0;", "+" }
                                    }
                                    div {
                                        class: "px-8 pb-8 text-sm leading-relaxed pt-8",
                                        style: "color: #94a3b8; border-top: 1px solid rgba(255,255,255,0.05);",
                                        "{a}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
