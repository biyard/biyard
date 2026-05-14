use dioxus::prelude::*;

use crate::common::{Category, Country, StoStatus};
use crate::features::catalog::controllers::{get_category_scale_handler, list_stos_handler};
use crate::features::catalog::{CategoryScaleResponse, StoSummary};

#[component]
pub fn HomeView() -> Element {
    let stos = use_loader(move || async move { list_stos_handler().await })?;
    let scale = use_loader(move || async move { get_category_scale_handler().await })?;
    let snapshot = stos();
    let scale_snap = scale();
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
            HeroOfferings {}
            RecentStoPanel { items: recent }
            if !liquidated.is_empty() {
                LiquidatedPanel { items: liquidated }
            }
            CategoryScale { scale: scale_snap.clone() }
            IdentityBanners {}
        }
    }
}

#[component]
pub fn Topbar(active: String) -> Element {
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
                nav { class: "flex items-center gap-1 flex-1", "aria-label": "주요 화면",
                    { item("assets", "STO 시장", "/market") }
                    { item("index", "평가지표", "/biyard-index") }
                    { item("launchpad", "런치패드", "/launchpad") }
                    { item("news", "뉴스", "/news") }
                    { item("pricing", "가격", "/pricing") }
                }
                div { class: "flex items-center gap-2.5",
                    input {
                        class: "w-[280px] bg-panel-muted border border-border rounded-full px-3.5 py-2 text-foreground text-[13px]",
                        r#type: "search",
                        placeholder: "STO 검색...",
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
fn HeroOfferings() -> Element {
    rsx! {
        Panel { extra_class: "mb-4".to_string(),
            div { class: "flex items-center justify-between mb-2.5",
                h2 { class: "text-xl font-bold tracking-tight", "공모 진행·예정" }
                span { class: "text-[11px] text-foreground-muted bg-panel-muted px-2 py-0.5 rounded",
                    "증권사 제공 정보 · 예시"
                }
            }
            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3",
                OfferCard { cat: "🎨 미술", title: "유영국 — Work (1984)",
                    issuer: "투게더아트", role: "인수", broker: "가람증권",
                    end: "~ 05.18", amount: "6.6억" }
                OfferCard { cat: "🐂 한우", title: "뱅카우 한우 제2-1호",
                    issuer: "스탁키퍼", role: "계좌관리", broker: "해솔투자증권",
                    end: "~ 05.22", amount: "4.8억" }
                OfferCard { cat: "🎵 음악", title: "NCT DREAM — ANL 후속",
                    issuer: "뮤직카우", role: "중개", broker: "다온증권",
                    end: "~ 05.25", amount: "3.2억" }
            }
            p { class: "text-[11px] text-foreground-muted mt-2.5",
                "※ 위 카드는 데모용 예시 데이터입니다. 실제 서비스에서는 발행사·증권사가 직접 제공한 공모 정보를 표시하며, Biyard 는 추천·평가하지 않습니다."
            }
        }
    }
}

#[component]
fn OfferCard(
    cat: String,
    title: String,
    issuer: String,
    role: String,
    broker: String,
    end: String,
    amount: String,
) -> Element {
    rsx! {
        div { class: "bg-panel border border-brand rounded-lg px-4 py-3.5 relative overflow-hidden",
            span { class: "inline-block text-[11px] px-2 py-0.5 rounded-[10px] bg-panel-muted text-foreground-soft mb-2",
                {cat}
            }
            div { class: "text-sm font-semibold mb-2", {title} }
            div { class: "text-xs text-foreground-soft mb-1.5",
                "발행 "
                strong { class: "text-foreground font-semibold", {issuer} }
                " · "
                {role}
                " "
                strong { class: "text-foreground font-semibold", {broker} }
            }
            div { class: "flex justify-between text-xs text-foreground-soft",
                span {
                    "청약 "
                    span { class: "text-foreground font-semibold", {end} }
                }
                span {
                    "모집 "
                    span { class: "text-foreground font-semibold", {amount} }
                }
            }
        }
    }
}

#[component]
fn RecentStoPanel(items: Vec<StoSummary>) -> Element {
    rsx! {
        Panel { extra_class: "mb-4".to_string(),
            PanelHead {
                title: "최근 발행 STO".to_string(),
                more_href: "/market".to_string(),
                more_label: "전체 보기 →".to_string(),
            }
            div { class: "overflow-x-auto",
                StoTable { items, show_status: true }
            }
        }
    }
}

#[component]
fn LiquidatedPanel(items: Vec<StoSummary>) -> Element {
    rsx! {
        Panel { extra_class: "mb-4".to_string(),
            PanelHead {
                title: "최근 청산·분배 완료".to_string(),
                more_href: "/market?status=LIQUIDATED".to_string(),
                more_label: "전체 보기 →".to_string(),
            }
            div { class: "overflow-x-auto",
                StoTable { items, show_status: true }
            }
        }
    }
}

#[component]
pub fn StoTable(items: Vec<StoSummary>, show_status: bool) -> Element {
    let th = "text-left px-3 py-2.5 text-xs font-semibold text-foreground-muted bg-panel-muted border-b border-border whitespace-nowrap";
    rsx! {
        table { class: "w-full border-collapse text-[13px]",
            thead {
                tr {
                    th { class: "{th} w-7", "" }
                    th { class: "{th} w-7", "" }
                    th { class: "{th}", "자산명 / 기초자산" }
                    th { class: "{th} w-[110px]", "카테고리" }
                    th { class: "{th} w-[150px]", "발행사" }
                    if show_status {
                        th { class: "{th} w-[110px]", "상태" }
                    }
                    th { class: "{th} w-[90px] text-right", "발행" }
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
    let href = format!("/sto/{}", item.sto_id);
    let nav = use_navigator();
    let td = "px-3 py-2.5 border-b border-border align-middle whitespace-nowrap";
    rsx! {
        tr {
            class: "cursor-pointer transition-colors hover:bg-panel-muted",
            onclick: move |_| { nav.push(href.clone()); },
            td { class: "{td} w-7", { flag_for(item.country) } }
            td { class: "{td} w-7 text-base", { category_icon(item.category) } }
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
                    { category_label(item.category) }
                }
            }
            td { class: "{td} w-[150px] text-foreground-soft text-xs",
                {item.issuer_id.clone().unwrap_or_default()}
            }
            if show_status {
                td { class: "{td} w-[110px] text-[11px] {status_color_class(item.status)}",
                    { status_label(item.status) }
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
                h2 { class: "text-xl font-bold tracking-tight m-0", "카테고리별 투자 규모" }
            }
            div { class: "grid grid-cols-1 md:grid-cols-[220px_1fr] gap-7 items-center",
                svg {
                    class: "shrink-0 justify-self-center",
                    view_box: "0 0 220 220",
                    width: "220",
                    height: "220",
                    "aria-label": "카테고리별 파이 차트",
                    for (path, color, _cat, _count, _amount, _pct) in arcs.iter() {
                        path {
                            d: "{path}",
                            fill: "{color}",
                            stroke: "var(--color-background)",
                            stroke_width: "2",
                        }
                    }
                    text { x: "110", y: "104", text_anchor: "middle", font_size: "11", fill: "var(--color-foreground-muted)",
                        "누적 모집액"
                    }
                    text { x: "110", y: "126", text_anchor: "middle", font_size: "16", font_weight: "700", fill: "var(--color-foreground)", font_family: "var(--font-mono)",
                        { format_amount_krw(scale.total_amount) }
                    }
                }
                ul { class: "grid gap-1.5 m-0 p-0 list-none",
                    for (_path, color, cat, count, amount, pct) in arcs.iter() {
                        li {
                            class: "grid grid-cols-[14px_1fr_60px_90px_100px] gap-2.5 items-center px-2 py-1.5 rounded text-[13px] hover:bg-panel-muted",
                            span { class: "inline-block w-3.5 h-3.5 rounded-[3px]", style: "background: {color};" }
                            span { class: "whitespace-nowrap", { category_label(*cat) } }
                            span { class: "text-right font-mono font-semibold text-foreground", "{pct:.1}%" }
                            span { class: "text-right font-mono text-xs text-foreground-soft", { format_amount_krw(*amount) } }
                            span { class: "text-right text-[11px] font-mono text-foreground-muted", "{count} 건" }
                        }
                    }
                }
            }
            p { class: "text-[11px] text-foreground-muted mt-3 leading-relaxed",
                "누적 모집액 기준 (DART 공시 발행가). 모집액 미공시 STO 는 건수에는 포함되나 합산에서는 제외됨."
            }
        }
    }
}

pub fn format_amount_krw(amount: i64) -> String {
    if amount >= 1_000_000_000_000 {
        format!("{:.1}조 원", amount as f64 / 1e12)
    } else if amount >= 100_000_000 {
        format!("{:.1}억 원", amount as f64 / 1e8)
    } else if amount >= 10_000 {
        format!("{:.0}만 원", amount as f64 / 1e4)
    } else if amount > 0 {
        format!("{} 원", amount)
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
    rsx! {
        section { class: "grid grid-cols-1 md:grid-cols-2 gap-3 mb-4",
            a {
                class: "bg-panel border border-brand rounded-lg px-5 py-4 flex flex-col gap-2 transition-colors hover:bg-panel-muted",
                href: "/biyard-index",
                span { class: "inline-block bg-brand-soft text-brand text-[10px] font-bold tracking-wider px-1.5 py-0.5 rounded self-start", "BIYARD INDEX" }
                div { class: "text-base font-bold", "Web3 기반 STO 평가지표" }
                p { class: "text-xs text-foreground-soft leading-relaxed m-0",
                    "온체인 발행 무결성·컨트랙트 보안·지갑 분포·거버넌스 등 기존 신용평가가 다루지 못하는 Web3 신뢰 신호를 6개 축으로 환산해 등급을 부여합니다."
                }
                span { class: "text-brand text-xs font-semibold", "백서 보기 →" }
            }
            a {
                class: "bg-panel border border-brand rounded-lg px-5 py-4 flex flex-col gap-2 transition-colors hover:bg-panel-muted",
                href: "/launchpad",
                span { class: "inline-block bg-brand-soft text-brand text-[10px] font-bold tracking-wider px-1.5 py-0.5 rounded self-start", "BIYARD LAUNCHPAD" }
                div { class: "text-base font-bold", "브랜드 토큰 PaaS" }
                p { class: "text-xs text-foreground-soft leading-relaxed m-0",
                    "STO 와 결합 가능한 유틸리티 토큰 발행 인프라. 발행사·증권사가 자체 브랜드 토큰을 운영할 수 있는 PaaS 서비스를 제공합니다."
                }
                span { class: "text-brand text-xs font-semibold", "자세히 보기 →" }
            }
        }
    }
}

pub fn category_icon(c: Category) -> &'static str {
    use Category::*;
    match c {
        RealEstate => "🏢",
        Art => "🎨",
        Music => "🎵",
        Livestock => "🐂",
        Unknown => "·",
    }
}

pub fn category_label(c: Category) -> &'static str {
    use Category::*;
    match c {
        RealEstate => "🏢 부동산",
        Art => "🎨 미술품",
        Music => "🎵 음악 IP",
        Livestock => "🐂 한우·축산",
        Unknown => "기타",
    }
}

pub fn flag_for(c: Country) -> &'static str {
    use Country::*;
    match c {
        Kr => "🇰🇷",
        Us => "🇺🇸",
        Sg => "🇸🇬",
        Eu => "🇪🇺",
        Other | Unknown => "🌍",
    }
}

pub fn country_display(c: Country) -> &'static str {
    use Country::*;
    match c {
        Kr => "🇰🇷 한국",
        Us => "🇺🇸 미국",
        Sg => "🇸🇬 싱가포르",
        Eu => "🇪🇺 유럽",
        Other | Unknown => "🌍 해외",
    }
}

pub fn status_label(s: StoStatus) -> &'static str {
    use StoStatus::*;
    match s {
        Issued => "발행 완료",
        Filed => "공모 진행",
        Withdrawn => "철회",
        Liquidated => "청산 완료",
        Unknown => "—",
    }
}

pub fn status_color_class(s: StoStatus) -> &'static str {
    use StoStatus::*;
    match s {
        Issued | Liquidated | Filed => "text-brand",
        Withdrawn => "text-warning",
        Unknown => "text-foreground-muted",
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
