use dioxus::prelude::*;
use dioxus_translate::Translate;

use crate::common::{Category, StoStatus, use_language, use_translate};
use crate::features::catalog::controllers::{
    get_category_scale_handler, list_planned_stos_handler, list_stos_handler,
};
use crate::features::catalog::{
    CatalogTranslate, CategoryScaleResponse, PlannedStoSummary, StoSummary, TableColumn,
};

#[component]
pub fn HomeView() -> Element {
    let stos = use_loader(move || async move { list_stos_handler().await })?;
    let scale = use_loader(move || async move { get_category_scale_handler().await })?;
    let planned = use_loader(move || async move { list_planned_stos_handler().await })?;
    let snapshot = stos();
    let scale_snap = scale();
    let planned_snap = planned();
    let items = snapshot.items.clone();
    let recent: Vec<_> = items.iter().take(10).cloned().collect();
    let liquidated: Vec<_> = items
        .iter()
        .filter(|s| s.status == StoStatus::Liquidated)
        .take(8)
        .cloned()
        .collect();

    rsx! {
        Topbar { active: "assets".to_string() }
        main { class: "mx-auto w-full max-w-[1400px] px-7 py-7",
            HeroOfferings { items: planned_snap.items }
            RecentStoPanel { items: recent }
            if !liquidated.is_empty() {
                LiquidatedPanel { items: liquidated }
            }
            CategoryScale { scale: scale_snap }
            IdentityBanners {}
        }
    }
}

#[component]
pub fn Topbar(active: String) -> Element {
    let t: CatalogTranslate = use_translate();
    let lang = use_language();
    let lang_now = lang();
    let lang_btn_label = match lang_now {
        crate::common::Language::Ko => "EN",
        _ => "한국어",
    };
    let on_toggle_lang = move |_| {
        // Language::switch() 가 시그널 set + localStorage + cookie 까지 일괄 영구화.
        // signal.set(next) 만 호출하면 다음 navigation/refresh 에서 부트스트랩이 다시
        // 기본값으로 돌아간다.
        let _ = lang_now.switch();
    };
    let item = |key: &str, label: &str, href: &str| {
        let base = "px-3.5 py-2 text-sm font-medium rounded-lg transition-colors hover:bg-panel-muted hover:text-foreground";
        let cls = if active == key {
            format!("{base} bg-brand-soft text-brand")
        } else {
            format!("{base} text-foreground-soft")
        };
        rsx! { a { href: "{href}", class: "{cls}", "{label}" } }
    };
    rsx! {
        header { class: "sticky top-0 z-50 bg-[var(--backdrop)] backdrop-blur-md border-b border-border",
            div { class: "mx-auto max-w-[1400px] px-7 py-3.5 flex items-center gap-8",
                a { class: "flex items-center gap-2.5 font-bold text-base tracking-tight", href: "/", "aria-label": "Biyard",
                    img { class: "w-7 h-7 rounded-md", src: asset!("/assets/biyard-logo.png"), alt: "Biyard" }
                    span { class: "text-foreground", "Biyard" }
                    small { class: "text-foreground-muted font-normal text-xs ml-1", "STO" }
                }
                nav { class: "flex items-center gap-1 flex-1", "aria-label": "{t.main_nav_aria}",
                    { item("assets", t.nav_market_short, "/market") }
                    { item("index", t.nav_index_short, "/biyard-index") }
                    { item("news", t.nav_news_short, "/news") }
                    { item("pricing", t.nav_pricing_short, "/pricing") }
                }
                div { class: "flex items-center gap-2.5",
                    button {
                        class: "bg-panel-muted border border-border rounded-lg px-3 py-2 text-xs font-medium text-foreground-soft hover:border-brand hover:text-brand transition-colors cursor-pointer whitespace-nowrap",
                        "aria-label": "{t.topbar_lang_toggle_aria}",
                        onclick: on_toggle_lang,
                        "{lang_btn_label}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn Panel(children: Element, #[props(default = String::new())] extra_class: String) -> Element {
    let cls = format!("bg-panel border border-border rounded-xl p-6 mb-6 {extra_class}");
    rsx! { section { class: "{cls}", {children} } }
}

#[component]
pub fn PanelHead(
    title: String,
    #[props(default)] more_href: Option<String>,
    #[props(default)] more_label: Option<String>,
) -> Element {
    rsx! {
        div { class: "flex items-center justify-between mb-3 pb-2.5 border-b border-border",
            h2 { class: "text-xl font-bold tracking-tight m-0", {title} }
            if let (Some(href), Some(label)) = (more_href, more_label) {
                a {
                    class: "inline-block bg-brand-soft text-brand border border-brand px-3 py-1.5 rounded-md text-xs font-semibold hover:bg-brand hover:text-brand-contrast transition-colors",
                    href: "{href}",
                    {label}
                }
            }
        }
    }
}

#[component]
fn HeroOfferings(items: Vec<PlannedStoSummary>) -> Element {
    let t: CatalogTranslate = use_translate();
    if items.is_empty() {
        return rsx! {};
    }
    rsx! {
        Panel { extra_class: "mb-4".to_string(),
            div { class: "flex items-center justify-between mb-2.5",
                h2 { class: "text-xl font-bold tracking-tight", {t.hero_title} }
                span { class: "text-[11px] text-foreground-muted bg-panel-muted px-2 py-0.5 rounded",
                    {t.hero_disclaimer_pill}
                }
            }
            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3",
                for p in items.iter() {
                    OfferCard { planned: p.clone() }
                }
            }
            p { class: "text-[11px] text-foreground-muted mt-2.5",
                {t.hero_disclaimer_body}
            }
        }
    }
}

#[component]
fn OfferCard(planned: PlannedStoSummary) -> Element {
    let t: CatalogTranslate = use_translate();
    let lang_now = use_language()();
    let cat_label = planned.category.translate(&lang_now);
    let issuer_label = planned
        .issuer_name
        .clone()
        .unwrap_or_else(|| planned.issuer_id.clone());
    let window_label = planned
        .expected_window
        .clone()
        .unwrap_or_else(|| "—".to_string());
    let amount_label = planned
        .expected_amount
        .map(|a| format_amount_krw(a, &t))
        .unwrap_or_else(|| "—".to_string());
    rsx! {
        div { class: "bg-panel border border-brand rounded-lg px-4 py-3.5 relative overflow-hidden",
            span { class: "inline-block text-[11px] px-2 py-0.5 rounded-[10px] bg-panel-muted text-foreground-soft mb-2",
                {cat_label}
            }
            div { class: "text-sm font-semibold mb-2", {planned.name.clone()} }
            div { class: "text-xs text-foreground-soft mb-1.5",
                {t.issuer} " "
                strong { class: "text-foreground font-semibold", {issuer_label} }
                if let (Some(role), Some(broker)) = (planned.broker_role.as_ref(), planned.broker.as_ref()) {
                    " · "
                    {role.clone()}
                    " "
                    strong { class: "text-foreground font-semibold", {broker.clone()} }
                }
            }
            div { class: "flex justify-between text-xs text-foreground-soft",
                span {
                    {t.subscription_end} " "
                    span { class: "text-foreground font-semibold", {window_label} }
                }
                span {
                    {t.raise_amount} " "
                    span { class: "text-foreground font-semibold", {amount_label} }
                }
            }
        }
    }
}

#[component]
fn RecentStoPanel(items: Vec<StoSummary>) -> Element {
    let t: CatalogTranslate = use_translate();
    rsx! {
        Panel { extra_class: "mb-4".to_string(),
            PanelHead {
                title: t.section_recent_title.to_string(),
                more_href: "/market".to_string(),
                more_label: t.section_more_arrow.to_string(),
            }
            div { class: "overflow-x-auto",
                StoTable { items, show_status: true }
            }
        }
    }
}

#[component]
fn LiquidatedPanel(items: Vec<StoSummary>) -> Element {
    let t: CatalogTranslate = use_translate();
    rsx! {
        Panel { extra_class: "mb-4".to_string(),
            PanelHead {
                title: t.section_liquidated_title.to_string(),
                more_href: "/market?status=LIQUIDATED".to_string(),
                more_label: t.section_more_arrow.to_string(),
            }
            div { class: "overflow-x-auto",
                StoTable { items, show_status: true }
            }
        }
    }
}

#[component]
pub fn StoTable(items: Vec<StoSummary>, show_status: bool) -> Element {
    let lang = use_language();
    let lang_now = lang();
    let th = "text-left px-3 py-2.5 text-xs font-semibold text-foreground-muted bg-panel-muted border-b border-border whitespace-nowrap";
    rsx! {
        table { class: "w-full border-collapse text-[13px]",
            thead {
                tr {
                    th { class: "{th} w-7", "" }
                    th { class: "{th} w-7", "" }
                    th { class: "{th}", { TableColumn::AssetUnderlying.translate(&lang_now) } }
                    th { class: "{th} w-[110px]", { TableColumn::Category.translate(&lang_now) } }
                    th { class: "{th} w-[150px]", { TableColumn::Issuer.translate(&lang_now) } }
                    if show_status {
                        th { class: "{th} w-[110px]", { TableColumn::Status.translate(&lang_now) } }
                    }
                    th { class: "{th} w-[90px] text-right", { TableColumn::Filed.translate(&lang_now) } }
                }
            }
            tbody {
                for s in items.iter() {
                    StoTableRow { item: s.clone(), show_status }
                }
            }
        }
    }
}

#[component]
fn StoTableRow(item: StoSummary, show_status: bool) -> Element {
    let lang = use_language();
    let lang_now = lang();
    let href = format!("/sto/{}", item.sto_id);
    let nav = use_navigator();
    let td = "px-3 py-2.5 border-b border-border align-middle whitespace-nowrap";
    rsx! {
        tr {
            class: "cursor-pointer transition-colors hover:bg-panel-muted",
            onclick: move |_| { nav.push(href.clone()); },
            td { class: "{td} w-7", { item.country.flag() } }
            td { class: "{td} w-7 text-base", { item.category.icon() } }
            td { class: "{td}",
                div { class: "font-semibold mb-0.5", {item.name.clone()} }
                if let Some(u) = &item.underlying {
                    if !u.is_empty() && u.as_str() != item.name.as_str() {
                        div { class: "text-[11px] text-foreground-muted leading-snug", {u.clone()} }
                    }
                }
            }
            td { class: "{td} w-[110px]",
                span { class: "bg-panel-muted text-foreground-soft text-[11px] px-2 py-0.5 rounded whitespace-nowrap",
                    { item.category.translate(&lang_now) }
                }
            }
            td { class: "{td} w-[150px] text-foreground-soft text-xs",
                { item.issuer_name.clone().or_else(|| item.issuer_id.clone()).unwrap_or_default() }
            }
            if show_status {
                td { class: "{td} w-[110px] text-[11px] {item.status.color_class()}",
                    { item.status.translate(&lang_now) }
                }
            }
            td { class: "{td} w-[90px] text-foreground-muted text-xs font-mono text-right",
                { format_issued_date(item.issued_at) }
            }
        }
    }
}

#[component]
fn CategoryScale(scale: CategoryScaleResponse) -> Element {
    let t: CatalogTranslate = use_translate();
    let lang = use_language();
    let lang_now = lang();
    let total_amount = scale.total_amount.max(1);
    let mut rows: Vec<(Category, i64, i64, f64)> = vec![
        (
            Category::Music,
            scale.music_count,
            scale.music_amount,
            scale.music_amount as f64 / total_amount as f64 * 100.0,
        ),
        (
            Category::Art,
            scale.art_count,
            scale.art_amount,
            scale.art_amount as f64 / total_amount as f64 * 100.0,
        ),
        (
            Category::RealEstate,
            scale.real_estate_count,
            scale.real_estate_amount,
            scale.real_estate_amount as f64 / total_amount as f64 * 100.0,
        ),
        (
            Category::Livestock,
            scale.livestock_count,
            scale.livestock_amount,
            scale.livestock_amount as f64 / total_amount as f64 * 100.0,
        ),
    ];
    rows.sort_by(|a, b| b.2.cmp(&a.2));

    let cx = 110.0_f64;
    let cy = 110.0_f64;
    let r_out = 90.0_f64;
    let r_in = 56.0_f64;
    let mut start = -std::f64::consts::FRAC_PI_2;

    let arcs: Vec<(String, &'static str, Category, i64, i64, f64)> = rows
        .iter()
        .filter(|(_, _, amount, _)| *amount > 0)
        .map(|(cat, count, amount, pct)| {
            let ratio = pct / 100.0;
            let end = start + ratio * std::f64::consts::TAU;
            let large = if ratio > 0.5 { 1 } else { 0 };
            let (x1, y1) = (cx + r_out * start.cos(), cy + r_out * start.sin());
            let (x2, y2) = (cx + r_out * end.cos(), cy + r_out * end.sin());
            let (x3, y3) = (cx + r_in * end.cos(), cy + r_in * end.sin());
            let (x4, y4) = (cx + r_in * start.cos(), cy + r_in * start.sin());
            let path = format!(
                "M {x1} {y1} A {r_out} {r_out} 0 {large} 1 {x2} {y2} L {x3} {y3} A {r_in} {r_in} 0 {large} 0 {x4} {y4} Z"
            );
            start = end;
            (path, category_color(*cat), *cat, *count, *amount, *pct)
        })
        .collect();

    rsx! {
        Panel { extra_class: "mb-4".to_string(),
            div { class: "flex items-center justify-between mb-3 pb-2.5 border-b border-border",
                h2 { class: "text-xl font-bold tracking-tight m-0", {t.section_category_scale_title} }
            }
            div { class: "grid grid-cols-1 md:grid-cols-[220px_1fr] gap-7 items-center",
                svg {
                    class: "shrink-0 justify-self-center",
                    view_box: "0 0 220 220",
                    width: "220",
                    height: "220",
                    "aria-label": "{t.donut_chart_aria}",
                    for (path, color, _cat, _count, _amount, _pct) in arcs.iter() {
                        path {
                            d: "{path}",
                            fill: "{color}",
                            stroke: "var(--color-background)",
                            stroke_width: "2",
                        }
                    }
                    text { x: "110", y: "104", text_anchor: "middle", font_size: "11", fill: "var(--color-foreground-muted)",
                        {t.donut_caption_raise}
                    }
                    text { x: "110", y: "126", text_anchor: "middle", font_size: "16", font_weight: "700", fill: "var(--color-foreground)", font_family: "var(--font-mono)",
                        { format_amount_krw(scale.total_amount, &t) }
                    }
                }
                ul { class: "grid gap-1.5 m-0 p-0 list-none",
                    for (_path, color, cat, count, amount, pct) in arcs.iter() {
                        li {
                            class: "grid grid-cols-[14px_1fr_60px_90px_100px] gap-2.5 items-center px-2 py-1.5 rounded text-[13px] hover:bg-panel-muted",
                            span { class: "inline-block w-3.5 h-3.5 rounded-[3px]", style: "background: {color};" }
                            span { class: "whitespace-nowrap", { cat.translate(&lang_now) } }
                            span { class: "text-right font-mono font-semibold text-foreground", "{pct:.1}%" }
                            span { class: "text-right font-mono text-xs text-foreground-soft", { format_amount_krw(*amount, &t) } }
                            span { class: "text-right text-[11px] font-mono text-foreground-muted", "{count} " {t.unit_count} }
                        }
                    }
                }
            }
            p { class: "text-[11px] text-foreground-muted mt-3 leading-relaxed",
                {t.section_category_scale_note}
            }
        }
    }
}

pub fn format_amount_krw(amount: i64, t: &CatalogTranslate) -> String {
    if amount >= 1_000_000_000_000 {
        format!("{:.1}조 {}", amount as f64 / 1e12, t.unit_won)
    } else if amount >= 100_000_000 {
        format!("{:.1}억 {}", amount as f64 / 1e8, t.unit_won)
    } else if amount >= 10_000 {
        format!("{:.0}만 {}", amount as f64 / 1e4, t.unit_won)
    } else if amount > 0 {
        format!("{} {}", amount, t.unit_won)
    } else {
        "—".to_string()
    }
}

fn category_color(c: Category) -> &'static str {
    use Category::*;
    match c {
        Music => "#00e5a0",
        Art => "#5eb0ff",
        RealEstate => "#ffb547",
        Livestock => "#ff8a4c",
        Unknown => "#4f5a6e",
    }
}

#[component]
fn IdentityBanners() -> Element {
    let t: CatalogTranslate = use_translate();
    rsx! {
        section { class: "mb-4",
            a {
                class: "block bg-panel border border-brand rounded-lg px-5 py-4 flex flex-col gap-2 transition-colors hover:bg-panel-muted",
                href: "/biyard-index",
                span { class: "inline-block bg-brand-soft text-brand text-[10px] font-bold tracking-wider px-1.5 py-0.5 rounded self-start", {t.identity_index_pill} }
                div { class: "text-base font-bold", {t.identity_index_title} }
                p { class: "text-xs text-foreground-soft leading-relaxed m-0",
                    {t.identity_index_body}
                }
                span { class: "text-brand text-xs font-semibold", {t.identity_index_cta} }
            }
        }
    }
}

pub fn format_issued_date(epoch_ms: i64) -> String {
    use chrono::TimeZone;
    chrono::Utc
        .timestamp_millis_opt(epoch_ms)
        .single()
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "—".to_string())
}
