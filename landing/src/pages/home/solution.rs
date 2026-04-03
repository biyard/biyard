use dioxus::prelude::*;

#[component]
pub(super) fn SolutionSection() -> Element {
    rsx! {
        section {
            id: "solution",
            class: "ui-section px-6 md:px-24",
            style: "background: rgba(0,0,0,0.2);",
            div {
                class: "max-w-6xl w-full mx-auto",
                div {
                    class: "text-center mb-24 reveal",
                    span { style: "color: #00dfc0; font-size: 10px; font-weight: 900; letter-spacing: 0.4em; text-transform: uppercase;", "Core Innovation" }
                    h2 { class: "text-4xl md:text-7xl font-black mt-4", "구매가 곧 " span { class: "glow-text", "투자" } "가 됩니다." }
                }
                div {
                    class: "grid lg:grid-cols-3 gap-8 interactive",
                    // Card 1
                    div {
                        class: "glass-panel p-10 rounded-3xl reveal h-full",
                        style: "border-top: 2px solid rgba(0,223,192,0.3);",
                        h4 { style: "color: #00dfc0; font-size: 10px; font-weight: 900; letter-spacing: 0.3em; text-transform: uppercase; margin-bottom: 24px;", "Revenue-Linked" }
                        h3 { class: "text-3xl font-bold mb-6 italic", "매출 연동 적립" }
                        div { class: "text-5xl font-mono font-black mb-8", "2-4%" }
                        p { class: "text-sm leading-relaxed", style: "color: #94a3b8;", "구매 금액의 일부가 자동으로 온체인 트레저리에 적립됩니다. 실제 매출이 토큰의 펀더멘털입니다." }
                    }
                    // Card 2
                    div {
                        class: "glass-panel p-10 rounded-3xl reveal h-full",
                        style: "border-top: 2px solid rgba(167,139,250,0.3); transition-delay: 0.1s;",
                        h4 { style: "color: #a78bfa; font-size: 10px; font-weight: 900; letter-spacing: 0.3em; text-transform: uppercase; margin-bottom: 24px;", "Floor Guarantee" }
                        h3 { class: "text-3xl font-bold mb-6 italic", "하한가 보장" }
                        div { class: "text-5xl font-mono font-black mb-8 tracking-tighter", style: "color: #a78bfa;", "Floor " span { class: "text-sm", "Price" } }
                        p { class: "text-sm leading-relaxed", style: "color: #94a3b8;", "스마트 컨트랙트가 수학적으로 하한가를 방어합니다. 덤핑해도 바닥이 보장되는 혁신적 구조입니다." }
                    }
                    // Card 3
                    div {
                        class: "glass-panel p-10 rounded-3xl reveal h-full",
                        style: "border-top: 2px solid rgba(96,165,250,0.3); transition-delay: 0.2s;",
                        h4 { style: "color: #60a5fa; font-size: 10px; font-weight: 900; letter-spacing: 0.3em; text-transform: uppercase; margin-bottom: 24px;", "Value Growth" }
                        h3 { class: "text-3xl font-bold mb-6 italic", "가치 성장" }
                        div { class: "text-6xl font-mono font-black mb-8", style: "color: #60a5fa;", "\u{221E}" }
                        p { class: "text-sm leading-relaxed", style: "color: #94a3b8;", "매출이 늘면 트레저리가 쌓이고 하한가가 상승합니다. 모든 홀더의 자산 가치가 함께 올라갑니다." }
                    }
                }
            }
        }
    }
}
