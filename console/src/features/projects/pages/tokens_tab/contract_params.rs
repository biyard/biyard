use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::common::ProjectPartition;
use crate::features::projects::i18n::ProjectsTranslate;

/// Returns (effective_months, cumulative_supply).
/// effective_months = month when emission drops below 1% of initial (or 0 if decay is 0 = unlimited).
/// cumulative_supply = total tokens emitted over effective_months (or 0 if unlimited).
pub(super) fn compute_emission_projection(monthly_emission: u64, decay_bps: u16) -> (u32, u64) {
    if decay_bps == 0 {
        return (0, 0); // unlimited — no convergence
    }
    let threshold = (monthly_emission as u128) / 100; // 1% of initial
    let mut total: u128 = 0;
    let mut emission = monthly_emission as u128;
    let mut months: u32 = 0;
    while emission >= threshold.max(1) && months < 1200 {
        total += emission;
        months += 1;
        emission = emission * (10000 - decay_bps as u128) / 10000;
    }
    (months, total as u64)
}

pub(super) fn format_with_commas(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

#[component]
pub(super) fn ContractParamsPanel(
    #[allow(unused)] project_id: ReadSignal<ProjectPartition>,
    token: crate::features::tokens::TokenResponse,
) -> Element {
    let t: ProjectsTranslate = use_translate();

    let monthly_emission = token.monthly_emission.max(0) as u64;
    let decay_bps = token.decay_rate_bps;
    let (effective_months, cumulative) = compute_emission_projection(monthly_emission, decay_bps);
    let params_configured = monthly_emission > 0;
    let slots_count = token.distribution_slots.len();
    let stable_label = token
        .stable_token_address
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(|a| {
            if a.len() > 10 {
                format!("{}...{}", &a[..6], &a[a.len()-4..])
            } else {
                a.to_string()
            }
        })
        .unwrap_or_else(|| "Not set".to_string());

    rsx! {
        div { class: "mt-5 rounded-[24px] border border-border bg-panel-muted p-5",
            p { class: "text-[11px] font-semibold uppercase tracking-[0.14em] text-foreground-muted",
                {t.contract_params_title}
            }
            p { class: "mt-1 text-xs text-foreground-muted leading-5",
                {t.contract_params_desc}
            }

            if params_configured {
                div { class: "mt-4 grid gap-3 sm:grid-cols-2",
                    div { class: "flex items-center justify-between rounded-2xl border border-border bg-panel px-4 py-3",
                        span { class: "text-xs text-foreground-muted", {t.monthly_emission_label} }
                        span { class: "text-sm font-semibold text-foreground",
                            "{format_with_commas(monthly_emission)}"
                        }
                    }
                    div { class: "flex items-center justify-between rounded-2xl border border-border bg-panel px-4 py-3",
                        span { class: "text-xs text-foreground-muted", {t.decay_rate_label} }
                        span { class: "text-sm font-semibold text-foreground",
                            "{decay_bps as f64 / 100.0:.1}%"
                        }
                    }
                    if effective_months > 0 {
                        div { class: "flex items-center justify-between rounded-2xl border border-border bg-panel px-4 py-3",
                            span { class: "text-xs text-foreground-muted", {t.emission_projection_label} }
                            span { class: "text-sm font-semibold text-foreground",
                                "{format_with_commas(cumulative)} ({effective_months} {t.months_label})"
                            }
                        }
                    } else {
                        div { class: "flex items-center justify-between rounded-2xl border border-border bg-panel px-4 py-3",
                            span { class: "text-xs text-foreground-muted", {t.emission_projection_label} }
                            span { class: "text-sm font-semibold text-foreground",
                                {t.unlimited_emission}
                            }
                        }
                    }
                    div { class: "flex items-center justify-between rounded-2xl border border-border bg-panel px-4 py-3",
                        span { class: "text-xs text-foreground-muted", {t.stable_token_label} }
                        span { class: "text-sm font-semibold text-foreground",
                            "{stable_label}"
                        }
                    }
                    div { class: "flex items-center justify-between rounded-2xl border border-border bg-panel px-4 py-3",
                        span { class: "text-xs text-foreground-muted", {t.distribution_slots_setup_title} }
                        span { class: "text-sm font-semibold text-foreground",
                            "{slots_count} slots"
                        }
                    }
                }
            } else {
                div { class: "mt-4 rounded-2xl border border-warning bg-warning-soft px-4 py-3",
                    p { class: "text-sm text-foreground",
                        {t.contract_params_not_set}
                    }
                }
            }
        }
    }
}
