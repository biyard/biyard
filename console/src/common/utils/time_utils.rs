pub fn get_now() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

pub fn timestamp_to_yyyy_mm() -> String {
    chrono::Utc::now().format("%Y-%m").to_string()
}

/// Compute the 0-based month index for `month_str` ("YYYY-MM") relative to
/// the token's emission start.
///
/// Uses `start_month` ("YYYY-MM") when available; falls back to
/// `created_at_ms` (epoch millis) otherwise.
pub fn month_index(month_str: &str, start_month: &Option<String>, created_at_ms: i64) -> u64 {
    let parts: Vec<&str> = month_str.split('-').collect();
    if parts.len() != 2 {
        return 0;
    }
    let target_year: i64 = parts[0].parse().unwrap_or(2026);
    let target_month: i64 = parts[1].parse().unwrap_or(1);
    let target = (target_year - 1970) * 12 + (target_month - 1);

    let origin = if let Some(sm) = start_month {
        let sm_parts: Vec<&str> = sm.split('-').collect();
        if sm_parts.len() == 2 {
            let y: i64 = sm_parts[0].parse().unwrap_or(2026);
            let m: i64 = sm_parts[1].parse().unwrap_or(1);
            (y - 1970) * 12 + (m - 1)
        } else {
            created_at_ms / 1000 / (30 * 24 * 3600)
        }
    } else {
        created_at_ms / 1000 / (30 * 24 * 3600)
    };

    let diff = target - origin;
    if diff < 0 { 0 } else { diff as u64 }
}
