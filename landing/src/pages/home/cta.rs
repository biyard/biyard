use dioxus::prelude::*;
use super::data::console_url;
use crate::Route;

#[component]
pub(super) fn CtaSection() -> Element {
    let console_href = console_url();
    rsx! {
        section {
            class: "ui-section px-6 md:px-24",
            div {
                class: "max-w-5xl w-full mx-auto text-center reveal",
                div {
                    class: "glass-panel p-16 md:p-24 rounded-3xl relative overflow-hidden",
                    style: "border-color: rgba(0,223,192,0.3);",
                    // Glow orbs
                    div { class: "absolute rounded-full", style: "top: -96px; left: -96px; width: 320px; height: 320px; background: rgba(0,223,192,0.1); filter: blur(120px);" }
                    div { class: "absolute rounded-full", style: "bottom: -96px; right: -96px; width: 320px; height: 320px; background: rgba(112,0,255,0.1); filter: blur(120px);" }
                    span { style: "color: #00dfc0; font-size: 10px; font-weight: 900; letter-spacing: 0.4em; text-transform: uppercase;", "Biyard Launchpad" }
                    h2 {
                        class: "text-4xl md:text-7xl font-black mb-12 mt-10",
                        style: "line-height: 1.1;",
                        "매출 기반 토큰 경제,"
                        br {}
                        span { class: "glow-text tracking-tighter", "지금 시작하세요." }
                    }
                    p { class: "text-lg md:text-xl mb-16 max-w-2xl mx-auto leading-relaxed", style: "color: #94a3b8;", "매출 기반 토큰 이코노미로 브랜드와 고객이 함께 성장합니다. 블록체인 지식 없이도 5분 만에 연동." }
                    div {
                        class: "flex flex-col sm:flex-row gap-6 justify-center interactive",
                        a {
                            href: "{console_href}",
                            class: "btn-hyper px-16 py-6 rounded-full font-black text-base uppercase tracking-widest text-center",
                            style: "box-shadow: 0 0 40px rgba(0,223,192,0.3);",
                            "Console로 이동하기 \u{2192}"
                        }
                        Link {
                            to: Route::Pricing {},
                            class: "glass-panel px-16 py-6 rounded-full font-bold text-base text-center interactive",
                            "요금제 보기"
                        }
                    }
                }
            }
        }
    }
}
