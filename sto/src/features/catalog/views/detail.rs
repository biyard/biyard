use dioxus::prelude::*;

use crate::features::catalog::controllers::get_sto_handler;
use crate::features::catalog::views::{Panel, PanelHead, Topbar, category_label, country_display, status_label};
use crate::features::catalog::{FilingSummary, StoDetailResponse};

#[component]
pub fn DetailView(sto_id: String) -> Element {
    let id_for_fetch = sto_id.clone();
    let data = use_loader(move || {
        let id = id_for_fetch.clone();
        async move { get_sto_handler(id).await }
    })?;
    let sto = data();
    rsx! {
        Topbar { active: "assets".to_string() }
        main { class: "mx-auto w-full max-w-[1400px] px-7 py-7",
            DetailBody { sto: sto.clone() }
        }
    }
}

#[component]
fn DetailBody(sto: StoDetailResponse) -> Element {
    rsx! {
        section { class: "bg-panel border border-border rounded-xl p-7 mb-[18px]",
            div { class: "flex gap-2 flex-wrap mb-3",
                CatTag { label: category_label(sto.category).to_string() }
                CatTag { label: country_display(sto.country).to_string() }
                if let Some(st) = &sto.security_type {
                    CatTag { label: st.clone() }
                }
                CatTag { label: status_label(sto.status).to_string() }
            }
            h1 { class: "text-2xl font-bold tracking-tight mb-2", {sto.name.clone()} }
            if let Some(u) = &sto.underlying {
                p { class: "text-sm text-foreground-muted m-0", {u.clone()} }
            }
        }

        div { class: "grid grid-cols-1 lg:grid-cols-[2fr_1fr] gap-[18px]",
            div {
                DetailPanel { title: "발행 개요".to_string(),
                    DetailGrid { entries: vec![
                        ("자산명".to_string(), sto.name.clone()),
                        ("기초자산".to_string(), sto.underlying.clone().unwrap_or_else(|| "—".to_string())),
                        ("증권 유형".to_string(), sto.security_type.clone().unwrap_or_else(|| "—".to_string())),
                        ("신고일".to_string(), format_date_ms(sto.issued_at)),
                        ("모집 상태".to_string(), status_label(sto.status).to_string()),
                        ("작가".to_string(), sto.artist.clone().unwrap_or_else(|| "—".to_string())),
                        ("권리 유형".to_string(), sto.rights_category.clone().unwrap_or_else(|| "—".to_string())),
                        ("신탁계약 번호".to_string(), sto.trust_no.clone().unwrap_or_else(|| "—".to_string())),
                        ("제작연도".to_string(), sto.year.clone().unwrap_or_else(|| "—".to_string())),
                    ] }
                }

                if let Some(o) = &sto.offering {
                    DetailPanel { title: "공모 조건".to_string(),
                        DetailGrid { entries: vec![
                            ("공모총액".to_string(), o.amount.map(|a| format_amount(a, &o.currency)).unwrap_or_else(|| "—".to_string())),
                            ("공모가".to_string(), o.unit_price.map(|p| format!("{} 원", number_format(p))).unwrap_or_else(|| "—".to_string())),
                            ("발행 수량".to_string(), o.total_units.map(|n| format!("{} 좌", number_format(n))).unwrap_or_else(|| "—".to_string())),
                            ("청약 기간".to_string(), format!(
                                "{} ~ {}",
                                o.subscription_start.clone().unwrap_or_else(|| "—".to_string()),
                                o.subscription_end.clone().unwrap_or_else(|| "—".to_string())
                            )),
                        ] }
                    }
                }

                if let Some(is_) = &sto.issuance_structure {
                    DetailPanel { title: "공모 구조".to_string(),
                        DetailGrid { entries: vec![
                            ("발행사".to_string(), is_.issuer.clone().unwrap_or_else(|| "—".to_string())),
                            ("신탁업자".to_string(), is_.trustee.clone().unwrap_or_else(|| "—".to_string())),
                            ("역할".to_string(), is_.trustee_role.clone().unwrap_or_else(|| "—".to_string())),
                            ("주관 증권사".to_string(), is_.underwriter.clone().unwrap_or_else(|| "—".to_string())),
                            ("계좌관리기관".to_string(), is_.custody.clone().unwrap_or_else(|| "—".to_string())),
                        ] }
                    }
                }
            }

            div {
                DetailPanel { title: "공식 페이지".to_string(),
                    if let Some(url) = &sto.external_url {
                        a {
                            class: "block text-brand text-sm break-all",
                            href: "{url}",
                            target: "_blank",
                            "원문 보기 ↗"
                        }
                        div { class: "text-[11px] text-foreground-muted mt-1.5 break-all", {url.clone()} }
                    } else {
                        div { class: "text-xs text-foreground-muted", "원본 링크가 없습니다." }
                    }
                }

                if !sto.sources.is_empty() {
                    DetailPanel { title: "근거 자료".to_string(),
                        ul { class: "list-none m-0 p-0 flex flex-col gap-1.5",
                            for s in sto.sources.iter() {
                                li { class: "flex items-center gap-2.5 text-[13px]",
                                    SourceBadge { label: s.src.clone() }
                                    span { class: "text-xs text-foreground-muted", "{s.label}" }
                                }
                            }
                        }
                    }
                }

                if let Some(iid) = &sto.issuer_id {
                    DetailPanel { title: "발행사".to_string(),
                        a { class: "text-brand text-sm", href: "/issuers/{iid}", "{iid} →" }
                    }
                }
            }
        }

        if !sto.filings.is_empty() {
            Panel { extra_class: "mt-[18px]".to_string(),
                PanelHead { title: format!("공시 ({})", sto.filings.len()) }
                div { class: "flex flex-col gap-3",
                    for f in sto.filings.iter() {
                        FilingCard { filing: f.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn CatTag(label: String) -> Element {
    rsx! {
        span { class: "inline-flex items-center gap-1 px-2.5 py-1 bg-panel-strong rounded-full text-xs leading-none h-6 text-foreground-soft font-medium whitespace-nowrap",
            {label}
        }
    }
}

#[component]
fn SourceBadge(label: String) -> Element {
    rsx! {
        span { class: "inline-block text-[11px] font-bold tracking-wider px-2 py-0.5 rounded bg-brand-soft text-brand border border-brand",
            {label}
        }
    }
}

#[component]
fn DetailPanel(title: String, children: Element) -> Element {
    rsx! {
        section { class: "bg-panel border border-border rounded-xl p-6 mb-[18px]",
            h3 { class: "m-0 mb-3.5 text-[13px] text-foreground-soft uppercase tracking-wider font-semibold", {title} }
            {children}
        }
    }
}

#[component]
fn DetailGrid(entries: Vec<(String, String)>) -> Element {
    rsx! {
        div { class: "grid grid-cols-1 md:grid-cols-2 gap-x-8",
            for (k, v) in entries.iter() {
                div { class: "grid grid-cols-[130px_1fr] gap-3 py-2.5 border-b border-border text-[13px]",
                    div { class: "text-foreground-muted text-xs", "{k}" }
                    div { class: "text-foreground font-medium break-words", "{v}" }
                }
            }
        }
    }
}

#[component]
fn FilingCard(filing: FilingSummary) -> Element {
    rsx! {
        article { class: "bg-panel-muted border border-border rounded-lg p-3.5",
            div { class: "flex gap-2 items-center flex-wrap mb-1.5",
                SourceBadge { label: filing.filing_source.to_string() }
                if let Some(t) = &filing.filing_type {
                    span { class: "text-[11px] px-1.5 py-0.5 rounded bg-panel-strong text-foreground-soft",
                        "{t}"
                    }
                }
                span { class: "text-[11px] text-foreground-muted font-mono",
                    { format_date_ms(filing.filed_at) }
                }
            }
            div { class: "text-sm font-semibold mb-2", {filing.title.clone()} }
            if let Some(url) = &filing.url {
                a { class: "text-brand text-xs", href: "{url}", target: "_blank", "원본 공시 ↗" }
            }
        }
    }
}

pub fn format_date_ms(epoch_ms: i64) -> String {
    use chrono::TimeZone;
    chrono::Utc
        .timestamp_millis_opt(epoch_ms)
        .single()
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "—".to_string())
}

fn number_format(n: i64) -> String {
    let s = n.to_string();
    let chars: Vec<char> = s.chars().rev().collect();
    let mut out = String::new();
    for (i, c) in chars.iter().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(*c);
    }
    out.chars().rev().collect()
}

fn format_amount(amount: i64, currency: &Option<String>) -> String {
    let cur = currency.clone().unwrap_or_else(|| "KRW".to_string());
    if cur == "KRW" {
        if amount >= 100_000_000 {
            format!("{:.2}억 원", amount as f64 / 1e8)
        } else if amount >= 10_000 {
            format!("{:.0}만 원", amount as f64 / 1e4)
        } else {
            format!("{} 원", number_format(amount))
        }
    } else {
        format!("{} {}", number_format(amount), cur)
    }
}
