use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::common::ProjectPartition;
use crate::common::components::file_uploader::FileUploader;
use crate::common::ui::*;
use crate::features::accounts::context::use_account_context;
use crate::features::console::i18n::ConsoleTranslate;
use crate::features::projects::ProjectResponse;
use crate::features::projects::i18n::ProjectsTranslate;

#[derive(Clone, PartialEq)]
pub enum ProjectEditorMode {
    Create,
    Edit {
        project_id: ProjectPartition,
        project: ProjectResponse,
    },
}

#[component]
pub fn ProjectCreate() -> Element {
    let t: ProjectsTranslate = use_translate();
    let console_t: ConsoleTranslate = use_translate();
    let nav = use_navigator();
    let account_ctx = use_account_context();
    let enterprise_name = account_ctx()
        .enterprise_name()
        .unwrap_or_else(|| "Default enterprise".to_string());

    // Viewers cannot create brands — bounce them back to the list. The
    // redirect runs in an effect (not inline) so every hook above is
    // still registered on every render.
    let can_write = account_ctx().can_write();
    use_effect(move || {
        if !can_write {
            nav.replace(Route::Projects {});
        }
    });
    if !can_write {
        return rsx! {};
    }

    rsx! {
        div { class: "space-y-8",
            PageHeader {
                title: t.create_project.to_string(),
                subtitle: t.create_brand_subtitle_in.replace("{enterprise}", &enterprise_name),
                scope: PageScope::Workspace,
                // Page is about creating a Brand — show "BRANDS" in the
                // breadcrumb tag, not "ENTERPRISE".
                workspace_label: t.brands_breadcrumb.to_string(),
                brand_label: console_t.brand_scope_label.to_string(),
                actions: rsx! {
                    Btn {
                        variant: BtnVariant::Secondary,
                        onclick: move |_| {
                            nav.push(Route::Projects {});
                        },
                        {t.back_to_projects}
                    }
                },
            }

            ProjectEditorCard {
                mode: ProjectEditorMode::Create,
            }
        }
    }
}

/// Legacy `/projects/:project_id/edit` URL.
///
/// Brand editing now lives inline in the Settings tab. Anyone who lands
/// on `/edit` (bookmarks, old links) is redirected to the new location.
#[component]
pub fn ProjectEdit(project_id: ReadSignal<ProjectPartition>) -> Element {
    let nav = use_navigator();
    use_effect(move || {
        nav.replace(Route::ProjectSettings {
            project_id: project_id(),
        });
    });
    rsx! {}
}

#[component]
pub fn ProjectEditorCard(
    mode: ProjectEditorMode,
    #[props(default)] on_saved: Option<EventHandler>,
) -> Element {
    let t: ProjectsTranslate = use_translate();
    let nav = use_navigator();
    let cancel_nav = nav.clone();
    let submit_nav = nav.clone();
    let is_create = matches!(&mode, ProjectEditorMode::Create);

    let existing_project = match &mode {
        ProjectEditorMode::Create => None,
        ProjectEditorMode::Edit { project, .. } => Some(project.clone()),
    };
    let existing_project_id = match &mode {
        ProjectEditorMode::Create => None,
        ProjectEditorMode::Edit { project_id, .. } => Some(project_id.clone()),
    };

    let seed_name = existing_project
        .as_ref()
        .map(|project| project.name.clone())
        .unwrap_or_default();
    let seed_description = existing_project
        .as_ref()
        .and_then(|project| project.description.clone())
        .unwrap_or_default();
    let seed_brand_logo_url = existing_project
        .as_ref()
        .and_then(|project| project.brand_logo_url.clone())
        .unwrap_or_default();
    let mut name = use_signal(move || seed_name.clone());
    let mut description = use_signal(move || seed_description.clone());
    let mut brand_logo_url = use_signal(move || seed_brand_logo_url.clone());
    let mut message = use_signal(|| None::<(AlertVariant, String)>);
    let mut loading = use_signal(|| false);

    let title = if is_create {
        t.create_project.to_string()
    } else {
        t.brand_settings.to_string()
    };
    let helper_text = if is_create {
        t.brand_create_helper
    } else {
        t.brand_edit_helper
    };

    // `preview_*_is_placeholder` flags let the live preview render
    // empty-state values in a muted/italic style so the user does not
    // mistake them for actual input.
    let preview_name_is_placeholder = name().trim().is_empty();
    let preview_name = if preview_name_is_placeholder {
        t.brand.to_string()
    } else {
        name()
    };
    let preview_description_is_placeholder = description().trim().is_empty();
    let preview_description = if preview_description_is_placeholder {
        t.brand_preview_description_placeholder.to_string()
    } else {
        description()
    };
    let preview_logo_url = {
        let current = brand_logo_url();
        if current.trim().is_empty() {
            None
        } else {
            Some(current)
        }
    };
    let settings_saved = t.settings_saved.to_string();
    let save_failure = t.save_failure.to_string();

    rsx! {
        SectionCard {
            div { class: "mb-6 flex flex-col gap-3 xl:flex-row xl:items-end xl:justify-between",
                div {
                    SectionTitle { "{title}" }
                    p { class: "text-sm leading-6 text-foreground-muted", "{helper_text}" }
                }
                if let Some(project_id) = existing_project_id.clone() {
                    div { class: "flex items-center gap-2",
                        code { class: "inline-flex rounded-full border border-border bg-panel-muted px-3 py-1 text-xs font-medium text-foreground-muted",
                            "{project_id}"
                        }
                        CopyButton { value: project_id.clone(), size: CopyButtonSize::Sm }
                    }
                }
            }

            if let Some((variant, text)) = message() {
                div { class: "mb-5",
                    AlertMessage { variant: variant, "{text}" }
                }
            }

            div { class: "grid gap-6 lg:grid-cols-[minmax(0,1.15fr)_minmax(0,0.85fr)]",
                div { class: "space-y-6",
                    div { class: "grid gap-4 md:grid-cols-2",
                        div { class: "md:col-span-2",
                            FormField {
                                label: t.brand_name,
                                value: name(),
                                oninput: move |e: FormEvent| name.set(e.value()),
                                placeholder: t.name_placeholder.to_string(),
                            }
                        }

                        div { class: "md:col-span-2",
                            FormLabel { {t.brand_logo} }
                            FileUploader {
                                prefix: "brand-logos".to_string(),
                                accept: "image/*".to_string(),
                                on_upload_success: move |url: String| brand_logo_url.set(url),
                                class: "mt-2 block".to_string(),
                                div { class: "flex items-center gap-4 rounded-2xl border border-dashed border-border bg-panel-muted px-4 py-4 text-sm text-foreground-muted transition-colors hover:border-brand hover:bg-panel-strong",
                                    if brand_logo_url().trim().is_empty() {
                                        div { class: "flex h-12 w-12 items-center justify-center rounded-xl border border-border bg-panel text-foreground-muted",
                                            IconUpload { class: "h-5 w-5" }
                                        }
                                        div { class: "flex-1",
                                            p { class: "font-semibold text-foreground", {t.brand_logo_upload_cta} }
                                            p { class: "text-xs text-foreground-muted", {t.brand_logo_upload_hint} }
                                        }
                                    } else {
                                        img {
                                            src: "{brand_logo_url()}",
                                            alt: "brand-logo",
                                            class: "h-12 w-12 rounded-xl border border-border bg-panel object-cover",
                                        }
                                        div { class: "flex-1 min-w-0",
                                            p { class: "truncate font-semibold text-foreground", "{brand_logo_url()}" }
                                            p { class: "text-xs text-foreground-muted", {t.brand_logo_change_cta} }
                                        }
                                    }
                                }
                            }
                        }

                        div { class: "md:col-span-2",
                            FormField {
                                label: t.description,
                                value: description(),
                                oninput: move |e: FormEvent| description.set(e.value()),
                                placeholder: t.description_placeholder.to_string(),
                            }
                        }

                        // monthly_supply and treasury_reserve_rate are hardcoded
                        // (1,000,000 and 5%) — will be refactored later.
                    }

                    div { class: "flex flex-col gap-3 pt-2 sm:flex-row sm:justify-end",
                        Btn {
                            variant: BtnVariant::Secondary,
                            disabled: loading(),
                            onclick: move |_| {
                                if is_create {
                                    cancel_nav.push(Route::Projects {});
                                } else if let Some(pid) = existing_project_id.clone() {
                                    cancel_nav.push(Route::ProjectDetail { project_id: pid });
                                }
                            },
                            {t.cancel}
                        }
                        Btn {
                            variant: BtnVariant::Primary,
                            class: "sm:min-w-40",
                            disabled: loading(),
                            onclick: move |_| {
                                let mode = mode.clone();
                                let nav = submit_nav.clone();
                                let on_saved = on_saved.clone();
                                let settings_saved = settings_saved.clone();
                                let save_failure = save_failure.clone();
                                let name_val = name();
                                let desc_val = {
                                    let value = description();
                                    if value.trim().is_empty() { None } else { Some(value) }
                                };
                                let brand_logo_url_val = {
                                    let value = brand_logo_url();
                                    if value.trim().is_empty() { None } else { Some(value) }
                                };
                                spawn(async move {
                                    loading.set(true);
                                    message.set(None);

                                    match mode {
                                        ProjectEditorMode::Create => {
                                            match crate::features::projects::controllers::create_project_handler(
                                                name_val,
                                                desc_val,
                                                brand_logo_url_val,
                                                1_000_000i64,
                                                0.05f64,
                                            )
                                            .await
                                            {
                                                Ok(project) => {
                                                    // Brand creation flows straight into token creation
                                                    // so the user can finish the "brand + token" setup
                                                    // in one pass. Hard navigation (full page load) so
                                                    // the sidebar's `brands_loader` re-fetches and the
                                                    // brand switcher reflects the brand-new project.
                                                    #[cfg(not(feature = "server"))]
                                                    {
                                                        let target = format!(
                                                            "/projects/{}/token/new",
                                                            project.id,
                                                        );
                                                        let _ = web_sys::window()
                                                            .and_then(|w| w.location().assign(&target).ok());
                                                    }
                                                    #[cfg(feature = "server")]
                                                    {
                                                        nav.push(Route::TokenCreate {
                                                            project_id: ProjectPartition::from(project.id),
                                                        });
                                                    }
                                                }
                                                Err(error) => {
                                                    message.set(Some((AlertVariant::Error, error.to_string())));
                                                }
                                            }
                                        }
                                        ProjectEditorMode::Edit { project_id, .. } => {
                                            match crate::features::projects::controllers::update_project_handler(
                                                project_id.clone(),
                                                Some(name_val),
                                                desc_val,
                                                brand_logo_url_val,
                                                Some(1_000_000i64),
                                                Some(0.05f64),
                                                None,
                                            )
                                            .await
                                            {
                                                Ok(_) => {
                                                    message.set(Some((AlertVariant::Success, settings_saved.clone())));
                                                    if let Some(on_saved) = on_saved {
                                                        on_saved.call(());
                                                    } else {
                                                        nav.push(Route::ProjectDetail { project_id });
                                                    }
                                                }
                                                Err(error) => {
                                                    message.set(Some((
                                                        AlertVariant::Error,
                                                        format!("{save_failure}{error}"),
                                                    )));
                                                }
                                            }
                                        }
                                    }

                                    loading.set(false);
                                });
                            },
                            if loading() {
                                if is_create { {t.creating} } else { {t.saving} }
                            } else if is_create {
                                {t.next_create_token}
                            } else {
                                {t.save_settings}
                            }
                        }
                    }
                }

                div { class: "space-y-4",
                    div { class: "rounded-[24px] border border-border bg-panel-muted p-5",
                        p { class: "text-[11px] font-semibold uppercase tracking-[0.14em] text-foreground-muted",
                            {t.live_preview}
                        }
                        div { class: "mt-4 flex items-start gap-4",
                            BrandAvatar {
                                name: preview_name.clone(),
                                logo_url: preview_logo_url,
                                size: BrandAvatarSize::Lg,
                            }
                            div { class: "space-y-2",
                                h4 {
                                    class: if preview_name_is_placeholder {
                                        "font-display text-[1.5rem] font-bold tracking-tight italic text-foreground-muted"
                                    } else {
                                        "font-display text-[1.5rem] font-bold tracking-tight text-foreground"
                                    },
                                    "{preview_name}"
                                }
                                p {
                                    class: if preview_description_is_placeholder {
                                        "text-sm leading-6 italic text-foreground-muted"
                                    } else {
                                        "text-sm leading-6 text-foreground-muted"
                                    },
                                    "{preview_description}"
                                }
                            }
                        }
                    }

                    div { class: "rounded-[24px] border border-border bg-panel-muted p-5",
                        p { class: "text-[11px] font-semibold uppercase tracking-[0.14em] text-foreground-muted",
                            {t.operating_defaults}
                        }
                        div { class: "mt-4 grid gap-3",
                            div { class: "rounded-2xl border border-border bg-panel px-4 py-3",
                                p { class: "text-[11px] font-semibold uppercase tracking-[0.12em] text-foreground-muted",
                                    {t.monthly_supply}
                                }
                                p { class: "mt-2 text-lg font-semibold text-foreground",
                                    "{format_number(1_000_000i64)}"
                                }
                            }
                            div { class: "rounded-2xl border border-border bg-panel px-4 py-3",
                                p { class: "text-[11px] font-semibold uppercase tracking-[0.12em] text-foreground-muted",
                                    {t.treasury_reserve_rate}
                                }
                                p { class: "mt-2 text-lg font-semibold text-foreground",
                                    "5%"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
