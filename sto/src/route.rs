use dioxus::prelude::*;

use crate::features::biyard_index::views::{BiyardIndexView, WhitepaperView};
use crate::features::catalog::views::{CatalogView, DetailView, HomeView};
use crate::features::issuers::views::{IssuerDetailView, IssuerListView};
use crate::features::news::NewsView;
use crate::features::pricing::PricingView;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[route("/")]
    HomeView {},

    #[route("/market")]
    CatalogView {},

    #[route("/sto/:sto_id")]
    DetailView { sto_id: String },

    #[route("/issuers")]
    IssuerListView {},

    #[route("/issuers/:issuer_id")]
    IssuerDetailView { issuer_id: String },

    #[route("/biyard-index")]
    BiyardIndexView {},

    #[route("/biyard-index/whitepaper")]
    WhitepaperView {},

    #[route("/news")]
    NewsView {},

    #[route("/pricing")]
    PricingView {},

    #[route("/:..rest")]
    PageNotFound { rest: Vec<String> },
}

#[component]
fn PageNotFound(rest: Vec<String>) -> Element {
    rsx! {
        div { class: "min-h-screen flex items-center justify-center text-foreground-soft",
            div { class: "text-center",
                h1 { class: "text-2xl font-bold mb-2", "404" }
                p { class: "text-sm text-foreground-muted", "Page not found: /{rest.join(\"/\")}" }
                a { href: "/", class: "text-brand text-sm mt-4 inline-block", "← 홈으로" }
            }
        }
    }
}
