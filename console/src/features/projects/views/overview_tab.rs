use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::common::ProjectPartition;
use crate::common::ui::*;
use crate::features::projects::ProjectResponse;
use crate::features::projects::i18n::ProjectsTranslate;

#[component]
pub fn OverviewTab(project_id: ReadSignal<ProjectPartition>, project: ProjectResponse) -> Element {
    let t: ProjectsTranslate = use_translate();

    let token = use_loader(move || async move {
        crate::features::tokens::controllers::get_token_handler(project_id()).await
    });

    let aggregation = use_loader(move || async move {
        let month = chrono::Utc::now().format("%Y-%m").to_string();
        crate::features::points::controllers::get_point_aggregation_handler(project_id(), month)
            .await
    });

    rsx! {
        div { class: "grid gap-6 xl:grid-cols-[minmax(0,0.92fr)_minmax(0,1.08fr)]",
            SectionCard {
                SectionTitle { {t.project_info} }
                // The brand avatar + name header lives in the page header
                // (`ProjectDetailLayout`) just above this card, so we don't
                // repeat it here. The optional description is the only
                // identity-level information that the page header doesn't
                // already show, so we surface it as a single intro line.
                div { class: "space-y-5",
                    if let Some(desc) = project.description.clone() {
                        p { class: "text-sm leading-6 text-foreground-soft", "{desc}" }
                    }

                    div { class: "grid gap-4 sm:grid-cols-2",
                        InfoRow {
                            label: t.project_id.to_string(),
                            value: project.id.clone(),
                            code_like: true,
                            copyable: true,
                        }
                        InfoRow {
                            label: t.created_at.to_string(),
                            value: format_timestamp(project.created_at),
                            code_like: false,
                        }
                        // Updated row only shown when it actually differs
                        // from Created — for a brand-new brand the two are
                        // the same to the millisecond, and showing both
                        // wastes a row and looks like a bug.
                        if project.updated_at > project.created_at {
                            InfoRow {
                                label: t.updated_at.to_string(),
                                value: format_timestamp(project.updated_at),
                                code_like: false,
                            }
                        }
                        InfoRow {
                            label: t.monthly_supply.to_string(),
                            value: format_number(project.monthly_token_supply),
                            code_like: false,
                        }
                        InfoRow {
                            label: t.exchange_ratio.to_string(),
                            value: "1 : 1".to_string(),
                            code_like: false,
                        }
                        InfoRow {
                            label: t.token_value.to_string(),
                            value: "-".to_string(),
                            code_like: false,
                        }
                    }
                }
            }

            SectionCard {
                SectionTitle { {t.token_info} }
                match &token {
                    Ok(loaded) => {
                        let loaded = loaded.read();
                        match &*loaded {
                            Some(tok) => rsx! {
                                div { class: "space-y-5",
                                    div { class: "flex items-center gap-4 rounded-[24px] border border-border bg-panel-muted p-5",
                                        div { class: "flex h-14 w-14 items-center justify-center rounded-[18px] bg-brand-soft text-brand",
                                            IconToken { class: "h-7 w-7" }
                                        }
                                        div {
                                            p { class: "font-display text-xl font-bold tracking-tight text-foreground",
                                                "{tok.name}"
                                            }
                                            StatusBadge { color: BadgeColor::Blue, "{tok.symbol}" }
                                        }
                                    }
                                    if let Some(ref desc) = tok.description {
                                        p { class: "text-sm leading-6 text-foreground-muted", "{desc}" }
                                    }
                                    div { class: "grid gap-4 sm:grid-cols-2",
                                        StatCard { color: StatColor::Gray, label: t.circulating_supply.to_string(), value: format_number(tok.circulating_supply) }
                                        StatCard { color: StatColor::Gray, label: t.decimals.to_string(), value: tok.decimals.to_string() }
                                    }
                                }
                            },
                            None => rsx! {
                                EmptyState {
                                    icon: rsx! { IconToken {} },
                                    title: t.no_token.to_string(),
                                    description: t.no_token_desc.to_string(),
                                }
                            },
                        }
                    }
                    // Backend error (network, auth, etc.) — distinct from "no token yet".
                    // We surface a quiet message rather than the empty-state CTA so the
                    // user is not invited to "create" a token that may already exist
                    // but failed to load.
                    Err(_) => rsx! {
                        div { class: "rounded-[24px] border border-border bg-panel-muted p-6 text-sm text-foreground-muted",
                            {t.token_load_error}
                        }
                    }
                }
            }

            SectionCard {
                SectionTitle { {t.point_info} }
                match &aggregation {
                    Ok(agg) => {
                        let agg = agg.read();
                        let has_activity = agg.awarded_points != 0 || agg.deducted_points != 0;
                        if has_activity {
                            rsx! {
                                div { class: "space-y-5",
                                    div { class: "grid gap-4 sm:grid-cols-2",
                                        StatCard { color: StatColor::Green, label: t.total_awarded.to_string(), value: format_number(agg.awarded_points) }
                                        StatCard { color: StatColor::Red, label: t.total_deducted.to_string(), value: format_number(agg.deducted_points) }
                                    }
                                }
                            }
                        } else {
                            rsx! {
                                EmptyState {
                                    icon: rsx! { IconStar {} },
                                    title: t.no_points_yet.to_string(),
                                    description: t.no_points_desc.to_string(),
                                }
                            }
                        }
                    }
                    Err(_) => rsx! {
                        div { class: "rounded-[24px] border border-border bg-panel-muted p-6 text-sm text-foreground-muted",
                            {t.point_load_error}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn InfoRow(
    label: String,
    value: String,
    code_like: bool,
    #[props(default)] copyable: bool,
) -> Element {
    rsx! {
        div { class: "rounded-[24px] border border-border bg-panel-muted p-4",
            p { class: "text-[11px] font-semibold uppercase tracking-[0.14em] text-foreground-muted",
                "{label}"
            }
            if code_like {
                div { class: "mt-2 flex items-center gap-2",
                    code { class: "block flex-1 break-all rounded-2xl border border-border bg-panel px-3 py-2 text-sm font-medium text-foreground",
                        "{value}"
                    }
                    if copyable {
                        CopyButton { value: value.clone() }
                    }
                }
            } else {
                p { class: "mt-2 text-sm font-semibold text-foreground",
                    "{value}"
                }
            }
        }
    }
}
