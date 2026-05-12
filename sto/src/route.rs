use dioxus::prelude::*;

use crate::features::catalog::views::HomeView;
use crate::features::catalog::views::CatalogView;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[route("/")]
    HomeView {},

    #[route("/assets")]
    CatalogView {},

    #[route("/:..rest")]
    PageNotFound { rest: Vec<String> },
}

#[component]
fn PageNotFound(rest: Vec<String>) -> Element {
    rsx! {
        div { class: "min-h-screen flex items-center justify-center text-ink-soft",
            div {
                h1 { class: "text-2xl font-bold mb-2", "404" }
                p { "Page not found: /{rest.join(\"/\")}" }
            }
        }
    }
}
