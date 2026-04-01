use dioxus::prelude::*;

mod components;
mod pages;

use pages::{Home, SignIn, Wallet, Dao, Team, Pricing};

fn main() {
    dioxus::logger::init(tracing::Level::INFO).expect("failed to init logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/signin")]
    SignIn {},
    #[route("/wallet")]
    Wallet {},
    #[route("/dao")]
    Dao {},
    #[route("/team")]
    Team {},
    #[route("/pricing")]
    Pricing {},
}
