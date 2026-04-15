use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::common::ProjectPartition;
use crate::common::components::dialog::*;
use crate::common::ui::*;
use crate::features::accounts::context::use_account_context;
use crate::features::projects::ProjectResponse;
use crate::features::projects::i18n::ProjectsTranslate;
use crate::features::projects::pages::editor_card::{ProjectEditorCard, ProjectEditorMode};

/// Brand settings tab. Renders the editable brand profile inline (name,
/// description, logo, monthly token supply, treasury reserve rate) plus a
/// Danger Zone card at the bottom for brand deletion.
///
/// Previously brand editing lived on a separate `/edit` page and the
/// `/settings` URL only contained the Danger Zone, which made the
/// "Settings" tab name misleading. Editing was reachable only via a
/// header button. Consolidating both into one tab matches typical SaaS
/// settings conventions: editable fields up top, destructive actions
/// behind a clear danger boundary at the bottom.
///
/// Brand deletion requires the user to **type the brand name** into
/// the confirmation dialog. This is the only place in the app where
/// a brand can be deleted — the brands list card intentionally has
/// no delete affordance.
#[component]
pub fn SettingsTab(project_id: ReadSignal<ProjectPartition>, project: ProjectResponse) -> Element {
    let t: ProjectsTranslate = use_translate();
    let nav = use_navigator();
    let account_ctx = use_account_context();
    let can_write = account_ctx().can_write();
    let mut show_delete = use_signal(|| false);
    let mut confirm_input = use_signal(String::new);
    let mut deleting = use_signal(|| false);
    let mut delete_error = use_signal(|| None::<String>);

    let brand_name = project.name.clone();
    let brand_name_for_match = brand_name.clone();
    let brand_name_for_action = brand_name.clone();
    let project_for_editor = project.clone();
    let pid_for_editor = project_id();

    let name_matches = confirm_input().trim() == brand_name_for_match;

    let on_confirm_delete = move |_| {
        if !name_matches {
            delete_error.set(Some(t.delete_brand_mismatch.to_string()));
            return;
        }
        deleting.set(true);
        delete_error.set(None);
        let pid = project_id();
        let expected_name = brand_name_for_action.clone();
        let input_now = confirm_input().trim().to_string();
        spawn(async move {
            // Defensive: even though the button is disabled on mismatch,
            // re-check right before the network call in case the input
            // changed between click and spawn.
            if input_now != expected_name {
                deleting.set(false);
                return;
            }
            match crate::features::projects::controllers::delete_project_handler(pid).await {
                Ok(_) => {
                    nav.push(Route::Projects {});
                }
                Err(e) => {
                    delete_error.set(Some(e.to_string()));
                    deleting.set(false);
                }
            }
        });
    };

    if !can_write {
        return rsx! {
            div { class: "space-y-6",
                AlertMessage { variant: AlertVariant::Info, {t.viewer_readonly_notice} }
                SectionCard {
                    SectionTitle { {t.brand_profile} }
                    div { class: "grid gap-4 sm:grid-cols-2",
                        div {
                            p { class: "text-[11px] font-semibold uppercase tracking-[0.14em] text-foreground-muted",
                                {t.name}
                            }
                            p { class: "mt-1 text-sm font-semibold text-foreground",
                                {project.name.clone()}
                            }
                        }
                        if let Some(desc) = project.description.clone() {
                            div {
                                p { class: "text-[11px] font-semibold uppercase tracking-[0.14em] text-foreground-muted",
                                    {t.description}
                                }
                                p { class: "mt-1 text-sm text-foreground", {desc} }
                            }
                        }
                        div {
                            p { class: "text-[11px] font-semibold uppercase tracking-[0.14em] text-foreground-muted",
                                {t.monthly_supply}
                            }
                            p { class: "mt-1 text-sm font-semibold text-foreground",
                                "{project.monthly_token_supply}"
                            }
                        }
                    }
                }
            }
        };
    }

    rsx! {
        div { class: "space-y-8",
            // Editable brand profile — name, description, logo, supply, treasury rate.
            ProjectEditorCard {
                mode: ProjectEditorMode::Edit {
                    project_id: pid_for_editor,
                    project: project_for_editor,
                },
            }

            DangerCard {
                div { class: "flex flex-col gap-5 md:flex-row md:items-start md:justify-between",
                    div { class: "flex items-start gap-4",
                        div { class: "mt-1 flex h-11 w-11 items-center justify-center rounded-2xl bg-danger text-white",
                            IconAlertTriangle { class: "h-5 w-5" }
                        }
                        div {
                            p { class: "text-[11px] font-semibold uppercase tracking-[0.14em] text-danger",
                                {t.delete_project}
                            }
                            h3 { class: "mt-2 font-display text-xl font-bold tracking-tight text-foreground",
                                {t.delete_project}
                            }
                            p { class: "mt-2 max-w-2xl text-sm leading-6 text-foreground-soft",
                                {t.delete_brand_warning}
                            }
                        }
                    }
                    Btn {
                        variant: BtnVariant::Danger,
                        onclick: move |_| {
                            confirm_input.set(String::new());
                            delete_error.set(None);
                            show_delete.set(true);
                        },
                        {t.delete_project}
                    }
                }
            }

            DialogRoot {
                open: show_delete(),
                on_open_change: move |v: bool| {
                    if !v {
                        confirm_input.set(String::new());
                        delete_error.set(None);
                    }
                    show_delete.set(v);
                },
                DialogContent {
                    DialogTitle { {t.delete_project} }
                    DialogDescription { {t.delete_brand_warning} }

                    div { class: "mt-2 space-y-3",
                        p { class: "text-sm text-foreground-soft", {t.delete_brand_confirm_prompt} }
                        p { class: "rounded-2xl border border-border bg-panel-muted px-3 py-2 font-mono text-sm font-semibold text-foreground",
                            "{brand_name}"
                        }
                        FormField {
                            label: t.name,
                            r#type: "text",
                            value: confirm_input(),
                            oninput: move |e: FormEvent| {
                                confirm_input.set(e.value());
                                delete_error.set(None);
                            },
                            placeholder: t.delete_brand_confirm_placeholder.to_string(),
                        }
                        if let Some(msg) = delete_error() {
                            AlertMessage { variant: AlertVariant::Error, "{msg}" }
                        }
                    }

                    DialogActions {
                        Btn {
                            variant: BtnVariant::Secondary,
                            disabled: deleting(),
                            onclick: move |_| {
                                confirm_input.set(String::new());
                                delete_error.set(None);
                                show_delete.set(false);
                            },
                            {t.cancel}
                        }
                        Btn {
                            variant: BtnVariant::Danger,
                            disabled: !name_matches || deleting(),
                            onclick: on_confirm_delete,
                            if deleting() {
                                {t.deleting}
                            } else {
                                {t.delete_brand_button}
                            }
                        }
                    }
                }
            }
        }
    }
}
