use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::common::ui::*;
use crate::features::accounts::context::{AccountContext, use_account_context};
use crate::features::accounts::i18n::SignInTranslate;

#[component]
pub fn SignIn() -> Element {
    let t: SignInTranslate = use_translate();
    let nav = use_navigator();
    let mut account_ctx = use_account_context();
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);

    let handle_submit = move |e: FormEvent| {
        e.prevent_default();
        let email_val = email();
        let password_val = password();

        spawn(async move {
            loading.set(true);
            error.set(None);

            match crate::features::accounts::controllers::signin_handler(email_val, password_val).await {
                Ok(resp) => {
                    account_ctx.set(AccountContext {
                        account: Some(resp),
                    });
                    nav.push(Route::Dashboard {});
                }
                Err(e) => {
                    error.set(Some(e.to_string()));
                }
            }
            loading.set(false);
        });
    };

    rsx! {
        div { class: "flex justify-center items-center px-4 min-h-screen bg-gray-50 dark:bg-gray-900",
            div { class: "space-y-8 w-full max-w-md",
                div { class: "text-center",
                    h1 { class: "text-4xl font-bold text-gray-900 dark:text-white",
                        {t.title}
                    }
                    p { class: "mt-2 text-sm text-gray-600 dark:text-gray-400",
                        {t.tagline}
                    }
                    h2 { class: "mt-6 text-3xl font-extrabold text-gray-900 dark:text-white",
                        {t.sign_in_heading}
                    }
                }

                form { class: "mt-8 space-y-6", method: "post", onsubmit: handle_submit,
                    if let Some(err) = error() {
                        AlertMessage { variant: AlertVariant::Error, "{err}" }
                    }

                    div { class: "space-y-4",
                        FormFieldWithIcon {
                            label: t.email,
                            id: "email",
                            r#type: "email",
                            value: email(),
                            oninput: move |e: FormEvent| email.set(e.value()),
                            placeholder: t.email_placeholder.to_string(),
                            autocomplete: "email",
                            icon: rsx! {
                                svg {
                                    class: "w-5 h-5 text-gray-400",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "24",
                                    height: "24",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    rect {
                                        width: "20",
                                        height: "16",
                                        x: "2",
                                        y: "4",
                                        rx: "2",
                                    }
                                    path { d: "m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" }
                                }
                            },
                        }

                        FormFieldWithIcon {
                            label: t.password,
                            id: "password",
                            r#type: "password",
                            value: password(),
                            oninput: move |e: FormEvent| password.set(e.value()),
                            placeholder: t.password_placeholder.to_string(),
                            autocomplete: "current-password",
                            icon: rsx! {
                                svg {
                                    class: "w-5 h-5 text-gray-400",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "24",
                                    height: "24",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    rect {
                                        width: "18",
                                        height: "11",
                                        x: "3",
                                        y: "11",
                                        rx: "2",
                                        ry: "2",
                                    }
                                    path { d: "M7 11V7a5 5 0 0 1 10 0v4" }
                                }
                            },
                        }
                    }

                    div {
                        SubmitBtn { disabled: loading(),
                            if loading() {
                                Spinner {}
                                {t.signing_in}
                            } else {
                                {t.sign_in}
                            }
                        }
                    }

                    div { class: "text-center",
                        p { class: "text-sm text-gray-600 dark:text-gray-400",
                            {t.no_account} " "
                            Link {
                                to: Route::SignUp {},
                                class: "font-medium text-blue-600 dark:text-blue-400 hover:text-blue-500",
                                {t.sign_up_link}
                            }
                        }
                    }
                }
            }
        }
    }
}
