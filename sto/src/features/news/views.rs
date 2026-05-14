use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::features::catalog::views::Topbar;
use crate::features::news::NewsTranslate;

#[component]
pub fn NewsView() -> Element {
    let t: NewsTranslate = use_translate();
    rsx! {
        Topbar { active: "news".to_string() }
        main { class: "max-w-7xl mx-auto px-6 py-8",
            h1 { class: "text-xl font-bold mb-1", {t.page_title} }
            p { class: "text-foreground-muted text-sm mb-6", {t.page_subtitle} }
            section { class: "bg-panel border border-dashed border-border rounded-2xl p-12 text-center text-foreground-muted",
                {t.empty_body}
            }
        }
    }
}
