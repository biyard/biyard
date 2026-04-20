use dioxus::prelude::*;

use crate::common::ProjectPartition;

use super::points_tab::PointsTab;

/// `/projects/:project_id/points` — point transaction history.
#[component]
pub fn ProjectPoints(project_id: ReadSignal<ProjectPartition>) -> Element {
    rsx! {
        PointsTab { project_id: project_id }
    }
}
