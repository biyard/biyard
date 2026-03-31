use dioxus::prelude::*;
use dioxus_translate::use_translate;

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
            div { class: "bg-white dark:bg-gray-800 shadow rounded-lg p-12 text-center",
                svg {
                    class: "mx-auto h-12 w-12 text-gray-400",
                    xmlns: "http://www.w3.org/2000/svg", width: "24", height: "24",
                    view_box: "0 0 24 24", fill: "none", stroke: "currentColor",
                    stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                    polygon { points: "12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" }
                }
                h3 { class: "mt-2 text-sm font-medium text-gray-900 dark:text-white", {t.no_transactions} }
                p { class: "mt-1 text-sm text-gray-500 dark:text-gray-400", {t.no_transactions_desc} }
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
                        thead { class: "bg-gray-50 dark:bg-gray-700",
                            tr {
                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase", {t.transaction_type} }
                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase", {t.user_id} }
                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase", {t.amount} }
                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase", {t.month} }
                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase", {t.description} }
                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase", {t.created_at} }
                            }
                        }
                        tbody { class: "bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700",
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
                                            td { class: "px-6 py-4 whitespace-nowrap",
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
                                            td { class: "px-6 py-4 whitespace-nowrap",
                                                span { class: "text-sm text-gray-900 dark:text-white", "{user_id}" }
                                                if let Some(ref target_id) = target {
                                                    span { class: "text-sm text-gray-500 dark:text-gray-400", " → {target_id}" }
                                                }
                                            }
                                            td { class: "px-6 py-4 whitespace-nowrap",
                                                span {
                                                    class: format!("text-sm font-medium {amount_color}"),
                                                    "{format_amount(&tx_type, amount)}"
                                                }
                                            }
                                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "{month}" }
                                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "{desc}" }
                                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "{created}" }
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
