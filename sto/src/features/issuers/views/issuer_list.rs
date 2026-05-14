use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::features::catalog::views::{Topbar, country_display};
use crate::features::issuers::controllers::list_issuers_handler;
use crate::features::issuers::{IssuersTranslate, issuer_status_label};

#[component]
pub fn IssuerListView() -> Element {
    let t: IssuersTranslate = use_translate();
    let data = use_loader(move || async move { list_issuers_handler().await })?;
    let snapshot = data();

    rsx! {
        Topbar { active: "issuers".to_string() }
        main { class: "max-w-7xl mx-auto px-6 py-8",
            h1 { class: "text-xl font-bold mb-1", {t.page_title} }
            p { class: "text-foreground-muted text-sm mb-6", {t.page_subtitle} }
            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3",
                for issuer in snapshot.items.iter() {
                    a { href: "/issuers/{issuer.issuer_id}",
                        class: "bg-panel border border-border rounded-2xl p-5 hover:border-brand transition-colors block",
                        div { class: "flex items-center gap-2 mb-2 flex-wrap",
                            span { class: "text-xs px-2 py-0.5 rounded bg-panel-muted text-foreground-soft", { country_display(issuer.country) } }
                            span { class: "text-xs px-2 py-0.5 rounded bg-brand-soft text-brand", { issuer_status_label(issuer.status, &t) } }
                            if let Some(note) = &issuer.status_note {
                                span { class: "text-xs px-2 py-0.5 rounded bg-panel-muted text-foreground-muted", {note.clone()} }
                            }
                        }
                        h2 { class: "text-base font-bold mb-1", {issuer.name.clone()} }
                        p { class: "text-xs text-foreground-muted leading-relaxed line-clamp-3", {issuer.description.clone()} }
                    }
                }
            }
        }
    }
}
