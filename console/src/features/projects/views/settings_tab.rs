use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::common::ProjectPartition;
use crate::common::components::dialog::*;
use crate::common::ui::*;
use crate::features::projects::ProjectResponse;
use crate::features::projects::i18n::ProjectsTranslate;

/// Brand settings tab. Currently just the danger zone (brand delete);
/// once more brand-level settings land they'll be added here as
/// additional sections above the danger card.
///
/// Brand deletion requires the user to **type the brand name** into
/// the confirmation dialog. This is the only place in the app where
/// a brand can be deleted — the brands list card intentionally has
/// no delete affordance.
#[component]
pub fn SettingsTab(
    project_id: ReadSignal<ProjectPartition>,
    project: ProjectResponse,
) -> Element {
    let t: ProjectsTranslate = use_translate();
    let nav = use_navigator();
    let mut show_delete = use_signal(|| false);
    let mut confirm_input = use_signal(String::new);
    let mut deleting = use_signal(|| false);
    let mut delete_error = use_signal(|| None::<String>);

    let brand_name = project.name.clone();
    let brand_name_for_match = brand_name.clone();
    let brand_name_for_action = brand_name.clone();

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

    rsx! {
        div { class: "space-y-6",
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
                        p { class: "text-sm text-foreground-soft",
                            {t.delete_brand_confirm_prompt}
                        }
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
                            if deleting() { {t.deleting} } else { {t.delete_brand_button} }
                        }
                    }
                }
            }
        }
    }
}
