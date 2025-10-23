use crate::config;

#[derive(Clone)]
pub struct AppState {}

impl AppState {
    pub fn new(_conf: &config::Config) -> Self {
        AppState {}
    }
}

unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}
