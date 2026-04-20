use dioxus::prelude::*;

use crate::Route;
use crate::common::ProjectPartition;

/// Legacy `/projects/:project_id/edit` URL.
///
/// Brand editing now lives inline in the Settings tab. Anyone who lands
/// on `/edit` (bookmarks, old links) is redirected to the new location.
#[component]
pub fn ProjectEdit(project_id: ReadSignal<ProjectPartition>) -> Element {
    let nav = use_navigator();
    use_effect(move || {
        nav.replace(Route::ProjectSettings {
            project_id: project_id(),
        });
    });
    rsx! {}
}
