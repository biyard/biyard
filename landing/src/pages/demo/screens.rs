use dioxus::prelude::*;

// ─────────────────────────────────────────────────────────
// Le Mouton native screens (their existing app — they built this)
// ─────────────────────────────────────────────────────────

/// Home — product feed (Le Mouton own)
#[component]
pub(super) fn LmHomeScreen() -> Element {
    rsx! {
        div { style: "height: 100%; display: flex; flex-direction: column; background: #FAF7F2;",
            // Header
            div { class: "lm-header",
                span { style: "font-size: 20px; font-family: Georgia, serif; color: #2C2420;", "Le Mouton" }
                div { style: "display: flex; gap: 12px; color: #6B5D52; font-size: 18px;", "\u{1F50D}  \u{1F6D2}" }
            }
            // Banner
            div {
                style: "background: linear-gradient(135deg, #E8E0D3, #D4C5B0); padding: 40px 20px; text-align: center;",
                p { style: "color: #6B5D52; font-size: 11px; font-weight: 600; letter-spacing: 0.2em; margin-bottom: 8px;", "MERINO WOOL" }
                p { style: "color: #2C2420; font-size: 20px; font-weight: 700;", "벗고 싶지 않은 편안함" }
                p { style: "color: #6B5D52; font-size: 12px; margin-top: 6px;", "르무통 겨울 신상 컬렉션" }
            }
            // Product grid
            div { style: "flex: 1; padding: 16px; overflow: auto;",
                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                    for (name, price, emoji) in [
                        ("르무통 메이트", "116,900원", "\u{1F45F}"),
                        ("르무통 레츠", "116,900원", "\u{1F45F}"),
                        ("르무통 업", "116,900원", "\u{1F45F}"),
                        ("르무통 클래식", "129,000원", "\u{1F45F}"),
                    ] {
                        div { class: "lm-card", style: "padding: 12px;",
                            div { style: "aspect-ratio: 1; background: #E8E0D3; border-radius: 6px; display: flex; align-items: center; justify-content: center; font-size: 40px; color: #8B7355; margin-bottom: 10px;",
                                "{emoji}"
                            }
                            p { style: "font-size: 12px; color: #2C2420; font-weight: 500;", "{name}" }
                            p { style: "font-size: 11px; color: #6B5D52; margin-top: 2px;", "{price}" }
                        }
                    }
                }
            }
            // Bottom nav
            div { style: "padding: 10px; border-top: 1px solid #E8E0D3; background: #fff; display: flex; justify-content: space-around; font-size: 10px; color: #6B5D52;",
                span { "홈" }
                span { "카테고리" }
                span { "검색" }
                span { "좋아요" }
                span { "MY" }
            }
        }
    }
}

/// Product detail (Le Mouton own)
#[component]
pub(super) fn LmProductScreen() -> Element {
    rsx! {
        div { style: "height: 100%; display: flex; flex-direction: column; background: #FAF7F2;",
            div { class: "lm-header",
                span { style: "font-size: 18px; color: #2C2420;", "\u{2190}" }
                span { style: "font-size: 14px; color: #2C2420;", "상품 상세" }
                span { style: "font-size: 18px; color: #2C2420;", "\u{2661}" }
            }
            // Product image
            div { style: "aspect-ratio: 1; background: linear-gradient(135deg, #F5EDE0, #D4C5B0); display: flex; align-items: center; justify-content: center; font-size: 100px; color: #8B7355;",
                "\u{1F45F}"
            }
            // Info
            div { style: "padding: 20px; flex: 1;",
                p { style: "font-size: 11px; color: #6B5D52; letter-spacing: 0.1em;", "MEN · WOMEN" }
                h2 { style: "font-size: 20px; font-weight: 700; color: #2C2420; margin: 6px 0;", "르무통 메이트" }
                p { style: "font-size: 12px; color: #6B5D52; margin-bottom: 12px;", "\u{2B50}\u{2B50}\u{2B50}\u{2B50}\u{2B50} 4.9 (2,340)" }
                p { style: "font-size: 22px; font-weight: 700; color: #2C2420; margin-bottom: 20px;", "129,000원" }
                div { style: "padding: 12px; background: #fff; border: 1px solid #E8E0D3; border-radius: 6px; font-size: 11px; color: #6B5D52;",
                    "\u{1F4E6} 무료배송 \u{00B7} 당일출고"
                    br {}
                    "\u{267B}\u{FE0F} 무제한 무료반품 & 교환"
                }
                p { style: "margin-top: 16px; font-size: 11px; color: #6B5D52;", "\u{1F4B0} 적립금 \u{00B7} 구매 시 " span { style: "color: #C65D47; font-weight: 700;", "3,870원" } " (3%)" }
            }
            // CTA
            div { style: "padding: 14px 16px; background: #fff; border-top: 1px solid #E8E0D3;",
                div { class: "lm-btn", "구매하기" }
            }
        }
    }
}

/// Checkout (Le Mouton own)
#[component]
pub(super) fn LmCheckoutScreen() -> Element {
    rsx! {
        div { style: "height: 100%; display: flex; flex-direction: column; background: #FAF7F2;",
            div { class: "lm-header",
                span { style: "font-size: 18px;", "\u{2190}" }
                span { style: "font-size: 14px; font-weight: 700; color: #2C2420;", "결제" }
                span {}
            }
            div { style: "flex: 1; padding: 16px; overflow: auto;",
                // Item
                div { class: "lm-card", style: "padding: 14px; display: flex; gap: 12px; margin-bottom: 12px;",
                    div { style: "width: 64px; height: 64px; background: #E8E0D3; border-radius: 6px; display: flex; align-items: center; justify-content: center; font-size: 28px;", "\u{1F45F}" }
                    div { style: "flex: 1;",
                        p { style: "font-size: 13px; font-weight: 600; color: #2C2420;", "르무통 메이트" }
                        p { style: "font-size: 10px; color: #6B5D52; margin-top: 2px;", "블랙 / 250" }
                        p { style: "font-size: 13px; font-weight: 700; color: #2C2420; margin-top: 6px;", "129,000원" }
                    }
                }
                // Delivery
                div { class: "lm-card", style: "padding: 14px; margin-bottom: 12px;",
                    p { style: "font-size: 11px; color: #6B5D52; margin-bottom: 6px;", "배송지" }
                    p { style: "font-size: 13px; color: #2C2420;", "서울시 강남구 테헤란로 427" }
                    p { style: "font-size: 11px; color: #6B5D52; margin-top: 2px;", "김서연 \u{00B7} 010-1234-5678" }
                }
                // Summary
                div { class: "lm-card", style: "padding: 14px;",
                    for (label, value) in [("상품 금액", "129,000원"), ("배송비", "0원"), ("적립금 사용", "0원")] {
                        div { style: "display: flex; justify-content: space-between; padding: 4px 0; font-size: 12px; color: #6B5D52;",
                            span { "{label}" }
                            span { style: "color: #2C2420;", "{value}" }
                        }
                    }
                    div { style: "display: flex; justify-content: space-between; padding: 8px 0 0; margin-top: 8px; border-top: 1px solid #E8E0D3;",
                        span { style: "font-weight: 700; color: #2C2420;", "총 결제 금액" }
                        span { style: "font-weight: 700; color: #2C2420; font-size: 16px;", "129,000원" }
                    }
                }
            }
            div { style: "padding: 14px 16px; background: #fff; border-top: 1px solid #E8E0D3;",
                div { class: "lm-btn", "129,000원 결제하기" }
            }
        }
    }
}
