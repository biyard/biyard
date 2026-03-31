use crate::common::ProjectPartition;
use crate::features::accounts::views::{SignIn, SignUp};
use crate::features::console::layout::Layout as ConsoleLayout;
use crate::features::console::views::{Dashboard, Settings};
use crate::features::credentials::views::Credentials;
use crate::features::projects::views::{ProjectDetail, Projects};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // Auth (no sidebar, public)
    #[route("/signin")]
    SignIn {},
    #[route("/signup")]
    SignUp {},

    // Console (sidebar + auth guard)
    #[layout(ConsoleLayout)]
        #[route("/")]
        RedirectToDashboard {},
        #[route("/dashboard")]
        Dashboard {},
        #[route("/projects")]
        Projects {},
        #[route("/projects/:project_id")]
        ProjectDetail { project_id: ProjectPartition },
        #[route("/credentials")]
        Credentials {},
        #[route("/settings")]
        Settings {},
    #[end_layout]

    #[route("/:..rest")]
    PageNotFound { rest: Vec<String> },
}

#[component]
fn RedirectToDashboard() -> Element {
    let nav = use_navigator();
    nav.push(Route::Dashboard {});
    rsx! {}
}

#[component]
fn PageNotFound(rest: Vec<String>) -> Element {
    rsx! {
        div { class: "flex items-center justify-center min-h-screen",
            h1 { class: "text-2xl", "Page not found" }
        }
    }
}
