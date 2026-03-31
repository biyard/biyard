use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::common::ProjectPartition;
use crate::features::projects::ProjectResponse;
use crate::features::projects::i18n::ProjectsTranslate;

#[component]
pub fn OverviewTab(project_id: ReadSignal<ProjectPartition>, project: ProjectResponse) -> Element {
    let t: ProjectsTranslate = use_translate();

    let token = use_loader(move || async move {
        crate::features::tokens::controllers::get_token_handler(project_id()).await
    });

    let aggregation = use_loader(move || async move {
        let month = chrono::Utc::now().format("%Y-%m").to_string();
        crate::features::points::controllers::get_point_aggregation_handler(
            project_id(),
            month,
        )
        .await
    });

    rsx! {
        div { class: "space-y-6",
            // Project Overview Card
            div { class: "bg-white dark:bg-gray-800 shadow rounded-lg p-6",
                h3 { class: "text-lg font-medium text-gray-900 dark:text-white mb-4",
                    {t.overview}
                }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    dl { class: "space-y-4",
                        div {
                            dt { class: "text-sm font-medium text-gray-500 dark:text-gray-400",
                                {t.project_id}
                            }
                            dd { class: "mt-1",
                                code { class: "text-sm text-gray-900 dark:text-white bg-gray-100 dark:bg-gray-700 px-2 py-1 rounded",
                                    "{project.id}"
                                }
                            }
                        }
                        div {
                            dt { class: "text-sm font-medium text-gray-500 dark:text-gray-400",
                                {t.created_at}
                            }
                            dd { class: "mt-1 text-sm text-gray-900 dark:text-white",
                                "{format_timestamp(project.created_at)}"
                            }
                        }
                        div {
                            dt { class: "text-sm font-medium text-gray-500 dark:text-gray-400",
                                {t.updated_at}
                            }
                            dd { class: "mt-1 text-sm text-gray-900 dark:text-white",
                                "{format_timestamp(project.updated_at)}"
                            }
                        }
                    }
                    dl { class: "space-y-4",
                        div {
                            dt { class: "text-sm font-medium text-gray-500 dark:text-gray-400",
                                {t.monthly_supply}
                            }
                            dd { class: "mt-1 text-sm text-gray-900 dark:text-white",
                                "{format_number(project.monthly_token_supply)}"
                            }
                        }
                        div {
                            dt { class: "text-sm font-medium text-gray-500 dark:text-gray-400",
                                {t.exchange_ratio}
                            }
                            dd { class: "mt-1 text-sm text-gray-900 dark:text-white",
                                "1 : 1"
                            }
                        }
                        div {
                            dt { class: "text-sm font-medium text-gray-500 dark:text-gray-400",
                                {t.token_value}
                            }
                            dd { class: "mt-1 text-sm text-gray-900 dark:text-white",
                                "-"
                            }
                        }
                    }
                }
            }

            // Token Info Card
            div { class: "bg-white dark:bg-gray-800 shadow rounded-lg p-6",
                h3 { class: "text-lg font-medium text-gray-900 dark:text-white mb-4",
                    {t.token_info}
                }
                match &token {
                    Ok(tok) => {
                        let tok = &*tok.read();
                        rsx! {
                            div {
                                div { class: "flex items-center space-x-4 mb-6",
                                    div { class: "p-3 bg-blue-100 dark:bg-blue-900 rounded-full",
                                        svg {
                                            class: "h-8 w-8 text-blue-600 dark:text-blue-400",
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
                                    }
                                    div {
                                        h4 { class: "text-xl font-semibold text-gray-900 dark:text-white",
                                            "{tok.name}"
                                        }
                                        span { class: "px-2 py-1 text-xs font-semibold bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 rounded",
                                            "{tok.symbol}"
                                        }
                                    }
                                }
                                if let Some(ref desc) = tok.description {
                                    p { class: "text-gray-500 dark:text-gray-400 mb-6", "{desc}" }
                                }
                                div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                                    div { class: "bg-gray-50 dark:bg-gray-700 rounded-lg p-4",
                                        dt { class: "text-sm font-medium text-gray-500 dark:text-gray-400", {t.total_supply} }
                                        dd { class: "mt-1 text-2xl font-semibold text-gray-900 dark:text-white", "{format_number(tok.total_supply)}" }
                                    }
                                    div { class: "bg-gray-50 dark:bg-gray-700 rounded-lg p-4",
                                        dt { class: "text-sm font-medium text-gray-500 dark:text-gray-400", {t.circulating_supply} }
                                        dd { class: "mt-1 text-2xl font-semibold text-gray-900 dark:text-white", "{format_number(tok.circulating_supply)}" }
                                    }
                                    div { class: "bg-gray-50 dark:bg-gray-700 rounded-lg p-4",
                                        dt { class: "text-sm font-medium text-gray-500 dark:text-gray-400", {t.decimals} }
                                        dd { class: "mt-1 text-2xl font-semibold text-gray-900 dark:text-white", "{tok.decimals}" }
                                    }
                                }
                            }
                        }
                    },
                    Err(_) => rsx! {
                        div { class: "text-center py-8",
                            svg {
                                class: "mx-auto h-12 w-12 text-gray-400",
                                xmlns: "http://www.w3.org/2000/svg", width: "24", height: "24",
                                view_box: "0 0 24 24", fill: "none", stroke: "currentColor",
                                stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                circle { cx: "8", cy: "8", r: "6" }
                                path { d: "M18.09 10.37A6 6 0 1 1 10.34 18" }
                                path { d: "M7 6h1v4" }
                                path { d: "m16.71 13.88.7.71-2.82 2.82" }
                            }
                            h3 { class: "mt-2 text-sm font-medium text-gray-900 dark:text-white", {t.no_token} }
                            p { class: "mt-1 text-sm text-gray-500 dark:text-gray-400", {t.no_token_desc} }
                        }
                    },
                }
            }

            // Point Info Card
            div { class: "bg-white dark:bg-gray-800 shadow rounded-lg p-6",
                div { class: "flex items-center justify-between mb-4",
                    h3 { class: "text-lg font-medium text-gray-900 dark:text-white", {t.point_info} }
                }
                match &aggregation {
                    Ok(agg) => {
                        let agg = &*agg.read();
                        rsx! {
                            div {
                                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 mb-6",
                                    div { class: "bg-green-50 dark:bg-green-900/20 rounded-lg p-4",
                                        dt { class: "text-sm font-medium text-green-600 dark:text-green-400", {t.total_awarded} }
                                        dd { class: "mt-1 text-2xl font-semibold text-green-700 dark:text-green-300", "{format_number(agg.awarded_points)}" }
                                    }
                                    div { class: "bg-red-50 dark:bg-red-900/20 rounded-lg p-4",
                                        dt { class: "text-sm font-medium text-red-600 dark:text-red-400", {t.total_deducted} }
                                        dd { class: "mt-1 text-2xl font-semibold text-red-700 dark:text-red-300", "{format_number(agg.deducted_points)}" }
                                    }
                                }
                            }
                        }
                    },
                    Err(_) => rsx! {
                        div { class: "text-center py-8",
                            svg {
                                class: "mx-auto h-12 w-12 text-gray-400",
                                xmlns: "http://www.w3.org/2000/svg", width: "24", height: "24",
                                view_box: "0 0 24 24", fill: "none", stroke: "currentColor",
                                stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                polygon { points: "12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" }
                            }
                            h3 { class: "mt-2 text-sm font-medium text-gray-900 dark:text-white", {t.no_points_yet} }
                            p { class: "mt-1 text-sm text-gray-500 dark:text-gray-400", {t.no_points_desc} }
                        }
                    },
                }
            }
        }
    }
}

fn format_timestamp(ts: i64) -> String {
    let secs = ts / 1000;
    match chrono::DateTime::from_timestamp(secs, 0) {
        Some(dt) => dt.format("%Y-%m-%d %H:%M").to_string(),
        None => ts.to_string(),
    }
}

fn format_number(n: i64) -> String {
    let s = n.to_string();
    let bytes = s.as_bytes();
    let mut result = String::new();
    let len = bytes.len();
    for (i, &b) in bytes.iter().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push(',');
        }
        result.push(b as char);
    }
    result
}
