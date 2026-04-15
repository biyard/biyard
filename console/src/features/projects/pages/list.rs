use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::common::ui::*;
use crate::features::accounts::context::use_account_context;
use crate::features::console::i18n::ConsoleTranslate;
use crate::features::projects::ProjectStatus;
use crate::features::projects::i18n::ProjectsTranslate;

#[component]
pub fn Projects() -> Element {
    let t: ProjectsTranslate = use_translate();
    let console_t: ConsoleTranslate = use_translate();
    let nav = use_navigator();
    let account_ctx = use_account_context();
    let can_write = account_ctx().can_write();
    let enterprise_name = account_ctx()
        .enterprise_name()
        .unwrap_or_else(|| "Default enterprise".to_string());

    let projects = use_loader(move || async move {
        crate::features::projects::controllers::list_projects_handler(100, None).await
    })?;

    let projects_data = projects();
    let active_projects = projects_data
        .items
        .iter()
        .filter(|project| project.status == ProjectStatus::Active)
        .count();
    let total_supply = projects_data
        .items
        .iter()
        .map(|project| project.monthly_token_supply)
        .sum::<i64>();

    rsx! {
        div { class: "space-y-8",
            PageHeader {
                title: t.title.to_string(),
                subtitle: t.brands_page_subtitle_in.replace("{enterprise}", &enterprise_name),
                scope: PageScope::Workspace,
                workspace_label: console_t.enterprise_scope_label.to_string(),
                brand_label: console_t.brand_scope_label.to_string(),
                actions: rsx! {
                    if can_write {
                        Btn {
                            variant: BtnVariant::Primary,
                            class: "flex items-center",
                            onclick: move |_| {
                                nav.push(Route::ProjectCreate {});
                            },
                            IconPlus { class: "h-5 w-5" }
                            {t.create_new}
                        }
                    }
                },
            }

            div { class: "grid gap-4 sm:grid-cols-2 lg:grid-cols-3",
                StatCard {
                    color: StatColor::Blue,
                    label: t.title.to_string(),
                    value: projects_data.items.len().to_string(),
                }
                StatCard {
                    color: StatColor::Green,
                    label: t.active.to_string(),
                    value: active_projects.to_string(),
                }
                StatCard {
                    color: StatColor::Purple,
                    label: t.monthly_supply.to_string(),
                    value: format_number(total_supply),
                }
            }

            if projects_data.items.is_empty() {
                EmptyState {
                    icon: rsx! { IconFolderOpen {} },
                    title: t.no_projects.to_string(),
                    description: t.no_projects_desc.to_string(),
                    actions: rsx! {
                        if can_write {
                            Btn {
                                variant: BtnVariant::Primary,
                                class: "flex items-center",
                                onclick: move |_| {
                                    nav.push(Route::ProjectCreate {});
                                },
                                {t.create_new}
                            }
                        }
                    },
                }
            } else {
                // One brand per row. Brands are few and each card gets
                // a roomy landscape layout that makes the key info
                // readable at a glance.
                div { class: "flex flex-col gap-5",
                    for project in projects_data.items.iter() {
                        BrandCard {
                            key: "{project.id}",
                            id: project.id.clone(),
                            name: project.name.clone(),
                            description: project.description.clone(),
                            logo_url: project.brand_logo_url.clone(),
                            status: project.status,
                            monthly_supply: project.monthly_token_supply,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn BrandCard(
    id: String,
    name: String,
    description: Option<String>,
    logo_url: Option<String>,
    status: ProjectStatus,
    monthly_supply: i64,
) -> Element {
    let t: ProjectsTranslate = use_translate();
    let nav = use_navigator();

    let id_for_nav = id.clone();

    // Delete affordance is intentionally absent here. Brand deletion
    // is a destructive, irreversible action and must go through the
    // brand settings page where the user types the brand name to
    // confirm. Keeping a one-click trash icon on a list card makes it
    // too easy to misfire.
    //
    // The card is rendered as a `<div role="link">` rather than a
    // `<button>` because it contains an `<h3>` heading and a `<code>`
    // chip — flow content that HTML forbids inside an interactive
    // element. The keyboard handler restores Enter/Space activation.
    rsx! {
        div {
            class: "group flex w-full flex-col gap-6 rounded-3xl border border-border bg-panel p-8 text-left transition-all hover:-translate-y-0.5 hover:border-brand/40 md:flex-row md:items-center md:gap-8",
            role: "link",
            tabindex: "0",
            onclick: move |_| {
                nav.push(Route::ProjectDetail {
                    project_id: id_for_nav.clone().into(),
                });
            },

            // Logo / avatar
            BrandAvatar {
                name: name.clone(),
                logo_url: logo_url.clone(),
                size: BrandAvatarSize::Xl,
            }

            // Name + description + id
            div { class: "min-w-0 flex-1 space-y-3",
                div { class: "flex flex-wrap items-center gap-3",
                    h3 { class: "font-display text-2xl font-bold tracking-tight text-foreground",
                        "{name}"
                    }
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
                if let Some(desc) = description.clone() {
                    p { class: "line-clamp-2 max-w-2xl text-sm leading-6 text-foreground-muted",
                        "{desc}"
                    }
                }
                code {
                    class: "inline-flex rounded-full border border-border bg-panel-muted px-3 py-1 text-xs font-medium text-foreground-muted",
                    title: "{id}",
                    "{shorten_id(&id)}"
                }
            }

            // Monthly supply stat on the right
            div { class: "shrink-0 rounded-2xl border border-border bg-panel-muted px-5 py-4 text-right",
                p { class: "text-[10px] font-semibold uppercase tracking-[0.14em] text-foreground-muted",
                    {t.monthly_supply}
                }
                p { class: "mt-1 font-display text-xl font-bold tracking-tight text-foreground",
                    "{format_number(monthly_supply)}"
                }
            }
        }
    }
}
