use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::features::accounts::context::{AccountContext, use_account_context};
use crate::features::accounts::i18n::SignUpTranslate;

#[component]
pub fn SignUp() -> Element {
    let t: SignUpTranslate = use_translate();
    let nav = use_navigator();
    let mut account_ctx = use_account_context();
    let mut name = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut confirm_password = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);

    let handle_submit = move |e: FormEvent| {
        e.prevent_default();

        if password() != confirm_password() {
            error.set(Some(t.password_mismatch.to_string()));
            return;
        }

        let name_val = name();
        let email_val = email();
        let password_val = password();

        spawn(async move {
            loading.set(true);
            error.set(None);

            match crate::features::accounts::controllers::signup_handler(
                name_val,
                email_val,
                password_val,
            )
            .await
            {
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
                        {t.sign_up_heading}
                    }
                }

                form { class: "mt-8 space-y-6", method: "post", onsubmit: handle_submit,
                    if let Some(err) = error() {
                        div { class: "p-4 bg-red-50 rounded-md dark:bg-red-900/20",
                            p { class: "text-sm text-red-800 dark:text-red-400",
                                "{err}"
                            }
                        }
                    }

                    div { class: "space-y-4",
                        // Name field
                        div {
                            label {
                                r#for: "name",
                                class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                {t.name}
                            }
                            div { class: "relative mt-1",
                                div { class: "flex absolute inset-y-0 left-0 items-center pl-3 pointer-events-none",
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
                                        path { d: "M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2" }
                                        circle { cx: "12", cy: "7", r: "4" }
                                    }
                                }
                                input {
                                    id: "name",
                                    name: "name",
                                    r#type: "text",
                                    autocomplete: "name",
                                    required: true,
                                    value: "{name}",
                                    oninput: move |e: FormEvent| name.set(e.value()),
                                    class: "block py-2 pr-3 pl-10 w-full placeholder-gray-400 rounded-md border border-gray-300 shadow-sm appearance-none dark:text-white dark:bg-gray-800 dark:border-gray-600 focus:border-blue-500 focus:ring-blue-500 focus:outline-none",
                                    placeholder: "{t.name_placeholder}",
                                }
                            }
                        }

                        // Email field
                        div {
                            label {
                                r#for: "email",
                                class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                {t.email}
                            }
                            div { class: "relative mt-1",
                                div { class: "flex absolute inset-y-0 left-0 items-center pl-3 pointer-events-none",
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
                                }
                                input {
                                    id: "email",
                                    name: "email",
                                    r#type: "email",
                                    autocomplete: "email",
                                    required: true,
                                    value: "{email}",
                                    oninput: move |e: FormEvent| email.set(e.value()),
                                    class: "block py-2 pr-3 pl-10 w-full placeholder-gray-400 rounded-md border border-gray-300 shadow-sm appearance-none dark:text-white dark:bg-gray-800 dark:border-gray-600 focus:border-blue-500 focus:ring-blue-500 focus:outline-none",
                                    placeholder: "{t.email_placeholder}",
                                }
                            }
                        }

                        // Password field
                        div {
                            label {
                                r#for: "password",
                                class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                {t.password}
                            }
                            div { class: "relative mt-1",
                                div { class: "flex absolute inset-y-0 left-0 items-center pl-3 pointer-events-none",
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
                                }
                                input {
                                    id: "password",
                                    name: "password",
                                    r#type: "password",
                                    autocomplete: "new-password",
                                    required: true,
                                    value: "{password}",
                                    oninput: move |e: FormEvent| password.set(e.value()),
                                    class: "block py-2 pr-3 pl-10 w-full placeholder-gray-400 rounded-md border border-gray-300 shadow-sm appearance-none dark:text-white dark:bg-gray-800 dark:border-gray-600 focus:border-blue-500 focus:ring-blue-500 focus:outline-none",
                                    placeholder: "{t.password_placeholder}",
                                }
                            }
                        }

                        // Confirm Password field
                        div {
                            label {
                                r#for: "confirm_password",
                                class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                {t.confirm_password}
                            }
                            div { class: "relative mt-1",
                                div { class: "flex absolute inset-y-0 left-0 items-center pl-3 pointer-events-none",
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
                                }
                                input {
                                    id: "confirm_password",
                                    name: "confirm_password",
                                    r#type: "password",
                                    autocomplete: "new-password",
                                    required: true,
                                    value: "{confirm_password}",
                                    oninput: move |e: FormEvent| confirm_password.set(e.value()),
                                    class: "block py-2 pr-3 pl-10 w-full placeholder-gray-400 rounded-md border border-gray-300 shadow-sm appearance-none dark:text-white dark:bg-gray-800 dark:border-gray-600 focus:border-blue-500 focus:ring-blue-500 focus:outline-none",
                                    placeholder: "{t.confirm_password_placeholder}",
                                }
                            }
                        }
                    }

                    div {
                        button {
                            r#type: "submit",
                            disabled: loading(),
                            class: "flex justify-center py-2 px-4 w-full text-sm font-medium text-white bg-blue-600 rounded-md border border-transparent shadow-sm hover:bg-blue-700 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:outline-none disabled:opacity-50 disabled:cursor-not-allowed",
                            if loading() {
                                svg {
                                    class: "mr-2 -ml-1 w-5 h-5 animate-spin",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "24",
                                    height: "24",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    path { d: "M21 12a9 9 0 1 1-6.219-8.56" }
                                }
                                {t.signing_up}
                            } else {
                                {t.sign_up}
                            }
                        }
                    }

                    div { class: "text-center",
                        p { class: "text-sm text-gray-600 dark:text-gray-400",
                            {t.have_account} " "
                            Link {
                                to: Route::SignIn {},
                                class: "font-medium text-blue-600 dark:text-blue-400 hover:text-blue-500",
                                {t.sign_in_link}
                            }
                        }
                    }
                }
            }
        }
    }
}
