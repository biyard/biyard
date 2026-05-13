use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::features::catalog::controllers::get_sto;
use crate::features::catalog::views::{Topbar, status_label};
use crate::features::catalog::{CatalogTranslate, FilingSummary, StoDetailResponse};

#[component]
pub fn DetailView(sto_id: String) -> Element {
    let id_for_fetch = sto_id.clone();
    let data = use_server_future(move || {
        let id = id_for_fetch.clone();
        async move { get_sto(id).await }
    })?;
    let resp = data.read();
    let resp_ref = resp.as_ref();

    let t: CatalogTranslate = use_translate();
    rsx! {
        Topbar { active: "assets".to_string() }
        main { class: "max-w-7xl mx-auto px-6 py-8",
            match resp_ref {
                Some(Ok(sto)) => rsx! { DetailBody { sto: sto.clone() } },
                Some(Err(e)) => rsx! {
                    div { class: "text-danger", "{t.load_failed}: {e}" }
                    a { href: "/market", class: "text-brand text-sm mt-2 inline-block", "← 공모 자산 목록으로" }
                },
                None => rsx! {
                    div { class: "text-foreground-muted", "{t.loading}" }
                },
            }
        }
    }
}

#[component]
fn DetailBody(sto: StoDetailResponse) -> Element {
    let t: CatalogTranslate = use_translate();
    rsx! {
        div { class: "flex items-center gap-2 text-xs text-foreground-muted mb-2",
            a { href: "/", class: "hover:text-brand", "{t.detail_breadcrumb_home}" }
            span { "/" }
            a { href: "/market", class: "hover:text-brand", "{t.detail_breadcrumb_market}" }
            span { "/" }
            span { class: "text-foreground-soft", "{sto.name}" }
        }
        h1 { class: "text-2xl font-bold mb-2", "{sto.name}" }
        div { class: "flex gap-2 items-center mb-6 flex-wrap",
            span { class: "text-xs px-2 py-1 rounded bg-panel-muted text-foreground-soft",
                { category_label(&sto.category) }
            }
            span { class: "text-xs px-2 py-1 rounded bg-panel-muted text-foreground-soft",
                "{sto.country}"
            }
            span { class: "text-xs px-2 py-1 rounded bg-brand-soft text-brand font-bold",
                {
                    let label = status_label(&sto.status, &t);
                    if label.is_empty() { sto.status.clone() } else { label.to_string() }
                }
            }
            if let Some(c) = &sto.classification {
                span { class: "text-xs px-2 py-1 rounded bg-panel-muted text-foreground-muted", "{c}" }
            }
        }

        div { class: "grid grid-cols-1 lg:grid-cols-3 gap-6 mb-6",
            // 좌측: 발행 구조 + 모집 정보
            div { class: "lg:col-span-2 space-y-6",
                DetailPanel { title: t.detail_section_info.to_string(),
                    KvList { entries: vec![
                        (t.field_asset_name.to_string(), sto.name.clone()),
                        (t.field_underlying.to_string(), sto.underlying.clone().unwrap_or_else(|| "—".to_string())),
                        (t.field_security_type.to_string(), sto.security_type.clone().unwrap_or_else(|| "—".to_string())),
                        (t.field_issued_at.to_string(), format_date_ms(sto.issued_at)),
                        (t.field_status.to_string(), {
                            let label = status_label(&sto.status, &t);
                            if label.is_empty() { sto.status.clone() } else { label.to_string() }
                        }),
                        (t.field_artist.to_string(), sto.artist.clone().unwrap_or_else(|| "—".to_string())),
                        (t.field_rights_category.to_string(), sto.rights_category.clone().unwrap_or_else(|| "—".to_string())),
                        (t.field_trust_no.to_string(), sto.trust_no.clone().unwrap_or_else(|| "—".to_string())),
                        (t.field_year.to_string(), sto.year.clone().unwrap_or_else(|| "—".to_string())),
                    ] }
                }

                if let Some(o) = &sto.offering {
                    DetailPanel { title: t.detail_section_offering.to_string(),
                        KvList { entries: vec![
                            (t.field_amount.to_string(), o.amount.map(|a| format_amount(a, &o.currency)).unwrap_or_else(|| "—".to_string())),
                            (t.field_unit_price.to_string(), o.unit_price.map(|p| format!("{} 원", number_format(p))).unwrap_or_else(|| "—".to_string())),
                            (t.field_total_units.to_string(), o.total_units.map(|n| format!("{} 좌", number_format(n))).unwrap_or_else(|| "—".to_string())),
                            (t.field_subscription.to_string(), format!("{} ~ {}",
                                o.subscription_start.clone().unwrap_or_else(|| "—".to_string()),
                                o.subscription_end.clone().unwrap_or_else(|| "—".to_string()))),
                        ] }
                    }
                }

                if let Some(is_) = &sto.issuance_structure {
                    DetailPanel { title: t.detail_section_structure.to_string(),
                        KvList { entries: vec![
                            (t.field_issuer.to_string(), is_.issuer.clone().unwrap_or_else(|| "—".to_string())),
                            (t.field_trustee.to_string(), is_.trustee.clone().unwrap_or_else(|| "—".to_string())),
                            (t.field_role.to_string(), is_.trustee_role.clone().unwrap_or_else(|| "—".to_string())),
                            (t.field_underwriter.to_string(), is_.underwriter.clone().unwrap_or_else(|| "—".to_string())),
                            (t.field_custody.to_string(), is_.custody.clone().unwrap_or_else(|| "—".to_string())),
                        ] }
                    }
                }
            }

            // 우측: 출처·외부 링크·발행사
            div { class: "space-y-6",
                DetailPanel { title: t.detail_section_links.to_string(),
                    div { class: "space-y-2",
                        if let Some(url) = &sto.external_url {
                            a {
                                href: "{url}", target: "_blank",
                                class: "block text-sm text-brand hover:underline break-all",
                                "{t.detail_external_origin}"
                            }
                            div { class: "text-xs text-foreground-muted truncate", "{url}" }
                        } else {
                            div { class: "text-xs text-foreground-muted", "{t.detail_no_external}" }
                        }
                    }
                }

                if !sto.sources.is_empty() {
                    DetailPanel { title: t.detail_section_sources.to_string(),
                        div { class: "space-y-1.5",
                            for s in sto.sources.iter() {
                                div { class: "flex gap-2 items-start text-xs",
                                    span { class: "px-1.5 py-0.5 rounded bg-panel-muted text-foreground-soft font-mono shrink-0", "{s.src}" }
                                    span { class: "text-foreground-muted", "{s.label}" }
                                }
                            }
                        }
                    }
                }

                if let Some(iid) = &sto.issuer_id {
                    DetailPanel { title: t.detail_section_issuer.to_string(),
                        a { href: "/issuers/{iid}", class: "text-sm text-brand hover:underline", "{iid} →" }
                    }
                }
            }
        }

        if !sto.filings.is_empty() {
            section {
                h2 { class: "text-base font-bold mb-3 pb-2 border-b border-border",
                    "{t.detail_section_filings} ({sto.filings.len()})"
                }
                div { class: "space-y-3",
                    for f in sto.filings.iter() {
                        FilingCard { filing: f.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn DetailPanel(title: String, children: Element) -> Element {
    rsx! {
        section { class: "bg-panel border border-border rounded-2xl p-5",
            h2 { class: "text-sm font-bold mb-3 pb-2 border-b border-border text-foreground", "{title}" }
            {children}
        }
    }
}

#[component]
fn KvList(entries: Vec<(String, String)>) -> Element {
    rsx! {
        dl { class: "grid grid-cols-[140px_1fr] gap-y-2.5 text-sm",
            for (k, v) in entries.iter() {
                dt { class: "text-foreground-muted", "{k}" }
                dd { class: "text-foreground-soft", "{v}" }
            }
        }
    }
}

#[component]
fn FilingCard(filing: FilingSummary) -> Element {
    rsx! {
        article { class: "bg-panel border border-border rounded-sm p-4",
            div { class: "flex items-center gap-2 mb-2 flex-wrap",
                span { class: "text-[10px] px-2 py-0.5 rounded bg-brand-soft text-brand font-bold", "{filing.filing_source}" }
                if let Some(t) = &filing.filing_type {
                    span { class: "text-[10px] px-2 py-0.5 rounded bg-panel-muted text-foreground-soft", "{t}" }
                }
                span { class: "text-xs text-foreground-muted font-mono", {format_date_ms(filing.filed_at)} }
            }
            div { class: "text-sm font-semibold text-foreground mb-2", "{filing.title}" }
            if let Some(url) = &filing.url {
                a { href: "{url}", target: "_blank", class: "text-xs text-brand hover:underline",
                    "원본 공시 ↗"
                }
            }
            if !filing.attachments.is_empty() {
                div { class: "mt-3 pt-3 border-t border-border space-y-1.5",
                    for att in filing.attachments.iter() {
                        a {
                            href: "{att.url}", target: "_blank",
                            class: "block text-xs text-foreground-soft hover:text-brand",
                            "📄 {att.name}"
                            if let Some(sz) = att.size_bytes {
                                span { class: "text-foreground-muted ml-2", "({format_size(sz)})" }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn category_label(c: &str) -> &'static str {
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

/// epoch ms → `YYYY-MM-DD` (UTC). 표시 외 용도로는 쓰지 말 것.
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

fn format_size(bytes: i64) -> String {
    if bytes >= 1_000_000 {
        format!("{:.1} MB", bytes as f64 / 1_000_000.0)
    } else if bytes >= 1_000 {
        format!("{:.0} KB", bytes as f64 / 1_000.0)
    } else {
        format!("{} B", bytes)
    }
}
