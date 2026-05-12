use dioxus::prelude::*;

use crate::features::catalog::controllers::list_stos;
use crate::features::catalog::views::{StoTable, Topbar};

#[component]
pub fn CatalogView() -> Element {
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
            h1 { class: "text-xl font-bold mb-4", "STO 시장" }

            // 필터바
            section { class: "bg-panel border border-line rounded-card p-4 mb-4 space-y-3",
                // 카테고리 (탭 형태)
                div { class: "flex gap-2 flex-wrap",
                    for (key, label) in [
                        ("all", "전체"),
                        ("real_estate", "🏢 부동산"),
                        ("art", "🎨 미술품"),
                        ("music", "🎵 음악 IP"),
                        ("livestock", "🐄 축산"),
                    ].iter() {
                        FilterPill {
                            label: label.to_string(),
                            active: category() == *key,
                            on_click: {
                                let key = key.to_string();
                                EventHandler::new(move |_| category.set(key.clone()))
                            }
                        }
                    }
                }
                // 지역 + 상태 + 검색
                div { class: "flex gap-3 flex-wrap items-center text-sm",
                    div { class: "flex gap-1",
                        SegmentBtn { label: "전체".to_string(), active: region() == "all",
                            on_click: EventHandler::new(move |_| region.set("all".to_string())) }
                        SegmentBtn { label: "🇰🇷 한국".to_string(), active: region() == "KR",
                            on_click: EventHandler::new(move |_| region.set("KR".to_string())) }
                        SegmentBtn { label: "🌍 해외".to_string(), active: region() == "GLOBAL",
                            on_click: EventHandler::new(move |_| region.set("GLOBAL".to_string())) }
                    }
                    select {
                        class: "bg-panel-2 border border-line rounded px-3 py-1.5 text-sm",
                        onchange: move |e| status_filter.set(e.value()),
                        option { value: "all", "모든 상태" }
                        option { value: "발행완료", "발행 완료" }
                        option { value: "신고중", "신고 중" }
                        option { value: "철회", "철회" }
                    }
                    select {
                        class: "bg-panel-2 border border-line rounded px-3 py-1.5 text-sm",
                        onchange: move |e| issuer_filter.set(e.value()),
                        option { value: "all", "모든 발행사" }
                        option { value: "stockeeper", "스탁키퍼" }
                        option { value: "datagen", "데이터젠" }
                        option { value: "togetherart", "투게더아트" }
                        option { value: "yeolmae", "열매컴퍼니" }
                        option { value: "seoulauctionblue", "서울옥션블루" }
                        option { value: "artipio", "아티피오" }
                        option { value: "musicow", "뮤직카우" }
                    }
                    input {
                        r#type: "search",
                        placeholder: "자산명·아티스트 검색",
                        class: "bg-panel-2 border border-line rounded px-3 py-1.5 text-sm flex-1 min-w-[200px]",
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
                            let hay = format!("{} {}", s.name, s.artist.clone().unwrap_or_default()).to_lowercase();
                            hay.contains(&q)
                        })
                        .cloned()
                        .collect();
                    let total = filtered.len();
                    let shown: Vec<_> = filtered.into_iter().take(page_size()).collect();
                    let shown_len = shown.len();
                    rsx! {
                        div { class: "flex justify-between items-center mb-3 text-xs text-muted",
                            span {
                                "검색 결과 "
                                span { class: "text-ink font-bold", "{total}" }
                                " 건 (표시 "
                                span { class: "text-ink", "{shown_len}" }
                                ")"
                            }
                            div { class: "flex gap-1",
                                for size in [20usize, 50, 100, 500].iter() {
                                    {
                                        let s = *size;
                                        let active = page_size() == s;
                                        let cls = if active {
                                            "px-2 py-0.5 rounded text-brand bg-brand-soft border border-brand-line"
                                        } else {
                                            "px-2 py-0.5 rounded border border-line text-muted hover:text-ink"
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
                        StoTable { items: shown, show_status: true }
                    }
                },
                Some(Err(e)) => rsx! {
                    div { class: "text-danger", "데이터 로드 실패: {e}" }
                },
                None => rsx! {
                    div { class: "text-muted", "로딩 중..." }
                },
            }
        }
    }
}

#[component]
fn FilterPill(label: String, active: bool, on_click: EventHandler<MouseEvent>) -> Element {
    let cls = if active {
        "px-3 py-1 text-xs rounded border bg-brand-soft text-brand border-brand-line font-semibold"
    } else {
        "px-3 py-1 text-xs rounded border bg-transparent text-muted border-line hover:text-ink hover:border-ink-soft"
    };
    rsx! {
        button { class: "{cls}", onclick: move |e| on_click.call(e), "{label}" }
    }
}

#[component]
fn SegmentBtn(label: String, active: bool, on_click: EventHandler<MouseEvent>) -> Element {
    let cls = if active {
        "px-3 py-1.5 text-xs bg-brand-soft text-brand border border-brand-line font-semibold"
    } else {
        "px-3 py-1.5 text-xs bg-transparent text-muted border border-line hover:text-ink"
    };
    rsx! {
        button { class: "{cls}", onclick: move |e| on_click.call(e), "{label}" }
    }
}
