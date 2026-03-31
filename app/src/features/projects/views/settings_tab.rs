use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::common::ProjectPartition;
use crate::features::projects::ProjectResponse;
use crate::features::projects::i18n::ProjectsTranslate;

#[component]
pub fn SettingsTab(project_id: ReadSignal<ProjectPartition>, project: ProjectResponse) -> Element {
    let t: ProjectsTranslate = use_translate();
    let nav = use_navigator();
    let mut show_delete = use_signal(|| false);

    rsx! {
        div { class: "bg-white dark:bg-gray-800 shadow rounded-lg p-6",
            h3 { class: "text-lg font-medium text-gray-900 dark:text-white mb-4",
                {t.settings_tab}
            }
            p { class: "text-gray-500 dark:text-gray-400",
                {t.settings_placeholder}
            }
        }

        // Delete confirmation dialog
        if show_delete() {
            div {
                class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50",
                div { class: "bg-white dark:bg-gray-800 rounded-lg max-w-md w-full p-6",
                    h3 { class: "text-lg font-semibold text-gray-900 dark:text-white mb-4",
                        {t.delete_project}
                    }
                    p { class: "text-sm text-gray-500 dark:text-gray-400 mb-6",
                        {t.delete_confirm}
                    }
                    div { class: "flex justify-end space-x-3",
                        button {
                            class: "px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-700 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600",
                            onclick: move |_| show_delete.set(false),
                            {t.cancel}
                        }
                        button {
                            class: "px-4 py-2 text-sm font-medium text-white bg-red-600 rounded-md hover:bg-red-700",
                            onclick: move |_| {
                                let pid = project_id();
                                spawn(async move {
                                    let _ = crate::features::projects::controllers::delete_project_handler(pid).await;
                                    nav.push(Route::Projects {});
                                });
                            },
                            {t.delete}
                        }
                    }
                }
            }
        }
    }
}
