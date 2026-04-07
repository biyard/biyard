use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::common::Result;
use crate::common::ui::*;
use crate::features::accounts::context::{AccountContext, use_account_context};
use crate::features::accounts::utils::password_policy_error_message;
use crate::features::enterprises::{EnterpriseTranslate, InvitationPreviewResponse};

/// Public, unauthenticated /invite/:token page. Loads the invitation
/// preview, then renders a sign-up form that calls
/// `signup_with_invite_handler`. On success the visitor lands on the
/// dashboard already signed in.
#[component]
pub fn InviteAccept(token: String) -> Element {
    let t: EnterpriseTranslate = use_translate();
    let nav = use_navigator();
    let mut account_ctx = use_account_context();

    let token_for_loader = token.clone();
    let preview = use_loader(move || {
        let token = token_for_loader.clone();
        async move {
            let result: Result<Option<InvitationPreviewResponse>> = Ok(
                crate::features::enterprises::controllers::get_invitation_preview_handler(token)
                    .await
                    .ok(),
            );
            result
        }
    })?;

    let mut name = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);

    let preview_data = preview();

    let token_for_submit = token.clone();
    rsx! {
        div { class: "flex justify-center items-center px-4 min-h-screen bg-background",
            div { class: "space-y-8 w-full max-w-md",
                if let Some(invitation) = preview_data {
                    div { class: "rounded-[28px] border border-border bg-panel p-8 shadow-[0_18px_40px_rgba(15,23,42,0.06)]",
                        div { class: "text-center",
                            h1 { class: "font-display text-3xl font-bold tracking-tight text-foreground",
                                {t.accept_title}
                            }
                            p { class: "mt-2 text-sm text-foreground-muted", {t.accept_subtitle} }
                            p { class: "mt-3 font-display text-2xl font-bold tracking-tight text-brand",
                                "{invitation.enterprise_name}"
                            }
                            p { class: "mt-4 text-xs uppercase tracking-[0.14em] text-foreground-muted",
                                {t.accept_role_label}
                            }
                            p { class: "mt-1 text-sm font-semibold text-foreground",
                                {format!("{:?}", invitation.role)}
                            }
                        }

                        if let Some(msg) = error() {
                            div { class: "mt-5",
                                AlertMessage { variant: AlertVariant::Error, "{msg}" }
                            }
                        }

                        form { class: "mt-6 space-y-4",
                            method: "post",
                            onsubmit: move |e: FormEvent| {
                                e.prevent_default();
                                let name_val = name();
                                let email_val = email();
                                let password_val = password();
                                let token_val = token_for_submit.clone();

                                if let Some(msg) = password_policy_error_message(
                                    &password_val,
                                    Some(&email_val),
                                    Some(&name_val),
                                ) {
                                    error.set(Some(msg));
                                    return;
                                }

                                spawn(async move {
                                    loading.set(true);
                                    error.set(None);
                                    let res = crate::features::accounts::controllers::signup_with_invite_handler(
                                        name_val,
                                        email_val,
                                        password_val,
                                        token_val,
                                    )
                                    .await;
                                    match res {
                                        Ok(account) => {
                                            let current_enterprise =
                                                crate::features::enterprises::controllers::get_current_enterprise_handler()
                                                    .await
                                                    .ok();
                                            account_ctx.set(AccountContext {
                                                account: Some(account),
                                                current_enterprise,
                                            });
                                            nav.push(Route::Dashboard {});
                                        }
                                        Err(e) => error.set(Some(e.to_string())),
                                    }
                                    loading.set(false);
                                });
                            },
                            FormField {
                                label: t.name,
                                value: name(),
                                oninput: move |e: FormEvent| name.set(e.value()),
                                placeholder: "Your name".to_string(),
                            }
                            FormField {
                                label: t.email,
                                r#type: "email",
                                value: email(),
                                oninput: move |e: FormEvent| email.set(e.value()),
                                placeholder: "you@example.com".to_string(),
                            }
                            FormField {
                                label: "Password",
                                r#type: "password",
                                value: password(),
                                oninput: move |e: FormEvent| password.set(e.value()),
                                placeholder: "Choose a password".to_string(),
                            }

                            SubmitBtn { disabled: loading(),
                                if loading() {
                                    Spinner {}
                                }
                                {t.accept_create_account}
                            }
                        }

                        p { class: "mt-4 text-center text-xs text-foreground-muted",
                            {t.accept_already_have_account}
                        }
                    }
                } else {
                    div { class: "rounded-[28px] border border-danger bg-danger-soft p-8 text-center",
                        p { class: "font-display text-xl font-bold text-foreground",
                            {t.invitation_invalid}
                        }
                        Link {
                            to: Route::SignIn {},
                            class: "mt-4 inline-block text-sm font-semibold text-brand",
                            "Go to sign in"
                        }
                    }
                }
            }
        }
    }
}
