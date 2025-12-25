use chrono::Utc;

pub fn get_now() -> i64 {
    // use UTC time
    Utc::now().timestamp_millis()
}

pub fn timestamp_to_yyyy_mm() -> String {
    let now = Utc::now();
    now.format("%Y-%m").to_string()
}
