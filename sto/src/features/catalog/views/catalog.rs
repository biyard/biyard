use dioxus::prelude::*;
use dioxus_translate::Translate;

use crate::common::{Category, StoStatus, use_language, use_translate};
use crate::features::catalog::controllers::list_stos_handler;
use crate::features::catalog::views::{Panel, StoTable, Topbar};
use crate::features::catalog::CatalogTranslate;

#[component]
pub fn CatalogView() -> Element {
    let t: CatalogTranslate = use_translate();
    let lang = use_language();
    let lang_now = lang();

    let mut category = use_signal(|| "all".to_string());
    let mut country_filter = use_signal(|| "all".to_string());
    let mut issuer_filter = use_signal(|| "all".to_string());
    let mut status_filter = use_signal(|| "all".to_string());
    let mut search_q = use_signal(String::new);

    let data = use_loader(move || async move { list_stos_handler().await })?;
    let snapshot = data();

    let cat_filter = category();
    let country_f = country_filter();
    let iss = issuer_filter();
    let st_f = status_filter();
    let q = search_q().to_lowercase();
    let want_cat: Option<Category> = cat_filter.parse().ok();
    let want_status: Option<StoStatus> = st_f.parse().ok();

    let filtered: Vec<_> = snapshot
        .items
        .iter()
        .filter(|s| want_cat.map(|c| s.category == c).unwrap_or(true))
        .filter(|s| match country_f.as_str() {
            "all" => true,
            "KR" => s.country == crate::common::Country::Kr,
            "GLOBAL" => s.country.is_global(),
            _ => true,
        })
        .filter(|s| iss == "all" || s.issuer_id.as_deref() == Some(iss.as_str()))
        .filter(|s| want_status.map(|st| s.status == st).unwrap_or(true))
        .filter(|s| {
            if q.is_empty() {
                return true;
            }
            let hay = format!(
                "{} {} {}",
                s.name,
                s.artist.clone().unwrap_or_default(),
                s.underlying.clone().unwrap_or_default()
            )
            .to_lowercase();
            hay.contains(&q)
        })
        .cloned()
        .collect();

    let total = filtered.len();
    let results_text = t.filter_results_count.replace("{n}", &total.to_string());

    let category_filters: [(&str, Category); 4] = [
        ("REAL_ESTATE", Category::RealEstate),
        ("ART", Category::Art),
        ("MUSIC", Category::Music),
        ("LIVESTOCK", Category::Livestock),
    ];

    let select_cls = "bg-panel-muted border border-border rounded-lg px-3 py-2 text-xs text-foreground cursor-pointer max-w-[240px] appearance-none pr-8";

    rsx! {
        Topbar { active: "assets".to_string() }
        main { class: "mx-auto w-full max-w-[1400px] px-7 py-7",
            Panel { extra_class: "mb-4 p-4".to_string(),
                div { class: "flex flex-wrap gap-3 items-center",
                    input {
                        r#type: "search",
                        placeholder: "{t.filter_search_placeholder}",
                        class: "flex-1 min-w-[240px] bg-panel-muted border border-border rounded-lg px-3 py-2 text-foreground text-[13px]",
                        oninput: move |e| search_q.set(e.value()),
                    }
                    div { class: "flex gap-1",
                        FilterPill { label: t.filter_all.to_string(), active: country_f == "all",
                            on_click: EventHandler::new(move |_| country_filter.set("all".to_string())) }
                        FilterPill { label: t.filter_kr_label.to_string(), active: country_f == "KR",
                            on_click: EventHandler::new(move |_| country_filter.set("KR".to_string())) }
                        FilterPill { label: t.filter_global_label.to_string(), active: country_f == "GLOBAL",
                            on_click: EventHandler::new(move |_| country_filter.set("GLOBAL".to_string())) }
                    }
                    select { class: "{select_cls}", onchange: move |e| issuer_filter.set(e.value()),
                        option { value: "all", {t.filter_issuer_placeholder} }
                        option { value: "stockeeper", "스탁키퍼 (뱅카우)" }
                        option { value: "datagen", "데이터젠 (핀돈)" }
                        option { value: "togetherart", "투게더아트" }
                        option { value: "yeolmae", "열매컴퍼니" }
                        option { value: "seoulauctionblue", "서울옥션블루" }
                        option { value: "artipio", "아티피오" }
                        option { value: "musicow", "뮤직카우" }
                        option { value: "kasa", "카사" }
                        option { value: "lucentblock", "루센트블록" }
                        option { value: "funble", "펀블" }
                    }
                    select { class: "{select_cls}", onchange: move |e| status_filter.set(e.value()),
                        option { value: "all", {t.filter_status_placeholder} }
                        option { value: "FILED", { StoStatus::Filed.translate(&lang_now) } }
                        option { value: "ISSUED", { StoStatus::Issued.translate(&lang_now) } }
                        option { value: "LIQUIDATED", { StoStatus::Liquidated.translate(&lang_now) } }
                        option { value: "WITHDRAWN", { StoStatus::Withdrawn.translate(&lang_now) } }
                    }
                    span { class: "ml-auto text-xs text-foreground-muted font-mono whitespace-nowrap",
                        "{results_text}"
                    }
                }
                div { class: "flex gap-1 flex-wrap mt-2.5",
                    FilterPill {
                        label: t.filter_cat_all.to_string(),
                        active: cat_filter == "all",
                        on_click: EventHandler::new(move |_| category.set("all".to_string())),
                    }
                    for (key, cat) in category_filters.iter() {
                        FilterPill {
                            label: cat.translate(&lang_now).to_string(),
                            active: cat_filter == *key,
                            on_click: EventHandler::new({
                                let k = key.to_string();
                                move |_| category.set(k.clone())
                            })
                        }
                    }
                }
            }
            Panel {
                div { class: "overflow-x-auto",
                    StoTable { items: filtered, show_status: true }
                }
            }
        }
    }
}

#[component]
fn FilterPill(label: String, active: bool, on_click: EventHandler<MouseEvent>) -> Element {
    let base = "px-3 py-1.5 rounded-lg text-xs font-medium cursor-pointer transition-colors border";
    let cls = if active {
        format!("{base} bg-brand-soft text-brand border-brand")
    } else {
        format!("{base} bg-panel-muted text-foreground border-border hover:border-brand")
    };
    rsx! { button { class: "{cls}", onclick: move |e| on_click.call(e), {label} } }
}
