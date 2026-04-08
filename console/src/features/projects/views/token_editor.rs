use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::common::ProjectPartition;
use crate::common::ui::*;
use crate::features::accounts::context::use_account_context;
use crate::features::console::i18n::ConsoleTranslate;
use crate::features::projects::i18n::ProjectsTranslate;
use crate::features::tokens::TokenResponse;

#[derive(Clone, PartialEq)]
pub enum TokenEditorMode {
    Create,
    Edit { token: TokenResponse },
}

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
                    scope: PageScope::Brand { name: brand_name_1 },
                    workspace_label: console_t.enterprise_scope_label.to_string(),
                    brand_label: console_t.brand_scope_label.to_string(),
                    actions: rsx! {
                        Btn {
                            variant: BtnVariant::Secondary,
                            onclick: move |_| {
                                nav.push(Route::ProjectDetail { project_id: to_detail.clone() });
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
                                    nav.push(Route::TokenEdit { project_id: to_edit.clone() });
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
                scope: PageScope::Brand { name: brand_name.clone() },
                workspace_label: console_t.enterprise_scope_label.to_string(),
                brand_label: console_t.brand_scope_label.to_string(),
                actions: rsx! {
                    Btn {
                        variant: BtnVariant::Secondary,
                        onclick: move |_| {
                            nav.push(Route::ProjectDetail { project_id: pid_back.clone() });
                        },
                        {t.skip_for_now}
                    }
                },
            }

            TokenEditorCard {
                project_id: project_id,
                mode: TokenEditorMode::Create,
            }
        }
    }
}

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
                    scope: PageScope::Brand { name: brand_name.clone() },
                    workspace_label: console_t.enterprise_scope_label.to_string(),
                    brand_label: console_t.brand_scope_label.to_string(),
                }
                SectionCard {
                    EmptyState {
                        icon: rsx! { IconToken {} },
                        title: t.no_token.to_string(),
                        description: t.no_token_desc.to_string(),
                        actions: rsx! {
                            Btn {
                                variant: BtnVariant::Primary,
                                onclick: move |_| {
                                    nav.push(Route::TokenCreate { project_id: to_create.clone() });
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
                    scope: PageScope::Brand { name: brand_name_1 },
                    workspace_label: console_t.enterprise_scope_label.to_string(),
                    brand_label: console_t.brand_scope_label.to_string(),
                    actions: rsx! {
                        Btn {
                            variant: BtnVariant::Secondary,
                            onclick: move |_| {
                                nav.push(Route::ProjectDetail { project_id: to_detail.clone() });
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
                scope: PageScope::Brand { name: brand_name.clone() },
                workspace_label: console_t.enterprise_scope_label.to_string(),
                brand_label: console_t.brand_scope_label.to_string(),
                actions: rsx! {
                    Btn {
                        variant: BtnVariant::Secondary,
                        onclick: move |_| {
                            nav.push(Route::ProjectDetail { project_id: pid_back.clone() });
                        },
                        {t.back_to_brand}
                    }
                },
            }

            TokenEditorCard {
                project_id: project_id,
                mode: TokenEditorMode::Edit { token: token_data },
            }
        }
    }
}

#[component]
pub fn TokenEditorCard(project_id: ReadSignal<ProjectPartition>, mode: TokenEditorMode) -> Element {
    let t: ProjectsTranslate = use_translate();
    let nav = use_navigator();
    let is_create = matches!(&mode, TokenEditorMode::Create);

    let existing_token = match &mode {
        TokenEditorMode::Create => None,
        TokenEditorMode::Edit { token } => Some(token.clone()),
    };

    let seed_name = existing_token
        .as_ref()
        .map(|t| t.name.clone())
        .unwrap_or_default();
    let seed_symbol = existing_token
        .as_ref()
        .map(|t| t.symbol.clone())
        .unwrap_or_default();
    let seed_decimals = existing_token
        .as_ref()
        .map(|t| t.decimals.to_string())
        .unwrap_or_else(|| "18".to_string());
    let seed_description = existing_token
        .as_ref()
        .and_then(|t| t.description.clone())
        .unwrap_or_default();

    let mut name = use_signal(move || seed_name.clone());
    let mut symbol = use_signal(move || seed_symbol.clone());
    let mut decimals = use_signal(move || seed_decimals.clone());
    let mut description = use_signal(move || seed_description.clone());
    let mut message = use_signal(|| None::<(AlertVariant, String)>);
    let mut loading = use_signal(|| false);

    let title = if is_create {
        t.create_token.to_string()
    } else {
        t.edit_token.to_string()
    };
    let helper_text = if is_create {
        t.token_create_helper
    } else {
        t.token_edit_helper
    };

    let preview_name_is_placeholder = name().trim().is_empty();
    let preview_name = if preview_name_is_placeholder {
        t.token_name.to_string()
    } else {
        name()
    };
    let preview_symbol_is_placeholder = symbol().trim().is_empty();
    let preview_symbol = if preview_symbol_is_placeholder {
        "TKN".to_string()
    } else {
        symbol()
    };

    let save_failure = t.save_failure.to_string();
    let token_saved = t.token_saved.to_string();
    let required_fields_msg = t.token_required_fields.to_string();

    rsx! {
        SectionCard {
            div { class: "mb-6",
                SectionTitle { "{title}" }
                p { class: "mt-1 text-sm leading-6 text-foreground-muted", "{helper_text}" }
            }

            if let Some((variant, text)) = message() {
                div { class: "mb-5",
                    AlertMessage { variant: variant, "{text}" }
                }
            }

            div { class: "grid gap-6 lg:grid-cols-[minmax(0,1.15fr)_minmax(0,0.85fr)]",
                div { class: "space-y-6",
                    div { class: "grid gap-4 md:grid-cols-2",
                        FormField {
                            label: t.token_name,
                            value: name(),
                            oninput: move |e: FormEvent| name.set(e.value()),
                            placeholder: t.token_name_placeholder.to_string(),
                            required: true,
                        }
                        FormField {
                            label: t.token_symbol,
                            value: symbol(),
                            oninput: move |e: FormEvent| symbol.set(e.value()),
                            placeholder: t.symbol_placeholder.to_string(),
                            maxlength: "10",
                            required: true,
                        }
                        FormField {
                            label: t.token_decimals,
                            r#type: "number",
                            value: decimals(),
                            oninput: move |e: FormEvent| decimals.set(e.value()),
                            placeholder: t.decimals_placeholder.to_string(),
                            min: "0",
                            max: "18",
                            required: true,
                        }
                        div { class: "md:col-span-2",
                            FormField {
                                label: t.description,
                                value: description(),
                                oninput: move |e: FormEvent| description.set(e.value()),
                                placeholder: t.description_placeholder.to_string(),
                            }
                        }
                    }

                    div { class: "flex flex-col gap-3 pt-2 sm:flex-row sm:justify-end",
                        Btn {
                            variant: BtnVariant::Secondary,
                            disabled: loading(),
                            onclick: move |_| {
                                nav.push(Route::ProjectDetail { project_id: project_id() });
                            },
                            {t.cancel}
                        }
                        Btn {
                            variant: BtnVariant::Primary,
                            class: "sm:min-w-40",
                            disabled: loading(),
                            onclick: move |_| {
                                let pid = project_id();
                                let mode = mode.clone();
                                let nav = nav.clone();
                                let token_saved = token_saved.clone();
                                let save_failure = save_failure.clone();
                                let required_fields_msg = required_fields_msg.clone();
                                let name_val = name().trim().to_string();
                                let symbol_val = symbol().trim().to_string();
                                let decimals_input = decimals().trim().to_string();
                                let desc_val = {
                                    let value = description();
                                    if value.trim().is_empty() { None } else { Some(value) }
                                };

                                if name_val.is_empty()
                                    || symbol_val.is_empty()
                                    || decimals_input.is_empty()
                                {
                                    message.set(Some((AlertVariant::Error, required_fields_msg)));
                                    return;
                                }

                                let Ok(decimals_val) = decimals_input.parse::<u8>() else {
                                    message.set(Some((AlertVariant::Error, required_fields_msg)));
                                    return;
                                };

                                spawn(async move {
                                    loading.set(true);
                                    message.set(None);

                                    let result = match mode {
                                        TokenEditorMode::Create => {
                                            crate::features::tokens::controllers::create_token_handler(
                                                pid.clone(),
                                                name_val,
                                                symbol_val,
                                                decimals_val,
                                                desc_val,
                                            )
                                            .await
                                            .map(|_| ())
                                        }
                                        TokenEditorMode::Edit { .. } => {
                                            crate::features::tokens::controllers::update_token_handler(
                                                pid.clone(),
                                                Some(name_val),
                                                Some(symbol_val),
                                                Some(decimals_val),
                                                desc_val,
                                            )
                                            .await
                                            .map(|_| ())
                                        }
                                    };

                                    match result {
                                        Ok(_) => {
                                            message.set(Some((AlertVariant::Success, token_saved.clone())));
                                            nav.push(Route::ProjectToken { project_id: pid });
                                        }
                                        Err(error) => {
                                            message.set(Some((
                                                AlertVariant::Error,
                                                format!("{save_failure}{error}"),
                                            )));
                                        }
                                    }

                                    loading.set(false);
                                });
                            },
                            if loading() {
                                if is_create { {t.creating} } else { {t.saving} }
                            } else if is_create {
                                {t.create_token}
                            } else {
                                {t.save_token}
                            }
                        }
                    }
                }

                div { class: "space-y-4",
                    div { class: "rounded-[24px] border border-border bg-panel-muted p-5",
                        p { class: "text-[11px] font-semibold uppercase tracking-[0.14em] text-foreground-muted",
                            {t.live_preview}
                        }
                        div { class: "mt-4 space-y-3",
                            div { class: "rounded-2xl border border-border bg-panel px-4 py-3",
                                p { class: "text-[11px] font-semibold uppercase tracking-[0.12em] text-foreground-muted",
                                    {t.token_name}
                                }
                                p {
                                    class: if preview_name_is_placeholder {
                                        "mt-2 text-lg font-semibold italic text-foreground-muted"
                                    } else {
                                        "mt-2 text-lg font-semibold text-foreground"
                                    },
                                    "{preview_name}"
                                }
                            }
                            div { class: "rounded-2xl border border-border bg-panel px-4 py-3",
                                p { class: "text-[11px] font-semibold uppercase tracking-[0.12em] text-foreground-muted",
                                    {t.token_symbol}
                                }
                                p {
                                    class: if preview_symbol_is_placeholder {
                                        "mt-2 text-lg font-semibold italic text-foreground-muted"
                                    } else {
                                        "mt-2 text-lg font-semibold text-foreground"
                                    },
                                    "{preview_symbol}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
