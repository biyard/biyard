use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::features::projects::ProjectStatus;
use crate::features::projects::i18n::ProjectsTranslate;

#[component]
pub fn Projects() -> Element {
    let t: ProjectsTranslate = use_translate();
    let nav = use_navigator();
    let mut dialog_open = use_signal(|| false);

    let mut projects = use_loader(move || async move {
        crate::features::projects::controllers::list_projects_handler(100, None).await
    })?;

    let projects_data = projects();

    rsx! {
        div {
            // Page Header
            div { class: "mb-6 flex items-center justify-between",
                div {
                    h1 { class: "text-3xl font-bold text-gray-900 dark:text-white",
                        {t.title}
                    }
                }
                button {
                    class: "flex items-center px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700",
                    onclick: move |_| dialog_open.set(true),
                    // Plus icon
                    svg {
                        class: "h-5 w-5 mr-2",
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M5 12h14" }
                        path { d: "M12 5v14" }
                    }
                    {t.create_new}
                }
            }

            // Main Content
            div {
                if projects_data.items.is_empty() {
                    // Empty state
                    div { class: "bg-white dark:bg-gray-800 shadow rounded-lg p-12 text-center",
                        // FolderOpen icon
                        svg {
                            class: "mx-auto h-12 w-12 text-gray-400",
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "24",
                            height: "24",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "m6 14 1.5-2.9A2 2 0 0 1 9.24 10H20a2 2 0 0 1 1.94 2.5l-1.54 6a2 2 0 0 1-1.95 1.5H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H18a2 2 0 0 1 2 2v2" }
                        }
                        h3 { class: "mt-2 text-sm font-medium text-gray-900 dark:text-white",
                            {t.no_projects}
                        }
                        p { class: "mt-1 text-sm text-gray-500 dark:text-gray-400",
                            {t.no_projects_desc}
                        }
                        div { class: "mt-6",
                            button {
                                class: "inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700",
                                onclick: move |_| dialog_open.set(true),
                                // Plus icon
                                svg {
                                    class: "h-5 w-5 mr-2",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "24",
                                    height: "24",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    path { d: "M5 12h14" }
                                    path { d: "M12 5v14" }
                                }
                                {t.create_new}
                            }
                        }
                    }
                } else {
                    // Table
                    div { class: "bg-white dark:bg-gray-800 shadow rounded-lg overflow-hidden",
                        table { class: "min-w-full divide-y divide-gray-200 dark:divide-gray-700",
                            thead { class: "bg-gray-50 dark:bg-gray-700",
                                tr {
                                    th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase",
                                        {t.project_name}
                                    }
                                    th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase",
                                        {t.project_id}
                                    }
                                    th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase",
                                        {t.monthly_supply}
                                    }
                                    th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase",
                                        {t.status}
                                    }
                                    th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase",
                                        {t.actions}
                                    }
                                }
                            }
                            tbody { class: "bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700",
                                for project in projects_data.items.iter() {
                                    {
                                        let id = project.id.clone();
                                        let delete_id = id.clone();
                                        let name = project.name.clone();
                                        let status = project.status;
                                        let supply = project.monthly_token_supply;
                                        rsx! {
                                            tr {
                                                class: "cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700",
                                                onclick: move |_| {
                                                    nav.push(Route::ProjectDetail {
                                                        project_id: id.clone().into(),
                                                    });
                                                },
                                                td { class: "px-6 py-4 whitespace-nowrap",
                                                    div { class: "text-sm font-medium text-gray-900 dark:text-white",
                                                        "{name}"
                                                    }
                                                }
                                                td { class: "px-6 py-4 whitespace-nowrap",
                                                    code { class: "text-xs text-gray-600 dark:text-gray-400 bg-gray-100 dark:bg-gray-700 px-2 py-1 rounded",
                                                        "{project.id}"
                                                    }
                                                }
                                                td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400",
                                                    "{format_number(supply)}"
                                                }
                                                td { class: "px-6 py-4 whitespace-nowrap",
                                                    span {
                                                        class: format!(
                                                            "px-2 inline-flex text-xs leading-5 font-semibold rounded-full {}",
                                                            match status {
                                                                ProjectStatus::Active => "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
                                                                ProjectStatus::Inactive => "bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200",
                                                            }
                                                        ),
                                                        match status {
                                                            ProjectStatus::Active => {t.active},
                                                            ProjectStatus::Inactive => {t.inactive},
                                                        }
                                                    }
                                                }
                                                td { class: "px-6 py-4 whitespace-nowrap text-sm",
                                                    button {
                                                        class: "text-red-600 hover:text-red-900 dark:text-red-400 dark:hover:text-red-300",
                                                        onclick: move |e: MouseEvent| {
                                                            e.stop_propagation();
                                                            let pid = delete_id.clone();
                                                            spawn(async move {
                                                                let _ = crate::features::projects::controllers::delete_project_handler(pid.into()).await;
                                                                projects.restart();
                                                            });
                                                        },
                                                        // Trash2 icon
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
                                                            path { d: "M3 6h18" }
                                                            path { d: "M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" }
                                                            path { d: "M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" }
                                                            line { x1: "10", y1: "11", x2: "10", y2: "17" }
                                                            line { x1: "14", y1: "11", x2: "14", y2: "17" }
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
            }

            // Create Project Dialog
            if dialog_open() {
                CreateProjectDialog {
                    on_close: move |_| {
                        dialog_open.set(false);
                    },
                    on_created: move |_| {
                        dialog_open.set(false);
                        projects.restart();
                    },
                }
            }
        }
    }
}

#[component]
fn CreateProjectDialog(on_close: EventHandler, on_created: EventHandler) -> Element {
    let t: ProjectsTranslate = use_translate();
    let mut name = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut symbol = use_signal(String::new);
    let mut supply = use_signal(|| "1000000".to_string());
    let mut decimals = use_signal(|| "18".to_string());
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);

    rsx! {
        div {
            class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50",
            div { class: "bg-white dark:bg-gray-800 rounded-lg max-w-md w-full p-6",
                h3 { class: "text-lg font-semibold text-gray-900 dark:text-white mb-4",
                    {t.create_project}
                }

                if let Some(err) = error() {
                    div { class: "mb-4 p-3 bg-red-50 dark:bg-red-900/20 rounded-md",
                        p { class: "text-sm text-red-800 dark:text-red-400", "{err}" }
                    }
                }

                div { class: "space-y-4",
                    div {
                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                            {t.project_name}
                        }
                        input {
                            r#type: "text",
                            value: "{name}",
                            oninput: move |e: FormEvent| name.set(e.value()),
                            placeholder: "{t.name_placeholder}",
                            class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md dark:bg-gray-700 dark:text-white",
                        }
                    }

                    div {
                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                            {t.description}
                        }
                        input {
                            r#type: "text",
                            value: "{description}",
                            oninput: move |e: FormEvent| description.set(e.value()),
                            placeholder: "{t.description_placeholder}",
                            class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md dark:bg-gray-700 dark:text-white",
                        }
                    }

                    div {
                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                            {t.token_symbol}
                        }
                        input {
                            r#type: "text",
                            value: "{symbol}",
                            oninput: move |e: FormEvent| symbol.set(e.value()),
                            placeholder: "{t.symbol_placeholder}",
                            maxlength: "10",
                            class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md dark:bg-gray-700 dark:text-white",
                        }
                    }

                    div {
                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                            {t.monthly_supply}
                        }
                        input {
                            r#type: "number",
                            value: "{supply}",
                            oninput: move |e: FormEvent| supply.set(e.value()),
                            placeholder: "{t.monthly_supply_placeholder}",
                            class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md dark:bg-gray-700 dark:text-white",
                        }
                    }

                    div {
                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                            {t.token_decimals}
                        }
                        input {
                            r#type: "number",
                            value: "{decimals}",
                            oninput: move |e: FormEvent| decimals.set(e.value()),
                            placeholder: "{t.decimals_placeholder}",
                            min: "0",
                            max: "18",
                            class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md dark:bg-gray-700 dark:text-white",
                        }
                    }
                }

                div { class: "flex justify-end space-x-3 mt-6",
                    button {
                        class: "px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-700 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600",
                        disabled: loading(),
                        onclick: move |_| on_close.call(()),
                        {t.cancel}
                    }
                    button {
                        class: "px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 disabled:opacity-50",
                        disabled: loading(),
                        onclick: move |_| {
                            let name_val = name();
                            let desc_val = {
                                let d = description();
                                if d.is_empty() { None } else { Some(d) }
                            };
                            let supply_val: i64 = supply().parse().unwrap_or(1_000_000);
                            let symbol_val = symbol();
                            let decimals_val: u8 = decimals().parse().unwrap_or(18);

                            spawn(async move {
                                loading.set(true);
                                error.set(None);

                                match crate::features::projects::controllers::create_project_handler(
                                    name_val,
                                    desc_val,
                                    supply_val,
                                    symbol_val,
                                    decimals_val,
                                )
                                .await
                                {
                                    Ok(_) => {
                                        on_created.call(());
                                    }
                                    Err(e) => {
                                        error.set(Some(e.to_string()));
                                    }
                                }
                                loading.set(false);
                            });
                        },
                        if loading() {
                            {t.creating}
                        } else {
                            {t.create_project}
                        }
                    }
                }
            }
        }
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
