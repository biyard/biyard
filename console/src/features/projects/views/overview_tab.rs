use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::common::ProjectPartition;
use crate::common::ui::*;
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

    let floor_price = match &token {
        Ok(tok) => {
            let total_supply = tok.read().total_supply;
            if total_supply > 0 {
                (project.treasury_balance as f64) / (total_supply as f64)
            } else {
                0.0
            }
        }
        Err(_) => 0.0,
    };

    rsx! {
        div { class: "space-y-6",
            // Project Overview Card
            SectionCard {
                SectionTitle { {t.overview} }
                if project.brand_logo_url.is_some() {
                    div { class: "mb-6 p-4 rounded-lg bg-gray-50 dark:bg-gray-700/40",
                        div { class: "flex items-center gap-4",
                            if let Some(logo) = &project.brand_logo_url {
                                img {
                                    src: "{logo}",
                                    alt: "brand-logo",
                                    class: "h-14 w-14 rounded-lg object-cover border border-gray-200 dark:border-gray-600 bg-white",
                                }
                            } else {
                                div { class: "h-14 w-14 rounded-lg bg-gray-200 dark:bg-gray-600" }
                            }
                            div {
                                p { class: "text-xs text-gray-500 dark:text-gray-400", {t.brand} }
                                p { class: "text-base font-semibold text-gray-900 dark:text-white",
                                    "{project.name}"
                                }
                            }
                        }
                    }
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
            SectionCard {
                SectionTitle { {t.token_info} }
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
                                    StatCard { color: StatColor::Gray, label: t.total_supply.to_string(), value: format_number(tok.total_supply) }
                                    StatCard { color: StatColor::Gray, label: t.circulating_supply.to_string(), value: format_number(tok.circulating_supply) }
                                    StatCard { color: StatColor::Gray, label: t.decimals.to_string(), value: tok.decimals.to_string() }
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
            SectionCard {
                SectionTitle { {t.point_info} }
                match &aggregation {
                    Ok(agg) => {
                        let agg = &*agg.read();
                        rsx! {
                            div {
                                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 mb-6",
                                    StatCard { color: StatColor::Green, label: t.total_awarded.to_string(), value: format_number(agg.awarded_points) }
                                    StatCard { color: StatColor::Red, label: t.total_deducted.to_string(), value: format_number(agg.deducted_points) }
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

            // Treasury Simulation Card
            SectionCard {
                SectionTitle { {t.treasury_simulation} }
                div { class: "grid grid-cols-1 md:grid-cols-4 gap-4",
                    StatCard { color: StatColor::Emerald, label: t.treasury_balance.to_string(), value: format_number(project.treasury_balance) }
                    StatCard { color: StatColor::Indigo, label: t.simulated_sales_total.to_string(), value: format_number(project.simulated_sales_total) }
                    StatCard { color: StatColor::Blue, label: t.treasury_reserve_rate.to_string(), value: format!("{}%", (project.treasury_reserve_rate * 100.0).round()) }
                    StatCard { color: StatColor::Amber, label: t.estimated_floor_price.to_string(), value: format_floor_price(floor_price) }
                }
                p { class: "mt-3 text-xs text-gray-500 dark:text-gray-400",
                    {t.floor_price_formula}
                }
            }
        }
    }
}
