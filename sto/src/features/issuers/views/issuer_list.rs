use dioxus::prelude::*;

use crate::features::catalog::views::Topbar;
use crate::features::issuers::controllers::list_issuers;

#[component]
pub fn IssuerListView() -> Element {
    let data = use_server_future(|| async { list_issuers().await })?;
    let resp = data.read();
    let resp_ref = resp.as_ref();

    rsx! {
        Topbar { active: "issuers".to_string() }
        main { class: "max-w-7xl mx-auto px-6 py-8",
            h1 { class: "text-xl font-bold mb-2", "발행사" }
            p { class: "text-foreground-muted text-sm mb-6", "국내 STO·조각투자 발행사 목록." }
            match resp_ref {
                Some(Ok(r)) => rsx! {
                    div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3",
                        for issuer in r.items.iter() {
                            a { href: "/issuers/{issuer.issuer_id}",
                                class: "bg-panel border border-border rounded-2xl p-5 hover:border-brand transition-colors block",
                                div { class: "flex items-center gap-2 mb-2",
                                    span { class: "text-xs px-2 py-0.5 rounded bg-panel-muted text-foreground-soft", "{issuer.country}" }
                                    span { class: "text-xs px-2 py-0.5 rounded bg-brand-soft text-brand", "{issuer.status}" }
                                }
                                h2 { class: "text-base font-bold mb-1", "{issuer.name}" }
                                p { class: "text-xs text-foreground-muted leading-relaxed line-clamp-3", "{issuer.description}" }
                            }
                        }
                    }
                },
                Some(Err(e)) => rsx! {
                    div { class: "text-danger", "로드 실패: {e}" }
                },
                None => rsx! {
                    div { class: "text-foreground-muted", "로딩 중..." }
                },
            }
        }
    }
}
