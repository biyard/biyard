use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::common::ui::*;
use crate::features::enterprises::EnterpriseTranslate;

/// Empty-state shown to accounts whose `enterprise_id` is `None`. This
/// happens after an admin removes them from an enterprise. The user can
/// still log in but cannot access enterprise-scoped features until they
/// receive and accept a new invitation.
#[component]
pub fn NoEnterprisePage() -> Element {
    let t: EnterpriseTranslate = use_translate();

    rsx! {
        div { class: "flex min-h-[60vh] items-center justify-center px-6",
            div { class: "max-w-lg text-center",
                EmptyState {
                    icon: rsx! { IconBuilding { class: "h-12 w-12" } },
                    title: t.no_enterprise_title.to_string(),
                    description: t.no_enterprise_desc.to_string(),
                }
            }
        }
    }
}
