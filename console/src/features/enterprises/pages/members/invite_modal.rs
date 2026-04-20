use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::common::OrganizationRole;
use crate::common::components::dialog::*;
use crate::common::ui::*;
use crate::features::enterprises::EnterpriseTranslate;

#[component]
pub fn InviteModal(open: bool, on_close: EventHandler, on_success: EventHandler) -> Element {
    let t: EnterpriseTranslate = use_translate();

    let mut email = use_signal(String::new);
    let mut role = use_signal(|| OrganizationRole::Viewer);
    let mut creating = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut generated_token = use_signal(|| None::<String>);
    let mut copied = use_signal(|| false);

    let invite_url = generated_token().map(|token| {
        #[cfg(not(feature = "server"))]
        {
            let origin = web_sys::window()
                .and_then(|w| w.location().origin().ok())
                .unwrap_or_default();
            format!("{origin}/invite/{token}")
        }
        #[cfg(feature = "server")]
        {
            format!("/invite/{token}")
        }
    });

    // Reset internal state whenever the modal transitions from closed to
    // open so re-opening after a successful invite always shows the fresh
    // form instead of the previous link screen. The `open` prop is not
    // reactive on its own, so we mirror it into a signal and drive the
    // reset from a `use_effect` that reads that signal.
    let mut open_signal = use_signal(|| open);
    if *open_signal.peek() != open {
        open_signal.set(open);
    }
    use_effect(move || {
        if open_signal() {
            email.set(String::new());
            role.set(OrganizationRole::Viewer);
            error.set(None);
            generated_token.set(None);
            copied.set(false);
        }
    });

    rsx! {
        DialogRoot {
            open: open,
            on_open_change: move |v: bool| {
                if !v {
                    email.set(String::new());
                    role.set(OrganizationRole::Viewer);
                    error.set(None);
                    generated_token.set(None);
                    copied.set(false);
                    on_close.call(());
                }
            },
            DialogContent {
                DialogTitle { {t.invite_modal_title} }
                DialogDescription { {t.invite_modal_description} }

                if let Some(msg) = error() {
                    AlertMessage { variant: AlertVariant::Error, "{msg}" }
                }

                if let Some(url) = invite_url {
                    div { class: "mt-4 space-y-3",
                        FormLabel { {t.invite_link_label} }
                        div { class: "flex gap-2",
                            input {
                                class: "flex-1 rounded-2xl border border-border bg-panel px-4 py-3 text-sm font-medium text-foreground",
                                value: "{url}",
                                readonly: true,
                            }
                            Btn {
                                variant: BtnVariant::Secondary,
                                onclick: {
                                    let _url = url.clone();
                                    move |_| {
                                        #[cfg(not(feature = "server"))]
                                        {
                                            if let Some(win) = web_sys::window() {
                                                let _ = win
                                                    .navigator()
                                                    .clipboard()
                                                    .write_text(&_url);
                                            }
                                        }
                                        copied.set(true);
                                    }
                                },
                                if copied() { {t.copied} } else { {t.copy_invite_link} }
                            }
                        }
                    }
                } else {
                    div { class: "mt-4 space-y-4",
                        FormField {
                            label: t.invited_email,
                            r#type: "email",
                            value: email(),
                            oninput: move |e: FormEvent| email.set(e.value()),
                            placeholder: t.invited_email_placeholder.to_string(),
                        }

                        div {
                            FormLabel { {t.role} }
                            // Same appearance-none + custom chevron pattern as
                            // the chain picker — avoids the browser's default
                            // arrow colliding with the label text.
                            div { class: "relative mt-2",
                                select {
                                    class: "block w-full appearance-none rounded-2xl border border-border bg-panel pl-4 pr-11 py-3 text-sm font-medium text-foreground focus:border-brand focus:outline-none focus:ring-2 focus:ring-brand",
                                    value: match role() {
                                        OrganizationRole::Admin => "admin",
                                        OrganizationRole::Viewer => "viewer",
                                        OrganizationRole::Owner => "viewer",
                                    },
                                    onchange: move |e: FormEvent| {
                                        role.set(match e.value().as_str() {
                                            "admin" => OrganizationRole::Admin,
                                            _ => OrganizationRole::Viewer,
                                        });
                                    },
                                    option { value: "viewer", {t.role_viewer} }
                                    option { value: "admin", {t.role_admin} }
                                }
                                span { class: "pointer-events-none absolute inset-y-0 right-3 flex items-center text-foreground-muted",
                                    IconChevronDown { class: "h-4 w-4" }
                                }
                            }
                        }
                    }
                }

                DialogActions {
                    if generated_token().is_some() {
                        Btn {
                            variant: BtnVariant::Primary,
                            onclick: move |_| {
                                email.set(String::new());
                                role.set(OrganizationRole::Viewer);
                                error.set(None);
                                generated_token.set(None);
                                copied.set(false);
                                on_close.call(());
                            },
                            {t.close}
                        }
                    } else {
                        Btn {
                            variant: BtnVariant::Secondary,
                            onclick: move |_| on_close.call(()),
                            {t.cancel}
                        }
                        Btn {
                            variant: BtnVariant::Primary,
                            disabled: creating(),
                            onclick: move |_| {
                                let email_val = email();
                                let role_val = role();
                                spawn(async move {
                                    creating.set(true);
                                    error.set(None);
                                    let res = crate::features::enterprises::controllers::create_invitation_handler(
                                        email_val,
                                        role_val,
                                    )
                                    .await;
                                    match res {
                                        Ok(invitation) => {
                                            generated_token.set(Some(invitation.token));
                                            on_success.call(());
                                        }
                                        Err(e) => error.set(Some(e.to_string())),
                                    }
                                    creating.set(false);
                                });
                            },
                            if creating() { {t.creating_invite} } else { {t.create_invite} }
                        }
                    }
                }
            }
        }
    }
}
