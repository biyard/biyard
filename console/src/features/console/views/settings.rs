use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::features::accounts::context::use_account_context;
use crate::features::console::i18n::ConsoleTranslate;

#[component]
pub fn Settings() -> Element {
    let t: ConsoleTranslate = use_translate();
    let account_ctx = use_account_context();

    let nav = use_navigator();
    let mut show_delete_dialog = use_signal(|| false);

    let account = if let Some(account) = account_ctx().account {
        account
    } else {
        return rsx! {
            div { class: "text-gray-900 dark:text-white", {t.loading} }
        };
    };

    rsx! {
        div { class: "max-w-3xl",
            // Page Header
            div { class: "mb-6",
                h1 { class: "text-3xl font-bold text-gray-900 dark:text-white", {t.account_settings} }
            }

            // Profile Section
            div { class: "mb-6",
                div { class: "bg-white dark:bg-gray-800 shadow rounded-lg p-6",
                    h2 { class: "text-xl font-semibold text-gray-900 dark:text-white mb-4",
                        {t.profile}
                    }
                    div { class: "space-y-4",
                        div {
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                {t.name}
                            }
                            p { class: "mt-1 text-sm text-gray-900 dark:text-white",
                                "{account.name}"
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                {t.email}
                            }
                            p { class: "mt-1 text-sm text-gray-900 dark:text-white",
                                "{account.email}"
                            }
                        }
                        div {
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                {t.account_id}
                            }
                            p { class: "mt-1 text-sm text-gray-900 dark:text-white font-mono",
                                "{account.pk}"
                            }
                        }
                    }
                }
            }

            // Danger Zone
            div {
                div { class: "bg-white dark:bg-gray-800 shadow rounded-lg p-6 border-2 border-red-200 dark:border-red-900",
                    div { class: "flex items-start",
                        // AlertTriangle icon
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "24",
                            height: "24",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            class: "h-6 w-6 text-red-600 dark:text-red-400 mt-0.5",
                            path { d: "m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z" }
                            path { d: "M12 9v4" }
                            path { d: "M12 17h.01" }
                        }
                        div { class: "ml-3 flex-1",
                            h2 { class: "text-xl font-semibold text-red-600 dark:text-red-400 mb-2",
                                {t.delete_account}
                            }
                            p { class: "text-sm text-gray-600 dark:text-gray-400 mb-4",
                                {t.delete_account_desc}
                            }
                            button {
                                onclick: move |_| show_delete_dialog.set(true),
                                class: "px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2",
                                {t.delete_account}
                            }
                        }
                    }
                }
            }

            // Confirmation Dialog
            if show_delete_dialog() {
                div { class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50",
                    div { class: "bg-white dark:bg-gray-800 rounded-lg max-w-md w-full p-6",
                        div { class: "flex items-center mb-4",
                            // AlertTriangle icon
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                width: "24",
                                height: "24",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                class: "h-6 w-6 text-red-600 dark:text-red-400",
                                path { d: "m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z" }
                                path { d: "M12 9v4" }
                                path { d: "M12 17h.01" }
                            }
                            h3 { class: "ml-2 text-lg font-semibold text-gray-900 dark:text-white",
                                {t.delete_account_confirm}
                            }
                        }
                        p { class: "text-sm text-gray-600 dark:text-gray-400 mb-6",
                            {t.delete_account_warning}
                        }
                        div { class: "flex justify-end space-x-3",
                            button {
                                onclick: move |_| show_delete_dialog.set(false),
                                class: "px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-700 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600",
                                {t.cancel}
                            }
                            button {
                                onclick: move |_| {
                                    spawn(async move {
                                        let _ = crate::features::accounts::controllers::withdrawal_handler().await;
                                        nav.push(Route::SignIn {});
                                    });
                                },
                                class: "px-4 py-2 text-sm font-medium text-white bg-red-600 rounded-md hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2",
                                {t.confirm_delete}
                            }
                        }
                    }
                }
            }
        }
    }
}
