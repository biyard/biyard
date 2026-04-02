use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::common::ProjectPartition;
use crate::common::components::dialog::*;
use crate::common::ui::*;
use crate::features::projects::ProjectResponse;
use crate::features::projects::i18n::ProjectsTranslate;

#[component]
pub fn SettingsTab(project_id: ReadSignal<ProjectPartition>, project: ProjectResponse) -> Element {
    let t: ProjectsTranslate = use_translate();
    let nav = use_navigator();
    let mut show_delete = use_signal(|| false);
    let mut saving = use_signal(|| false);
    let mut simulating = use_signal(|| false);
    let mut message = use_signal(|| None::<String>);

    let mut project_name = use_signal(|| project.name.clone());
    let mut description = use_signal(|| project.description.clone().unwrap_or_default());
    let mut brand_logo_url = use_signal(|| project.brand_logo_url.clone().unwrap_or_default());
    let mut monthly_supply = use_signal(|| project.monthly_token_supply.to_string());
    let mut reserve_rate = use_signal(|| format!("{:.2}", project.treasury_reserve_rate));
    let mut revenue_input = use_signal(String::new);
    let token = use_loader(move || async move {
        crate::features::tokens::controllers::get_token_handler(project_id()).await
    });

    rsx! {
        div { class: "space-y-6",
            match &token {
                Ok(token) => rsx! {
                    SectionCard {
                        SectionTitle { {t.token_info_immutable} }
                        div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                            StatCard {
                                label: t.token_name.to_string(),
                                value: token.read().name.clone(),
                                color: StatColor::Gray,
                            }
                            StatCard {
                                label: t.token_symbol.to_string(),
                                value: token.read().symbol.clone(),
                                color: StatColor::Gray,
                            }
                            StatCard {
                                label: t.total_supply.to_string(),
                                value: format_number(token.read().total_supply),
                                color: StatColor::Gray,
                            }
                        }
                        p { class: "mt-3 text-xs text-gray-500 dark:text-gray-400",
                            {t.token_immutable_desc}
                        }
                    }
                },
                Err(_) => rsx! {},
            }

            SectionCard {
                SectionTitle { {t.brand_settings} }

                if let Some(msg) = message() {
                    div { class: "mb-4",
                        AlertMessage { variant: AlertVariant::Info, "{msg}" }
                    }
                }

                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    FormField {
                        label: t.brand_name,
                        r#type: "text",
                        value: project_name(),
                        oninput: move |e: FormEvent| project_name.set(e.value()),
                    }
                    div {
                        FormLabel { {t.brand_display_name} }
                        p { class: "w-full px-3 py-2 border border-gray-200 dark:border-gray-700 rounded-md bg-gray-50 dark:bg-gray-700/40 text-gray-900 dark:text-white",
                            "{project_name}"
                        }
                        p { class: "text-xs text-gray-500 dark:text-gray-400 mt-1",
                            {t.brand_display_name_desc}
                        }
                    }
                    div { class: "md:col-span-2",
                        FormField {
                            label: t.brand_logo_url,
                            r#type: "url",
                            value: brand_logo_url(),
                            oninput: move |e: FormEvent| brand_logo_url.set(e.value()),
                            placeholder: "https://...".to_string(),
                        }
                    }
                    div { class: "md:col-span-2",
                        FormField {
                            label: t.description,
                            r#type: "text",
                            value: description(),
                            oninput: move |e: FormEvent| description.set(e.value()),
                        }
                    }
                    FormField {
                        label: t.monthly_supply,
                        r#type: "number",
                        value: monthly_supply(),
                        oninput: move |e: FormEvent| monthly_supply.set(e.value()),
                        min: "0",
                    }
                    div {
                        FormField {
                            label: t.treasury_reserve_rate,
                            r#type: "number",
                            value: reserve_rate(),
                            oninput: move |e: FormEvent| reserve_rate.set(e.value()),
                            min: "0",
                            max: "1",
                            step: "0.01",
                        }
                        p { class: "text-xs text-gray-500 dark:text-gray-400 mt-1",
                            {t.treasury_reserve_rate_desc}
                        }
                    }
                }

                div { class: "flex justify-end mt-6",
                    Btn {
                        variant: BtnVariant::Primary,
                        disabled: saving(),
                        onclick: move |_| {
                            let pid = project_id();
                            let pid_for_nav = pid.clone();
                            let name_val = project_name();
                            let desc_val = {
                                let d = description();
                                if d.is_empty() { None } else { Some(d) }
                            };
                            let brand_logo_val = {
                                let b = brand_logo_url();
                                if b.is_empty() { None } else { Some(b) }
                            };
                            let monthly_supply_val = monthly_supply().parse::<i64>().unwrap_or(project.monthly_token_supply);
                            let reserve_rate_val = reserve_rate().parse::<f64>().unwrap_or(project.treasury_reserve_rate);

                            spawn(async move {
                                saving.set(true);
                                message.set(None);
                                let res = crate::features::projects::controllers::update_project_handler(
                                    pid,
                                    Some(name_val),
                                    desc_val,
                                    brand_logo_val,
                                    Some(monthly_supply_val),
                                    Some(reserve_rate_val),
                                    None,
                                )
                                .await;

                                match res {
                                    Ok(_) => {
                                        message.set(Some(t.settings_saved.to_string()));
                                        nav.push(Route::ProjectDetail {
                                            project_id: pid_for_nav,
                                        });
                                    }
                                    Err(e) => message.set(Some(format!("{}{e}", t.save_failure))),
                                }
                                saving.set(false);
                            });
                        },
                        if saving() { {t.saving} } else { {t.save_settings} }
                    }
                }
            }

            SectionCard {
                SectionTitle { {t.revenue_to_treasury_simulation} }
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-4 items-end",
                    div { class: "md:col-span-2",
                        FormField {
                            label: t.revenue_input,
                            r#type: "number",
                            value: revenue_input(),
                            oninput: move |e: FormEvent| revenue_input.set(e.value()),
                            placeholder: t.revenue_input_placeholder.to_string(),
                            min: "0",
                        }
                    }
                    button {
                        class: "px-4 py-2 text-sm font-medium text-white bg-emerald-600 rounded-md hover:bg-emerald-700 disabled:opacity-50",
                        disabled: simulating(),
                        onclick: move |_| {
                            let pid = project_id();
                            let pid_for_nav = pid.clone();
                            let revenue = revenue_input().parse::<i64>().unwrap_or(0);
                            spawn(async move {
                                simulating.set(true);
                                message.set(None);
                                let res = crate::features::projects::controllers::simulate_revenue_handler(pid, revenue).await;
                                match res {
                                    Ok(_) => {
                                        message.set(Some(t.simulation_success.to_string()));
                                        revenue_input.set(String::new());
                                        nav.push(Route::ProjectDetail {
                                            project_id: pid_for_nav,
                                        });
                                    }
                                    Err(e) => message.set(Some(format!("{}{e}", t.simulation_failure))),
                                }
                                simulating.set(false);
                            });
                        },
                        if simulating() { {t.applying} } else { {t.apply_revenue} }
                    }
                }

                div { class: "mt-4 text-sm text-gray-500 dark:text-gray-400",
                    {t.current_treasury_balance}
                    span { class: "font-semibold text-gray-900 dark:text-white", "{format_number(project.treasury_balance)}" }
                    {t.cumulative_sales_label}
                    span { class: "font-semibold text-gray-900 dark:text-white", "{format_number(project.simulated_sales_total)}" }
                }
                p { class: "mt-2 text-xs text-gray-500 dark:text-gray-400",
                    {t.floor_price_overview_note}
                }
            }

            SectionCard {
                h3 { class: "text-lg font-medium text-red-600 dark:text-red-400 mb-4",
                    {t.delete_project}
                }
                p { class: "text-sm text-gray-500 dark:text-gray-400 mb-4",
                    {t.delete_confirm}
                }
                Btn {
                    variant: BtnVariant::Danger,
                    onclick: move |_| show_delete.set(true),
                    {t.delete_project}
                }
            }

            DialogRoot {
                open: show_delete(),
                on_open_change: move |v| show_delete.set(v),
                DialogContent {
                    DialogTitle { {t.delete_project} }
                    DialogDescription { {t.delete_confirm} }
                    DialogActions {
                        Btn {
                            variant: BtnVariant::Secondary,
                            onclick: move |_| show_delete.set(false),
                            {t.cancel}
                        }
                        Btn {
                            variant: BtnVariant::Danger,
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
