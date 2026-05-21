use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::features::catalog::views::Topbar;
use crate::features::pricing::PricingTranslate;

#[component]
pub fn PricingView() -> Element {
    let t: PricingTranslate = use_translate();
    let individual_features: Vec<&'static str> = vec![
        t.track_individual_feat1,
        t.track_individual_feat2,
        t.track_individual_feat3,
        t.track_individual_feat4,
    ];
    let org_features: Vec<&'static str> = vec![
        t.track_org_feat1,
        t.track_org_feat2,
        t.track_org_feat3,
        t.track_org_feat4,
    ];
    rsx! {
        Topbar { active: "pricing".to_string() }
        main { class: "max-w-7xl mx-auto px-6 py-8",
            section { class: "text-center mb-10",
                h1 { class: "text-3xl md:text-4xl font-bold mb-3", {t.page_title} }
                p { class: "text-foreground-muted text-sm md:text-base max-w-2xl mx-auto",
                    {t.page_subtitle}
                }
            }

            section { class: "grid grid-cols-1 md:grid-cols-2 gap-4 mb-12",
                Track {
                    badge: t.track_individual_badge,
                    badge_class: "bg-info-soft text-info",
                    title: t.track_individual_title,
                    desc: t.track_individual_desc,
                    features: individual_features,
                }
                Track {
                    badge: t.track_org_badge,
                    badge_class: "bg-brand-soft text-brand",
                    title: t.track_org_title,
                    desc: t.track_org_desc,
                    features: org_features,
                }
            }

        }
    }
}

#[component]
fn Track(
    badge: &'static str,
    badge_class: &'static str,
    title: &'static str,
    desc: &'static str,
    features: Vec<&'static str>,
) -> Element {
    rsx! {
        article { class: "bg-panel border border-border rounded-2xl p-8",
            span { class: "inline-block text-[10px] font-bold tracking-widest px-2 py-1 rounded mb-3 {badge_class}", "{badge}" }
            h2 { class: "text-xl font-bold mb-2", "{title}" }
            p { class: "text-foreground-muted text-sm leading-relaxed mb-5", "{desc}" }
            ul { class: "space-y-2",
                for feat in features.iter() {
                    li { class: "flex gap-2 items-start text-sm text-foreground-soft",
                        span { class: "text-brand font-bold shrink-0", "✓" }
                        span { "{feat}" }
                    }
                }
            }
        }
    }
}
