pub fn format_number(n: i64) -> String {
    let s = n.to_string();
    let bytes = s.as_bytes();
    let mut result = String::new();
    let len = bytes.len();
    for (i, &b) in bytes.iter().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push(',');
        }
        result.push(b as char);
    }
    result
}

pub fn format_timestamp(ts: i64) -> String {
    let secs = ts / 1000;
    match chrono::DateTime::from_timestamp(secs, 0) {
        Some(dt) => dt.format("%Y-%m-%d %H:%M").to_string(),
        None => ts.to_string(),
    }
}

pub fn format_floor_price(value: f64) -> String {
    format!("{value:.6}")
}

/// Shorten a UUID-like identifier for display in chips and headers.
/// Keeps the first 4 and last 4 characters joined by an ellipsis, e.g.
/// `019d6787-cf72-70d2-ad75-5be9039f2a65` → `019d…ea2e`. The full id is
/// still shown one place per page (typically inside the Settings tab),
/// so users who need it can copy it from there.
pub fn shorten_id(id: &str) -> String {
    if id.len() <= 9 {
        return id.to_string();
    }
    let head: String = id.chars().take(4).collect();
    let tail: String = id
        .chars()
        .rev()
        .take(4)
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    format!("{head}…{tail}")
}
