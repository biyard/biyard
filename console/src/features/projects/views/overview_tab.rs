use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::common::ProjectPartition;
use crate::common::ui::*;
use crate::features::projects::ProjectResponse;
use crate::features::projects::i18n::ProjectsTranslate;

#[component]
pub fn OverviewTab(project_id: ReadSignal<ProjectPartition>, project: ProjectResponse) -> Element {
    let t: ProjectsTranslate = use_translate();

    let token_result = use_loader(move || async move {
        crate::features::tokens::controllers::get_token_handler(project_id()).await
    });

    let treasury_result = use_loader(move || async move {
        crate::features::projects::controllers::get_treasury_status_handler(project_id()).await
    });

    let aggregation_result = use_loader(move || async move {
        let tok =
            crate::features::tokens::controllers::get_token_handler(project_id()).await?;
        let treasury =
            crate::features::projects::controllers::get_treasury_status_handler(project_id())
                .await?;

        let month = if treasury.deployed {
            if let Some(ref tok) = tok {
                month_index_to_str(treasury.current_month, tok.created_at)
            } else {
                chrono::Utc::now().format("%Y-%m").to_string()
            }
        } else {
            chrono::Utc::now().format("%Y-%m").to_string()
        };

        crate::features::points::controllers::get_point_aggregation_handler(project_id(), month)
            .await
    });

    // Suspend after all hooks are registered (hook call order rule).
    let token = token_result?;
    let treasury = treasury_result?;
    let aggregation = aggregation_result?;

    let tok = token();
    let treasury_status = treasury();
    let agg = aggregation();

    let (total_supply_display, circulating_display) = if treasury_status.deployed {
        (
            format_token_amount(&treasury_status.total_supply_raw, treasury_status.token_decimals),
            format_token_amount(
                &treasury_status.circulating_supply_raw,
                treasury_status.token_decimals,
            ),
        )
    } else {
        (
            "0".to_string(),
            format_number(tok.as_ref().map(|t| t.circulating_supply).unwrap_or(0)),
        )
    };

    rsx! {
        div { class: "flex flex-col gap-6",
            SectionCard {
                SectionTitle { {t.project_info} }
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
                        if project.updated_at > project.created_at {
                            InfoRow {
                                label: t.updated_at.to_string(),
                                value: format_timestamp(project.updated_at),
                                code_like: false,
                            }
                        }
                    }
                }
            }

            SectionCard {
                SectionTitle { {t.token_info} }
                match &tok {
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
                                StatCard { color: StatColor::Gray, label: t.treasury_onchain_total_supply.to_string(), value: total_supply_display.clone() }
                                StatCard { color: StatColor::Gray, label: t.circulating_supply.to_string(), value: circulating_display.clone() }
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

            SectionCard {
                SectionTitle { {t.point_info} }
                {
                    let has_activity = agg.awarded_points != 0 || agg.deducted_points != 0 || agg.exchanged_points != 0;
                    if has_activity {
                        rsx! {
                            div { class: "space-y-5",
                                div { class: "grid gap-4 sm:grid-cols-2 lg:grid-cols-3",
                                    StatCard { color: StatColor::Green, label: t.total_awarded.to_string(), value: format_number(agg.awarded_points) }
                                    StatCard { color: StatColor::Red, label: t.total_deducted.to_string(), value: format_number(agg.deducted_points) }
                                    StatCard { color: StatColor::Blue, label: t.total_exchanged.to_string(), value: format_number(agg.exchanged_points) }
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
            }
        }
    }
}

/// Convert an on-chain month index back to a "YYYY-MM" string using the
/// token's `created_at` timestamp (milliseconds) as the deploy epoch.
///
/// The contract computes `currentMonth = (now - deployTimestamp) / 30 days + monthOffset`.
/// Here we reverse that: `deploy_months_since_epoch + month_index` → year/month.
fn month_index_to_str(month_index: u64, token_created_at_ms: i64) -> String {
    let deploy_secs = token_created_at_ms / 1000;
    let deploy_months_since_epoch = deploy_secs / (30 * 24 * 3600);
    let target = deploy_months_since_epoch as u64 + month_index;
    let year = 1970 + target / 12;
    let month = (target % 12) + 1;
    format!("{year:04}-{month:02}")
}

/// Format a raw on-chain token amount (u128 string) into a human-readable
/// display with decimal point and comma separators.
fn format_token_amount(raw: &str, decimals: u8) -> String {
    let Ok(value) = raw.parse::<u128>() else {
        return raw.to_string();
    };

    if decimals == 0 {
        return format_number(value as i64);
    }

    let divisor = 10u128.pow(decimals as u32);
    let whole = value / divisor;
    let frac = value % divisor;

    if frac == 0 {
        format_number(whole as i64)
    } else {
        let frac_str = format!("{:0>width$}", frac, width = decimals as usize);
        let trimmed = frac_str.trim_end_matches('0');
        format!("{}.{}", format_number(whole as i64), trimmed)
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
