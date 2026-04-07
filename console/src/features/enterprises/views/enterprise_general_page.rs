use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::common::OrganizationRole;
use crate::common::ui::*;
use crate::features::accounts::context::use_app_context;
use crate::features::console::i18n::ConsoleTranslate;
use crate::features::enterprises::EnterpriseTranslate;

/// `/enterprise/settings/general` — edit the enterprise name.
///
/// Only Owners can save changes. Viewers and Admins see the form in
/// disabled state with an inline notice so they know *why*.
///
/// After a successful save, `ctx.refresh()` re-fetches account and
/// enterprise and writes them back into the shared store, so the
/// sidebar, page headers, and any other consumer of `AccountContext`
/// reflect the new name without a hard reload.
#[component]
pub fn EnterpriseGeneralPage() -> Element {
    let t: EnterpriseTranslate = use_translate();
    let console_t: ConsoleTranslate = use_translate();
    let ctx = use_app_context();
    let store = ctx.account_context;

    let current = store().current_enterprise.clone();
    let (initial_name, role) = match current.as_ref() {
        Some(ent) => (ent.enterprise.name.clone(), ent.role),
        None => (String::new(), OrganizationRole::Viewer),
    };
    let can_edit = role.allows(OrganizationRole::Owner);

    let mut name = use_signal(|| initial_name);
    let mut saving = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut success = use_signal(|| None::<String>);

    let on_save = move |_| {
        let next_name = name();
        let trimmed_name = next_name.trim().to_string();
        if trimmed_name.is_empty() {
            return;
        }

        saving.set(true);
        error.set(None);
        success.set(None);

        spawn(async move {
            let res = crate::features::enterprises::controllers::update_enterprise_handler(
                Some(trimmed_name),
            )
            .await;
            match res {
                Ok(_) => {
                    ctx.refresh();
                    success.set(Some(t.settings_saved.to_string()));
                }
                Err(e) => error.set(Some(e.to_string())),
            }
            saving.set(false);
        });
    };

    rsx! {
        div { class: "space-y-8",
            PageHeader {
                title: t.general_title.to_string(),
                subtitle: t.general_subtitle.to_string(),
                scope: PageScope::Workspace,
                workspace_label: console_t.enterprise_scope_label.to_string(),
                brand_label: console_t.brand_scope_label.to_string(),
            }

            if !can_edit {
                AlertMessage { variant: AlertVariant::Info, {t.owner_only_edit} }
            }
            if let Some(msg) = error() {
                AlertMessage { variant: AlertVariant::Error, "{msg}" }
            }
            if let Some(msg) = success() {
                AlertMessage { variant: AlertVariant::Success, "{msg}" }
            }

            SectionCard {
                div { class: "space-y-6",
                    div {
                        FormField {
                            label: t.enterprise_name,
                            r#type: "text",
                            value: name(),
                            oninput: move |e: FormEvent| name.set(e.value()),
                            disabled: !can_edit,
                        }
                        p { class: "mt-2 text-xs text-foreground-muted",
                            {t.enterprise_name_help}
                        }
                    }

                    div { class: "flex justify-end",
                        Btn {
                            variant: BtnVariant::Primary,
                            disabled: !can_edit || saving() || name().trim().is_empty(),
                            onclick: on_save,
                            if saving() { {t.saving} } else { {t.save_changes} }
                        }
                    }
                }
            }
        }
    }
}
