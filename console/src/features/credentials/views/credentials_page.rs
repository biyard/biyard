use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::features::credentials::CredentialStatus;
use crate::features::credentials::i18n::CredentialsTranslate;

#[component]
pub fn Credentials() -> Element {
    let t: CredentialsTranslate = use_translate();
    let mut show_create_dialog = use_signal(|| false);
    let mut generated_key = use_signal(|| None::<String>);
    let mut copied_key = use_signal(|| None::<String>);

    let mut credentials = use_loader(move || async move {
        crate::features::credentials::controllers::list_credentials_handler().await
    })?;

    let creds_data = credentials();

    rsx! {
        div {
            // Page Header
            div { class: "mb-6 flex justify-between items-center",
                div {
                    h1 { class: "text-3xl font-bold text-gray-900 dark:text-white",
                        {t.title}
                    }
                    p { class: "mt-1 text-sm text-gray-600 dark:text-gray-400",
                        {t.description}
                    }
                }
                button {
                    class: "flex items-center py-2 px-4 text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:outline-none",
                    onclick: move |_| show_create_dialog.set(true),
                    svg {
                        class: "mr-2 w-5 h-5",
                        fill: "none",
                        stroke: "currentColor",
                        view_box: "0 0 24 24",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        line { x1: "12", y1: "5", x2: "12", y2: "19" }
                        line { x1: "5", y1: "12", x2: "19", y2: "12" }
                    }
                    {t.create_new}
                }
            }

            // Main Content
            div {
                if creds_data.is_empty() {
                    // Empty state
                    div { class: "p-12 text-center bg-white rounded-lg shadow dark:bg-gray-800",
                            // Key icon
                            svg {
                                class: "mx-auto w-12 h-12 text-gray-400",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "m21 2-2 2m-7.61 7.61a5.5 5.5 0 1 1-7.778 7.778 5.5 5.5 0 0 1 7.777-7.777zm0 0L15.5 7.5m0 0 3 3L22 7l-3-3m-3.5 3.5L19 4" }
                            }
                            h3 { class: "mt-2 text-sm font-medium text-gray-900 dark:text-white",
                                {t.no_credentials}
                            }
                            div { class: "mt-6",
                                button {
                                    class: "inline-flex items-center py-2 px-4 text-sm font-medium text-white bg-blue-600 rounded-md border border-transparent shadow-sm hover:bg-blue-700 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:outline-none",
                                    onclick: move |_| show_create_dialog.set(true),
                                    // Plus icon
                                    svg {
                                        class: "mr-2 w-5 h-5",
                                        fill: "none",
                                        stroke: "currentColor",
                                        view_box: "0 0 24 24",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        line { x1: "12", y1: "5", x2: "12", y2: "19" }
                                        line { x1: "5", y1: "12", x2: "19", y2: "12" }
                                    }
                                    {t.create_new}
                                }
                            }
                        }
                } else {
                    // Credentials table
                    div { class: "overflow-hidden bg-white rounded-lg shadow dark:bg-gray-800",
                        table { class: "min-w-full divide-y divide-gray-200 dark:divide-gray-700",
                                thead { class: "bg-gray-50 dark:bg-gray-700",
                                    tr {
                                        th { class: "py-3 px-6 text-xs font-medium tracking-wider text-left text-gray-500 uppercase dark:text-gray-300",
                                            {t.name}
                                        }
                                        th { class: "py-3 px-6 text-xs font-medium tracking-wider text-left text-gray-500 uppercase dark:text-gray-300",
                                            {t.api_key}
                                        }
                                        th { class: "py-3 px-6 text-xs font-medium tracking-wider text-left text-gray-500 uppercase dark:text-gray-300",
                                            {t.created_at}
                                        }
                                        th { class: "py-3 px-6 text-xs font-medium tracking-wider text-left text-gray-500 uppercase dark:text-gray-300",
                                            {t.status}
                                        }
                                        th { class: "py-3 px-6 text-xs font-medium tracking-wider text-left text-gray-500 uppercase dark:text-gray-300",
                                            {t.actions}
                                        }
                                    }
                                }
                                tbody { class: "bg-white divide-y divide-gray-200 dark:bg-gray-800 dark:divide-gray-700",
                                    for cred in creds_data.iter() {
                                        {
                                            let id = cred.id.clone();
                                            let name = cred.name.clone();
                                            let api_key_prefix = cred.api_key_prefix.clone();
                                            let status = cred.status;
                                            let created_at = cred.created_at;
                                            let masked_key = mask_key(&api_key_prefix);
                                            let status_class = match status {
                                                CredentialStatus::Active => "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
                                                CredentialStatus::Revoked => "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200",
                                            };
                                            let status_text = match status {
                                                CredentialStatus::Active => t.active,
                                                CredentialStatus::Revoked => t.inactive,
                                            };
                                            rsx! {
                                                tr {
                                                    td { class: "py-4 px-6 text-sm font-medium text-gray-900 whitespace-nowrap dark:text-white",
                                                        "{name}"
                                                    }
                                                    td { class: "py-4 px-6 text-sm text-gray-500 whitespace-nowrap dark:text-gray-400",
                                                        div { class: "flex items-center",
                                                            code { class: "mr-2", "{masked_key}" }
                                                            {
                                                                let prefix_for_copy = api_key_prefix.clone();
                                                                rsx! {
                                                                    button {
                                                                        class: "p-1 rounded hover:bg-gray-100 dark:hover:bg-gray-700",
                                                                        title: "{t.copy}",
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
                                                                            // Check icon
                                                                            svg {
                                                                                class: "w-4 h-4 text-green-600",
                                                                                fill: "none",
                                                                                stroke: "currentColor",
                                                                                view_box: "0 0 24 24",
                                                                                stroke_width: "2",
                                                                                stroke_linecap: "round",
                                                                                stroke_linejoin: "round",
                                                                                polyline { points: "20 6 9 17 4 12" }
                                                                            }
                                                                        } else {
                                                                            // Copy icon
                                                                            svg {
                                                                                class: "w-4 h-4",
                                                                                fill: "none",
                                                                                stroke: "currentColor",
                                                                                view_box: "0 0 24 24",
                                                                                stroke_width: "2",
                                                                                stroke_linecap: "round",
                                                                                stroke_linejoin: "round",
                                                                                rect { x: "9", y: "9", width: "13", height: "13", rx: "2", ry: "2" }
                                                                                path { d: "M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    td { class: "py-4 px-6 text-sm text-gray-500 whitespace-nowrap dark:text-gray-400",
                                                        "{format_timestamp(created_at)}"
                                                    }
                                                    td { class: "py-4 px-6 whitespace-nowrap",
                                                        span { class: "px-2 inline-flex text-xs leading-5 font-semibold rounded-full {status_class}",
                                                            "{status_text}"
                                                        }
                                                    }
                                                    td { class: "py-4 px-6 text-sm whitespace-nowrap",
                                                        button {
                                                            class: "text-red-600 dark:text-red-400 hover:text-red-900 dark:hover:text-red-300",
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
                                                            // Trash2 icon
                                                            svg {
                                                                class: "w-4 h-4",
                                                                fill: "none",
                                                                stroke: "currentColor",
                                                                view_box: "0 0 24 24",
                                                                stroke_width: "2",
                                                                stroke_linecap: "round",
                                                                stroke_linejoin: "round",
                                                                polyline { points: "3 6 5 6 21 6" }
                                                                path { d: "M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" }
                                                                line { x1: "10", y1: "11", x2: "10", y2: "17" }
                                                                line { x1: "14", y1: "11", x2: "14", y2: "17" }
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
                }
            }

            // Create Credential Dialog
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

            // Generated Key Dialog
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

            match crate::features::credentials::controllers::create_credential_handler(name_val).await {
                Ok(response) => {
                    name.set(String::new());
                    on_created.call(response.api_key);
                }
                Err(_e) => {
                    // Error handled silently
                }
            }
            loading.set(false);
        });
    };

    rsx! {
        div { class: "flex fixed inset-0 z-50 justify-center items-center p-4 bg-black bg-opacity-50",
            div { class: "p-6 w-full max-w-md bg-white rounded-lg dark:bg-gray-800",
                h3 { class: "mb-4 text-lg font-semibold text-gray-900 dark:text-white",
                    {t.create_new}
                }
                div { class: "mb-4",
                    label { class: "block mb-2 text-sm font-medium text-gray-700 dark:text-gray-300",
                        {t.name}
                    }
                    input {
                        r#type: "text",
                        value: "{name}",
                        oninput: move |e: FormEvent| name.set(e.value()),
                        class: "py-2 px-3 w-full rounded-md border border-gray-300 shadow-sm dark:text-white dark:bg-gray-700 dark:border-gray-600 focus:border-blue-500 focus:ring-blue-500 focus:outline-none",
                        placeholder: "{t.name_placeholder}",
                    }
                }
                div { class: "flex justify-end space-x-3",
                    button {
                        class: "py-2 px-4 text-sm font-medium text-gray-700 bg-gray-100 rounded-md dark:text-gray-300 dark:bg-gray-700 hover:bg-gray-200 disabled:opacity-50 dark:hover:bg-gray-600",
                        disabled: loading(),
                        onclick: move |_| {
                            on_close.call(());
                        },
                        {t.cancel}
                    }
                    button {
                        class: "flex items-center py-2 px-4 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:outline-none disabled:opacity-50 disabled:cursor-not-allowed",
                        disabled: name().trim().is_empty() || loading(),
                        onclick: handle_create,
                        if loading() {
                            // Loader2 spinner
                            svg {
                                class: "mr-2 -ml-1 w-4 h-4 animate-spin",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M21 12a9 9 0 1 1-6.219-8.56" }
                            }
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
        div { class: "flex fixed inset-0 z-50 justify-center items-center p-4 bg-black bg-opacity-50",
            div { class: "p-6 w-full max-w-md bg-white rounded-lg dark:bg-gray-800",
                h3 { class: "mb-4 text-lg font-semibold text-gray-900 dark:text-white",
                    {t.key_generated}
                }
                div { class: "p-4 mb-4 bg-gray-50 rounded-md dark:bg-gray-900",
                    div { class: "flex justify-between items-center",
                        code { class: "text-sm text-gray-900 break-all dark:text-white",
                            "{api_key}"
                        }
                        button {
                            class: "p-2 ml-2 rounded hover:bg-gray-200 dark:hover:bg-gray-700",
                            onclick: move |_| {
                                on_copy.call(key_for_onclick.clone());
                            },
                            if copied_key.as_deref() == Some(&*api_key) {
                                // Check icon
                                svg {
                                    class: "w-5 h-5 text-green-600",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    polyline { points: "20 6 9 17 4 12" }
                                }
                            } else {
                                // Copy icon
                                svg {
                                    class: "w-5 h-5",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    rect { x: "9", y: "9", width: "13", height: "13", rx: "2", ry: "2" }
                                    path { d: "M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" }
                                }
                            }
                        }
                    }
                }
                p { class: "mb-4 text-sm text-red-600 dark:text-red-400",
                    {t.key_generated_warning}
                }
                div { class: "flex justify-end",
                    button {
                        class: "py-2 px-4 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700",
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

fn format_timestamp(ts: i64) -> String {
    let secs = ts / 1000;
    match chrono::DateTime::from_timestamp(secs, 0) {
        Some(dt) => dt.format("%Y-%m-%d %H:%M").to_string(),
        None => ts.to_string(),
    }
}

fn copy_to_clipboard(_text: &str) {
    #[cfg(not(feature = "server"))]
    {
        let js = format!("navigator.clipboard.writeText('{}')", _text);
        document::eval(&js);
    }
}
