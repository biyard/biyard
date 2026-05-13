use std::collections::BTreeMap;

use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::features::catalog::controllers::list_stos;
use crate::features::catalog::{CatalogTranslate, StoSummary};

#[component]
pub fn HomeView() -> Element {
    let t: CatalogTranslate = use_translate();
    let data = use_server_future(|| async { list_stos().await })?;
    let resp = data.read();
    let resp_ref = resp.as_ref();

    rsx! {
        Topbar { active: "home".to_string() }
        main { class: "max-w-7xl mx-auto px-6 py-8",
            HeroOfferings {}
            match resp_ref {
                Some(Ok(r)) => rsx! {
                    Panel { title: t.section_recent.to_string(), more_href: Some("/market".to_string()),
                        StoTable {
                            items: r.items.iter().take(15).cloned().collect::<Vec<_>>(),
                            show_status: true,
                        }
                    }
                    CategoryScale { items: r.items.clone() }
                    IdentityBanners {}
                },
                Some(Err(e)) => rsx! {
                    div { class: "text-danger", "{t.load_failed}: {e}" }
                },
                None => rsx! {
                    div { class: "text-foreground-muted", "{t.loading}" }
                },
            }
        }
    }
}

#[component]
pub fn Topbar(active: String) -> Element {
    let t: CatalogTranslate = use_translate();
    let item = |key: &str, label: &str, href: &str| {
        let cls = if active == key {
            "px-3.5 py-2 rounded-md text-sm font-medium bg-panel-muted text-foreground"
        } else {
            "px-3.5 py-2 rounded-md text-sm font-medium text-foreground-soft hover:bg-panel-muted hover:text-foreground"
        };
        rsx! {
            a { href: "{href}", class: "{cls}", "{label}" }
        }
    };
    rsx! {
        header { class: "sticky top-0 z-10 border-b border-border bg-panel/85 backdrop-blur",
            div { class: "max-w-[1400px] mx-auto px-7 py-3.5 flex items-center gap-8",
                a { href: "/", class: "flex items-center gap-2.5 font-bold",
                    img {
                        src: asset!("/assets/biyard-logo.png"),
                        alt: "Biyard",
                        class: "w-7 h-7 rounded-md",
                    }
                    span { class: "text-foreground", "Biyard" }
                    small { class: "text-foreground-muted font-normal text-xs ml-1", "STO" }
                }
                nav { class: "flex items-center gap-1 flex-1",
                    { item("home", t.nav_home, "/") }
                    { item("assets", t.nav_market, "/market") }
                    { item("issuers", t.nav_issuers, "/issuers") }
                    { item("index", t.nav_index, "/index") }
                    { item("launchpad", t.nav_launchpad, "/launchpad") }
                    { item("news", t.nav_news, "/news") }
                    { item("pricing", t.nav_pricing, "/pricing") }
                }
                div { class: "flex items-center gap-2.5",
                    input {
                        r#type: "search",
                        placeholder: "STO 검색...",
                        class: "w-[280px] bg-panel-muted border border-border rounded-full px-3.5 py-2 text-sm text-foreground placeholder:text-foreground-muted focus:outline-none focus:border-brand",
                    }
                }
            }
        }
    }
}

#[component]
pub fn Panel(title: String, more_href: Option<String>, children: Element) -> Element {
    let t: CatalogTranslate = use_translate();
    rsx! {
        section { class: "bg-panel border border-border rounded-2xl p-5 mb-5",
            div { class: "flex justify-between items-center mb-3 pb-2 border-b border-border",
                h2 { class: "text-base font-bold", "{title}" }
                if let Some(href) = more_href {
                    a { href: "{href}",
                        class: "text-xs px-3 py-1.5 rounded-sm font-semibold text-brand bg-brand-soft border border-brand hover:bg-brand-soft",
                        "{t.section_more}"
                    }
                }
            }
            {children}
        }
    }
}

#[component]
pub fn StoTable(items: Vec<StoSummary>, show_status: bool) -> Element {
    let t: CatalogTranslate = use_translate();
    rsx! {
        div { class: "overflow-x-auto",
            table { class: "w-full text-sm border-collapse",
                thead {
                    tr { class: "text-left text-[11px] text-foreground-muted uppercase tracking-wide",
                        th { class: "px-3 py-2.5 bg-panel-muted border-b border-border w-7", "" }
                        th { class: "px-3 py-2.5 bg-panel-muted border-b border-border w-7", "" }
                        th { class: "px-3 py-2.5 bg-panel-muted border-b border-border", "{t.th_asset}" }
                        th { class: "px-3 py-2.5 bg-panel-muted border-b border-border w-32", "{t.th_category}" }
                        th { class: "px-3 py-2.5 bg-panel-muted border-b border-border w-36", "{t.th_issuer}" }
                        if show_status {
                            th { class: "px-3 py-2.5 bg-panel-muted border-b border-border w-28", "{t.th_status}" }
                        }
                        th { class: "px-3 py-2.5 bg-panel-muted border-b border-border w-28 text-right", "{t.th_issued_at}" }
                    }
                }
                tbody {
                    for s in items.iter() {
                        tr { class: "border-b border-border/40 hover:bg-panel-muted/60 transition-colors",
                            td { class: "px-3 py-3 w-7 text-base align-middle", { flag_for(&s.region) } }
                            td { class: "px-3 py-3 w-7 text-base align-middle", { category_icon(&s.category) } }
                            td { class: "px-3 py-3",
                                a { href: "/sto/{s.sto_id}", class: "block hover:text-brand",
                                    div { class: "font-semibold text-foreground text-sm leading-tight", {s.name.clone()} }
                                    if let Some(underlying) = &s.underlying {
                                        if !underlying.is_empty() && Some(underlying.as_str()) != Some(s.name.as_str()) {
                                            div { class: "text-[11px] text-foreground-muted mt-0.5 leading-snug", {underlying.clone()} }
                                        }
                                    } else if let Some(artist) = &s.artist {
                                        div { class: "text-[11px] text-foreground-muted mt-0.5 leading-snug", {artist.clone()} }
                                    }
                                }
                            }
                            td { class: "px-3 py-3 align-middle",
                                span { class: "inline-block px-2 py-0.5 text-[11px] rounded bg-panel-muted text-foreground-soft whitespace-nowrap",
                                    { category_label(&s.category) }
                                }
                            }
                            td { class: "px-3 py-3 text-foreground-soft text-xs align-middle", {s.issuer_id.clone().unwrap_or_default()} }
                            if show_status {
                                td { class: "px-3 py-3 text-xs align-middle", { status_pill(&s.status, &t) } }
                            }
                            td { class: "px-3 py-3 text-[11px] font-mono text-foreground-muted text-right align-middle", {crate::features::catalog::views::format_date_ms(s.issued_at)} }
                        }
                    }
                }
            }
        }
    }
}

pub fn category_icon(c: &str) -> &'static str {
    match c {
        "real_estate" => "🏢",
        "art" => "🎨",
        "music" => "🎵",
        "livestock" => "🐂",
        "luxury" => "💎",
        "infra" => "⚡",
        "content" => "🎬",
        _ => "·",
    }
}

#[component]
fn HeroOfferings() -> Element {
    let t: CatalogTranslate = use_translate();
    rsx! {
        section { class: "bg-panel border border-border rounded-2xl p-5 mb-5",
            div { class: "flex justify-between items-center mb-3",
                h2 { class: "text-base font-bold", "{t.section_offerings}" }
                span { class: "text-xs text-foreground-muted bg-panel-muted px-2 py-1 rounded", "{t.offerings_label}" }
            }
            div { class: "grid grid-cols-1 md:grid-cols-3 gap-3",
                OfferingCard {
                    cat: "🎨 미술",
                    title: "유영국 — Work (1984)",
                    issuer: "투게더아트",
                    underwriter: "가람증권",
                    end: "~ 05.18",
                    amount: "6.6억",
                }
                OfferingCard {
                    cat: "🐂 한우",
                    title: "뱅카우 한우 제2-1호",
                    issuer: "스탁키퍼",
                    underwriter: "해솔투자증권",
                    end: "~ 05.22",
                    amount: "4.8억",
                }
                OfferingCard {
                    cat: "🎵 음악",
                    title: "NCT DREAM — ANL 후속",
                    issuer: "뮤직카우",
                    underwriter: "다온증권",
                    end: "~ 05.25",
                    amount: "3.2억",
                }
            }
            div { class: "mt-3 text-[11px] text-foreground-muted",
                "※ {t.offerings_disclaimer}"
            }
        }
    }
}

#[component]
fn OfferingCard(
    cat: &'static str,
    title: &'static str,
    issuer: &'static str,
    underwriter: &'static str,
    end: &'static str,
    amount: &'static str,
) -> Element {
    rsx! {
        article { class: "relative bg-panel border border-brand rounded-sm p-4",
            span { class: "absolute top-2 right-2 text-[9px] font-bold text-foreground-muted bg-panel-muted px-1.5 py-0.5 rounded", "AD" }
            span { class: "inline-block text-[11px] px-2 py-0.5 rounded-full bg-panel-muted text-foreground-soft mb-2", "{cat}" }
            div { class: "text-sm font-bold mb-1 leading-tight", "{title}" }
            div { class: "text-xs text-foreground-muted mb-3",
                "발행 "
                span { class: "text-foreground-soft font-semibold", "{issuer}" }
                " · 인수 "
                span { class: "text-foreground-soft font-semibold", "{underwriter}" }
            }
            div { class: "flex justify-between text-[11px] text-foreground-muted pt-2 border-t border-dashed border-border",
                span {
                    "청약 "
                    span { class: "text-foreground-soft font-semibold", "{end}" }
                }
                span {
                    "모집 "
                    span { class: "text-foreground-soft font-semibold", "{amount}" }
                }
            }
        }
    }
}

#[component]
fn CategoryScale(items: Vec<StoSummary>) -> Element {
    // 카테고리별 발행 건수 집계 (모집액은 StoSummary 에 없으므로 건수만 표시)
    let mut counts: BTreeMap<&'static str, (i32, &'static str)> = BTreeMap::new();
    for s in items.iter() {
        let cat_key = match s.category.as_str() {
            "real_estate" => "🏢 부동산",
            "art" => "🎨 미술품",
            "music" => "🎵 음악 IP",
            "livestock" => "🐄 한우·축산",
            "luxury" => "💎 명품",
            "infra" => "⚡ 인프라",
            "content" => "🎬 콘텐츠",
            _ => "기타",
        };
        let entry = counts.entry(cat_key).or_insert((0, cat_key));
        entry.0 += 1;
    }
    let mut rows: Vec<(i32, &'static str)> = counts.into_iter().map(|(_, v)| v).collect();
    rows.sort_by(|a, b| b.0.cmp(&a.0));
    let max = rows.iter().map(|(c, _)| *c).max().unwrap_or(1);

    rsx! {
        Panel { title: "카테고리별 발행 건수".to_string(),
            div { class: "space-y-2",
                for (cnt, label) in rows.iter() {
                    div { class: "grid grid-cols-[140px_1fr_60px] gap-3 items-center text-sm",
                        span { class: "text-foreground-soft", "{label}" }
                        div { class: "h-2 bg-panel-muted rounded-full overflow-hidden",
                            div {
                                class: "h-full bg-gradient-to-r from-brand to-blue-400 rounded-full",
                                style: "width: {(*cnt as f64 / max as f64 * 100.0) as i32}%",
                            }
                        }
                        span { class: "text-xs font-mono text-foreground-muted text-right", "{cnt} 건" }
                    }
                }
            }
            div { class: "text-[11px] text-foreground-muted mt-3",
                "모집액 집계는 후속 작업. 현재는 발행 건수만 표시."
            }
        }
    }
}

#[component]
fn IdentityBanners() -> Element {
    rsx! {
        section { class: "grid grid-cols-1 md:grid-cols-2 gap-3 mb-5",
            a { href: "/index", class: "bg-panel border border-brand rounded-2xl p-5 block hover:border-brand transition-colors",
                span { class: "text-[11px] font-bold text-brand tracking-wide", "BIYARD INDEX" }
                div { class: "text-base font-bold mt-2 mb-2", "Web3 기반 STO 평가지표" }
                p { class: "text-xs text-foreground-muted leading-relaxed",
                    "온체인 발행 무결성·컨트랙트 보안·지갑 분포·거버넌스 등 기존 신용평가가 다루지 못하는 Web3 신뢰 신호를 6개 축으로 환산해 등급을 부여합니다."
                }
                span { class: "text-xs text-brand mt-2 inline-block font-semibold", "백서 보기 →" }
            }
            a { href: "/launchpad", class: "bg-panel border border-brand rounded-2xl p-5 block hover:border-brand transition-colors",
                span { class: "text-[11px] font-bold text-brand tracking-wide", "BIYARD LAUNCHPAD" }
                div { class: "text-base font-bold mt-2 mb-2", "브랜드 토큰 PaaS" }
                p { class: "text-xs text-foreground-muted leading-relaxed",
                    "STO 와 결합 가능한 유틸리티 토큰 발행 인프라. 발행사·증권사가 자체 브랜드 토큰을 운영할 수 있는 PaaS 서비스를 제공합니다."
                }
                span { class: "text-xs text-brand mt-2 inline-block font-semibold", "자세히 보기 →" }
            }
        }
    }
}

pub fn category_label(c: &str) -> &'static str {
    match c {
        "real_estate" => "🏢 부동산",
        "art" => "🎨 미술품",
        "music" => "🎵 음악 IP",
        "livestock" => "🐄 한우·축산",
        "luxury" => "💎 명품",
        "infra" => "⚡ 인프라",
        "content" => "🎬 콘텐츠",
        _ => "기타",
    }
}

pub fn flag_for(region: &str) -> &'static str {
    match region {
        "KR" => "🇰🇷",
        _ => "🌍",
    }
}

pub fn status_label(status: &str, t: &CatalogTranslate) -> &'static str {
    match status {
        "발행완료" | "발행 완료" | "청산 완료" => t.status_issued,
        "신고중" | "증권신고서 제출" => t.status_filed,
        "철회" => t.status_withdrawn,
        _ => "",
    }
}

pub fn status_pill(status: &str, t: &CatalogTranslate) -> Element {
    let (bg, fg) = match status {
        "발행완료" | "발행 완료" | "청산 완료" => ("bg-brand-soft", "text-brand"),
        "신고중" | "증권신고서 제출" => ("bg-warning/15", "text-warning"),
        "철회" => ("bg-danger/15", "text-danger"),
        _ => ("bg-panel-muted", "text-foreground-soft"),
    };
    let label = status_label(status, t);
    let display = if label.is_empty() { status } else { label };
    rsx! {
        span { class: "text-[10px] px-2 py-0.5 rounded font-bold {bg} {fg}", "{display}" }
    }
}
