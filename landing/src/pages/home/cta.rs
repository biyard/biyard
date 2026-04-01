use dioxus::prelude::*;
use crate::Route;
use super::data::console_url;

#[component]
pub(super) fn CtaSection() -> Element {
    let console_href = console_url();
    rsx! {
        section {
            class: "py-14 px-4 text-center relative overflow-hidden",
            style: "background: #0c1018;",
            div {
                class: "relative z-10 reveal-fade",
                h2 {
                    class: "text-3xl font-extrabold mb-3",
                    style: "color: #e8eefc;",
                    "지금 시작하세요"
                }
                p {
                    class: "mb-6 max-w-xl mx-auto text-base",
                    style: "color: #7a8ba6;",
                    "매출 기반 토큰 이코노미로 브랜드와 고객이 함께 성장합니다."
                }
                div {
                    class: "flex items-center justify-center gap-4 flex-wrap",
                    a {
                        class: "inline-flex items-center px-7 py-3 rounded-xl font-bold text-base shadow-lg",
                        style: "background: #00d4aa; color: #0c1018;",
                        href: "{console_href}",
                        "Console로 이동하기 →"
                    }
                    Link {
                        to: Route::Pricing {},
                        class: "inline-flex items-center px-7 py-3 rounded-xl font-bold text-base",
                        style: "border: 1px solid rgba(0,212,170,0.3); color: #00d4aa;",
                        "요금제 보기"
                    }
                }
            }
        }
    }
}
