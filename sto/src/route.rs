use dioxus::prelude::*;

use crate::features::catalog::views::{CatalogView, DetailView, HomeView};

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[route("/")]
    HomeView {},

    #[route("/assets")]
    CatalogView {},

    #[route("/sto/:sto_id")]
    DetailView { sto_id: String },

    #[route("/:..rest")]
    PageNotFound { rest: Vec<String> },
}

#[component]
fn PageNotFound(rest: Vec<String>) -> Element {
    rsx! {
        div { class: "min-h-screen flex items-center justify-center text-ink-soft",
            div { class: "text-center",
                h1 { class: "text-2xl font-bold mb-2", "404" }
                p { class: "text-sm text-muted", "Page not found: /{rest.join(\"/\")}" }
                a { href: "/", class: "text-brand text-sm mt-4 inline-block", "← 홈으로" }
            }
        }
    }
}
