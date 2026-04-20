use crate::common::ui::format_number;

use super::types::MonthRow;

pub(super) const KRW_PER_USDT: f64 = 1500.0;

pub(super) fn build_rows(
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

pub(super) fn format_floor_display(value: f64) -> String {
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

pub(super) fn format_with_commas(value: i64) -> String {
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

pub(super) fn parse_commas(s: &str) -> f64 {
    s.trim().replace(',', "").parse::<f64>().unwrap_or(0.0)
}

pub(super) fn reformat_commas(input: &str) -> String {
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

pub(super) fn format_compact(value: f64) -> String {
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
