use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        Router::<Route> {}
    }
}
