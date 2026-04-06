use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::common::ui::*;
use crate::common::ProjectPartition;
use crate::features::projects::i18n::ProjectsTranslate;

#[component]
pub fn PointsTab(project_id: ReadSignal<ProjectPartition>) -> Element {
    let t: ProjectsTranslate = use_translate();

    let transactions = use_loader(move || async move {
        crate::features::points::controllers::list_transactions_handler(project_id(), 50, None).await
    })?;

    let list = transactions();

    if list.items.is_empty() {
        rsx! {
            EmptyState {
                icon: rsx! {
                    svg {
                        class: "mx-auto h-12 w-12 text-gray-400",
                        xmlns: "http://www.w3.org/2000/svg", width: "24", height: "24",
                        view_box: "0 0 24 24", fill: "none", stroke: "currentColor",
                        stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        polygon { points: "12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" }
                    }
                },
                title: t.no_transactions.to_string(),
                description: t.no_transactions_desc.to_string(),
            }
        }
    } else {
        rsx! {
            div { class: "bg-white dark:bg-gray-800 shadow rounded-lg overflow-hidden",
                div { class: "px-6 py-4 border-b border-gray-200 dark:border-gray-700",
                    h3 { class: "text-lg font-medium text-gray-900 dark:text-white", {t.transactions} }
                }
                div { class: "overflow-x-auto",
                    table { class: "min-w-full divide-y divide-gray-200 dark:divide-gray-700",
                        TableHead {
                            TableHeadCell { {t.transaction_type} }
                            TableHeadCell { {t.user_id} }
                            TableHeadCell { {t.amount} }
                            TableHeadCell { {t.month} }
                            TableHeadCell { {t.description} }
                            TableHeadCell { {t.created_at} }
                        }
                        TableBody {
                            for tx in list.items.iter() {
                                {
                                    let tx_type = tx.transaction_type.to_string();
                                    let tx_type_color = get_transaction_type_color(&tx_type);
                                    let tx_type_icon_color = get_transaction_icon_color(&tx_type);
                                    let tx_type_icon_path = get_transaction_icon_path(&tx_type);
                                    let user_id = tx.meta_user_id.clone();
                                    let target = tx.target_user_id.clone();
                                    let amount = tx.amount;
                                    let amount_color = get_amount_color(&tx_type);
                                    let month = tx.month.clone();
                                    let desc = tx.description.clone().unwrap_or_else(|| "-".to_string());
                                    let created = format_timestamp(tx.created_at);
                                    rsx! {
                                        tr {
                                            TableCell {
                                                div { class: "flex items-center",
                                                    svg {
                                                        class: format!("h-4 w-4 {tx_type_icon_color}"),
                                                        xmlns: "http://www.w3.org/2000/svg", width: "24", height: "24",
                                                        view_box: "0 0 24 24", fill: "none", stroke: "currentColor",
                                                        stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                                        path { d: "{tx_type_icon_path}" }
                                                    }
                                                    span {
                                                        class: format!("ml-2 px-2 py-1 text-xs font-semibold rounded {tx_type_color}"),
                                                        "{tx_type}"
                                                    }
                                                }
                                            }
                                            TableCell {
                                                span { class: "text-sm text-gray-900 dark:text-white", "{user_id}" }
                                                if let Some(ref target_id) = target {
                                                    span { class: "text-sm text-gray-500 dark:text-gray-400", " → {target_id}" }
                                                }
                                            }
                                            TableCell {
                                                span {
                                                    class: format!("text-sm font-medium {amount_color}"),
                                                    "{format_amount(&tx_type, amount)}"
                                                }
                                            }
                                            TableCell { class: "text-sm text-gray-500 dark:text-gray-400", "{month}" }
                                            TableCell { class: "text-sm text-gray-500 dark:text-gray-400", "{desc}" }
                                            TableCell { class: "text-sm text-gray-500 dark:text-gray-400", "{created}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn get_transaction_type_color(tx_type: &str) -> &'static str {
    match tx_type {
        "Award" => "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
        "Deduct" => "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200",
        "Transfer" => "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200",
        "Exchange" => "bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-200",
        _ => "bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200",
    }
}

fn get_transaction_icon_color(tx_type: &str) -> &'static str {
    match tx_type {
        "Award" => "text-green-500",
        "Deduct" => "text-red-500",
        "Transfer" => "text-blue-500",
        "Exchange" => "text-purple-500",
        _ => "text-gray-500",
    }
}

fn get_transaction_icon_path(tx_type: &str) -> &'static str {
    match tx_type {
        "Award" => "M7 7h10v10 M7 17L17 7",
        "Deduct" => "M7 7h10v10 M17 17L7 7",
        "Transfer" => "M8 3L4 7l4 4 M4 7h16 M16 21l4-4-4-4 M20 17H4",
        "Exchange" => "M17 1l4 4-4 4 M3 11V9a4 4 0 014-4h14 M7 23l-4-4 4-4 M21 13v2a4 4 0 01-4 4H3",
        _ => "M12 2l3.09 6.26L22 9.27l-5 4.87L18.18 21 12 17.77 5.82 21 7 14.14l-5-4.87 6.91-1.01L12 2",
    }
}

fn get_amount_color(tx_type: &str) -> &'static str {
    match tx_type {
        "Award" => "text-green-600 dark:text-green-400",
        "Deduct" => "text-red-600 dark:text-red-400",
        _ => "text-gray-900 dark:text-white",
    }
}

fn format_amount(tx_type: &str, amount: i64) -> String {
    match tx_type {
        "Award" => format!("+{}", format_number(amount)),
        "Deduct" => format!("-{}", format_number(amount)),
        _ => format_number(amount),
    }
}
