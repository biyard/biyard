use dioxus::prelude::*;

use crate::common::ProjectPartition;

use super::tokens_tab::TokensTab;

/// `/projects/:project_id/token` — token overview, deploy, mint.
///
/// The URL segment is singular because a Project has 1:1 Token by
/// product rule (see CLAUDE.md Token Cardinality Rule).
#[component]
pub fn ProjectToken(project_id: ReadSignal<ProjectPartition>) -> Element {
    rsx! {
        TokensTab { project_id: project_id }
    }
}
