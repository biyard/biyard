use dioxus::prelude::*;
use dioxus_translate::Translate;

use crate::common::{use_language, use_translate, Category};
use crate::features::catalog::controllers::get_sto_handler;
use crate::features::catalog::views::{Panel, PanelHead, Topbar};
use crate::features::catalog::{
    CatalogTranslate, FilingSummary, IssuanceField, OfferingField, OverviewField,
    StoDetailResponse,
};

#[component]
pub fn DetailView(sto_id: ReadSignal<String>) -> Element {
    let data = use_loader(move || async move { get_sto_handler(sto_id()).await })?;
    let sto = data();
    rsx! {
        Topbar { active: "assets".to_string() }
        main { class: "mx-auto w-full max-w-[1400px] px-7 py-7",
            DetailBody { sto: sto }
        }
    }
}

#[component]
fn DetailBody(sto: StoDetailResponse) -> Element {
    let t: CatalogTranslate = use_translate();
    let lang = use_language();
    let lang_now = lang();
    let dash = || "—".to_string();

    let mut overview_entries: Vec<(String, String)> = vec![
        (
            OverviewField::AssetName.translate(&lang_now).to_string(),
            sto.name.clone(),
        ),
        (
            OverviewField::Underlying.translate(&lang_now).to_string(),
            sto.underlying.clone().unwrap_or_else(dash),
        ),
        (
            OverviewField::SecurityType.translate(&lang_now).to_string(),
            sto.security_type.clone().unwrap_or_else(dash),
        ),
        (
            OverviewField::FiledAt.translate(&lang_now).to_string(),
            format_date_ms(sto.issued_at),
        ),
        (
            OverviewField::Status.translate(&lang_now).to_string(),
            sto.status.translate(&lang_now).to_string(),
        ),
    ];

    match sto.category {
        Category::Music => {
            overview_entries.extend([
                (
                    OverviewField::Artist.translate(&lang_now).to_string(),
                    sto.artist.clone().unwrap_or_else(dash),
                ),
                (
                    OverviewField::RightsCategory.translate(&lang_now).to_string(),
                    sto.rights_category.clone().unwrap_or_else(dash),
                ),
                (
                    OverviewField::TrustNo.translate(&lang_now).to_string(),
                    sto.trust_no.clone().unwrap_or_else(dash),
                ),
                (
                    OverviewField::Year.translate(&lang_now).to_string(),
                    sto.year.clone().unwrap_or_else(dash),
                ),
            ]);
        }
        Category::RealEstate => {
            let mut re_entries: Vec<(String, String)> = vec![
                (OverviewField::Address.translate(&lang_now).to_string(), sto.address.clone().unwrap_or_else(dash)),
                (OverviewField::BuildingType.translate(&lang_now).to_string(), sto.building_type.clone().unwrap_or_else(dash)),
                (OverviewField::FloorArea.translate(&lang_now).to_string(), sto.floor_area.clone().unwrap_or_else(dash)),
            ];
            if sto.land_area.is_some() {
                re_entries.push((OverviewField::LandArea.translate(&lang_now).to_string(), sto.land_area.clone().unwrap_or_else(dash)));
            }
            if sto.floors.is_some() {
                re_entries.push((OverviewField::Floors.translate(&lang_now).to_string(), sto.floors.clone().unwrap_or_else(dash)));
            }
            if sto.completion_date.is_some() {
                re_entries.push((OverviewField::CompletionDate.translate(&lang_now).to_string(), sto.completion_date.clone().unwrap_or_else(dash)));
            }
            if sto.tenant.is_some() {
                re_entries.push((OverviewField::Tenant.translate(&lang_now).to_string(), sto.tenant.clone().unwrap_or_else(dash)));
            }
            if sto.lease_term.is_some() {
                re_entries.push((OverviewField::LeaseTerm.translate(&lang_now).to_string(), sto.lease_term.clone().unwrap_or_else(dash)));
            }
            if sto.total_offering.is_some() {
                re_entries.push((OverviewField::TotalOffering.translate(&lang_now).to_string(), sto.total_offering.clone().unwrap_or_else(dash)));
            }
            if sto.total_units_str.is_some() {
                re_entries.push((OverviewField::TotalUnitsStr.translate(&lang_now).to_string(), sto.total_units_str.clone().unwrap_or_else(dash)));
            }
            if sto.unit_price_str.is_some() {
                re_entries.push((OverviewField::UnitPriceStr.translate(&lang_now).to_string(), sto.unit_price_str.clone().unwrap_or_else(dash)));
            }
            if sto.upfront_fee.is_some() {
                re_entries.push((OverviewField::UpfrontFee.translate(&lang_now).to_string(), sto.upfront_fee.clone().unwrap_or_else(dash)));
            }
            if sto.dividend_frequency.is_some() {
                re_entries.push((OverviewField::DividendFrequency.translate(&lang_now).to_string(), sto.dividend_frequency.clone().unwrap_or_else(dash)));
            }
            overview_entries.extend(re_entries);
        }
        Category::Art => {
            overview_entries.extend([
                (
                    OverviewField::ArtArtist.translate(&lang_now).to_string(),
                    sto.art_artist.clone().unwrap_or_else(dash),
                ),
                (
                    OverviewField::ArtworkYear.translate(&lang_now).to_string(),
                    sto.artwork_year.clone().unwrap_or_else(dash),
                ),
                (
                    OverviewField::Medium.translate(&lang_now).to_string(),
                    sto.medium.clone().unwrap_or_else(dash),
                ),
                (
                    OverviewField::Dimensions.translate(&lang_now).to_string(),
                    sto.dimensions.clone().unwrap_or_else(dash),
                ),
            ]);
        }
        Category::Livestock => {
            overview_entries.extend([
                (
                    OverviewField::FarmName.translate(&lang_now).to_string(),
                    sto.farm_name.clone().unwrap_or_else(dash),
                ),
                (
                    OverviewField::Breed.translate(&lang_now).to_string(),
                    sto.breed.clone().unwrap_or_else(dash),
                ),
                (
                    OverviewField::HeadCount.translate(&lang_now).to_string(),
                    sto.head_count
                        .map(|n| n.to_string())
                        .unwrap_or_else(dash),
                ),
            ]);
        }
        _ => {}
    };

    let appraisal_entries: Option<Vec<(String, String)>> = sto.appraisal_values.as_ref().and_then(|v| {
        let arr = v.as_array()?;
        let entries = arr.iter().filter_map(|item| {
            let amount = item.get("amount_억원")?.as_f64()?;
            let appraiser = item.get("appraiser")?.as_str()?;
            Some((appraiser.to_string(), format!("{:.2}억원", amount)))
        }).collect::<Vec<_>>();
        if entries.is_empty() { None } else { Some(entries) }
    });

    let offering_entries: Option<Vec<(String, String)>> = sto.offering.as_ref().map(|o| {
        vec![
            (
                OfferingField::Amount.translate(&lang_now).to_string(),
                o.amount
                    .map(|a| format_amount(a, &o.currency, &t))
                    .unwrap_or_else(|| "—".to_string()),
            ),
            (
                OfferingField::UnitPrice.translate(&lang_now).to_string(),
                o.unit_price
                    .map(|p| format!("{} {}", number_format(p), t.unit_won))
                    .unwrap_or_else(|| "—".to_string()),
            ),
            (
                OfferingField::TotalUnits.translate(&lang_now).to_string(),
                o.total_units
                    .map(|n| format!("{} {}", number_format(n), t.unit_seat))
                    .unwrap_or_else(|| "—".to_string()),
            ),
            (
                OfferingField::Subscription.translate(&lang_now).to_string(),
                format!(
                    "{} ~ {}",
                    o.subscription_start.clone().unwrap_or_else(|| "—".to_string()),
                    o.subscription_end.clone().unwrap_or_else(|| "—".to_string())
                ),
            ),
        ]
    });

    let issuance_entries: Option<Vec<(String, String)>> =
        sto.issuance_structure.as_ref().map(|is_| {
            vec![
                (
                    IssuanceField::Issuer.translate(&lang_now).to_string(),
                    is_.issuer.clone().unwrap_or_else(|| "—".to_string()),
                ),
                (
                    IssuanceField::Trustee.translate(&lang_now).to_string(),
                    is_.trustee.clone().unwrap_or_else(|| "—".to_string()),
                ),
                (
                    IssuanceField::Role.translate(&lang_now).to_string(),
                    is_.trustee_role.clone().unwrap_or_else(|| "—".to_string()),
                ),
                (
                    IssuanceField::Underwriter.translate(&lang_now).to_string(),
                    is_.underwriter.clone().unwrap_or_else(|| "—".to_string()),
                ),
                (
                    IssuanceField::Custody.translate(&lang_now).to_string(),
                    is_.custody.clone().unwrap_or_else(|| "—".to_string()),
                ),
            ]
        });

    let filings_count = sto.filings.len();
    let filings_title = t
        .detail_filings_title_fmt
        .replace("{n}", &filings_count.to_string());

    rsx! {
        section { class: "bg-panel border border-border rounded-xl p-7 mb-[18px]",
            div { class: "flex gap-2 flex-wrap mb-3",
                CatTag { label: sto.category.translate(&lang_now).to_string() }
                CatTag { label: sto.country.translate(&lang_now).to_string() }
                if let Some(st) = &sto.security_type {
                    CatTag { label: st.clone() }
                }
                CatTag { label: sto.status.translate(&lang_now).to_string() }
            }
            h1 { class: "text-2xl font-bold tracking-tight mb-2", {sto.name.clone()} }
            if let Some(u) = &sto.underlying {
                p { class: "text-sm text-foreground-muted m-0", {u.clone()} }
            }
            if sto.category == Category::RealEstate {
                if let Some(addr) = &sto.address {
                    div { class: "mt-3 flex items-center gap-2 text-sm text-foreground-muted",
                        span { class: "text-base", "📍" }
                        span { {addr.clone()} }
                        a {
                            class: "ml-auto text-xs text-brand whitespace-nowrap",
                            href: "https://map.kakao.com/?q={addr}",
                            target: "_blank",
                            {t.detail_map_link}
                        }
                    }
                }
            }
        }

        div { class: "grid grid-cols-1 lg:grid-cols-[2fr_1fr] gap-[18px]",
            div {
                DetailPanel { title: t.detail_section_info.to_string(),
                    DetailGrid { entries: overview_entries }
                }

                if let Some(entries) = appraisal_entries {
                    DetailPanel { title: t.detail_section_appraisal.to_string(),
                        DetailGrid { entries }
                    }
                }

                if let Some(entries) = offering_entries {
                    DetailPanel { title: t.detail_section_offering.to_string(),
                        DetailGrid { entries }
                    }
                }

                if let Some(entries) = issuance_entries {
                    DetailPanel { title: t.detail_section_structure.to_string(),
                        DetailGrid { entries }
                    }
                }
            }

            div {
                DetailPanel { title: t.detail_section_links.to_string(),
                    if let Some(url) = &sto.external_url {
                        a {
                            class: "block text-brand text-sm break-all",
                            href: "{url}",
                            target: "_blank",
                            {t.detail_external_origin_arrow}
                        }
                        div { class: "text-[11px] text-foreground-muted mt-1.5 break-all", {url.clone()} }
                    } else {
                        div { class: "text-xs text-foreground-muted", {t.detail_no_external_short} }
                    }
                }

                if !sto.sources.is_empty() {
                    DetailPanel { title: t.detail_section_sources.to_string(),
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
                    DetailPanel { title: t.detail_section_issuer.to_string(),
                        a {
                            class: "text-brand text-sm",
                            href: "/issuers/{iid}",
                            { sto.issuer_name.clone().unwrap_or_else(|| iid.clone()) }
                            " →"
                        }
                    }
                }
            }
        }

        if !sto.filings.is_empty() {
            Panel { extra_class: "mt-[18px]".to_string(),
                PanelHead { title: filings_title }
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
    let t: CatalogTranslate = use_translate();
    rsx! {
        article { class: "bg-panel-muted border border-border rounded-lg p-3.5",
            div { class: "flex gap-2 items-center flex-wrap mb-1.5",
                SourceBadge { label: filing.filing_source.to_string() }
                if let Some(ft) = &filing.filing_type {
                    span { class: "text-[11px] px-1.5 py-0.5 rounded bg-panel-strong text-foreground-soft",
                        "{ft}"
                    }
                }
                span { class: "text-[11px] text-foreground-muted font-mono",
                    { format_date_ms(filing.filed_at) }
                }
            }
            div { class: "text-sm font-semibold mb-2", {filing.title.clone()} }
            if let Some(url) = &filing.url {
                a { class: "text-brand text-xs", href: "{url}", target: "_blank", {t.detail_filing_origin_arrow} }
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

fn format_amount(amount: i64, currency: &Option<String>, t: &CatalogTranslate) -> String {
    let cur = currency.clone().unwrap_or_else(|| "KRW".to_string());
    if cur == "KRW" {
        if amount >= 100_000_000 {
            format!("{:.2}억 {}", amount as f64 / 1e8, t.unit_won)
        } else if amount >= 10_000 {
            format!("{:.0}만 {}", amount as f64 / 1e4, t.unit_won)
        } else {
            format!("{} {}", number_format(amount), t.unit_won)
        }
    } else {
        format!("{} {}", number_format(amount), cur)
    }
}
