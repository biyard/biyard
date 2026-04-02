use dioxus::prelude::*;

#[component]
pub(super) fn WhyBiyardSection() -> Element {
    rsx! {
        section {
            class: "ui-section px-6 md:px-24",
            div {
                class: "max-w-6xl w-full mx-auto",
                div {
                    class: "text-center mb-20 reveal",
                    h2 {
                        class: "text-4xl md:text-6xl font-black mb-6",
                        "Biyard는 이 구조를"
                        br {}
                        span { class: "glow-text tracking-tighter", "완전히 뒤집습니다." }
                    }
                }
                div {
                    class: "grid lg:grid-cols-2 gap-12 items-center",
                    // Before
                    div {
                        class: "glass-panel p-10 rounded-3xl reveal",
                        style: "border-color: rgba(255,77,77,0.15);",
                        div { class: "text-center mb-10", span { class: "font-bold uppercase tracking-widest text-xs", style: "color: #ff4d4d;", "기존 마케팅 악순환" } }
                        div {
                            class: "flex flex-col items-center gap-4",
                            div { class: "grid grid-cols-2 gap-4",
                                div { class: "p-4 border border-white/5 rounded-xl text-center text-sm font-semibold", style: "color: #94a3b8;", "광고비" br {} "1억 지출" }
                                div { class: "p-4 border border-white/5 rounded-xl text-center text-sm font-semibold", style: "color: #94a3b8;", "고객 유치" br {} "일시적" }
                            }
                            div { class: "w-16 h-16 flex items-center justify-center text-3xl font-black", style: "color: #ff4d4d;", "\u{2193}" }
                            div { class: "p-8 glass-panel rounded-full font-black text-lg text-center", style: "border-color: rgba(255,77,77,0.2); color: #ff4d4d;", "돈이 빠져나감" }
                            div { class: "w-16 h-16 flex items-center justify-center text-3xl font-black", style: "color: #ff4d4d;", "\u{2193}" }
                            div { class: "grid grid-cols-2 gap-4",
                                div { class: "p-4 border border-white/5 rounded-xl text-center text-sm font-semibold", style: "color: #94a3b8;", "고객 이탈" br {} "재방문 X" }
                                div { class: "p-4 border border-white/5 rounded-xl text-center text-sm font-semibold", style: "color: #94a3b8;", "또 광고" br {} "반복 지출" }
                            }
                            p { class: "mt-8 text-xs text-center uppercase tracking-widest", style: "color: rgba(255,77,77,0.6);", "끝없는 악순환" }
                        }
                    }
                    // After
                    div {
                        class: "glass-panel p-10 rounded-3xl reveal",
                        style: "border-color: rgba(0,223,192,0.2); transition-delay: 0.1s;",
                        div { class: "text-center mb-10", span { class: "font-bold uppercase tracking-widest text-xs", style: "color: #00dfc0;", "Biyard 선순환" } }
                        div {
                            class: "flex flex-col items-center gap-4",
                            div { class: "grid grid-cols-2 gap-4",
                                div { class: "p-4 rounded-xl text-center text-sm font-semibold", style: "border: 1px solid rgba(0,223,192,0.1); color: #e2e8f0;", "매출 발생" br {} "2-4% 적립" }
                                div { class: "p-4 rounded-xl text-center text-sm font-semibold", style: "border: 1px solid rgba(0,223,192,0.1); color: #e2e8f0;", "토큰 가치" br {} "자동 상승" }
                            }
                            div { class: "w-16 h-16 flex items-center justify-center text-3xl font-black", style: "color: #00dfc0;", "\u{2193}" }
                            div { class: "p-8 glass-panel rounded-full font-black text-lg text-center", style: "border-color: rgba(0,223,192,0.5); color: #00dfc0; box-shadow: 0 0 25px rgba(0,223,192,0.3);", "가치가 순환한다" }
                            div { class: "w-16 h-16 flex items-center justify-center text-3xl font-black", style: "color: #00dfc0;", "\u{2193}" }
                            div { class: "grid grid-cols-2 gap-4",
                                div { class: "p-4 rounded-xl text-center text-sm font-semibold", style: "border: 1px solid rgba(0,223,192,0.1); color: #e2e8f0;", "고객 홍보" br {} "입소문 확산" }
                                div { class: "p-4 rounded-xl text-center text-sm font-semibold", style: "border: 1px solid rgba(0,223,192,0.1); color: #e2e8f0;", "신규 고객" br {} "자연 유입" }
                            }
                            p { class: "mt-8 text-xs text-center uppercase tracking-widest", style: "color: #00dfc0;", "선순환" }
                        }
                    }
                }
            }
        }
    }
}
