use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::common::ui::*;
use crate::features::accounts::context::use_account_context;
use crate::features::console::i18n::ConsoleTranslate;

#[component]
pub fn Dashboard() -> Element {
    let t: ConsoleTranslate = use_translate();
    let account_ctx = use_account_context();

    let account = if let Some(account) = account_ctx().account {
        account
    } else {
        return rsx! {
            div { class: "text-gray-900 dark:text-white", {t.loading} }
        };
    };

    rsx! {
        div {
            PageHeader {
                title: t.biyard_console.to_string(),
                subtitle: t.tagline.to_string(),
            }

            // Welcome Section
            div { class: "mb-6",
                SectionCard {
                    h2 { class: "text-2xl font-bold text-gray-900 dark:text-white mb-2",
                        {t.welcome}
                    }
                    p { class: "text-gray-600 dark:text-gray-400",
                        "{t.my_account}: {account.name} ({account.email})"
                    }
                }
            }

            // Quick Actions Grid
            div { class: "mb-6",
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    // My Projects Card
                    Link {
                        to: Route::Projects {},
                        class: "bg-white dark:bg-gray-800 shadow rounded-lg p-6 hover:shadow-lg transition-shadow",
                        div { class: "flex items-center",
                            div { class: "flex-shrink-0",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "32",
                                    height: "32",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    class: "h-8 w-8 text-purple-600 dark:text-purple-400",
                                    path { d: "M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z" }
                                    rect { x: "8", y: "10", width: "4", height: "6", rx: "1" }
                                    rect { x: "14", y: "10", width: "4", height: "4", rx: "1" }
                                }
                            }
                            div { class: "ml-4",
                                h3 { class: "text-lg font-medium text-gray-900 dark:text-white",
                                    {t.my_projects}
                                }
                                p { class: "mt-1 text-sm text-gray-600 dark:text-gray-400",
                                    {t.my_projects_desc}
                                }
                            }
                        }
                    }

                    // API Credentials Card
                    Link {
                        to: Route::Credentials {},
                        class: "bg-white dark:bg-gray-800 shadow rounded-lg p-6 hover:shadow-lg transition-shadow",
                        div { class: "flex items-center",
                            div { class: "flex-shrink-0",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "32",
                                    height: "32",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    class: "h-8 w-8 text-blue-600 dark:text-blue-400",
                                    path { d: "m15.5 7.5 2.3 2.3a1 1 0 0 0 1.4 0l2.1-2.1a1 1 0 0 0 0-1.4L19 4" }
                                    path { d: "m21 2-9.6 9.6" }
                                    circle { cx: "7.5", cy: "15.5", r: "5.5" }
                                }
                            }
                            div { class: "ml-4",
                                h3 { class: "text-lg font-medium text-gray-900 dark:text-white",
                                    {t.api_credentials}
                                }
                                p { class: "mt-1 text-sm text-gray-600 dark:text-gray-400",
                                    {t.api_credentials_desc}
                                }
                            }
                        }
                    }

                    // Account Settings Card
                    Link {
                        to: Route::Settings {},
                        class: "bg-white dark:bg-gray-800 shadow rounded-lg p-6 hover:shadow-lg transition-shadow",
                        div { class: "flex items-center",
                            div { class: "flex-shrink-0",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "32",
                                    height: "32",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    class: "h-8 w-8 text-gray-600 dark:text-gray-400",
                                    path { d: "M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" }
                                    circle { cx: "12", cy: "12", r: "3" }
                                }
                            }
                            div { class: "ml-4",
                                h3 { class: "text-lg font-medium text-gray-900 dark:text-white",
                                    {t.account_settings}
                                }
                                p { class: "mt-1 text-sm text-gray-600 dark:text-gray-400",
                                    "{t.profile}, {t.security}"
                                }
                            }
                        }
                    }
                }
            }

            // Account Info
            div {
                SectionCard {
                    SectionTitle { {t.profile} }
                    dl { class: "grid grid-cols-1 gap-4 sm:grid-cols-2",
                        div {
                            dt { class: "text-sm font-medium text-gray-500 dark:text-gray-400",
                                {t.name}
                            }
                            dd { class: "mt-1 text-sm text-gray-900 dark:text-white",
                                "{account.name}"
                            }
                        }
                        div {
                            dt { class: "text-sm font-medium text-gray-500 dark:text-gray-400",
                                {t.email}
                            }
                            dd { class: "mt-1 text-sm text-gray-900 dark:text-white",
                                "{account.email}"
                            }
                        }
                        div {
                            dt { class: "text-sm font-medium text-gray-500 dark:text-gray-400",
                                {t.account_id}
                            }
                            dd { class: "mt-1 text-sm text-gray-900 dark:text-white font-mono",
                                "{account.pk}"
                            }
                        }
                        div {
                            dt { class: "text-sm font-medium text-gray-500 dark:text-gray-400",
                                {t.created_at}
                            }
                            dd { class: "mt-1 text-sm text-gray-900 dark:text-white",
                                {format_timestamp(account.created_at)}
                            }
                        }
                    }
                }
            }
        }
    }
}
