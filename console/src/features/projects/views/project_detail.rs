use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::common::ProjectPartition;
use crate::features::projects::ProjectStatus;
use crate::features::projects::i18n::ProjectsTranslate;

use super::overview_tab::OverviewTab;
use super::points_tab::PointsTab;
use super::settings_tab::SettingsTab;
use super::tokens_tab::TokensTab;

#[component]
pub fn ProjectDetail(project_id: ReadSignal<ProjectPartition>) -> Element {
    let t: ProjectsTranslate = use_translate();
    let nav = use_navigator();
    let mut tab_value = use_signal(|| "overview".to_string());

    let project = use_loader(move || async move {
        crate::features::projects::controllers::get_project_handler(project_id()).await
    })?;

    let project_data = project();

    rsx! {
        div {
            // Header
            div { class: "mb-6",
                button {
                    class: "flex items-center text-sm text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 mb-4",
                    onclick: move |_| { nav.push(Route::Projects {}); },
                    svg {
                        class: "h-4 w-4 mr-1",
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "m12 19-7-7 7-7" }
                        path { d: "M19 12H5" }
                    }
                    {t.back_to_projects}
                }

                div { class: "flex items-center justify-between",
                    div {
                        h1 { class: "text-3xl font-bold text-gray-900 dark:text-white",
                            "{project_data.name}"
                        }
                        if let Some(ref desc) = project_data.description {
                            p { class: "mt-1 text-gray-500 dark:text-gray-400",
                                "{desc}"
                            }
                        }
                    }
                    span {
                        class: format!(
                            "px-3 py-1 text-sm font-semibold rounded-full {}",
                            match project_data.status {
                                ProjectStatus::Active => "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
                                ProjectStatus::Inactive => "bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200",
                            }
                        ),
                        match project_data.status {
                            ProjectStatus::Active => {t.active},
                            ProjectStatus::Inactive => {t.inactive},
                        }
                    }
                }
            }

            // Tabs
            div { class: "border-b border-gray-200 dark:border-gray-700 mb-6",
                nav { class: "-mb-px flex space-x-8",
                    button {
                        class: format!(
                            "flex items-center py-4 px-1 border-b-2 text-sm font-medium {}",
                            if tab_value() == "overview" {
                                "border-blue-500 text-blue-600 dark:text-blue-400"
                            } else {
                                "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 dark:text-gray-400 dark:hover:text-gray-200"
                            }
                        ),
                        onclick: move |_| tab_value.set("overview".to_string()),
                        svg {
                            class: "h-4 w-4",
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "24",
                            height: "24",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            rect { width: "7", height: "9", x: "3", y: "3", rx: "1" }
                            rect { width: "7", height: "5", x: "14", y: "3", rx: "1" }
                            rect { width: "7", height: "9", x: "14", y: "12", rx: "1" }
                            rect { width: "7", height: "5", x: "3", y: "16", rx: "1" }
                        }
                        span { class: "ml-2", {t.overview} }
                    }

                    button {
                        class: format!(
                            "flex items-center py-4 px-1 border-b-2 text-sm font-medium {}",
                            if tab_value() == "tokens" {
                                "border-blue-500 text-blue-600 dark:text-blue-400"
                            } else {
                                "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 dark:text-gray-400 dark:hover:text-gray-200"
                            }
                        ),
                        onclick: move |_| tab_value.set("tokens".to_string()),
                        svg {
                            class: "h-4 w-4",
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "24",
                            height: "24",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            circle { cx: "8", cy: "8", r: "6" }
                            path { d: "M18.09 10.37A6 6 0 1 1 10.34 18" }
                            path { d: "M7 6h1v4" }
                            path { d: "m16.71 13.88.7.71-2.82 2.82" }
                        }
                        span { class: "ml-2", {t.tokens} }
                    }

                    button {
                        class: format!(
                            "flex items-center py-4 px-1 border-b-2 text-sm font-medium {}",
                            if tab_value() == "points" {
                                "border-blue-500 text-blue-600 dark:text-blue-400"
                            } else {
                                "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 dark:text-gray-400 dark:hover:text-gray-200"
                            }
                        ),
                        onclick: move |_| tab_value.set("points".to_string()),
                        svg {
                            class: "h-4 w-4",
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "24",
                            height: "24",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polygon { points: "12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" }
                        }
                        span { class: "ml-2", {t.points} }
                    }

                    button {
                        class: format!(
                            "flex items-center py-4 px-1 border-b-2 text-sm font-medium {}",
                            if tab_value() == "settings" {
                                "border-blue-500 text-blue-600 dark:text-blue-400"
                            } else {
                                "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 dark:text-gray-400 dark:hover:text-gray-200"
                            }
                        ),
                        onclick: move |_| tab_value.set("settings".to_string()),
                        svg {
                            class: "h-4 w-4",
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "24",
                            height: "24",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" }
                            circle { cx: "12", cy: "12", r: "3" }
                        }
                        span { class: "ml-2", {t.settings_tab} }
                    }
                }
            }

            // Tab Content
            if tab_value() == "overview" {
                OverviewTab {
                    project_id: project_id,
                    project: project_data.clone(),
                }
            }
            if tab_value() == "tokens" {
                TokensTab { project_id: project_id }
            }
            if tab_value() == "points" {
                PointsTab { project_id: project_id }
            }
            if tab_value() == "settings" {
                SettingsTab {
                    project_id: project_id,
                    project: project_data.clone(),
                }
            }
        }
    }
}
