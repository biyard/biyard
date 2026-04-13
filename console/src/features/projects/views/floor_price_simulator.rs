use dioxus::prelude::*;
use dioxus_translate::use_translate;
use serde::Serialize;

use crate::common::components::dialog::{
    DialogActions, DialogContent, DialogDescription, DialogRoot, DialogTitle,
};
use crate::common::ui::*;
use crate::features::projects::i18n::ProjectsTranslate;

#[cfg(not(feature = "server"))]
use wasm_bindgen::prelude::*;

#[cfg(not(feature = "server"))]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = eval)]
    fn js_eval(script: &str) -> wasm_bindgen::JsValue;
}

#[cfg(not(feature = "server"))]
fn get_simulator_ns() -> Option<js_sys::Object> {
    let window = web_sys::window()?;
    let biyard = js_sys::Reflect::get(&window, &"biyard".into())
        .ok()
        .filter(|v| !v.is_undefined() && !v.is_null())?;
    js_sys::Reflect::get(&biyard, &"simulator".into())
        .ok()
        .filter(|v| !v.is_undefined() && !v.is_null())
        .map(|v| v.unchecked_into())
}

#[cfg(not(feature = "server"))]
fn render_chart(canvas_id: &str, payload_json: &str) {
    use wasm_bindgen::JsCast;
    let Some(simulator) = get_simulator_ns() else {
        return;
    };
    let Ok(func) = js_sys::Reflect::get(&simulator, &"render_chart".into()) else {
        return;
    };
    let Ok(func) = func.dyn_into::<js_sys::Function>() else {
        return;
    };
    let _ = func.call2(&simulator, &canvas_id.into(), &payload_json.into());
}

#[cfg(not(feature = "server"))]
fn set_on_treasury_drag(cb: &Closure<dyn FnMut(i32, f64)>) {
    use wasm_bindgen::JsCast;
    let Some(simulator) = get_simulator_ns() else {
        return;
    };
    let Ok(func) = js_sys::Reflect::get(&simulator, &"set_on_treasury_drag".into()) else {
        return;
    };
    let Ok(func) = func.dyn_into::<js_sys::Function>() else {
        return;
    };
    let _ = func.call1(&simulator, cb.as_ref().unchecked_ref());
}

#[cfg(not(feature = "server"))]
fn destroy_chart(canvas_id: &str) {
    use wasm_bindgen::JsCast;
    let Some(simulator) = get_simulator_ns() else {
        return;
    };
    let Ok(func) = js_sys::Reflect::get(&simulator, &"destroy_chart".into()) else {
        return;
    };
    let Ok(func) = func.dyn_into::<js_sys::Function>() else {
        return;
    };
    let _ = func.call1(&simulator, &canvas_id.into());
}

const CANVAS_ID: &str = "floor-price-simulator-chart";

/// Hardcoded KRW→USDT rate used for converting sales/treasury amounts
/// throughout the simulator. Not user-configurable by design (the goal
/// is a stable reference point for the what-if tool).
const KRW_PER_USDT: f64 = 1500.0;

/// What-if floor price simulator.
///
/// Models a brand whose token has:
/// - a fixed monthly emission (taken from the project setting),
/// - a treasury that starts at some seed value and grows by
///   `monthly_sales × reserve_rate` each month,
/// - monthly sales that grow (or shrink) by a configurable rate,
/// - monthly emission that grows (or shrinks) by a configurable rate.
///
/// The chart and table answer the question: "if my brand brings in this
/// much sales every month, where does the floor price land over the next
/// N months?"
#[component]
pub fn FloorPriceSimulatorDialog(
    open: ReadSignal<bool>,
    on_close: EventHandler<()>,
    initial_reserve_rate: f64,
    monthly_token_supply: i64,
) -> Element {
    let t: ProjectsTranslate = use_translate();

    let mut reserve_rate_pct = use_signal(|| (initial_reserve_rate * 100.0).round() as i64);
    let mut initial_treasury_input = use_signal(|| String::from("0"));
    let mut monthly_sales_input = use_signal(|| format_with_commas(10_000_000));
    let mut sales_growth_pct = use_signal(|| 10i64);
    let mut supply_decrease_pct = use_signal(|| 5i64);
    let mut horizon_months = use_signal(|| 12i64);
    // Per-month treasury overrides from chart drag or table edit.
    // Key = month number, value = new treasury USDT.
    // Only the edited month is replaced; other months keep base values.
    // Cleared when any Input changes.
    let mut treasury_overrides = use_signal(|| std::collections::HashMap::<i64, f64>::new());

    let rows_memo = use_memo(move || {
        let initial_treasury: f64 = parse_commas(&initial_treasury_input()).max(0.0);
        let monthly_sales: f64 = parse_commas(&monthly_sales_input()).max(0.0);
        let rate = (reserve_rate_pct() as f64 / 100.0).clamp(0.0, 1.0);
        let sales_growth = sales_growth_pct() as f64 / 100.0;
        let supply_growth = -(supply_decrease_pct() as f64) / 100.0;
        let months = horizon_months().clamp(1, 120);
        build_rows(
            months,
            initial_treasury,
            monthly_sales,
            rate,
            sales_growth,
            monthly_token_supply as f64,
            supply_growth,
        )
    });

    // Clear per-month overrides when base inputs change.
    use_effect(move || {
        let _ = rows_memo();
        treasury_overrides.set(Default::default());
    });

    // Apply per-month overrides on top of the base rows.
    // Only the specific edited months are replaced; others stay as-is.
    let final_rows_memo = use_memo(move || {
        let base = rows_memo();
        let overrides = treasury_overrides();
        if overrides.is_empty() {
            return base;
        }
        base.into_iter()
            .map(|mut r| {
                if let Some(&new_treasury) = overrides.get(&r.month) {
                    r.treasury = new_treasury;
                    r.floor = if r.supply > 0.0 {
                        r.treasury / r.supply
                    } else {
                        0.0
                    };
                }
                r
            })
            .collect()
    });

    let labels_en = ChartLabels {
        treasury: t.simulator_chart_treasury,
        supply: t.simulator_chart_supply,
        floor: t.simulator_chart_floor,
        x: t.simulator_chart_x,
        y_left: t.simulator_chart_y_left,
        y_right: t.simulator_chart_y_right,
        month_suffix: t.simulator_chart_month_suffix,
    };

    let payload_memo = use_memo(move || {
        let rows = final_rows_memo();
        let payload = ChartPayload {
            labels: rows.iter().map(|r| r.month).collect(),
            treasury: rows.iter().map(|r| r.treasury.round()).collect(),
            supply: rows.iter().map(|r| r.supply.round()).collect(),
            floor: rows.iter().map(|r| r.floor).collect(),
            t: labels_en,
        };
        serde_json::to_string(&payload).unwrap_or_default()
    });

    // Register JS→Rust drag callback so chart drags update the override map.
    use_effect(move || {
        #[cfg(not(feature = "server"))]
        {
            use wasm_bindgen::prelude::*;
            let cb = Closure::<dyn FnMut(i32, f64)>::new(move |month: i32, value: f64| {
                treasury_overrides.write().insert(month as i64, value);
            });
            set_on_treasury_drag(&cb);
            cb.forget();
        }
        #[cfg(feature = "server")]
        {
            let _ = treasury_overrides;
        }
    });

    use_effect(move || {
        let json = payload_memo();
        let is_open = open();
        #[cfg(not(feature = "server"))]
        {
            if is_open {
                render_chart(CANVAS_ID, &json);
            } else {
                destroy_chart(CANVAS_ID);
            }
        }
        #[cfg(feature = "server")]
        {
            let _ = json;
            let _ = is_open;
        }
    });

    let rows = final_rows_memo();
    let last_row = rows.last().cloned();
    let final_floor_display = last_row
        .as_ref()
        .map(|r| format!("{} USDT", format_floor_display(r.floor)))
        .unwrap_or_else(|| "0 USDT".to_string());
    let final_treasury_display = last_row
        .as_ref()
        .map(|r| format!("{} USDT", format_compact(r.treasury)))
        .unwrap_or_else(|| "0 USDT".to_string());
    let final_supply_display = last_row
        .as_ref()
        .map(|r| format!("{} tokens", format_compact(r.supply)))
        .unwrap_or_else(|| "0 tokens".to_string());

    let reset = move |_| {
        reserve_rate_pct.set((initial_reserve_rate * 100.0).round() as i64);
        initial_treasury_input.set(String::from("0"));
        monthly_sales_input.set(format_with_commas(10_000_000));
        sales_growth_pct.set(10);
        supply_decrease_pct.set(5);
        horizon_months.set(12);
    };

    rsx! {
        document::Script { src: "https://cdn.jsdelivr.net/npm/chart.js@4.4.1/dist/chart.umd.min.js" }
        document::Script { src: asset!("/assets/floor_price_chart.js") }

        DialogRoot {
            open: open(),
            on_open_change: move |v: bool| {
                if !v {
                    on_close.call(());
                }
            },
            DialogContent {
                class: "w-full max-w-5xl max-h-[90vh] overflow-y-auto rounded-[28px] border border-border bg-panel p-6 shadow-[0_24px_60px_rgba(2,6,23,0.26)]",
                DialogTitle { {t.simulator_title} }
                DialogDescription { {t.simulator_subtitle} }

                div { class: "mt-2 grid gap-3 sm:grid-cols-3",
                    StatCard {
                        color: StatColor::Emerald,
                        label: t.simulator_final_treasury_label.to_string(),
                        value: final_treasury_display,
                    }
                    StatCard {
                        color: StatColor::Gray,
                        label: t.simulator_final_supply_label.to_string(),
                        value: final_supply_display,
                    }
                    StatCard {
                        color: StatColor::Indigo,
                        label: t.simulator_final_floor_label.to_string(),
                        value: final_floor_display,
                    }
                }

                div { class: "mt-4 rounded-2xl border border-border bg-panel-muted p-3",
                    p { class: "mb-2 text-sm font-semibold text-foreground",
                        {t.simulator_config_title}
                    }
                    div { class: "grid gap-3 sm:grid-cols-2 lg:grid-cols-3",
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
                        FormField {
                            label: t.simulator_initial_treasury,
                            value: initial_treasury_input(),
                            oninput: move |e: FormEvent| {
                                initial_treasury_input.set(reformat_commas(&e.value()));
                            },
                            suffix: "USDT",
                        }
                        FormField {
                            label: t.simulator_monthly_sales,
                            value: monthly_sales_input(),
                            oninput: move |e: FormEvent| {
                                monthly_sales_input.set(reformat_commas(&e.value()));
                            },
                            suffix: "₩",
                        }
                        FormField {
                            label: t.simulator_sales_growth,
                            r#type: "number",
                            value: "{sales_growth_pct()}",
                            oninput: move |e: FormEvent| {
                                if let Ok(v) = e.value().trim().parse::<i64>() {
                                    sales_growth_pct.set(v.clamp(-100, 100));
                                }
                            },
                            min: "-100",
                            max: "100",
                            suffix: "%",
                        }
                        FormField {
                            label: t.simulator_supply_decrease_rate,
                            r#type: "number",
                            value: "{supply_decrease_pct()}",
                            oninput: move |e: FormEvent| {
                                if let Ok(v) = e.value().trim().parse::<i64>() {
                                    supply_decrease_pct.set(v.clamp(0, 100));
                                }
                            },
                            min: "0",
                            max: "100",
                            suffix: "%",
                        }
                        FormField {
                            label: t.simulator_horizon,
                            r#type: "number",
                            value: "{horizon_months()}",
                            oninput: move |e: FormEvent| {
                                if let Ok(v) = e.value().trim().parse::<i64>() {
                                    horizon_months.set(v.clamp(1, 120));
                                }
                            },
                            min: "1",
                            max: "120",
                        }
                    }
                    p { class: "mt-3 text-xs text-foreground-muted",
                        {t.simulator_monthly_supply_hint}
                        " "
                        span { class: "font-mono text-foreground",
                            "{format_number(monthly_token_supply)}"
                        }
                    }
                }

                div { class: "mt-4 rounded-2xl border border-border bg-panel p-3",
                    p { class: "mb-2 text-sm font-semibold text-foreground",
                        {t.simulator_chart_title}
                    }
                    div { class: "relative h-56 w-full",
                        canvas { id: CANVAS_ID }
                    }
                }

                div { class: "mt-4",
                    p { class: "mb-2 text-sm font-semibold text-foreground",
                        {t.simulator_table_title}
                    }
                    div { class: "max-h-40 overflow-y-auto rounded-xl border border-border",
                        table { class: "w-full text-left text-xs",
                            thead { class: "sticky top-0 bg-panel-muted text-foreground-muted",
                                tr {
                                    th { class: "px-3 py-2", {t.simulator_col_month} }
                                    th { class: "px-3 py-2 text-right", {t.simulator_col_treasury} }
                                    th { class: "px-3 py-2 text-right", {t.simulator_col_supply} }
                                    th { class: "px-3 py-2 text-right", {t.simulator_col_floor} }
                                }
                            }
                            tbody {
                                for row in rows.iter() {
                                    {
                                        let row_month = row.month;
                                        let treasury_str = format_with_commas(row.treasury.round() as i64);
                                        rsx! {
                                            tr {
                                                key: "{row_month}",
                                                class: "border-t border-border",
                                                td { class: "px-3 py-2 font-medium text-foreground",
                                                    "{row_month}"
                                                }
                                                td { class: "px-3 py-2 text-right font-mono",
                                                    input {
                                                        r#type: "text",
                                                        value: "{treasury_str}",
                                                        class: "w-full bg-transparent text-right font-mono text-foreground focus:outline-none focus:ring-1 focus:ring-brand rounded px-1",
                                                        onchange: move |e: FormEvent| {
                                                            let parsed = parse_commas(&e.value());
                                                            treasury_overrides.write().insert(row_month, parsed);
                                                        },
                                                    }
                                                }
                                                td { class: "px-3 py-2 text-right font-mono",
                                                    "{format_compact(row.supply)}"
                                                }
                                                td { class: "px-3 py-2 text-right font-mono",
                                                    "{format_floor_display(row.floor)}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                DialogActions {
                    Btn { variant: BtnVariant::Secondary, onclick: reset, {t.simulator_reset} }
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

#[derive(Clone, PartialEq)]
struct MonthRow {
    month: i64,
    treasury: f64,
    supply: f64,
    floor: f64,
}

#[derive(Serialize)]
struct ChartPayload {
    labels: Vec<i64>,
    treasury: Vec<f64>,
    supply: Vec<f64>,
    floor: Vec<f64>,
    t: ChartLabels,
}

#[derive(Serialize, Clone, Copy)]
struct ChartLabels {
    treasury: &'static str,
    supply: &'static str,
    floor: &'static str,
    x: &'static str,
    y_left: &'static str,
    y_right: &'static str,
    month_suffix: &'static str,
}

fn build_rows(
    months: i64,
    initial_treasury_usdt: f64,
    monthly_sales_krw: f64,
    rate: f64,
    sales_growth: f64,
    monthly_supply: f64,
    supply_growth: f64,
) -> Vec<MonthRow> {
    let mut rows = Vec::with_capacity(months as usize);
    // Treasury is in USDT end-to-end (initial value, accumulation,
    // chart, table, Floor Price). Only sales come in as KRW from the
    // UI and are converted to USDT once when added to the treasury.
    let mut treasury = initial_treasury_usdt;
    let mut supply = 0.0_f64;
    let mut sales_m = monthly_sales_krw;
    let mut supply_m = monthly_supply;

    for m in 1..=months {
        treasury += (sales_m * rate) / KRW_PER_USDT;
        supply += supply_m;

        let floor = if supply > 0.0 { treasury / supply } else { 0.0 };
        rows.push(MonthRow {
            month: m,
            treasury,
            supply,
            floor,
        });

        sales_m *= 1.0 + sales_growth;
        supply_m *= 1.0 + supply_growth;
        if sales_m < 0.0 {
            sales_m = 0.0;
        }
        if supply_m < 0.0 {
            supply_m = 0.0;
        }
    }
    rows
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

fn format_with_commas(value: i64) -> String {
    let s = value.abs().to_string();
    let mut out = String::new();
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    let mut result: String = out.chars().rev().collect();
    if value < 0 {
        result.insert(0, '-');
    }
    result
}

fn parse_commas(s: &str) -> f64 {
    s.trim().replace(',', "").parse::<f64>().unwrap_or(0.0)
}

fn reformat_commas(input: &str) -> String {
    let digits: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        return String::new();
    }
    let trimmed = digits.trim_start_matches('0');
    let normalized = if trimmed.is_empty() { "0" } else { trimmed };
    match normalized.parse::<i64>() {
        Ok(n) => format_with_commas(n),
        Err(_) => normalized.to_string(),
    }
}

fn format_compact(value: f64) -> String {
    let abs = value.abs();
    let (scaled, suffix) = if abs >= 1e12 {
        (value / 1e12, "T")
    } else if abs >= 1e9 {
        (value / 1e9, "B")
    } else if abs >= 1e6 {
        (value / 1e6, "M")
    } else if abs >= 1e3 {
        (value / 1e3, "K")
    } else {
        return format_number(value.round() as i64);
    };
    if scaled.abs() >= 100.0 {
        format!("{:.0}{suffix}", scaled)
    } else if scaled.abs() >= 10.0 {
        format!("{:.1}{suffix}", scaled)
    } else {
        format!("{:.2}{suffix}", scaled)
    }
}
