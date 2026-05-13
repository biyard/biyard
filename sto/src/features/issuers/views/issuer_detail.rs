use dioxus::prelude::*;

use crate::features::catalog::views::{StoTable, Topbar};
use crate::features::issuers::controllers::get_issuer;

#[component]
pub fn IssuerDetailView(issuer_id: String) -> Element {
    let id_for_fetch = issuer_id.clone();
    let data = use_server_future(move || {
        let id = id_for_fetch.clone();
        async move { get_issuer(id).await }
    })?;
    let resp = data.read();
    let resp_ref = resp.as_ref();

    rsx! {
        Topbar { active: "issuers".to_string() }
        main { class: "max-w-7xl mx-auto px-6 py-8",
            match resp_ref {
                Some(Ok(r)) => rsx! {
                    div { class: "flex items-center gap-2 text-xs text-foreground-muted mb-2",
                        a { href: "/issuers", class: "hover:text-brand", "발행사" }
                        span { "/" }
                        span { class: "text-foreground-soft", "{r.issuer.name}" }
                    }
                    h1 { class: "text-2xl font-bold mb-2", "{r.issuer.name}" }
                    div { class: "flex gap-2 mb-4 flex-wrap",
                        span { class: "text-xs px-2 py-1 rounded bg-panel-muted text-foreground-soft", "{r.issuer.country}" }
                        span { class: "text-xs px-2 py-1 rounded bg-brand-soft text-brand", "{r.issuer.status}" }
                        if let Some(s) = &r.issuer.sandbox {
                            span { class: "text-xs px-2 py-1 rounded bg-panel-muted text-foreground-muted", "샌드박스: {s}" }
                        }
                    }
                    p { class: "text-foreground-soft leading-relaxed mb-8 max-w-3xl", "{r.issuer.description}" }
                    section {
                        h2 { class: "text-base font-bold mb-3 pb-2 border-b border-border",
                            "발행 자산 " span { class: "text-foreground-muted font-normal text-sm ml-1", "{r.stos.len()}건" }
                        }
                        if r.stos.is_empty() {
                            div { class: "text-foreground-muted py-8 text-center", "아직 등록된 발행 자산이 없습니다." }
                        } else {
                            StoTable { items: r.stos.clone(), show_status: true }
                        }
                    }
                },
                Some(Err(e)) => rsx! {
                    div { class: "text-danger", "정보를 불러오지 못했습니다: {e}" }
                    a { href: "/issuers", class: "text-brand text-sm inline-block mt-2", "← 발행사 목록으로" }
                },
                None => rsx! {
                    div { class: "text-foreground-muted", "불러오는 중" }
                },
            }
        }
    }
}
