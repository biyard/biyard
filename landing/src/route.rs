use dioxus::prelude::*;

use crate::pages::{Dao, Home, LemoutonDemo, Pricing, SignIn, Team, Wallet};

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
    #[route("/demo/lemouton")]
    LemoutonDemo {},
}
