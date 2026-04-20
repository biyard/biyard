use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::common::ui::*;
use crate::features::accounts::context::use_account_context;
use crate::features::console::i18n::ConsoleTranslate;
use crate::features::projects::i18n::ProjectsTranslate;

use super::editor_card::{ProjectEditorCard, ProjectEditorMode};

#[component]
pub fn ProjectCreate() -> Element {
    let t: ProjectsTranslate = use_translate();
    let console_t: ConsoleTranslate = use_translate();
    let nav = use_navigator();
    let account_ctx = use_account_context();
    let enterprise_name = account_ctx()
        .enterprise_name()
        .unwrap_or_else(|| "Default enterprise".to_string());

    // Viewers cannot create brands — bounce them back to the list. The
    // redirect runs in an effect (not inline) so every hook above is
    // still registered on every render.
    let can_write = account_ctx().can_write();
    use_effect(move || {
        if !can_write {
            nav.replace(Route::Projects {});
        }
    });
    if !can_write {
        return rsx! {};
    }

    rsx! {
        div { class: "space-y-8",
            PageHeader {
                title: t.create_project.to_string(),
                subtitle: t.create_brand_subtitle_in.replace("{enterprise}", &enterprise_name),
                scope: PageScope::Workspace,
                workspace_label: t.brands_breadcrumb.to_string(),
                brand_label: console_t.brand_scope_label.to_string(),
                actions: rsx! {
                    Btn {
                        variant: BtnVariant::Secondary,
                        onclick: move |_| {
                            nav.push(Route::Projects {});
                        },
                        {t.back_to_projects}
                    }
                },
            }

            ProjectEditorCard {
                mode: ProjectEditorMode::Create,
            }
        }
    }
}
