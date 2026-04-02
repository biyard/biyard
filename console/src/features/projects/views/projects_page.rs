use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::common::components::dialog::*;
use crate::common::ui::*;
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
            PageHeader {
                title: t.title.to_string(),
                actions: rsx! {
                    Btn {
                        variant: BtnVariant::Primary,
                        class: "flex items-center",
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
                },
            }

            // Main Content
            div {
                if projects_data.items.is_empty() {
                    EmptyState {
                        icon: rsx! {
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
                        },
                        title: t.no_projects.to_string(),
                        description: t.no_projects_desc.to_string(),
                        actions: rsx! {
                            Btn {
                                variant: BtnVariant::Primary,
                                class: "flex items-center",
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
                        },
                    }
                } else {
                    DataTable {
                        TableHead {
                            TableHeadCell { {t.project_name} }
                            TableHeadCell { {t.project_id} }
                            TableHeadCell { {t.monthly_supply} }
                            TableHeadCell { {t.status} }
                            TableHeadCell { {t.actions} }
                        }
                        TableBody {
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
                                            TableCell {
                                                div { class: "text-sm font-medium text-gray-900 dark:text-white",
                                                    "{name}"
                                                }
                                            }
                                            TableCell {
                                                code { class: "text-xs text-gray-600 dark:text-gray-400 bg-gray-100 dark:bg-gray-700 px-2 py-1 rounded",
                                                    "{project.id}"
                                                }
                                            }
                                            TableCell {
                                                class: "text-sm text-gray-500 dark:text-gray-400",
                                                "{format_number(supply)}"
                                            }
                                            TableCell {
                                                StatusBadge {
                                                    color: match status {
                                                        ProjectStatus::Active => BadgeColor::Green,
                                                        ProjectStatus::Inactive => BadgeColor::Gray,
                                                    },
                                                    match status {
                                                        ProjectStatus::Active => {t.active},
                                                        ProjectStatus::Inactive => {t.inactive},
                                                    }
                                                }
                                            }
                                            TableCell {
                                                class: "text-sm",
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
    let mut token_name = use_signal(String::new);
    let mut brand_logo_url = use_signal(String::new);
    let mut symbol = use_signal(String::new);
    let mut supply = use_signal(|| "1000000".to_string());
    let mut initial_supply = use_signal(|| "1000000".to_string());
    let mut reserve_rate = use_signal(|| "0.1".to_string());
    let mut decimals = use_signal(|| "18".to_string());
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);

    rsx! {
        DialogRoot {
            open: true,
            on_open_change: move |v: bool| { if !v { on_close.call(()); } },
            DialogContent {
                DialogTitle { {t.create_project} }

            if let Some(err) = error() {
                div { class: "mb-4",
                    AlertMessage {
                        variant: AlertVariant::Error,
                        "{err}"
                    }
                }
            }

            div { class: "space-y-4",
                FormField {
                    label: t.project_name,
                    value: name(),
                    oninput: move |e: FormEvent| name.set(e.value()),
                    placeholder: t.name_placeholder.to_string(),
                }

                FormField {
                    label: t.description,
                    value: description(),
                    oninput: move |e: FormEvent| description.set(e.value()),
                    placeholder: t.description_placeholder.to_string(),
                }

                FormField {
                    label: t.token_name,
                    value: token_name(),
                    oninput: move |e: FormEvent| token_name.set(e.value()),
                    placeholder: t.token_name_placeholder.to_string(),
                }

                FormField {
                    label: t.brand_logo_url,
                    r#type: "url",
                    value: brand_logo_url(),
                    oninput: move |e: FormEvent| brand_logo_url.set(e.value()),
                    placeholder: "https://...".to_string(),
                }

                FormField {
                    label: t.token_symbol,
                    value: symbol(),
                    oninput: move |e: FormEvent| symbol.set(e.value()),
                    placeholder: t.symbol_placeholder.to_string(),
                    maxlength: "10",
                }

                FormField {
                    label: t.initial_total_supply,
                    r#type: "number",
                    value: initial_supply(),
                    oninput: move |e: FormEvent| initial_supply.set(e.value()),
                    placeholder: "1000000".to_string(),
                    min: "0",
                }

                FormField {
                    label: t.monthly_supply,
                    r#type: "number",
                    value: supply(),
                    oninput: move |e: FormEvent| supply.set(e.value()),
                    placeholder: t.monthly_supply_placeholder.to_string(),
                }

                div {
                    FormField {
                        label: t.treasury_reserve_rate,
                        r#type: "number",
                        value: reserve_rate(),
                        oninput: move |e: FormEvent| reserve_rate.set(e.value()),
                        placeholder: "0.1".to_string(),
                        min: "0",
                        max: "1",
                        step: "0.01",
                    }
                    p { class: "text-xs text-gray-500 dark:text-gray-400 mt-1",
                        {t.treasury_reserve_rate_desc}
                    }
                }

                FormField {
                    label: t.token_decimals,
                    r#type: "number",
                    value: decimals(),
                    oninput: move |e: FormEvent| decimals.set(e.value()),
                    placeholder: t.decimals_placeholder.to_string(),
                    min: "0",
                    max: "18",
                }
            }

            DialogActions {
                Btn {
                    variant: BtnVariant::Secondary,
                    disabled: loading(),
                    onclick: move |_| on_close.call(()),
                    {t.cancel}
                }
                Btn {
                    variant: BtnVariant::Primary,
                    disabled: loading(),
                    onclick: move |_| {
                        let name_val = name();
                        let desc_val = {
                            let d = description();
                            if d.is_empty() { None } else { Some(d) }
                        };
                        let token_name_val = {
                            let n = token_name();
                            if n.is_empty() { None } else { Some(n) }
                        };
                        let brand_logo_url_val = {
                            let b = brand_logo_url();
                            if b.is_empty() { None } else { Some(b) }
                        };
                        let supply_val: i64 = supply().parse().unwrap_or(1_000_000);
                        let initial_supply_val: i64 = initial_supply().parse().unwrap_or(1_000_000);
                        let reserve_rate_val: f64 = reserve_rate().parse().unwrap_or(0.1);
                        let symbol_val = symbol();
                        let decimals_val: u8 = decimals().parse().unwrap_or(18);

                        spawn(async move {
                            loading.set(true);
                            error.set(None);

                            match crate::features::projects::controllers::create_project_handler(
                                name_val,
                                desc_val,
                                token_name_val,
                                brand_logo_url_val,
                                supply_val,
                                reserve_rate_val,
                                symbol_val,
                                decimals_val,
                                initial_supply_val,
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
