use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::features::biyard_index::BiyardIndexTranslate;
use crate::features::catalog::views::Topbar;

#[component]
pub fn BiyardIndexView() -> Element {
    let t: BiyardIndexTranslate = use_translate();

    rsx! {
        Topbar { active: "index".to_string() }
        main { class: "max-w-7xl mx-auto px-6 py-8",
            // Hero
            section { class: "relative bg-panel border border-brand rounded-2xl p-14 mb-6 overflow-hidden",
                div { class: "text-xs font-bold text-brand tracking-widest mb-4", "{t.eyebrow}" }
                h1 { class: "text-3xl md:text-4xl font-bold leading-tight mb-4 max-w-2xl", "{t.hero_title}" }
                p { class: "text-sm md:text-base text-foreground-soft leading-relaxed max-w-2xl mb-6", "{t.hero_sub}" }
                div { class: "flex gap-2 flex-wrap",
                    a { href: "mailto:hi@biyard.co?subject=Biyard Index Application",
                        class: "bg-brand text-brand-contrast font-bold px-5 py-2.5 rounded-md text-sm hover:bg-brand-strong",
                        "{t.apply_cta}"
                    }
                    a { href: "/index/whitepaper",
                        class: "bg-transparent text-foreground border border-border px-5 py-2.5 rounded-md text-sm hover:bg-panel-muted",
                        "{t.whitepaper_cta}"
                    }
                }
            }

            // Why
            div { class: "text-center mb-8",
                div { class: "text-xs font-bold text-brand tracking-widest mb-2", "{t.why_eyebrow}" }
                h2 { class: "text-2xl font-bold mb-2", "{t.why_title}" }
                p { class: "text-sm text-foreground-muted max-w-2xl mx-auto leading-relaxed", "{t.why_sub}" }
            }
            section { class: "grid grid-cols-1 md:grid-cols-3 gap-3 mb-12",
                FeatureCard { icon: "🔗", title: t.feat_onchain_title.to_string(), body: t.feat_onchain_body.to_string() }
                FeatureCard { icon: "🛡", title: t.feat_contract_title.to_string(), body: t.feat_contract_body.to_string() }
                FeatureCard { icon: "📡", title: t.feat_realtime_title.to_string(), body: t.feat_realtime_body.to_string() }
            }

            // Methodology + Axes
            section { class: "bg-panel border border-brand rounded-2xl p-8 mb-12",
                div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 items-center",
                    div {
                        div { class: "text-xs font-bold text-brand tracking-widest mb-2", "{t.method_eyebrow}" }
                        h2 { class: "text-2xl font-bold mb-3", "{t.method_title}" }
                        p { class: "text-sm text-foreground-muted leading-relaxed mb-5", "{t.method_body}" }
                        div { class: "grid grid-cols-2 gap-2",
                            AxisChip { num: "01", name: t.axis_01.to_string() }
                            AxisChip { num: "02", name: t.axis_02.to_string() }
                            AxisChip { num: "03", name: t.axis_03.to_string() }
                            AxisChip { num: "04", name: t.axis_04.to_string() }
                            AxisChip { num: "05", name: t.axis_05.to_string() }
                            AxisChip { num: "06", name: t.axis_06.to_string() }
                        }
                    }
                    // 가짜 점수 시각화
                    div { class: "bg-panel-muted border border-border rounded-2xl p-8 text-center",
                        div { class: "relative w-40 h-40 mx-auto mb-4 rounded-full",
                            style: "background: conic-gradient(var(--color-brand) 0% 78%, var(--color-panel) 78% 100%);",
                            div { class: "absolute inset-3 bg-panel-muted rounded-full flex flex-col items-center justify-center",
                                span { class: "text-4xl font-mono font-bold", "78" }
                                span { class: "text-[10px] text-foreground-muted tracking-widest mt-1", "/ 100" }
                            }
                        }
                        div { class: "text-5xl font-mono font-black text-brand", "A" }
                        div { class: "text-xs text-foreground-muted mt-2", "토큰증권 부문 · 상위 18%" }
                        div { class: "inline-block text-[10px] bg-panel text-foreground-muted px-2 py-1 rounded mt-3 tracking-wider", "PREVIEW · 실제 산출 아님" }
                    }
                }
            }
        }
    }
}

#[component]
fn FeatureCard(icon: &'static str, title: String, body: String) -> Element {
    rsx! {
        article { class: "bg-panel border border-border rounded-2xl p-6",
            div { class: "w-9 h-9 rounded-md bg-brand-soft text-brand flex items-center justify-center text-lg font-bold mb-3", "{icon}" }
            h3 { class: "text-base font-bold mb-2", "{title}" }
            p { class: "text-sm text-foreground-muted leading-relaxed", "{body}" }
        }
    }
}

#[component]
fn AxisChip(num: &'static str, name: String) -> Element {
    rsx! {
        div { class: "bg-panel-muted border border-border rounded-md px-3 py-2 text-xs",
            span { class: "font-mono font-bold text-brand mr-2", "{num}" }
            span { class: "text-foreground font-semibold", "{name}" }
        }
    }
}
