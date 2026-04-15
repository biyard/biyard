use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::common::ProjectPartition;
use crate::common::components::dialog::{
    DialogActions, DialogContent, DialogDescription, DialogRoot, DialogTitle,
};
use crate::common::ui::*;
use crate::features::projects::SalesLogResponse;
use crate::features::projects::i18n::ProjectsTranslate;

/// Real sales ledger manager (server-backed).
///
/// Lists `SalesLog` rows for the project with pagination, plus a
/// compact form to add new rows manually. Every row corresponds to a
/// real sale recorded against the brand's revenue stream — this is
/// **not** a simulation. Manual entry is intended for onboarding
/// (importing historical sales) and operator demos; normal production
/// flow is for the brand's POS/backend to POST `/v1/projects/:id/sales-logs`
/// directly.
#[component]
pub fn SalesLogDialog(
    open: bool,
    on_close: EventHandler<()>,
    project_id: ProjectPartition,
) -> Element {
    let t: ProjectsTranslate = use_translate();

    let pid_for_loader = project_id.clone();
    let logs_loader_result = use_loader(move || {
        let pid = pid_for_loader.clone();
        async move {
            crate::features::projects::controllers::list_sales_logs_handler(pid, 50, None).await
        }
    });
    let mut amount_input = use_signal(String::new);
    let mut memo_input = use_signal(String::new);
    let mut submitting = use_signal(|| false);
    let mut message = use_signal(|| None::<(AlertVariant, String)>);

    let mut logs_loader = logs_loader_result?;
    let logs_page = logs_loader();
    let logs: Vec<SalesLogResponse> = logs_page.items;

    let pid_for_submit = project_id.clone();
    let on_submit = move |_| {
        let raw = amount_input();
        let Ok(amount) = raw.trim().parse::<i64>() else {
            message.set(Some((
                AlertVariant::Error,
                t.sales_log_amount_invalid.to_string(),
            )));
            return;
        };
        if amount <= 0 {
            message.set(Some((
                AlertVariant::Error,
                t.sales_log_amount_invalid.to_string(),
            )));
            return;
        }

        let memo_raw = memo_input();
        let memo = {
            let trimmed = memo_raw.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        };

        submitting.set(true);
        message.set(None);
        let pid = pid_for_submit.clone();
        spawn(async move {
            let res =
                crate::features::projects::controllers::create_sales_log_handler(pid, amount, memo)
                    .await;
            match res {
                Ok(_) => {
                    amount_input.set(String::new());
                    memo_input.set(String::new());
                    logs_loader.restart();
                    message.set(Some((
                        AlertVariant::Success,
                        t.sales_log_add_success.to_string(),
                    )));
                }
                Err(e) => {
                    message.set(Some((
                        AlertVariant::Error,
                        format!("{}{e}", t.sales_log_add_failure),
                    )));
                }
            }
            submitting.set(false);
        });
    };

    let total_amount: i64 = logs.iter().map(|l| l.amount).sum();

    rsx! {
        DialogRoot {
            open,
            on_open_change: move |v: bool| {
                if !v {
                    on_close.call(());
                }
            },
            DialogContent {
                DialogTitle { {t.sales_log_title} }
                DialogDescription { {t.sales_log_subtitle} }

                // Summary strip
                div { class: "mt-2 grid gap-3 sm:grid-cols-2",
                    StatCard {
                        color: StatColor::Gray,
                        label: t.sales_log_count_label.to_string(),
                        value: format_number(logs.len() as i64),
                    }
                    StatCard {
                        color: StatColor::Emerald,
                        label: t.sales_log_total_label.to_string(),
                        value: format_number(total_amount),
                    }
                }

                // Add form
                div { class: "mt-5 rounded-2xl border border-border bg-panel-muted p-4",
                    p { class: "mb-3 text-sm font-semibold text-foreground",
                        {t.sales_log_add_title}
                    }
                    if let Some((variant, msg)) = message() {
                        div { class: "mb-3",
                            AlertMessage { variant, "{msg}" }
                        }
                    }
                    div { class: "grid gap-3 sm:grid-cols-2",
                        div {
                            FormLabel { {t.sales_log_amount_label} }
                            input {
                                r#type: "number",
                                value: amount_input(),
                                oninput: move |e: FormEvent| amount_input.set(e.value()),
                                placeholder: t.sales_log_amount_placeholder,
                                min: "0",
                                class: "w-full rounded-lg border border-border bg-panel px-3 py-2 text-sm text-foreground focus:border-brand focus:outline-none",
                            }
                        }
                        div {
                            FormLabel { {t.sales_log_memo_label} }
                            input {
                                r#type: "text",
                                value: memo_input(),
                                oninput: move |e: FormEvent| memo_input.set(e.value()),
                                placeholder: t.sales_log_memo_placeholder,
                                class: "w-full rounded-lg border border-border bg-panel px-3 py-2 text-sm text-foreground focus:border-brand focus:outline-none",
                            }
                        }
                    }
                    div { class: "mt-3 flex justify-end",
                        Btn {
                            variant: BtnVariant::Primary,
                            disabled: submitting() || amount_input().trim().is_empty(),
                            onclick: on_submit,
                            if submitting() {
                                {t.sales_log_submitting}
                            } else {
                                {t.sales_log_add_button}
                            }
                        }
                    }
                }

                // List
                div { class: "mt-6",
                    p { class: "mb-2 text-sm font-semibold text-foreground",
                        {t.sales_log_list_title}
                    }
                    if logs.is_empty() {
                        p { class: "rounded-xl border border-dashed border-border bg-panel-muted px-4 py-6 text-center text-sm text-foreground-muted",
                            {t.sales_log_empty}
                        }
                    } else {
                        div { class: "max-h-72 overflow-y-auto rounded-xl border border-border",
                            table { class: "w-full text-left text-xs",
                                thead { class: "sticky top-0 bg-panel-muted text-foreground-muted",
                                    tr {
                                        th { class: "px-3 py-2", {t.sales_log_col_created_at} }
                                        th { class: "px-3 py-2 text-right", {t.sales_log_col_amount} }
                                        th { class: "px-3 py-2", {t.sales_log_col_memo} }
                                    }
                                }
                                tbody {
                                    for log in logs.iter() {
                                        tr {
                                            key: "{log.id}",
                                            class: "border-t border-border",
                                            td { class: "px-3 py-2 font-mono text-foreground-muted",
                                                "{format_timestamp(log.created_at)}"
                                            }
                                            td { class: "px-3 py-2 text-right font-mono font-medium text-foreground",
                                                "{format_number(log.amount)}"
                                            }
                                            td { class: "px-3 py-2 text-foreground-soft",
                                                if let Some(memo) = log.memo.as_deref() {
                                                    "{memo}"
                                                } else {
                                                    "—"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                DialogActions {
                    Btn {
                        variant: BtnVariant::Primary,
                        onclick: move |_| on_close.call(()),
                        {t.close}
                    }
                }
            }
        }
    }
}
