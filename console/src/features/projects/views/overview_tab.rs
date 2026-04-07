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
                div { class: "space-y-5",
                    div { class: "flex items-start gap-4 rounded-[24px] border border-border bg-panel-muted p-5",
                        BrandAvatar {
                            name: project.name.clone(),
                            logo_url: project.brand_logo_url.clone(),
                            size: BrandAvatarSize::Md,
                        }
                        div { class: "space-y-2",
                            p { class: "font-display text-xl font-bold tracking-tight text-foreground",
                                "{project.name}"
                            }
                            if let Some(desc) = project.description.clone() {
                                p { class: "text-sm leading-6 text-foreground-muted", "{desc}" }
                            }
                        }
                    }

                    div { class: "grid gap-4 sm:grid-cols-2",
                        InfoRow {
                            label: t.project_id.to_string(),
                            value: project.id.clone(),
                            code_like: true,
                        }
                        InfoRow {
                            label: t.created_at.to_string(),
                            value: format_timestamp(project.created_at),
                            code_like: false,
                        }
                        InfoRow {
                            label: t.updated_at.to_string(),
                            value: format_timestamp(project.updated_at),
                            code_like: false,
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
                    Ok(tok) => {
                        let tok = &*tok.read();
                        rsx! {
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
                                div { class: "grid gap-4 sm:grid-cols-3",
                                    StatCard { color: StatColor::Gray, label: t.total_supply.to_string(), value: format_number(tok.total_supply) }
                                    StatCard { color: StatColor::Gray, label: t.circulating_supply.to_string(), value: format_number(tok.circulating_supply) }
                                    StatCard { color: StatColor::Gray, label: t.decimals.to_string(), value: tok.decimals.to_string() }
                                }
                            }
                        }
                    }
                    Err(_) => rsx! {
                        EmptyState {
                            icon: rsx! { IconToken {} },
                            title: t.no_token.to_string(),
                            description: t.no_token_desc.to_string(),
                        }
                    }
                }
            }

            SectionCard {
                SectionTitle { {t.point_info} }
                match &aggregation {
                    Ok(agg) => {
                        let agg = &*agg.read();
                        rsx! {
                            div { class: "space-y-5",
                                div { class: "grid gap-4 sm:grid-cols-2",
                                    StatCard { color: StatColor::Green, label: t.total_awarded.to_string(), value: format_number(agg.awarded_points) }
                                    StatCard { color: StatColor::Red, label: t.total_deducted.to_string(), value: format_number(agg.deducted_points) }
                                }
                                p { class: "text-sm leading-6 text-foreground-muted",
                                    {t.no_points_desc}
                                }
                            }
                        }
                    }
                    Err(_) => rsx! {
                        EmptyState {
                            icon: rsx! { IconStar {} },
                            title: t.no_points_yet.to_string(),
                            description: t.no_points_desc.to_string(),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn InfoRow(label: String, value: String, code_like: bool) -> Element {
    rsx! {
        div { class: "rounded-[24px] border border-border bg-panel-muted p-4",
            p { class: "text-[11px] font-semibold uppercase tracking-[0.14em] text-foreground-muted",
                "{label}"
            }
            if code_like {
                code { class: "mt-2 block break-all rounded-2xl border border-border bg-panel px-3 py-2 text-sm font-medium text-foreground",
                    "{value}"
                }
            } else {
                p { class: "mt-2 text-sm font-semibold text-foreground",
                    "{value}"
                }
            }
        }
    }
}
