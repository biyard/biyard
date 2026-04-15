use dioxus::prelude::*;

use crate::common::ProjectPartition;

use super::settings_tab::SettingsTab;

/// `/projects/:project_id/settings` — brand settings (treasury simulator,
/// danger zone). The token immutability card lives here too because it
/// is a brand-level setting in product terms.
#[component]
pub fn ProjectSettings(project_id: ReadSignal<ProjectPartition>) -> Element {
    let project = use_loader(move || async move {
        crate::features::projects::controllers::get_project_handler(project_id()).await
    })?;
    let project_data = project();

    rsx! {
        SettingsTab {
            project_id: project_id,
            project: project_data,
        }
    }
}
