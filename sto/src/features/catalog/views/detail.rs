use dioxus::prelude::*;

use crate::features::catalog::controllers::get_sto;
use crate::features::catalog::views::Topbar;
use crate::features::catalog::{FilingSummary, StoDetailResponse};

#[component]
pub fn DetailView(sto_id: String) -> Element {
    let id_for_fetch = sto_id.clone();
    let data = use_server_future(move || {
        let id = id_for_fetch.clone();
        async move { get_sto(id).await }
    })?;
    let resp = data.read();
    let resp_ref = resp.as_ref();

    rsx! {
        Topbar { active: "assets".to_string() }
        main { class: "max-w-7xl mx-auto px-6 py-8",
            match resp_ref {
                Some(Ok(sto)) => rsx! { DetailBody { sto: sto.clone() } },
                Some(Err(e)) => rsx! {
                    div { class: "text-danger", "로드 실패: {e}" }
                    a { href: "/assets", class: "text-brand text-sm mt-2 inline-block", "← 시장으로" }
                },
                None => rsx! {
                    div { class: "text-foreground-muted", "로딩 중..." }
                },
            }
        }
    }
}

#[component]
fn DetailBody(sto: StoDetailResponse) -> Element {
    rsx! {
        div { class: "flex items-center gap-2 text-xs text-foreground-muted mb-2",
            a { href: "/", class: "hover:text-brand", "홈" }
            span { "/" }
            a { href: "/assets", class: "hover:text-brand", "STO 시장" }
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
                "{sto.status}"
            }
            if let Some(c) = &sto.classification {
                span { class: "text-xs px-2 py-1 rounded bg-panel-muted text-foreground-muted", "{c}" }
            }
        }

        div { class: "grid grid-cols-1 lg:grid-cols-3 gap-6 mb-6",
            // 좌측: 발행 구조 + 모집 정보
            div { class: "lg:col-span-2 space-y-6",
                DetailPanel { title: "발행 정보",
                    KvList { entries: vec![
                        ("자산명".to_string(), sto.name.clone()),
                        ("기초자산".to_string(), sto.underlying.clone().unwrap_or_else(|| "—".to_string())),
                        ("증권 종류".to_string(), sto.security_type.clone().unwrap_or_else(|| "—".to_string())),
                        ("발행일".to_string(), sto.issued_at.clone()),
                        ("상태".to_string(), sto.status.clone()),
                        ("작가".to_string(), sto.artist.clone().unwrap_or_else(|| "—".to_string())),
                        ("권리 종류".to_string(), sto.rights_category.clone().unwrap_or_else(|| "—".to_string())),
                        ("신탁번호".to_string(), sto.trust_no.clone().unwrap_or_else(|| "—".to_string())),
                        ("제작·발매".to_string(), sto.year.clone().unwrap_or_else(|| "—".to_string())),
                    ] }
                }

                if let Some(o) = &sto.offering {
                    DetailPanel { title: "모집 정보",
                        KvList { entries: vec![
                            ("모집 총액".to_string(), o.amount.map(|a| format_amount(a, &o.currency)).unwrap_or_else(|| "—".to_string())),
                            ("1주당 발행가".to_string(), o.unit_price.map(|p| format!("{} 원", number_format(p))).unwrap_or_else(|| "—".to_string())),
                            ("총 발행 수량".to_string(), o.total_units.map(|n| format!("{} 좌/주", number_format(n))).unwrap_or_else(|| "—".to_string())),
                            ("청약 기간".to_string(), format!("{} ~ {}",
                                o.subscription_start.clone().unwrap_or_else(|| "—".to_string()),
                                o.subscription_end.clone().unwrap_or_else(|| "—".to_string()))),
                        ] }
                    }
                }

                if let Some(is_) = &sto.issuance_structure {
                    DetailPanel { title: "발행 구조",
                        KvList { entries: vec![
                            ("발행사".to_string(), is_.issuer.clone().unwrap_or_else(|| "—".to_string())),
                            ("신탁·보관기관".to_string(), is_.trustee.clone().unwrap_or_else(|| "—".to_string())),
                            ("역할".to_string(), is_.trustee_role.clone().unwrap_or_else(|| "—".to_string())),
                            ("인수인".to_string(), is_.underwriter.clone().unwrap_or_else(|| "—".to_string())),
                            ("계좌관리".to_string(), is_.custody.clone().unwrap_or_else(|| "—".to_string())),
                        ] }
                    }
                }
            }

            // 우측: 출처·외부 링크·발행사
            div { class: "space-y-6",
                DetailPanel { title: "외부 링크",
                    div { class: "space-y-2",
                        if let Some(url) = &sto.external_url {
                            a {
                                href: "{url}", target: "_blank",
                                class: "block text-sm text-brand hover:underline break-all",
                                "원본 페이지 ↗"
                            }
                            div { class: "text-xs text-foreground-muted truncate", "{url}" }
                        } else {
                            div { class: "text-xs text-foreground-muted", "외부 링크 없음" }
                        }
                    }
                }

                if !sto.sources.is_empty() {
                    DetailPanel { title: "출처",
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
                    DetailPanel { title: "발행사",
                        a { href: "/issuers/{iid}", class: "text-sm text-brand hover:underline", "{iid} →" }
                    }
                }
            }
        }

        if !sto.filings.is_empty() {
            section {
                h2 { class: "text-base font-bold mb-3 pb-2 border-b border-border",
                    "공시 자료 ({sto.filings.len()})"
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
                span { class: "text-xs text-foreground-muted font-mono", "{filing.filed_at}" }
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
