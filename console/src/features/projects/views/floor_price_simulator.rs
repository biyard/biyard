use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::common::components::dialog::{
    DialogActions, DialogContent, DialogDescription, DialogRoot, DialogTitle,
};
use crate::common::ui::*;
use crate::features::projects::i18n::ProjectsTranslate;

// Chart.js bridge. The actual `Chart` global is loaded from the CDN
// via a `document::Script` tag below; `floor_chart.js` registers a
// narrow facade on `window.biyard.floorChart`. All calls are gated
// behind `cfg(not(feature = "server"))` so the SSR build stays lean.
#[cfg(not(feature = "server"))]
#[wasm_bindgen::prelude::wasm_bindgen(js_namespace = ["window", "biyard", "floorChart"])]
extern "C" {
    #[wasm_bindgen(js_name = render)]
    fn floor_chart_render(
        canvas_id: &str,
        labels: wasm_bindgen::JsValue,
        treasury: wasm_bindgen::JsValue,
        supply: wasm_bindgen::JsValue,
        floor: wasm_bindgen::JsValue,
    );

    #[wasm_bindgen(js_name = destroy)]
    fn floor_chart_destroy(canvas_id: &str);
}

const CHART_CANVAS_ID: &str = "biyard-floor-simulator-chart";

/// What-if simulator for the floor price mechanism.
///
/// 100% client-side — nothing touches DynamoDB, the blockchain, or any
/// controller. The user can tune initial parameters (reserve rate,
/// circulating supply, treasury balance) and then fire actions:
/// - Record a sale: adds `amount * reserve_rate` to treasury (no supply change)
/// - Reward mint: adds tokens to circulating supply without adding treasury
///   (demonstrates floor price dilution)
/// - Redeem: removes tokens at current floor price and burns them
///   (demonstrates floor price preservation during buybacks)
///
/// Every action is logged in a table so operators can walk a customer
/// through the mechanism step-by-step. "Reset" clears state and the log.
#[component]
pub fn FloorPriceSimulatorDialog(
    open: bool,
    on_close: EventHandler<()>,
    initial_reserve_rate: f64,
) -> Element {
    let t: ProjectsTranslate = use_translate();

    // --- configuration inputs ---
    let mut reserve_rate_pct = use_signal(|| (initial_reserve_rate * 100.0).round() as i64);

    // --- simulator state ---
    // Treasury stable balance and circulating supply are the only two
    // quantities needed to compute the floor price.
    let mut treasury = use_signal(|| 0i64);
    let mut supply = use_signal(|| 0i64);

    // --- action form inputs ---
    let mut sale_amount_input = use_signal(String::new);
    let mut mint_amount_input = use_signal(String::new);
    let mut redeem_amount_input = use_signal(String::new);

    // --- action log ---
    let mut log = use_signal(Vec::<SimLogRow>::new);

    let floor_price = compute_floor(treasury(), supply());
    let floor_display = format_floor_display(floor_price);

    // Re-render the Chart.js instance whenever the log changes. The
    // series are reconstructed from the log in chronological order
    // (log is stored newest-first, so we reverse it for the chart).
    //
    // The render helper retries silently until Chart.js finishes
    // loading from the CDN, and also no-ops if the canvas isn't
    // currently in the DOM (dialog closed). That keeps this effect
    // idempotent and cheap.
    #[cfg(not(feature = "server"))]
    use_effect(move || {
        if !open {
            floor_chart_destroy(CHART_CANVAS_ID);
            return;
        }

        use wasm_bindgen::JsValue;

        let rows = log();
        let labels = js_sys::Array::new();
        let treasury_series = js_sys::Array::new();
        let supply_series = js_sys::Array::new();
        let floor_series = js_sys::Array::new();

        // Seed the chart with a "t=0" point so the reset state isn't
        // a blank canvas. This makes the slope of the first action
        // visible.
        labels.push(&JsValue::from_str("0"));
        treasury_series.push(&JsValue::from_f64(0.0));
        supply_series.push(&JsValue::from_f64(0.0));
        floor_series.push(&JsValue::from_f64(0.0));

        // `log` is newest-first; walk it in reverse so the chart
        // x-axis reads left-to-right chronologically.
        for (idx, row) in rows.iter().rev().enumerate() {
            labels.push(&JsValue::from_str(&(idx + 1).to_string()));
            treasury_series.push(&JsValue::from_f64(row.treasury_after as f64));
            supply_series.push(&JsValue::from_f64(row.supply_after as f64));
            floor_series.push(&JsValue::from_f64(row.floor_after));
        }

        floor_chart_render(
            CHART_CANVAS_ID,
            labels.into(),
            treasury_series.into(),
            supply_series.into(),
            floor_series.into(),
        );
    });

    let reset = move |_| {
        treasury.set(0);
        supply.set(0);
        sale_amount_input.set(String::new());
        mint_amount_input.set(String::new());
        redeem_amount_input.set(String::new());
        log.set(Vec::new());
    };

    let record_sale = move |_| {
        let Ok(amount) = sale_amount_input().trim().parse::<i64>() else {
            return;
        };
        if amount <= 0 {
            return;
        }
        let rate = (reserve_rate_pct() as f64 / 100.0).clamp(0.0, 1.0);
        let contribution = ((amount as f64) * rate).round() as i64;

        let new_treasury = treasury() + contribution;
        treasury.set(new_treasury);

        let floor_after = compute_floor(new_treasury, supply());
        let mut rows = log();
        rows.insert(
            0,
            SimLogRow {
                kind: SimLogKind::Sale,
                amount,
                contribution,
                treasury_after: new_treasury,
                supply_after: supply(),
                floor_after,
            },
        );
        log.set(rows);
        sale_amount_input.set(String::new());
    };

    let mint_reward = move |_| {
        let Ok(amount) = mint_amount_input().trim().parse::<i64>() else {
            return;
        };
        if amount <= 0 {
            return;
        }
        let new_supply = supply() + amount;
        supply.set(new_supply);

        let floor_after = compute_floor(treasury(), new_supply);
        let mut rows = log();
        rows.insert(
            0,
            SimLogRow {
                kind: SimLogKind::RewardMint,
                amount,
                contribution: 0,
                treasury_after: treasury(),
                supply_after: new_supply,
                floor_after,
            },
        );
        log.set(rows);
        mint_amount_input.set(String::new());
    };

    let redeem = move |_| {
        let Ok(amount) = redeem_amount_input().trim().parse::<i64>() else {
            return;
        };
        if amount <= 0 || amount > supply() {
            return;
        }
        let current_floor = compute_floor(treasury(), supply());
        // The redeem payout is exactly `amount * floor`. Done as an
        // integer proportional reduction so the math stays identical
        // to the smart contract (T - (amount/supply)*T, rounded).
        let payout = ((amount as f64) * current_floor).round() as i64;
        let payout = payout.min(treasury());

        let new_treasury = treasury() - payout;
        let new_supply = supply() - amount;
        treasury.set(new_treasury);
        supply.set(new_supply);

        let floor_after = compute_floor(new_treasury, new_supply);
        let mut rows = log();
        rows.insert(
            0,
            SimLogRow {
                kind: SimLogKind::Redeem,
                amount,
                contribution: payout,
                treasury_after: new_treasury,
                supply_after: new_supply,
                floor_after,
            },
        );
        log.set(rows);
        redeem_amount_input.set(String::new());
    };

    rsx! {
        DialogRoot {
            open: open,
            on_open_change: move |v: bool| {
                if !v {
                    on_close.call(());
                }
            },
            DialogContent {
                // Chart.js runtime from CDN + our thin facade. Both
                // tags are idempotent on re-render — the browser
                // deduplicates by `src`.
                document::Script { src: "https://cdn.jsdelivr.net/npm/chart.js" }
                document::Script { src: asset!("/assets/floor_chart.js") }

                DialogTitle { {t.simulator_title} }
                DialogDescription { {t.simulator_subtitle} }

                // Current state panel — what the contract would report
                // right now for the tuned inputs.
                div { class: "mt-2 grid gap-3 sm:grid-cols-3",
                    StatCard {
                        color: StatColor::Emerald,
                        label: t.treasury_onchain_balance.to_string(),
                        value: format_number(treasury()),
                    }
                    StatCard {
                        color: StatColor::Gray,
                        label: t.treasury_onchain_circulating.to_string(),
                        value: format_number(supply()),
                    }
                    StatCard {
                        color: StatColor::Indigo,
                        label: t.treasury_onchain_floor.to_string(),
                        value: floor_display,
                    }
                }

                // Configuration
                div { class: "mt-5 rounded-2xl border border-border bg-panel-muted p-4",
                    p { class: "mb-3 text-sm font-semibold text-foreground",
                        {t.simulator_config_title}
                    }
                    FormField {
                        label: t.simulator_reserve_rate,
                        r#type: "number",
                        value: "{reserve_rate_pct()}",
                        oninput: move |e: FormEvent| {
                            if let Ok(v) = e.value().trim().parse::<i64>() {
                                reserve_rate_pct.set(v.clamp(0, 100));
                            }
                        },
                        min: "0",
                        max: "100",
                        suffix: "%",
                    }
                }

                // Actions
                div { class: "mt-5 grid gap-4 md:grid-cols-3",
                    ActionPanel {
                        title: t.simulator_action_sale_title.to_string(),
                        hint: t.simulator_action_sale_hint.to_string(),
                        value: sale_amount_input(),
                        oninput: move |e: FormEvent| sale_amount_input.set(e.value()),
                        on_submit: record_sale,
                        button_label: t.simulator_apply_sale.to_string(),
                    }
                    ActionPanel {
                        title: t.simulator_action_mint_title.to_string(),
                        hint: t.simulator_action_mint_hint.to_string(),
                        value: mint_amount_input(),
                        oninput: move |e: FormEvent| mint_amount_input.set(e.value()),
                        on_submit: mint_reward,
                        button_label: t.simulator_apply_mint.to_string(),
                    }
                    ActionPanel {
                        title: t.simulator_action_redeem_title.to_string(),
                        hint: t.simulator_action_redeem_hint.to_string(),
                        value: redeem_amount_input(),
                        oninput: move |e: FormEvent| redeem_amount_input.set(e.value()),
                        on_submit: redeem,
                        button_label: t.simulator_apply_redeem.to_string(),
                    }
                }

                // Chart
                div { class: "mt-6",
                    p { class: "mb-2 text-sm font-semibold text-foreground",
                        {t.simulator_chart_title}
                    }
                    div { class: "rounded-xl border border-border bg-panel-muted p-3",
                        div { class: "relative h-64 w-full",
                            canvas { id: "{CHART_CANVAS_ID}" }
                        }
                    }
                }

                // Log
                div { class: "mt-6",
                    p { class: "mb-2 text-sm font-semibold text-foreground",
                        {t.simulator_log_title}
                    }
                    if log().is_empty() {
                        p { class: "rounded-xl border border-dashed border-border bg-panel-muted px-4 py-6 text-center text-sm text-foreground-muted",
                            {t.simulator_log_empty}
                        }
                    } else {
                        div { class: "max-h-60 overflow-y-auto rounded-xl border border-border",
                            table { class: "w-full text-left text-xs",
                                thead { class: "sticky top-0 bg-panel-muted text-foreground-muted",
                                    tr {
                                        th { class: "px-3 py-2", {t.simulator_log_col_action} }
                                        th { class: "px-3 py-2 text-right", {t.simulator_log_col_amount} }
                                        th { class: "px-3 py-2 text-right", {t.simulator_log_col_delta} }
                                        th { class: "px-3 py-2 text-right", {t.simulator_log_col_treasury} }
                                        th { class: "px-3 py-2 text-right", {t.simulator_log_col_supply} }
                                        th { class: "px-3 py-2 text-right", {t.simulator_log_col_floor} }
                                    }
                                }
                                tbody {
                                    for (idx, row) in log().iter().enumerate() {
                                        tr { key: "{idx}", class: "border-t border-border",
                                            td { class: "px-3 py-2 font-medium text-foreground",
                                                {sim_log_kind_label(&row.kind, &t)}
                                            }
                                            td { class: "px-3 py-2 text-right font-mono", "{format_number(row.amount)}" }
                                            td { class: "px-3 py-2 text-right font-mono",
                                                if row.contribution == 0 {
                                                    "—"
                                                } else {
                                                    "{format_number(row.contribution)}"
                                                }
                                            }
                                            td { class: "px-3 py-2 text-right font-mono", "{format_number(row.treasury_after)}" }
                                            td { class: "px-3 py-2 text-right font-mono", "{format_number(row.supply_after)}" }
                                            td { class: "px-3 py-2 text-right font-mono",
                                                "{format_floor_display(row.floor_after)}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                DialogActions {
                    Btn {
                        variant: BtnVariant::Secondary,
                        onclick: reset,
                        {t.simulator_reset}
                    }
                    Btn {
                        variant: BtnVariant::Primary,
                        onclick: move |_| on_close.call(()),
                        {t.close}
                    }
                }
            }
        }
    }
}

#[component]
fn ActionPanel(
    title: String,
    hint: String,
    value: String,
    oninput: EventHandler<FormEvent>,
    on_submit: EventHandler<MouseEvent>,
    button_label: String,
) -> Element {
    rsx! {
        div { class: "rounded-2xl border border-border bg-panel p-4",
            p { class: "text-sm font-semibold text-foreground", {title} }
            p { class: "mt-1 text-xs text-foreground-muted", {hint} }
            div { class: "mt-3",
                input {
                    r#type: "number",
                    value: value,
                    oninput: move |e| oninput.call(e),
                    min: "0",
                    class: "w-full rounded-lg border border-border bg-panel-muted px-3 py-2 text-sm text-foreground focus:border-brand focus:outline-none",
                }
            }
            div { class: "mt-3 flex justify-end",
                Btn {
                    variant: BtnVariant::Primary,
                    onclick: move |e| on_submit.call(e),
                    {button_label}
                }
            }
        }
    }
}

// ----------------------------------------------------------------------
// Simulator state types + math
// ----------------------------------------------------------------------

#[derive(Clone, PartialEq)]
enum SimLogKind {
    Sale,
    RewardMint,
    Redeem,
}

#[derive(Clone, PartialEq)]
struct SimLogRow {
    kind: SimLogKind,
    amount: i64,
    contribution: i64,
    treasury_after: i64,
    supply_after: i64,
    floor_after: f64,
}

fn compute_floor(treasury: i64, supply: i64) -> f64 {
    if supply <= 0 {
        return 0.0;
    }
    treasury as f64 / supply as f64
}

fn format_floor_display(value: f64) -> String {
    if value == 0.0 {
        "0".to_string()
    } else if value >= 100.0 {
        format!("{:.0}", value)
    } else if value >= 1.0 {
        format!("{:.2}", value)
    } else {
        format!("{:.4}", value)
    }
}

fn sim_log_kind_label(kind: &SimLogKind, t: &ProjectsTranslate) -> &'static str {
    match kind {
        SimLogKind::Sale => t.simulator_log_kind_sale,
        SimLogKind::RewardMint => t.simulator_log_kind_mint,
        SimLogKind::Redeem => t.simulator_log_kind_redeem,
    }
}
