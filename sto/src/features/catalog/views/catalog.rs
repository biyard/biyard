use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::features::catalog::controllers::list_stos;
use crate::features::catalog::views::{StoTable, Topbar};
use crate::features::catalog::CatalogTranslate;

#[component]
pub fn CatalogView() -> Element {
    let t: CatalogTranslate = use_translate();
    let data = use_server_future(|| async { list_stos().await })?;
    let mut category = use_signal(|| "all".to_string());
    let mut region = use_signal(|| "all".to_string());
    let mut status_filter = use_signal(|| "all".to_string());
    let mut issuer_filter = use_signal(|| "all".to_string());
    let mut search_q = use_signal(String::new);
    let mut page_size = use_signal(|| 50usize);

    let resp = data.read();
    let resp_ref = resp.as_ref();

    rsx! {
        Topbar { active: "assets".to_string() }
        main { class: "max-w-7xl mx-auto px-6 py-8",
            h1 { class: "text-xl font-bold mb-1", "{t.catalog_title}" }
            p { class: "text-sm text-foreground-muted mb-5", "{t.catalog_subtitle}" }

            // 필터바
            section { class: "bg-panel border border-border rounded-2xl p-4 mb-4 space-y-3",
                // 카테고리 (탭 형태)
                div { class: "flex gap-2 flex-wrap",
                    FilterPill {
                        label: t.filter_all.to_string(),
                        active: category() == "all",
                        on_click: EventHandler::new(move |_| category.set("all".to_string()))
                    }
                    FilterPill {
                        label: t.cat_real_estate.to_string(),
                        active: category() == "real_estate",
                        on_click: EventHandler::new(move |_| category.set("real_estate".to_string()))
                    }
                    FilterPill {
                        label: t.cat_art.to_string(),
                        active: category() == "art",
                        on_click: EventHandler::new(move |_| category.set("art".to_string()))
                    }
                    FilterPill {
                        label: t.cat_music.to_string(),
                        active: category() == "music",
                        on_click: EventHandler::new(move |_| category.set("music".to_string()))
                    }
                    FilterPill {
                        label: t.cat_livestock.to_string(),
                        active: category() == "livestock",
                        on_click: EventHandler::new(move |_| category.set("livestock".to_string()))
                    }
                }
                // 지역 + 상태 + 검색
                div { class: "flex gap-3 flex-wrap items-center text-sm",
                    div { class: "flex gap-1",
                        SegmentBtn { label: t.filter_all.to_string(), active: region() == "all",
                            on_click: EventHandler::new(move |_| region.set("all".to_string())) }
                        SegmentBtn { label: t.filter_kr.to_string(), active: region() == "KR",
                            on_click: EventHandler::new(move |_| region.set("KR".to_string())) }
                        SegmentBtn { label: t.filter_global.to_string(), active: region() == "GLOBAL",
                            on_click: EventHandler::new(move |_| region.set("GLOBAL".to_string())) }
                    }
                    select {
                        class: "bg-panel-muted border border-border rounded px-3 py-1.5 text-sm",
                        onchange: move |e| status_filter.set(e.value()),
                        option { value: "all", "{t.filter_status_all}" }
                        option { value: "발행완료", "{t.status_issued}" }
                        option { value: "신고중", "{t.status_filed}" }
                        option { value: "철회", "{t.status_withdrawn}" }
                    }
                    select {
                        class: "bg-panel-muted border border-border rounded px-3 py-1.5 text-sm",
                        onchange: move |e| issuer_filter.set(e.value()),
                        option { value: "all", "{t.filter_issuer_all}" }
                        option { value: "stockeeper", "스탁키퍼 (뱅카우)" }
                        option { value: "datagen", "데이터젠 (핀돈)" }
                        option { value: "togetherart", "투게더아트" }
                        option { value: "yeolmae", "열매컴퍼니 (아트앤가이드)" }
                        option { value: "seoulauctionblue", "서울옥션블루 (SOTWO)" }
                        option { value: "artipio", "아티피오" }
                        option { value: "musicow", "뮤직카우" }
                        option { value: "kasa", "카사" }
                        option { value: "lucentblock", "루센트블록 (소유)" }
                        option { value: "funble", "펀블" }
                    }
                    input {
                        r#type: "search",
                        placeholder: "{t.search_placeholder}",
                        class: "bg-panel-muted border border-border rounded px-3 py-1.5 text-sm flex-1 min-w-[200px]",
                        oninput: move |e| search_q.set(e.value()),
                    }
                }
            }

            match resp_ref {
                Some(Ok(r)) => {
                    let cat = category();
                    let reg = region();
                    let st = status_filter();
                    let iss = issuer_filter();
                    let q = search_q().to_lowercase();
                    let filtered: Vec<_> = r.items.iter()
                        .filter(|s| cat == "all" || s.category == cat)
                        .filter(|s| reg == "all" || s.region == reg)
                        .filter(|s| st == "all" || s.status == st)
                        .filter(|s| iss == "all" || s.issuer_id.as_deref() == Some(iss.as_str()))
                        .filter(|s| {
                            if q.is_empty() { return true; }
                            let hay = format!(
                                "{} {} {}",
                                s.name,
                                s.artist.clone().unwrap_or_default(),
                                s.underlying.clone().unwrap_or_default()
                            ).to_lowercase();
                            hay.contains(&q)
                        })
                        .cloned()
                        .collect();
                    let total = filtered.len();
                    let shown: Vec<_> = filtered.into_iter().take(page_size()).collect();
                    let shown_len = shown.len();
                    rsx! {
                        div { class: "flex justify-between items-center mb-3 text-xs text-foreground-muted",
                            span {
                                "{t.results_count} "
                                span { class: "text-foreground font-bold", "{total}" }
                                "건 · {t.showing} "
                                span { class: "text-foreground", "{shown_len}" }
                                "건"
                            }
                            div { class: "flex items-center gap-2",
                                span { class: "text-foreground-muted", "{t.page_size_label}" }
                                div { class: "flex gap-1",
                                    for size in [20usize, 50, 100, 500].iter() {
                                        {
                                            let s = *size;
                                            let active = page_size() == s;
                                            let cls = if active {
                                                "px-2 py-0.5 rounded text-brand bg-brand-soft border border-brand"
                                            } else {
                                                "px-2 py-0.5 rounded border border-border text-foreground-muted hover:text-foreground"
                                            };
                                            rsx! {
                                                button {
                                                    class: "{cls}",
                                                    onclick: move |_| page_size.set(s),
                                                    "{s}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        StoTable { items: shown, show_status: true }
                    }
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
fn FilterPill(label: String, active: bool, on_click: EventHandler<MouseEvent>) -> Element {
    let cls = if active {
        "px-3 py-1 text-xs rounded border bg-brand-soft text-brand border-brand font-semibold"
    } else {
        "px-3 py-1 text-xs rounded border bg-transparent text-foreground-muted border-border hover:text-foreground hover:border-foreground-soft"
    };
    rsx! {
        button { class: "{cls}", onclick: move |e| on_click.call(e), "{label}" }
    }
}

#[component]
fn SegmentBtn(label: String, active: bool, on_click: EventHandler<MouseEvent>) -> Element {
    let cls = if active {
        "px-3 py-1.5 text-xs bg-brand-soft text-brand border border-brand font-semibold first:rounded-l-md last:rounded-r-md"
    } else {
        "px-3 py-1.5 text-xs bg-transparent text-foreground-muted border border-border hover:text-foreground first:rounded-l-md last:rounded-r-md"
    };
    rsx! {
        button { class: "{cls}", onclick: move |e| on_click.call(e), "{label}" }
    }
}
