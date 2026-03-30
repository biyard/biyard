use crate::layout::AppLayout;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Home {},
    #[end_layout]

    #[route("/:..rest")]
    PageNotFound { rest: Vec<String> },
}

#[component]
fn Home() -> Element {
    rsx! {
        div { class: "flex items-center justify-center min-h-screen",
            h1 { class: "text-3xl font-bold text-fg", "Biyard" }
        }
    }
}

#[component]
fn PageNotFound(rest: Vec<String>) -> Element {
    rsx! {
        div { class: "flex items-center justify-center min-h-screen",
            h1 { class: "text-2xl", "Page not found" }
        }
    }
}
