use serde::Serialize;

#[derive(Clone, PartialEq)]
pub(super) struct MonthRow {
    pub(super) month: i64,
    pub(super) treasury: f64,
    pub(super) supply: f64,
    pub(super) floor: f64,
}

#[derive(Serialize)]
pub(super) struct ChartPayload {
    pub(super) labels: Vec<i64>,
    pub(super) treasury: Vec<f64>,
    pub(super) supply: Vec<f64>,
    pub(super) floor: Vec<f64>,
    pub(super) t: ChartLabels,
}

#[derive(Serialize, Clone, Copy)]
pub(super) struct ChartLabels {
    pub(super) treasury: &'static str,
    pub(super) supply: &'static str,
    pub(super) floor: &'static str,
    pub(super) x: &'static str,
    pub(super) y_left: &'static str,
    pub(super) y_right: &'static str,
    pub(super) month_suffix: &'static str,
}
