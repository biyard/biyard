use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn UserNav() -> Element {
    let current_url = use_route::<Route>();
    let is_wallet = matches!(current_url, Route::Wallet {});
    let is_dao = matches!(current_url, Route::Dao {});

    rsx! {
        nav {
            class: "h-14 flex items-center justify-between px-6",
            style: "background: #0a0e17; border-bottom: 1px solid rgba(0,212,170,0.12);",
            div {
                class: "flex items-center gap-6",
                Link {
                    to: Route::Home {},
                    class: "text-lg font-bold",
                    style: "color: #00d4aa;",
                    "Biyard"
                }
                div {
                    class: "flex gap-1",
                    Link {
                        to: Route::Wallet {},
                        class: if is_wallet { "px-3 py-1.5 rounded-lg text-sm font-medium text-white" } else { "px-3 py-1.5 rounded-lg text-sm font-medium text-gray-500 hover:text-white" },
                        style: if is_wallet { "background: #141c2b;" } else { "" },
                        "Wallet"
                    }
                    Link {
                        to: Route::Dao {},
                        class: if is_dao { "px-3 py-1.5 rounded-lg text-sm font-medium text-white" } else { "px-3 py-1.5 rounded-lg text-sm font-medium text-gray-500 hover:text-white" },
                        style: if is_dao { "background: #141c2b;" } else { "" },
                        "DAO"
                    }
                }
            }
            Link {
                to: Route::Home {},
                class: "text-sm text-red-400 hover:text-red-300",
                "Sign Out"
            }
        }
    }
}
