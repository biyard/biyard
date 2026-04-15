mod overview;
mod points;

pub use overview::ProjectDetail;
pub use points::ProjectPoints;

use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::common::ProjectPartition;
use crate::common::ui::*;
use crate::features::accounts::context::use_account_context;
use crate::features::console::i18n::ConsoleTranslate;
use crate::features::projects::ProjectStatus;
use crate::features::projects::i18n::ProjectsTranslate;

/// Shared layout for all `/projects/:project_id/*` detail pages.
///
/// Loads the project once, renders the header (brand badge, stats,
/// edit CTA), and delegates the body to the nested route via
/// `<Outlet>`. Tab navigation for Overview / Token / Points /
/// Settings lives in the sidebar (A-pattern brand scope).
#[component]
pub fn ProjectDetailLayout(project_id: ReadSignal<ProjectPartition>) -> Element {
    let t: ProjectsTranslate = use_translate();
    let console_t: ConsoleTranslate = use_translate();
    let nav = use_navigator();
    let account_ctx = use_account_context();
    let enterprise_name = account_ctx()
        .enterprise_name()
        .unwrap_or_else(|| "Default enterprise".to_string());

    let project = use_loader(move || async move {
        crate::features::projects::controllers::get_project_handler(project_id()).await
    })?;

    let project_data = project();

    rsx! {
        div { class: "space-y-8",
            div { class: "rounded-3xl border border-border bg-panel p-6",
                div { class: "mb-6 flex items-center gap-3",
                    button {
                        class: "inline-flex items-center gap-2 text-sm font-semibold text-foreground-muted transition-colors hover:text-foreground",
                        onclick: move |_| { nav.push(Route::Projects {}); },
                        IconArrowLeft { class: "h-4 w-4" }
                        {t.back_to_projects}
                    }
                }

                div { class: "space-y-5",
                    div { class: "flex items-start gap-4",
                        BrandAvatar {
                            name: project_data.name.clone(),
                            logo_url: project_data.brand_logo_url.clone(),
                            size: BrandAvatarSize::Md,
                        }
                        div { class: "space-y-3",
                            span { class: "inline-flex items-center gap-1.5 rounded-full border border-brand/30 bg-brand-soft px-3 py-1 text-[10px] font-semibold uppercase tracking-[0.16em] text-brand",
                                "{console_t.brand_scope_label} · {project_data.name}"
                            }
                            p { class: "text-[11px] font-semibold uppercase tracking-[0.14em] text-foreground-muted",
                                "{console_t.enterprise_scope_label} / {enterprise_name}"
                            }
                            div { class: "flex flex-wrap items-center gap-3",
                                h1 { class: "font-display text-2xl font-bold tracking-[-0.04em] text-foreground sm:text-3xl lg:text-[2.25rem]",
                                    "{project_data.name}"
                                }
                                StatusBadge {
                                    color: match project_data.status {
                                        ProjectStatus::Active => BadgeColor::Green,
                                        ProjectStatus::Inactive => BadgeColor::Gray,
                                    },
                                    match project_data.status {
                                        ProjectStatus::Active => {t.active},
                                        ProjectStatus::Inactive => {t.inactive},
                                    }
                                }
                            }
                            if let Some(ref desc) = project_data.description {
                                p { class: "max-w-2xl text-sm leading-6 text-foreground-muted",
                                    "{desc}"
                                }
                            }
                            code {
                                class: "inline-flex rounded-full border border-border bg-panel-muted px-3 py-1 text-xs font-medium text-foreground-muted",
                                title: "{project_data.id}",
                                "{shorten_id(&project_data.id)}"
                            }
                        }
                    }
                }
            }

            SuspenseBoundary {
                fallback: move |_| rsx! {
                    div { class: "flex justify-center py-10",
                        Spinner { class: "h-5 w-5 animate-spin" }
                    }
                },
                Outlet::<Route> {}
            }
        }
    }
}
