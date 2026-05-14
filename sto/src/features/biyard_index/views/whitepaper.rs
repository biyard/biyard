use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::features::biyard_index::BiyardIndexTranslate;
use crate::features::catalog::views::Topbar;

#[component]
pub fn WhitepaperView() -> Element {
    let t: BiyardIndexTranslate = use_translate();
    rsx! {
        Topbar { active: "index".to_string() }
        main { class: "max-w-5xl mx-auto px-6 py-8",
            div { class: "flex items-center gap-2 text-xs text-foreground-muted mb-2",
                a { href: "/biyard-index", class: "hover:text-brand", {t.wp_breadcrumb_root} }
                span { "/" }
                span { class: "text-foreground-soft", {t.wp_breadcrumb_doc} }
            }
            div { class: "relative",
                div { class: "absolute inset-0 z-10 flex items-center justify-center rounded-2xl bg-bg/60 backdrop-blur-md",
                    div { class: "bg-panel border border-border rounded-2xl px-8 py-6 text-center shadow-xl max-w-sm",
                        h2 { class: "text-xl font-bold mb-2", {t.wp_coming_soon_title} }
                        p { class: "text-sm text-foreground-muted", {t.wp_coming_soon_desc} }
                    }
                }
                article { class: "bg-panel border border-border rounded-2xl p-12 md:p-14 select-none pointer-events-none",
                    div { class: "flex gap-2 mb-3",
                        span { class: "bg-brand-soft text-brand text-[10px] font-bold tracking-wider px-2 py-1 rounded", {t.wp_version_pill} }
                        span { class: "text-[11px] text-foreground-muted self-center", {t.wp_draft_meta} }
                    }
                    h1 { class: "text-2xl md:text-3xl font-bold mb-3", {t.wp_title} }
                    p { class: "text-foreground-soft leading-relaxed mb-6 pb-6 border-b border-border",
                        {t.wp_intro}
                    }
                    Section { title: t.wp_sec1_title.to_string(),
                        p { class: "text-foreground-soft leading-relaxed mb-3",
                            {t.wp_sec1_body_lead}
                            strong { class: "text-foreground", {t.wp_sec1_body_emph} }
                            {t.wp_sec1_body_tail}
                        }
                    }
                    Section { title: t.wp_sec2_title.to_string(),
                        ol { class: "list-decimal list-inside space-y-2 text-foreground-soft text-sm leading-relaxed",
                            li { strong { class: "text-foreground", {t.wp_axis1_name} } {t.wp_axis1_body} }
                            li { strong { class: "text-foreground", {t.wp_axis2_name} } {t.wp_axis2_body} }
                            li { strong { class: "text-foreground", {t.wp_axis3_name} } {t.wp_axis3_body} }
                            li { strong { class: "text-foreground", {t.wp_axis4_name} } {t.wp_axis4_body} }
                            li { strong { class: "text-foreground", {t.wp_axis5_name} } {t.wp_axis5_body} }
                            li { strong { class: "text-foreground", {t.wp_axis6_name} } {t.wp_axis6_body} }
                        }
                    }
                    Section { title: t.wp_sec3_title.to_string(),
                        p { class: "text-foreground-soft mb-3 leading-relaxed",
                            {t.wp_sec3_body}
                        }
                        table { class: "w-full text-sm",
                            thead { tr { class: "text-foreground-muted text-xs uppercase",
                                th { class: "text-left py-2 px-3 bg-panel-muted", {t.wp_th_grade} }
                                th { class: "text-left py-2 px-3 bg-panel-muted", {t.wp_th_percentile} }
                                th { class: "text-left py-2 px-3 bg-panel-muted", {t.wp_th_desc} }
                            } }
                            tbody {
                                GradeRow { letter: "S", pct: t.wp_grade_s_pct, desc: t.wp_grade_s_desc }
                                GradeRow { letter: "A", pct: t.wp_grade_a_pct, desc: t.wp_grade_a_desc }
                                GradeRow { letter: "B", pct: t.wp_grade_b_pct, desc: t.wp_grade_b_desc }
                                GradeRow { letter: "C", pct: t.wp_grade_c_pct, desc: t.wp_grade_c_desc }
                                GradeRow { letter: "D", pct: t.wp_grade_d_pct, desc: t.wp_grade_d_desc }
                            }
                        }
                    }
                    Section { title: t.wp_sec4_title.to_string(),
                        p { class: "text-foreground-soft leading-relaxed",
                            {t.wp_sec4_lead}
                            strong { class: "text-foreground", {t.wp_sec4_emph} }
                            {t.wp_sec4_tail}
                        }
                    }

                    div { class: "flex gap-2 mt-10",
                        a { href: "/biyard-index", class: "bg-brand text-brand-contrast font-bold px-4 py-2 rounded text-sm",
                            {t.wp_back_to_product}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Section(title: String, children: Element) -> Element {
    rsx! {
        section { class: "mb-8",
            h2 { class: "text-xl font-bold mb-3", "{title}" }
            {children}
        }
    }
}

#[component]
fn GradeRow(letter: &'static str, pct: &'static str, desc: &'static str) -> Element {
    rsx! {
        tr { class: "border-b border-border",
            td { class: "py-2 px-3 font-mono font-bold text-brand", "{letter}" }
            td { class: "py-2 px-3 text-foreground-soft", "{pct}" }
            td { class: "py-2 px-3 text-foreground-muted text-xs", "{desc}" }
        }
    }
}
