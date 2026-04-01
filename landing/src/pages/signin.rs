use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn SignIn() -> Element {
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());

    rsx! {
        div {
            class: "min-h-screen flex items-center justify-center px-4",
            style: "background: #0a0e17;",
            div {
                class: "w-full max-w-md",
                div {
                    class: "text-center mb-8",
                    h1 { class: "text-3xl font-bold text-white", "Biyard" }
                    p { class: "text-gray-400 mt-1", "Launchpad Platform" }
                }
                div {
                    class: "rounded-2xl p-8",
                    style: "background: #0f1420; border: 1px solid rgba(0,212,170,0.12);",
                    h2 { class: "text-xl font-bold text-white mb-6", "Sign In" }
                    div {
                        class: "space-y-4",
                        div {
                            label { class: "block text-sm text-gray-400 mb-1", "Email" }
                            input {
                                r#type: "email",
                                class: "w-full px-4 py-3 rounded-lg text-sm text-white focus:outline-none",
                                style: "background: #0a0e17; border: 1px solid rgba(0,212,170,0.12);",
                                value: "{email}",
                                oninput: move |e| email.set(e.value())
                            }
                        }
                        div {
                            label { class: "block text-sm text-gray-400 mb-1", "Password" }
                            input {
                                r#type: "password",
                                class: "w-full px-4 py-3 rounded-lg text-sm text-white focus:outline-none",
                                style: "background: #0a0e17; border: 1px solid rgba(0,212,170,0.12);",
                                value: "{password}",
                                oninput: move |e| password.set(e.value())
                            }
                        }
                        Link {
                            to: Route::Wallet {},
                            class: "block w-full py-3 rounded-lg font-semibold text-center",
                            style: "background: #00d4aa; color: #0a0e17;",
                            "Sign In"
                        }
                    }
                }
            }
        }
    }
}
