use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::common::ui::*;
use crate::features::accounts::context::use_account_context;
use crate::features::console::i18n::ConsoleTranslate;
use crate::features::credentials::{CredentialStatus, i18n::CredentialsTranslate};
use crate::features::projects::{ProjectStatus, i18n::ProjectsTranslate};

#[component]
pub fn Dashboard() -> Element {
    let t: ConsoleTranslate = use_translate();
    let project_t: ProjectsTranslate = use_translate();
    let credential_t: CredentialsTranslate = use_translate();
    let account_ctx = use_account_context();

    let projects_result = use_loader(move || async move {
        crate::features::projects::controllers::list_projects_handler(100, None).await
    });
    let credentials_result = use_loader(move || async move {
        crate::features::credentials::controllers::list_credentials_handler().await
    });
    let projects = projects_result?;
    let credentials = credentials_result?;

    let Some(account) = account_ctx().account.clone() else {
        return rsx! {
            div { class: "space-y-8",
                div { class: "text-sm font-medium text-foreground-muted", {t.loading} }
            }
        };
    };
    let enterprise_name = account_ctx()
        .enterprise_name()
        .unwrap_or_else(|| "Default enterprise".to_string());

    let project_list = projects();
    let credential_list = credentials();

    let total_projects = project_list.items.len();
    let active_projects = project_list
        .items
        .iter()
        .filter(|project| project.status == ProjectStatus::Active)
        .count();
    let active_credentials = credential_list
        .iter()
        .filter(|credential| credential.status == CredentialStatus::Active)
        .count();
    let revoked_credentials = credential_list.len().saturating_sub(active_credentials);

    let enterprise_ready_headline = t
        .enterprise_ready_headline
        .replace("{name}", &account.name)
        .replace("{enterprise}", &enterprise_name);
    let enterprise_overview_subtitle = t
        .enterprise_overview_subtitle
        .replace("{enterprise}", &enterprise_name);

    rsx! {
        div { class: "space-y-8",
            PageHeader {
                title: t.enterprise_overview_title.to_string(),
                subtitle: enterprise_overview_subtitle,
                scope: PageScope::Workspace,
                workspace_label: t.enterprise_scope_label.to_string(),
                brand_label: t.brand_scope_label.to_string(),
                actions: rsx! {
                    Link {
                        to: Route::Projects {},
                        class: "inline-flex items-center justify-center gap-2 rounded-2xl border border-border bg-panel px-4 py-2.5 text-sm font-semibold text-foreground transition-colors hover:bg-panel-strong",
                        IconPlus { class: "h-4 w-4" }
                        {project_t.create_new}
                    }
                    Link {
                        to: Route::Credentials {},
                        class: "inline-flex items-center justify-center gap-2 rounded-2xl border border-brand bg-brand px-4 py-2.5 text-sm font-semibold text-brand-contrast transition-colors hover:border-brand-strong hover:bg-brand-strong",
                        {t.api_credentials}
                    }
                },
            }

            // Enterprise summary card. Uses the surface tokens (panel +
            // border) so it adapts to both light and dark themes instead
            // of relying on a single grey-on-grey gradient that read as
            // dead in light mode.
            div { class: "relative overflow-hidden rounded-3xl border border-border bg-panel px-8 py-8 text-foreground",
                div { class: "pointer-events-none absolute right-[-4rem] top-[-4rem] h-56 w-56 rounded-full bg-brand/10 blur-2xl" }

                div { class: "relative flex flex-col gap-8",
                    div { class: "max-w-3xl",
                        p { class: "text-[11px] font-semibold uppercase tracking-[0.18em] text-foreground-muted",
                            {t.enterprise_summary}
                        }
                        h2 { class: "mt-3 font-display text-2xl font-bold leading-tight tracking-[-0.04em] text-foreground sm:text-3xl",
                            "{enterprise_ready_headline}"
                        }
                        p { class: "mt-4 max-w-2xl text-sm font-medium leading-6 text-foreground-muted",
                            {t.welcome_description}
                        }
                        div { class: "mt-6 flex flex-wrap gap-3",
                            div { class: "rounded-full border border-border bg-panel-muted px-3 py-2 text-xs font-semibold uppercase tracking-[0.12em] text-foreground-soft",
                                "{project_t.title}: {total_projects}"
                            }
                            div { class: "rounded-full border border-border bg-panel-muted px-3 py-2 text-xs font-semibold uppercase tracking-[0.12em] text-foreground-soft",
                                "{credential_t.title}: {credential_list.len()}"
                            }
                            div { class: "rounded-full border border-border bg-panel-muted px-3 py-2 text-xs font-semibold uppercase tracking-[0.12em] text-foreground-soft",
                                "{account.email}"
                            }
                        }
                    }
                }
            }

            div { class: "grid gap-4 sm:grid-cols-2 lg:grid-cols-3",
                StatCard {
                    color: StatColor::Blue,
                    label: t.my_projects.to_string(),
                    value: total_projects.to_string(),
                }
                StatCard {
                    color: StatColor::Green,
                    label: project_t.active.to_string(),
                    value: active_projects.to_string(),
                }
                StatCard {
                    // Neutral by default — `0 active credentials` is not a warning,
                    // just a fresh account.
                    color: StatColor::Gray,
                    label: credential_t.active.to_string(),
                    value: active_credentials.to_string(),
                }
            }

            div { class: "grid gap-6 xl:grid-cols-[minmax(0,1.35fr)_minmax(0,0.85fr)]",
                SectionCard {
                    div { class: "mb-6 flex items-center justify-between gap-4",
                        div {
                            SectionTitle { {t.recent_brands_title} }
                            p { class: "text-sm text-foreground-muted", {t.recent_brands_desc} }
                        }
                        Link {
                            to: Route::Projects {},
                            class: "text-sm font-semibold text-brand transition-colors hover:text-brand-strong",
                            {project_t.back_to_projects}
                        }
                    }

                    if project_list.items.is_empty() {
                        EmptyState {
                            icon: rsx! {
                                IconFolderOpen {}
                            },
                            title: project_t.no_projects.to_string(),
                            description: project_t.no_projects_desc.to_string(),
                            actions: rsx! {
                                Link {
                                    to: Route::Projects {},
                                    class: "inline-flex items-center justify-center gap-2 rounded-2xl border border-brand bg-brand px-4 py-2.5 text-sm font-semibold text-brand-contrast transition-colors hover:border-brand-strong hover:bg-brand-strong",
                                    {project_t.create_new}
                                }
                            },
                        }
                    } else {
                        div { class: "space-y-4",
                            for project in project_list.items.iter().take(3) {
                                {
                                    let id = project.id.clone();
                                    let name = project.name.clone();
                                    let logo_url = project.brand_logo_url.clone();
                                    let description = project
                                        .description
                                        .clone()
                                        .unwrap_or_else(|| project_t.project_info.to_string());
                                    let monthly_supply = project.monthly_token_supply;
                                    let status = project.status;
                                    rsx! {
                                        Link {
                                            to: Route::ProjectDetail {
                                                project_id: id.clone().into(),
                                            },
                                            class: "block rounded-[24px] border border-border bg-panel-muted p-5 transition-all hover:-translate-y-0.5 hover:border-border-strong hover:bg-panel-strong",
                                            div { class: "flex flex-col gap-4 md:flex-row md:items-start md:justify-between",
                                                div { class: "space-y-2",
                                                    div { class: "flex items-center gap-3",
                                                        BrandAvatar {
                                                            name: name.clone(),
                                                            logo_url,
                                                            size: BrandAvatarSize::Sm,
                                                        }
                                                        div {
                                                            p { class: "font-display text-lg font-bold tracking-tight text-foreground",
                                                                "{name}"
                                                            }
                                                            p { class: "text-sm text-foreground-muted", "{description}" }
                                                        }
                                                    }
                                                    code { class: "inline-flex rounded-full border border-border bg-panel px-3 py-1 text-xs font-medium text-foreground-muted",
                                                        "{project.id}"
                                                    }
                                                }
                                                div { class: "flex flex-wrap items-center gap-3 md:justify-end",
                                                    // Self-shrink so the pill stays its natural
                                                    // size instead of stretching to match the
                                                    // taller stat tiles next to it.
                                                    div { class: "self-center",
                                                        StatusBadge {
                                                            color: match status {
                                                                ProjectStatus::Active => BadgeColor::Green,
                                                                ProjectStatus::Inactive => BadgeColor::Gray,
                                                            },
                                                            match status {
                                                                ProjectStatus::Active => project_t.active,
                                                                ProjectStatus::Inactive => project_t.inactive,
                                                            }
                                                        }
                                                    }
                                                    div { class: "rounded-2xl border border-border bg-panel px-3 py-2 text-right",
                                                        p { class: "text-[11px] font-semibold uppercase tracking-[0.14em] text-foreground-muted",
                                                            {project_t.monthly_supply}
                                                        }
                                                        p { class: "mt-1 text-sm font-semibold text-foreground",
                                                            "{format_number(monthly_supply)}"
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

                div { class: "space-y-6",
                    SectionCard {
                        div { class: "mb-6 flex items-center justify-between gap-4",
                            div {
                                SectionTitle { {t.api_credentials} }
                                p { class: "text-sm text-foreground-muted",
                                    {credential_t.description}
                                }
                            }
                            Link {
                                to: Route::Credentials {},
                                class: "text-sm font-semibold text-brand transition-colors hover:text-brand-strong",
                                {credential_t.create_new}
                            }
                        }
                        div { class: "grid gap-4 sm:grid-cols-3",
                            StatCard {
                                color: StatColor::Gray,
                                label: credential_t.title.to_string(),
                                value: credential_list.len().to_string(),
                            }
                            StatCard {
                                color: StatColor::Green,
                                label: credential_t.active.to_string(),
                                value: active_credentials.to_string(),
                            }
                            StatCard {
                                color: StatColor::Red,
                                label: credential_t.inactive.to_string(),
                                value: revoked_credentials.to_string(),
                            }
                        }
                    }

                    SectionCard {
                        SectionTitle { {t.account_info} }
                        div { class: "space-y-4",
                            div { class: "rounded-[24px] border border-border bg-panel-muted p-4",
                                p { class: "text-[11px] font-semibold uppercase tracking-[0.14em] text-foreground-muted",
                                    {t.my_account}
                                }
                                p { class: "mt-2 font-display text-xl font-bold tracking-tight text-foreground",
                                    "{account.name}"
                                }
                                p { class: "mt-1 text-sm text-foreground-muted", "{account.email}" }
                            }

                            div { class: "grid gap-4 sm:grid-cols-2",
                                div { class: "rounded-[24px] border border-border bg-panel-muted p-4",
                                    p { class: "text-[11px] font-semibold uppercase tracking-[0.14em] text-foreground-muted",
                                        {t.created_at}
                                    }
                                    p { class: "mt-2 text-sm font-semibold text-foreground",
                                        {format_timestamp(account.created_at)}
                                    }
                                }
                                div { class: "rounded-[24px] border border-border bg-panel-muted p-4",
                                    p { class: "text-[11px] font-semibold uppercase tracking-[0.14em] text-foreground-muted",
                                        {t.account_id}
                                    }
                                    p { class: "mt-2 break-all text-sm font-semibold text-foreground",
                                        "{account.id()}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if active_projects < total_projects {
                AlertMessage { variant: AlertVariant::Info, {t.some_brands_inactive} }
            }
        }
    }
}
