use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64
}

pub fn timestamp_to_yyyy_mm() -> String {
    let now = chrono::Utc::now();
    now.format("%Y-%m").to_string()
}

pub fn get_year_and_month() -> i64 {
    let now = chrono::Utc::now();
    now.format("%Y%m").to_string().parse::<i64>().unwrap()
}
