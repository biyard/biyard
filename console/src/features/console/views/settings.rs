use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::common::components::dialog::*;
use crate::common::ui::*;
use crate::features::accounts::context::use_app_context;
use crate::features::console::i18n::ConsoleTranslate;

/// `/account/profile` — personal profile page.
///
/// This page is intentionally **not** decorated with an enterprise
/// scope badge: it is personal-scope and belongs to the account, not
/// the current enterprise. Email is shown read-only because it is the
/// login identity; only display name is editable.
#[component]
pub fn Settings() -> Element {
    let t: ConsoleTranslate = use_translate();
    let ctx = use_app_context();
    let account_ctx = ctx.account_context;

    let nav = use_navigator();
    let mut show_delete_dialog = use_signal(|| false);
    let initial_name = account_ctx()
        .account
        .as_ref()
        .map(|a| a.name.clone())
        .unwrap_or_default();
    let mut name = use_signal(move || initial_name.clone());
    let mut saving = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut success = use_signal(|| None::<String>);

    let Some(account) = account_ctx().account else {
        return rsx! {};
    };

    let on_save = move |_| {
        let next_name = name();
        let trimmed = next_name.trim().to_string();
        if trimmed.is_empty() {
            return;
        }

        saving.set(true);
        error.set(None);
        success.set(None);

        spawn(async move {
            let res =
                crate::features::accounts::controllers::update_me_handler(Some(trimmed)).await;
            match res {
                Ok(_) => {
                    ctx.refresh();
                    success.set(Some(t.profile_saved.to_string()));
                }
                Err(e) => error.set(Some(e.to_string())),
            }
            saving.set(false);
        });
    };

    rsx! {
        div { class: "space-y-8",
            // No scope badge: profile is personal-scope, not enterprise.
            PageHeader {
                title: t.profile.to_string(),
                subtitle: t.profile_subtitle.to_string(),
            }

            if let Some(msg) = error() {
                AlertMessage { variant: AlertVariant::Error, "{msg}" }
            }
            if let Some(msg) = success() {
                AlertMessage { variant: AlertVariant::Success, "{msg}" }
            }

            SectionCard {
                SectionTitle { {t.profile} }

                div { class: "space-y-6",
                    div {
                        FormField {
                            label: t.display_name,
                            r#type: "text",
                            value: name(),
                            oninput: move |e: FormEvent| name.set(e.value()),
                        }
                        p { class: "mt-2 text-xs text-foreground-muted", {t.display_name_help} }
                    }

                    div {
                        FormField {
                            label: t.email,
                            r#type: "email",
                            value: account.email.clone(),
                            oninput: move |_: FormEvent| {},
                            disabled: true,
                        }
                        p { class: "mt-2 text-xs text-foreground-muted", {t.email_readonly_help} }
                    }

                    div { class: "grid gap-4 sm:grid-cols-2",
                        InfoBlock {
                            label: t.account_id.to_string(),
                            value: account.pk.to_string(),
                        }
                        InfoBlock {
                            label: t.created_at.to_string(),
                            value: format_timestamp(account.created_at),
                        }
                    }

                    div { class: "flex justify-end",
                        Btn {
                            variant: BtnVariant::Primary,
                            disabled: saving() || name().trim().is_empty(),
                            onclick: on_save,
                            if saving() {
                                {t.loading}
                            } else {
                                {t.save_profile}
                            }
                        }
                    }
                }
            }

            DangerCard {
                div { class: "flex flex-col gap-5 md:flex-row md:items-start md:justify-between",
                    div { class: "flex items-start gap-4",
                        div { class: "mt-1 flex h-11 w-11 items-center justify-center rounded-2xl bg-danger text-white",
                            IconAlertTriangle { class: "h-5 w-5" }
                        }
                        div {
                            p { class: "text-[11px] font-semibold uppercase tracking-[0.14em] text-danger",
                                {t.danger_zone}
                            }
                            h3 { class: "mt-2 font-display text-xl font-bold tracking-tight text-foreground",
                                {t.delete_account}
                            }
                            p { class: "mt-2 max-w-2xl text-sm leading-6 text-foreground-soft",
                                {t.delete_account_desc}
                            }
                        }
                    }
                    Btn {
                        variant: BtnVariant::Danger,
                        onclick: move |_| show_delete_dialog.set(true),
                        {t.delete_account}
                    }
                }
            }

            DialogRoot {
                open: show_delete_dialog(),
                on_open_change: move |v| show_delete_dialog.set(v),
                DialogContent {
                    DialogTitle { {t.delete_account_confirm} }
                    DialogDescription { {t.delete_account_warning} }
                    DialogActions {
                        Btn {
                            variant: BtnVariant::Secondary,
                            onclick: move |_| show_delete_dialog.set(false),
                            {t.cancel}
                        }
                        Btn {
                            variant: BtnVariant::Danger,
                            onclick: move |_| {
                                spawn(async move {
                                    let _ = crate::features::accounts::controllers::withdrawal_handler().await;
                                    nav.push(Route::SignIn {});
                                });
                            },
                            {t.confirm_delete}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn InfoBlock(label: String, value: String) -> Element {
    rsx! {
        div { class: "rounded-[24px] border border-border bg-panel-muted p-4",
            p { class: "text-[11px] font-semibold uppercase tracking-[0.14em] text-foreground-muted",
                "{label}"
            }
            p { class: "mt-2 break-all text-sm font-semibold text-foreground", "{value}" }
        }
    }
}
