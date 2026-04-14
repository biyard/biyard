use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::common::ProjectPartition;
use crate::common::ui::*;
use crate::features::accounts::context::use_account_context;
use crate::features::console::i18n::ConsoleTranslate;
use crate::features::projects::i18n::ProjectsTranslate;

use super::{TokenEditorCard, TokenEditorMode};

#[component]
pub fn TokenCreate(project_id: ReadSignal<ProjectPartition>) -> Element {
    let t: ProjectsTranslate = use_translate();
    let console_t: ConsoleTranslate = use_translate();
    let nav = use_navigator();
    let account_ctx = use_account_context();

    let existing_result = use_loader(move || async move {
        crate::features::tokens::controllers::get_token_handler(project_id()).await
    });
    let project_result = use_loader(move || async move {
        crate::features::projects::controllers::get_project_handler(project_id()).await
    });

    // Viewers cannot create tokens — bounce them to the brand detail.
    // Registered before the `?` below so the hook count stays stable.
    let can_write = account_ctx().can_write();
    use_effect(move || {
        if !can_write {
            nav.replace(Route::ProjectDetail { project_id: project_id() });
        }
    });

    let existing = existing_result?;
    let project = project_result?;

    if !can_write {
        return rsx! {};
    }

    let pid_back = project_id();
    let token_opt = existing();
    let brand_name = project().name.clone();

    if let Some(token) = token_opt {
        let to_detail = pid_back.clone();
        let to_edit = pid_back.clone();
        let brand_name_1 = brand_name.clone();
        return rsx! {
            div { class: "space-y-8",
                PageHeader {
                    title: t.token_already_exists.to_string(),
                    subtitle: t.token_already_exists_subtitle.to_string(),
                    scope: PageScope::Brand {
                        name: brand_name_1,
                    },
                    workspace_label: console_t.enterprise_scope_label.to_string(),
                    brand_label: console_t.brand_scope_label.to_string(),
                    actions: rsx! {
                        Btn {
                            variant: BtnVariant::Secondary,
                            onclick: move |_| {
                                nav.push(Route::ProjectDetail {
                                    project_id: to_detail.clone(),
                                });
                            },
                            {t.back_to_brand}
                        }
                    },
                }

                SectionCard {
                    SectionTitle { {t.token_info} }
                    div { class: "grid gap-4 sm:grid-cols-2",
                        StatCard {
                            label: t.token_name.to_string(),
                            value: token.name.clone(),
                            color: StatColor::Gray,
                        }
                        StatCard {
                            label: t.token_symbol.to_string(),
                            value: token.symbol.clone(),
                            color: StatColor::Blue,
                        }
                    }

                    div { class: "mt-6 flex justify-end",
                        if token.contract_address.is_none() {
                            Btn {
                                variant: BtnVariant::Primary,
                                onclick: move |_| {
                                    nav.push(Route::TokenEdit {
                                        project_id: to_edit.clone(),
                                    });
                                },
                                {t.edit_token}
                            }
                        }
                    }
                }
            }
        };
    }

    rsx! {
        div { class: "space-y-8",
            PageHeader {
                title: t.create_token.to_string(),
                subtitle: t.create_token_subtitle.to_string(),
                scope: PageScope::Brand {
                    name: brand_name.clone(),
                },
                workspace_label: console_t.enterprise_scope_label.to_string(),
                brand_label: console_t.brand_scope_label.to_string(),
                actions: rsx! {
                    Btn {
                        variant: BtnVariant::Secondary,
                        onclick: move |_| {
                            nav.push(Route::ProjectDetail {
                                project_id: pid_back.clone(),
                            });
                        },
                        {t.skip_for_now}
                    }
                },
            }

            TokenEditorCard { project_id, mode: TokenEditorMode::Create }
        }
    }
}
