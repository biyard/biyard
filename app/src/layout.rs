use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div { class: "antialiased bg-bg min-h-screen",
            Outlet::<Route> {}
        }
    }
}
