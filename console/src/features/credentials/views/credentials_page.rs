use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::common::components::dialog::*;
use crate::common::ui::*;
use crate::features::accounts::context::use_account_context;
use crate::features::console::i18n::ConsoleTranslate;
use crate::features::credentials::CredentialStatus;
use crate::features::credentials::i18n::CredentialsTranslate;

#[component]
pub fn Credentials() -> Element {
    let t: CredentialsTranslate = use_translate();
    let console_t: ConsoleTranslate = use_translate();
    let account_ctx = use_account_context();
    let can_write = account_ctx().can_write();
    let enterprise_name = account_ctx()
        .enterprise_name()
        .unwrap_or_else(|| "Default enterprise".to_string());
    let mut show_create_dialog = use_signal(|| false);
    let mut generated_key = use_signal(|| None::<String>);
    let mut copied_key = use_signal(|| None::<String>);

    let mut credentials = use_loader(move || async move {
        crate::features::credentials::controllers::list_credentials_handler().await
    })?;

    let creds_data = credentials();
    let active_count = creds_data
        .iter()
        .filter(|credential| credential.status == CredentialStatus::Active)
        .count();
    let revoked_count = creds_data.len().saturating_sub(active_count);

    rsx! {
        div { class: "space-y-8",
            PageHeader {
                title: t.title.to_string(),
                subtitle: t.subtitle_in.replace("{enterprise}", &enterprise_name),
                scope: PageScope::Workspace,
                workspace_label: console_t.enterprise_scope_label.to_string(),
                brand_label: console_t.brand_scope_label.to_string(),
                actions: rsx! {
                    if can_write {
                        Btn {
                            variant: BtnVariant::Primary,
                            onclick: move |_| show_create_dialog.set(true),
                            class: "flex items-center",
                            IconPlus { class: "h-5 w-5" }
                            {t.create_new}
                        }
                    }
                },
            }

            div { class: "grid gap-4 sm:grid-cols-2 lg:grid-cols-3",
                StatCard {
                    color: StatColor::Blue,
                    label: t.title.to_string(),
                    value: creds_data.len().to_string(),
                }
                StatCard {
                    color: StatColor::Green,
                    label: t.active.to_string(),
                    value: active_count.to_string(),
                }
                StatCard {
                    color: StatColor::Red,
                    label: t.inactive.to_string(),
                    value: revoked_count.to_string(),
                }
            }

            if creds_data.is_empty() {
                EmptyState {
                    icon: rsx! { IconSearchOff {} },
                    title: t.no_credentials.to_string(),
                    description: t.description.to_string(),
                    actions: rsx! {
                        if can_write {
                            Btn {
                                variant: BtnVariant::Primary,
                                onclick: move |_| show_create_dialog.set(true),
                                {t.create_new}
                            }
                        }
                    },
                }
            } else {
                DataTable {
                    TableHead {
                        TableHeadCell { {t.name} }
                        TableHeadCell { {t.api_key} }
                        TableHeadCell { {t.created_at} }
                        TableHeadCell { {t.status} }
                        TableHeadCell { {t.actions} }
                    }
                    TableBody {
                        for cred in creds_data.iter() {
                            {
                                let id = cred.id.clone();
                                let name = cred.name.clone();
                                let api_key_prefix = cred.api_key_prefix.clone();
                                let status = cred.status;
                                let created_at = cred.created_at;
                                let masked_key = mask_key(&api_key_prefix);
                                let badge_color = match status {
                                    CredentialStatus::Active => BadgeColor::Green,
                                    CredentialStatus::Revoked => BadgeColor::Red,
                                };
                                let status_text = match status {
                                    CredentialStatus::Active => t.active,
                                    CredentialStatus::Revoked => t.inactive,
                                };

                                rsx! {
                                    tr { class: "transition-colors hover:bg-panel-muted",
                                        TableCell {
                                            p { class: "font-semibold text-foreground", "{name}" }
                                        }
                                        TableCell {
                                            div { class: "flex items-center gap-2",
                                                code { class: "inline-flex rounded-full border border-border bg-panel-muted px-3 py-1 text-xs font-medium text-foreground-muted",
                                                    "{masked_key}"
                                                }
                                                {
                                                    let prefix_for_copy = api_key_prefix.clone();
                                                    rsx! {
                                                        button {
                                                            class: "inline-flex h-9 w-9 items-center justify-center rounded-2xl border border-border bg-panel text-foreground-muted transition-colors hover:bg-panel-strong hover:text-foreground",
                                                            title: "{t.copy}",
                                                            "aria-label": "{t.copy}",
                                                            onclick: move |_| {
                                                                let key = prefix_for_copy.clone();
                                                                copy_to_clipboard(&key);
                                                                copied_key.set(Some(key));
                                                                #[cfg(not(feature = "server"))]
                                                                {
                                                                    let mut copied = copied_key;
                                                                    spawn(async move {
                                                                        let mut eval = document::eval(
                                                                            "await new Promise(r => setTimeout(r, 2000)); dioxus.send(true);",
                                                                        );
                                                                        let _ = eval.recv::<bool>().await;
                                                                        copied.set(None);
                                                                    });
                                                                }
                                                            },
                                                            if copied_key().as_deref() == Some(&*api_key_prefix) {
                                                                IconCheck { class: "h-4 w-4 text-success" }
                                                            } else {
                                                                IconCopy { class: "h-4 w-4" }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        TableCell { class: "text-sm text-foreground-muted",
                                            "{format_timestamp(created_at)}"
                                        }
                                        TableCell {
                                            StatusBadge { color: badge_color,
                                                "{status_text}"
                                            }
                                        }
                                        TableCell {
                                            if can_write {
                                                button {
                                                    class: "inline-flex h-10 w-10 items-center justify-center rounded-2xl border border-border bg-panel text-danger transition-colors hover:bg-danger-soft",
                                                    "aria-label": "{t.actions}",
                                                    onclick: {
                                                        let id = id.clone();
                                                        move |_| {
                                                            let id = id.clone();
                                                            spawn(async move {
                                                                let _ = crate::features::credentials::controllers::revoke_credential_handler(id).await;
                                                                credentials.restart();
                                                            });
                                                        }
                                                    },
                                                    IconTrash { class: "h-4 w-4" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if show_create_dialog() {
                CreateCredentialDialog {
                    on_close: move |_| show_create_dialog.set(false),
                    on_created: move |key: String| {
                        show_create_dialog.set(false);
                        generated_key.set(Some(key));
                        credentials.restart();
                    },
                }
            }

            if let Some(ref key) = generated_key() {
                GeneratedKeyDialog {
                    api_key: key.clone(),
                    copied_key: copied_key(),
                    on_copy: move |key: String| {
                        copy_to_clipboard(&key);
                        copied_key.set(Some(key));
                        #[cfg(not(feature = "server"))]
                        {
                            let mut copied = copied_key;
                            spawn(async move {
                                let mut eval = document::eval(
                                    "await new Promise(r => setTimeout(r, 2000)); dioxus.send(true);",
                                );
                                let _ = eval.recv::<bool>().await;
                                copied.set(None);
                            });
                        }
                    },
                    on_close: move |_| {
                        generated_key.set(None);
                    },
                }
            }
        }
    }
}

#[component]
fn CreateCredentialDialog(on_close: EventHandler, on_created: EventHandler<String>) -> Element {
    let t: CredentialsTranslate = use_translate();
    let mut name = use_signal(String::new);
    let mut loading = use_signal(|| false);

    let handle_create = move |_| {
        let name_val = name().trim().to_string();
        if name_val.is_empty() {
            return;
        }

        spawn(async move {
            loading.set(true);

            match crate::features::credentials::controllers::create_credential_handler(name_val)
                .await
            {
                Ok(response) => {
                    name.set(String::new());
                    on_created.call(response.api_key);
                }
                Err(_e) => {}
            }

            loading.set(false);
        });
    };

    rsx! {
        DialogRoot {
            open: true,
            on_open_change: move |v: bool| {
                if !v {
                    on_close.call(());
                }
            },
            DialogContent {
                DialogTitle { {t.create_new} }
                DialogDescription {
                    "Create a new credential for server-to-server calls, automation, or operator tooling."
                }
                FormField {
                    label: t.name,
                    value: name(),
                    oninput: move |e: FormEvent| name.set(e.value()),
                    placeholder: t.name_placeholder.to_string(),
                }
                DialogActions {
                    Btn {
                        variant: BtnVariant::Secondary,
                        disabled: loading(),
                        onclick: move |_| on_close.call(()),
                        {t.cancel}
                    }
                    Btn {
                        disabled: name().trim().is_empty() || loading(),
                        onclick: handle_create,
                        class: "flex items-center",
                        if loading() {
                            Spinner { class: "h-4 w-4 animate-spin" }
                            {t.loading}
                        } else {
                            {t.generate_key}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn GeneratedKeyDialog(
    api_key: String,
    copied_key: Option<String>,
    on_copy: EventHandler<String>,
    on_close: EventHandler,
) -> Element {
    let t: CredentialsTranslate = use_translate();
    let key_for_onclick = api_key.clone();

    rsx! {
        DialogRoot {
            open: true,
            on_open_change: move |v: bool| {
                if !v {
                    on_close.call(());
                }
            },
            DialogContent {
                DialogTitle { {t.key_generated} }
                DialogDescription { {t.key_generated_warning} }
                div { class: "rounded-[24px] border border-border bg-panel-muted p-4",
                    div { class: "flex items-start gap-3",
                        code { class: "flex-1 break-all text-sm font-semibold text-foreground",
                            "{api_key}"
                        }
                        button {
                            class: "inline-flex h-10 w-10 items-center justify-center rounded-2xl border border-border bg-panel text-foreground-muted transition-colors hover:bg-panel-strong hover:text-foreground",
                            "aria-label": "{t.copy}",
                            onclick: move |_| {
                                on_copy.call(key_for_onclick.clone());
                            },
                            if copied_key.as_deref() == Some(&*api_key) {
                                IconCheck { class: "h-5 w-5 text-success" }
                            } else {
                                IconCopy { class: "h-5 w-5" }
                            }
                        }
                    }
                }
                AlertMessage { variant: AlertVariant::Error,
                    {t.key_generated_warning}
                }
                DialogActions {
                    Btn {
                        onclick: move |_| on_close.call(()),
                        {t.close}
                    }
                }
            }
        }
    }
}

fn mask_key(key: &str) -> String {
    if key.len() <= 16 {
        return key.to_string();
    }
    format!("{}...{}", &key[..12], &key[key.len() - 4..])
}

fn copy_to_clipboard(_text: &str) {
    #[cfg(not(feature = "server"))]
    {
        let js = format!("navigator.clipboard.writeText('{}')", _text);
        document::eval(&js);
    }
}
