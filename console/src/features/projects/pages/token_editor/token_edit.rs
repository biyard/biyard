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
pub fn TokenEdit(project_id: ReadSignal<ProjectPartition>) -> Element {
    let t: ProjectsTranslate = use_translate();
    let console_t: ConsoleTranslate = use_translate();
    let nav = use_navigator();
    let account_ctx = use_account_context();

    let token_result = use_loader(move || async move {
        crate::features::tokens::controllers::get_token_handler(project_id()).await
    });
    let project_result = use_loader(move || async move {
        crate::features::projects::controllers::get_project_handler(project_id()).await
    });

    // Viewers cannot edit tokens — bounce them to the brand detail.
    let can_write = account_ctx().can_write();
    use_effect(move || {
        if !can_write {
            nav.replace(Route::ProjectDetail { project_id: project_id() });
        }
    });

    let token = token_result?;
    let project = project_result?;

    if !can_write {
        return rsx! {};
    }

    let pid_back = project_id();
    let brand_name = project().name.clone();

    // No token configured yet — TokenEdit was navigated to directly. Send
    // the user to the create flow rather than rendering an empty form.
    let Some(token_data) = token() else {
        let to_create = pid_back.clone();
        return rsx! {
            div { class: "space-y-8",
                PageHeader {
                    title: t.create_token.to_string(),
                    subtitle: t.create_token_subtitle.to_string(),
                    scope: PageScope::Brand {
                        name: brand_name.clone(),
                    },
                    workspace_label: console_t.enterprise_scope_label.to_string(),
                    brand_label: console_t.brand_scope_label.to_string(),
                }
                SectionCard {
                    EmptyState {
                        icon: rsx! {
                            IconToken {}
                        },
                        title: t.no_token.to_string(),
                        description: t.no_token_desc.to_string(),
                        actions: rsx! {
                            Btn {
                                variant: BtnVariant::Primary,
                                onclick: move |_| {
                                    nav.push(Route::TokenCreate {
                                        project_id: to_create.clone(),
                                    });
                                },
                                {t.create_token}
                            }
                        },
                    }
                }
            }
        };
    };

    if token_data.contract_address.is_some() {
        let to_detail = pid_back.clone();
        let brand_name_1 = brand_name.clone();
        return rsx! {
            div { class: "space-y-8",
                PageHeader {
                    title: t.token_locked_title.to_string(),
                    subtitle: t.token_locked_subtitle.to_string(),
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
            }
        };
    }

    rsx! {
        div { class: "space-y-8",
            PageHeader {
                title: t.edit_token.to_string(),
                subtitle: t.edit_token_subtitle.to_string(),
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
                        {t.back_to_brand}
                    }
                },
            }

            TokenEditorCard {
                project_id,
                mode: TokenEditorMode::Edit {
                    token: token_data,
                },
            }
        }
    }
}
