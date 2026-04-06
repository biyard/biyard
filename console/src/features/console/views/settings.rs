use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::common::components::dialog::*;
use crate::common::ui::*;
use crate::features::accounts::context::use_account_context;
use crate::features::console::i18n::ConsoleTranslate;

#[component]
pub fn Settings() -> Element {
    let t: ConsoleTranslate = use_translate();
    let account_ctx = use_account_context();

    let nav = use_navigator();
    let mut show_delete_dialog = use_signal(|| false);

    let Some(account) = account_ctx().account else {
        return rsx! {};
    };

    rsx! {
        div { class: "max-w-3xl",
            PageHeader { title: t.account_settings.to_string() }

            // Profile Section
            div { class: "mb-6",
                SectionCard {
                    h2 { class: "text-xl font-semibold text-gray-900 dark:text-white mb-4",
                        {t.profile}
                    }
                    div { class: "space-y-4",
                        div {
                            FormLabel { {t.name} }
                            p { class: "mt-1 text-sm text-gray-900 dark:text-white",
                                "{account.name}"
                            }
                        }
                        div {
                            FormLabel { {t.email} }
                            p { class: "mt-1 text-sm text-gray-900 dark:text-white",
                                "{account.email}"
                            }
                        }
                        div {
                            FormLabel { {t.account_id} }
                            p { class: "mt-1 text-sm text-gray-900 dark:text-white font-mono",
                                "{account.pk}"
                            }
                        }
                    }
                }
            }

            // Danger Zone
            div {
                DangerCard {
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
                            Btn {
                                variant: BtnVariant::Danger,
                                onclick: move |_| show_delete_dialog.set(true),
                                {t.delete_account}
                            }
                        }
                    }
                }
            }

            // Confirmation Dialog
            DialogRoot {
                open: show_delete_dialog(),
                on_open_change: move |v| show_delete_dialog.set(v),
                DialogContent {
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
                    DialogActions {
                        Btn {
                            variant: BtnVariant::Secondary,
                            onclick: move |_| show_delete_dialog.set(false),
                            {t.cancel}
                        }
                        Btn {
                            variant: BtnVariant::Danger,
                            onclick: move |_| {
                                spawn(async move {
                                    let _ = crate::features::accounts::controllers::withdrawal_handler().await;
                                    nav.push(Route::SignIn {});
                                });
                            },
                            {t.confirm_delete}
                        }
                    }
                }
            }
        }
    }
}
