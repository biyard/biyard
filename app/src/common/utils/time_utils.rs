pub fn get_now() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

pub fn timestamp_to_yyyy_mm() -> String {
    chrono::Utc::now().format("%Y-%m").to_string()
}
