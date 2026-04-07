use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::common::ProjectPartition;
use crate::common::ui::*;
use crate::features::projects::ProjectResponse;
use crate::features::projects::i18n::ProjectsTranslate;

/// `/projects/:project_id/treasury` — brand treasury simulator.
///
/// This page is the **only** place in the UI that shows simulated
/// treasury/sales numbers. Everywhere else those values are hidden
/// because they aren't real operational metrics (see the CLAUDE.md
/// domain decisions around the treasury simulator).
///
/// Behavior:
/// - Shows the current simulated treasury balance and cumulative sales
///   side-by-side, clearly labeled as simulation.
/// - The revenue input applies `simulate_revenue_handler` which adds
///   the entered amount to `simulated_sales_total` and bumps
///   `treasury_balance` by `revenue * treasury_reserve_rate`.
/// - After a successful apply, the loader restarts so the displayed
///   balance reflects the new server values immediately.
#[component]
pub fn ProjectTreasury(project_id: ReadSignal<ProjectPartition>) -> Element {
    let t: ProjectsTranslate = use_translate();

    let mut project_loader = use_loader(move || async move {
        crate::features::projects::controllers::get_project_handler(project_id()).await
    })?;
    let project: ProjectResponse = project_loader();

    let mut revenue_input = use_signal(String::new);
    let mut applying = use_signal(|| false);
    let mut message = use_signal(|| None::<(AlertVariant, String)>);

    let reserve_rate_pct = (project.treasury_reserve_rate * 100.0).round();

    let on_apply = move |_| {
        let raw = revenue_input();
        let amount = raw.trim().parse::<i64>().unwrap_or(0);
        if amount <= 0 {
            message.set(Some((
                AlertVariant::Error,
                t.simulation_failure.to_string(),
            )));
            return;
        }

        applying.set(true);
        message.set(None);
        let pid = project_id();
        spawn(async move {
            let res = crate::features::projects::controllers::simulate_revenue_handler(
                pid, amount,
            )
            .await;
            match res {
                Ok(_) => {
                    revenue_input.set(String::new());
                    project_loader.restart();
                    message.set(Some((
                        AlertVariant::Success,
                        t.simulation_success.to_string(),
                    )));
                }
                Err(e) => {
                    message.set(Some((
                        AlertVariant::Error,
                        format!("{}{e}", t.simulation_failure),
                    )));
                }
            }
            applying.set(false);
        });
    };

    rsx! {
        div { class: "space-y-6",
            // Simulation scope banner — always visible so the user
            // never confuses these numbers for real operational data.
            AlertMessage { variant: AlertVariant::Info,
                {t.treasury_simulation}
            }

            SectionCard {
                SectionTitle { {t.treasury_simulation} }

                div { class: "grid gap-4 sm:grid-cols-3",
                    StatCard {
                        color: StatColor::Emerald,
                        label: t.treasury_balance.to_string(),
                        value: format_number(project.treasury_balance),
                    }
                    StatCard {
                        color: StatColor::Indigo,
                        label: t.simulated_sales_total.to_string(),
                        value: format_number(project.simulated_sales_total),
                    }
                    StatCard {
                        color: StatColor::Blue,
                        label: t.treasury_reserve_rate.to_string(),
                        value: format!("{reserve_rate_pct}%"),
                    }
                }
            }

            SectionCard {
                SectionTitle { {t.revenue_to_treasury_simulation} }
                div { class: "space-y-5",
                    if let Some((variant, msg)) = message() {
                        AlertMessage { variant: variant, "{msg}" }
                    }

                    FormField {
                        label: t.revenue_input,
                        r#type: "number",
                        value: revenue_input(),
                        oninput: move |e: FormEvent| revenue_input.set(e.value()),
                        placeholder: t.revenue_input_placeholder.to_string(),
                        min: "0",
                    }

                    div { class: "flex justify-end",
                        Btn {
                            variant: BtnVariant::Primary,
                            disabled: applying() || revenue_input().trim().is_empty(),
                            onclick: on_apply,
                            if applying() { {t.applying} } else { {t.apply_revenue} }
                        }
                    }
                }
            }
        }
    }
}
