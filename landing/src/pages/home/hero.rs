use dioxus::prelude::*;

use super::data::console_url;
use super::hero_cube::HeroCubeGroup;

#[component]
pub(super) fn HeroSection() -> Element {
    let console_href = console_url();
    rsx! {
        // Three.js canvas — fixed behind everything
        div {
            style: "position: fixed; top: 0; left: 0; width: 100%; height: 100%; z-index: 0; pointer-events: none;",
            HeroCubeGroup {}
        }

        section {
            class: "ui-section px-6 md:px-24",
            div {
                class: "max-w-5xl w-full",

                // Status badge
                div {
                    class: "inline-flex items-center gap-3 mb-8 px-5 py-2 rounded-full glass-panel reveal active interactive",
                    style: "border-color: rgba(0,223,192,0.2);",
                    span {
                        class: "relative flex h-2 w-2",
                        span { class: "animate-ping absolute inline-flex h-full w-full rounded-full opacity-75", style: "background: #00dfc0;" }
                        span { class: "relative inline-flex rounded-full h-2 w-2", style: "background: #00dfc0;" }
                    }
                    span {
                        style: "color: #00dfc0; font-size: 10px; font-weight: 900; letter-spacing: 0.4em; text-transform: uppercase;",
                        "Biyard Launchpad"
                    }
                }

                // Main title
                h1 {
                    class: "text-5xl md:text-8xl font-black mb-10 reveal active",
                    style: "line-height: 1.1;",
                    "매출이 "
                    span { class: "glow-text italic", "토큰의 가치가 되는" }
                    br {}
                    "시대를 만듭니다."
                }

                // Description
                p {
                    class: "text-lg md:text-2xl mb-12 leading-relaxed max-w-3xl reveal active",
                    style: "color: #94a3b8; transition-delay: 0.2s;",
                    "실제 매출에 연동된 토큰 이코노미로, 고객은 주주가 되고 브랜드는 함께 성장합니다. Biyard는 기업과 소비자를 연결하는 가장 투명한 분산형 인프라를 제공합니다."
                }

                // Buttons
                div {
                    class: "flex flex-col sm:flex-row gap-6 interactive reveal active",
                    style: "transition-delay: 0.4s;",
                    a {
                        href: "{console_href}",
                        class: "btn-hyper px-12 py-5 rounded-sm font-black text-sm uppercase tracking-widest text-center",
                        "Console 시작하기"
                    }
                    a {
                        href: "#about",
                        class: "glass-panel px-12 py-5 rounded-sm font-bold text-sm text-center",
                        "자세히 알아보기"
                    }
                }
            }
        }
    }
}
