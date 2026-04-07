use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::common::ui::*;
use crate::features::accounts::context::{AccountContext, use_account_context};
use crate::features::accounts::i18n::SignUpTranslate;
use crate::features::accounts::utils::password_policy_error_message;

#[component]
pub fn SignUp() -> Element {
    let t: SignUpTranslate = use_translate();
    let nav = use_navigator();
    let mut account_ctx = use_account_context();
    let mut name = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut confirm_password = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);

    let handle_submit = move |e: FormEvent| {
        e.prevent_default();

        if let Some(message) =
            password_policy_error_message(&password(), Some(&email()), Some(&name()))
        {
            error.set(Some(message));
            return;
        }

        if password() != confirm_password() {
            error.set(Some(t.password_mismatch.to_string()));
            return;
        }

        let name_val = name();
        let email_val = email();
        let password_val = password();

        spawn(async move {
            loading.set(true);
            error.set(None);

            match crate::features::accounts::controllers::signup_handler(
                name_val,
                email_val,
                password_val,
            )
            .await
            {
                Ok(resp) => {
                    let current_enterprise =
                        crate::features::enterprises::controllers::get_current_enterprise_handler()
                            .await
                            .ok();
                    account_ctx.set(AccountContext {
                        account: Some(resp),
                        current_enterprise,
                    });
                    nav.push(Route::Dashboard {});
                }
                Err(e) => {
                    error.set(Some(e.to_string()));
                }
            }
            loading.set(false);
        });
    };

    rsx! {
        div { class: "flex justify-center items-center px-4 min-h-screen bg-gray-50 dark:bg-gray-900",
            div { class: "space-y-8 w-full max-w-md",
                div { class: "text-center",
                    h1 { class: "text-4xl font-bold text-gray-900 dark:text-white",
                        {t.title}
                    }
                    p { class: "mt-2 text-sm text-gray-600 dark:text-gray-400",
                        {t.tagline}
                    }
                    h2 { class: "mt-6 text-3xl font-extrabold text-gray-900 dark:text-white",
                        {t.sign_up_heading}
                    }
                }

                form { class: "mt-8 space-y-6", method: "post", onsubmit: handle_submit,
                    if let Some(err) = error() {
                        AlertMessage { variant: AlertVariant::Error, "{err}" }
                    }

                    div { class: "space-y-4",
                        FormFieldWithIcon {
                            label: t.name,
                            id: "name",
                            r#type: "text",
                            value: name(),
                            oninput: move |e: FormEvent| name.set(e.value()),
                            placeholder: t.name_placeholder.to_string(),
                            autocomplete: "name",
                            icon: rsx! { IconUser { class: "w-5 h-5 text-gray-400" } },
                        }

                        FormFieldWithIcon {
                            label: t.email,
                            id: "email",
                            r#type: "email",
                            value: email(),
                            oninput: move |e: FormEvent| email.set(e.value()),
                            placeholder: t.email_placeholder.to_string(),
                            autocomplete: "email",
                            icon: rsx! { IconMail { class: "w-5 h-5 text-gray-400" } },
                        }

                        FormFieldWithIcon {
                            label: t.password,
                            id: "password",
                            r#type: "password",
                            value: password(),
                            oninput: move |e: FormEvent| password.set(e.value()),
                            placeholder: t.password_placeholder.to_string(),
                            autocomplete: "new-password",
                            icon: rsx! { IconLock { class: "w-5 h-5 text-gray-400" } },
                        }

                        FormFieldWithIcon {
                            label: t.confirm_password,
                            id: "confirm_password",
                            r#type: "password",
                            value: confirm_password(),
                            oninput: move |e: FormEvent| confirm_password.set(e.value()),
                            placeholder: t.confirm_password_placeholder.to_string(),
                            autocomplete: "new-password",
                            icon: rsx! { IconLock { class: "w-5 h-5 text-gray-400" } },
                        }
                    }

                    div {
                        SubmitBtn { disabled: loading(),
                            if loading() {
                                Spinner {}
                                {t.signing_up}
                            } else {
                                {t.sign_up}
                            }
                        }
                    }

                    div { class: "text-center",
                        p { class: "text-sm text-gray-600 dark:text-gray-400",
                            {t.have_account} " "
                            Link {
                                to: Route::SignIn {},
                                class: "font-medium text-blue-600 dark:text-blue-400 hover:text-blue-500",
                                {t.sign_in_link}
                            }
                        }
                    }
                }
            }
        }
    }
}
