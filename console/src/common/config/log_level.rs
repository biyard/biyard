use std::str::FromStr;

use dioxus::logger::tracing::Level;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Default for LogLevel {
    fn default() -> Self {
        let default_level = option_env!("RUST_LOG").unwrap_or("info");
        default_level.parse().unwrap_or(LogLevel::Info)
    }
}

impl FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let l = match s.to_lowercase().as_str() {
            "trace" => LogLevel::Trace,
            "debug" => LogLevel::Debug,
            "info" => LogLevel::Info,
            "warn" | "warning" => LogLevel::Warn,
            "error" => LogLevel::Error,
            _ => LogLevel::Info,
        };

        Ok(l)
    }
}

impl From<LogLevel> for Level {
    fn from(val: LogLevel) -> Self {
        match val {
            LogLevel::Trace => Level::TRACE,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Info => Level::INFO,
            LogLevel::Warn => Level::WARN,
            LogLevel::Error => Level::ERROR,
        }
    }
}
