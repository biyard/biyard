use dioxus::prelude::*;
use dioxus_translate::{Translate, use_translate};

use crate::common::use_language;
use crate::features::catalog::views::{StoTable, Topbar};
use crate::features::issuers::IssuersTranslate;
use crate::features::issuers::controllers::get_issuer_handler;

#[component]
pub fn IssuerDetailView(issuer_id: ReadSignal<String>) -> Element {
    let t: IssuersTranslate = use_translate();
    let lang = use_language();
    let lang_now = lang();
    let data = use_loader(move || async move { get_issuer_handler(issuer_id()).await })?;
    let r = data();

    rsx! {
        Topbar { active: "issuers".to_string() }
        main { class: "max-w-7xl mx-auto px-6 py-8",
            div { class: "flex items-center gap-2 text-xs text-foreground-muted mb-2",
                a { href: "/issuers", class: "hover:text-brand", {t.breadcrumb} }
                span { "/" }
                span { class: "text-foreground-soft", {r.issuer.name.clone()} }
            }
            h1 { class: "text-2xl font-bold mb-2", {r.issuer.name.clone()} }
            div { class: "flex gap-2 mb-4 flex-wrap",
                span { class: "text-xs px-2 py-1 rounded bg-panel-muted text-foreground-soft", { r.issuer.country.translate(&lang_now) } }
                span { class: "text-xs px-2 py-1 rounded bg-brand-soft text-brand", { r.issuer.status.translate(&lang_now) } }
                if let Some(note) = &r.issuer.status_note {
                    span { class: "text-xs px-2 py-1 rounded bg-panel-muted text-foreground-muted", {note.clone()} }
                }
                if let Some(s) = &r.issuer.sandbox {
                    span { class: "text-xs px-2 py-1 rounded bg-panel-muted text-foreground-muted", "{t.sandbox_label}: {s}" }
                }
            }
            p { class: "text-foreground-soft leading-relaxed mb-8 max-w-3xl", {r.issuer.description.clone()} }
            section {
                h2 { class: "text-base font-bold mb-3 pb-2 border-b border-border",
                    {t.section_assets} " "
                    span { class: "text-foreground-muted font-normal text-sm ml-1", "{r.stos.len()}" {t.count_unit} }
                }
                if r.stos.is_empty() {
                    div { class: "text-foreground-muted py-8 text-center", {t.empty_assets} }
                } else {
                    StoTable { items: r.stos.clone(), show_status: true }
                }
            }
        }
    }
}
