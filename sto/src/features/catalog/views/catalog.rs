use dioxus::prelude::*;

use crate::common::{Category, Country, StoStatus};
use crate::features::catalog::controllers::list_stos_handler;
use crate::features::catalog::views::{Panel, StoTable, Topbar};

#[component]
pub fn CatalogView() -> Element {
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
            "KR" => s.country == Country::Kr,
            "GLOBAL" => s.country != Country::Kr && s.country != Country::Unknown,
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

    let cats = [
        ("all", "전체 카테고리"),
        ("REAL_ESTATE", "🏢 부동산"),
        ("ART", "🎨 미술품"),
        ("MUSIC", "🎵 음악 IP"),
        ("LIVESTOCK", "🐂 한우·축산"),
    ];

    let select_cls = "bg-panel-muted border border-border rounded-lg px-3 py-2 text-xs text-foreground cursor-pointer max-w-[240px] appearance-none pr-8";

    rsx! {
        Topbar { active: "assets".to_string() }
        main { class: "mx-auto w-full max-w-[1400px] px-7 py-7",
            Panel { extra_class: "mb-4 p-4".to_string(),
                div { class: "flex flex-wrap gap-3 items-center",
                    input {
                        r#type: "search",
                        placeholder: "자산명·발행사·기초자산 검색...",
                        class: "flex-1 min-w-[240px] bg-panel-muted border border-border rounded-lg px-3 py-2 text-foreground text-[13px]",
                        oninput: move |e| search_q.set(e.value()),
                    }
                    div { class: "flex gap-1",
                        RegionBtn { value: "all", label: "전체", active: country_f == "all",
                            on_click: EventHandler::new(move |_| country_filter.set("all".to_string())) }
                        RegionBtn { value: "KR", label: "🇰🇷 한국", active: country_f == "KR",
                            on_click: EventHandler::new(move |_| country_filter.set("KR".to_string())) }
                        RegionBtn { value: "GLOBAL", label: "🌍 해외", active: country_f == "GLOBAL",
                            on_click: EventHandler::new(move |_| country_filter.set("GLOBAL".to_string())) }
                    }
                    select { class: "{select_cls}", onchange: move |e| issuer_filter.set(e.value()),
                        option { value: "all", "발행사 — 전체" }
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
                        option { value: "all", "상태 — 전체" }
                        option { value: "FILED", "공모 진행" }
                        option { value: "ISSUED", "발행 완료" }
                        option { value: "LIQUIDATED", "청산 완료" }
                        option { value: "WITHDRAWN", "철회" }
                    }
                    span { class: "ml-auto text-xs text-foreground-muted font-mono whitespace-nowrap",
                        "검색 결과 {total}건"
                    }
                }
                div { class: "flex gap-1 flex-wrap mt-2.5",
                    for (key, label) in cats.iter() {
                        CatBtn {
                            value: key.to_string(),
                            label: label.to_string(),
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
fn RegionBtn(
    value: String,
    label: String,
    active: bool,
    on_click: EventHandler<MouseEvent>,
) -> Element {
    let _ = value;
    let base = "px-3 py-1.5 rounded-lg text-xs font-medium cursor-pointer transition-colors border";
    let cls = if active {
        format!("{base} bg-brand-soft text-brand border-brand")
    } else {
        format!("{base} bg-panel-muted text-foreground border-border hover:border-brand")
    };
    rsx! { button { class: "{cls}", onclick: move |e| on_click.call(e), {label} } }
}

#[component]
fn CatBtn(
    value: String,
    label: String,
    active: bool,
    on_click: EventHandler<MouseEvent>,
) -> Element {
    let _ = value;
    let base = "px-3 py-1.5 rounded-lg text-xs font-medium cursor-pointer transition-colors border";
    let cls = if active {
        format!("{base} bg-brand-soft text-brand border-brand")
    } else {
        format!("{base} bg-panel-muted text-foreground border-border hover:border-brand")
    };
    rsx! { button { class: "{cls}", onclick: move |e| on_click.call(e), {label} } }
}
