use dioxus::prelude::*;

use crate::features::catalog::controllers::list_stos;
use crate::features::catalog::views::{StoTable, Topbar};

#[component]
pub fn CatalogView() -> Element {
    let data = use_server_future(|| async { list_stos().await })?;
    let mut category = use_signal(|| "all".to_string());
    let resp = data.read();
    let resp_ref = resp.as_ref();

    rsx! {
        Topbar {}
        main { class: "max-w-7xl mx-auto px-6 py-8",
            h1 { class: "text-xl font-bold mb-4", "STO 시장" }
            div { class: "flex gap-2 mb-6",
                for (key, label) in [
                    ("all", "전체"),
                    ("real_estate", "🏢 부동산"),
                    ("art", "🎨 미술품"),
                    ("music", "🎵 음악 IP"),
                    ("livestock", "🐄 축산"),
                ].iter() {
                    button {
                        class: format!(
                            "px-3 py-1 text-xs rounded border {}",
                            if category() == *key {
                                "bg-brand-soft text-brand border-brand-line"
                            } else {
                                "bg-transparent text-muted border-line"
                            }
                        ),
                        onclick: {
                            let key = key.to_string();
                            move |_| category.set(key.clone())
                        },
                        {*label}
                    }
                }
            }
            match resp_ref {
                Some(Ok(r)) => {
                    let cat = category();
                    let filtered: Vec<_> = r.items.iter()
                        .filter(|s| cat == "all" || s.category == cat)
                        .cloned()
                        .collect();
                    rsx! {
                        div { class: "text-xs text-muted mb-3",
                            "총 " span { class: "text-ink font-bold", "{filtered.len()}" } " 건"
                        }
                        StoTable { items: filtered }
                    }
                },
                Some(Err(e)) => rsx! {
                    div { class: "text-danger", "로드 실패: {e}" }
                },
                None => rsx! {
                    div { class: "text-muted", "로딩 중..." }
                },
            }
        }
    }
}
