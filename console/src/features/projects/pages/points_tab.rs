use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::common::ProjectPartition;
use crate::common::components::dialog::*;
use crate::common::hooks::use_infinite_query;
use crate::common::ui::*;
use crate::features::accounts::context::use_account_context;
use crate::features::projects::i18n::ProjectsTranslate;

#[component]
pub fn PointsTab(project_id: ReadSignal<ProjectPartition>) -> Element {
    let t: ProjectsTranslate = use_translate();
    let account_ctx = use_account_context();
    let can_write = account_ctx().can_write();

    let mut newest_first = use_signal(|| true);
    let mut show_award = use_signal(|| false);
    let mut award_message = use_signal(|| None::<(AlertVariant, String)>);

    let mut query = use_infinite_query(move |bookmark| async move {
        crate::features::points::controllers::list_transactions_handler(
            project_id(),
            50,
            bookmark,
            Some(newest_first()),
        )
        .await
    })?;

    let items = query.items();
    let more_element = query.more_element();
    let has_items = !items.is_empty();

    rsx! {
        if let Some((variant, msg)) = award_message() {
            AlertMessage { variant, "{msg}" }
        }

        SectionCard {
            div { class: "mb-4 flex flex-col gap-2 sm:flex-row sm:items-end sm:justify-between",
                div {
                    SectionTitle { class: "mb-0", {t.transactions} }
                    p { class: "mt-1 text-sm text-foreground-muted",
                        {t.transactions_subtitle}
                    }
                }
                div { class: "flex items-center gap-2",
                    if has_items {
                        SortToggle {
                            newest_first: newest_first(),
                            newest_label: t.sort_newest_first,
                            oldest_label: t.sort_oldest_first,
                            on_change: move |next_newest_first: bool| {
                                if newest_first() != next_newest_first {
                                    newest_first.set(next_newest_first);
                                    query.restart();
                                    #[cfg(not(feature = "server"))]
                                    {
                                        document::eval(
                                            "window.scrollTo({ top: 0, behavior: 'instant' });",
                                        );
                                    }
                                }
                            },
                        }
                    }
                    if can_write {
                        Btn {
                            variant: BtnVariant::Primary,
                            onclick: move |_| show_award.set(true),
                            {t.award_points_btn}
                        }
                    }
                }
            }

            if items.is_empty() && !query.is_loading() {
                EmptyState {
                    icon: rsx! { IconStar {} },
                    title: t.no_transactions.to_string(),
                    description: t.no_transactions_desc.to_string(),
                }
            } else {
                DataTable {
                    TableHead {
                        TableHeadCell { {t.transaction_type} }
                        TableHeadCell { {t.user_id} }
                        TableHeadCell { {t.amount} }
                        TableHeadCell { {t.month} }
                        TableHeadCell { {t.description} }
                        TableHeadCell { {t.created_at} }
                    }
                    TableBody {
                        for tx in items.iter() {
                            {
                                let tx_type = tx.transaction_type.to_string();
                                let badge_color = match tx_type.as_str() {
                                    "Award" => BadgeColor::Green,
                                    "Deduct" => BadgeColor::Red,
                                    "Transfer" => BadgeColor::Blue,
                                    "Exchange" => BadgeColor::Purple,
                                    _ => BadgeColor::Gray,
                                };
                                let icon_color = get_transaction_icon_color(&tx_type);
                                let tx_type_icon_path = get_transaction_icon_path(&tx_type);
                                let user_id = tx.meta_user_id.clone();
                                let target = tx.target_user_id.clone();
                                let amount = tx.amount;
                                let amount_color = get_amount_color(&tx_type);
                                let month = tx.month.clone();
                                let desc = tx.description.clone().unwrap_or_else(|| "-".to_string());
                                let created = format_timestamp(tx.created_at);

                                rsx! {
                                    tr { class: "transition-colors hover:bg-panel-muted",
                                        TableCell {
                                            div { class: "flex items-center gap-3",
                                                div { class: "flex h-9 w-9 items-center justify-center rounded-2xl bg-panel-muted",
                                                    IconPath {
                                                        d: tx_type_icon_path.to_string(),
                                                        class: "h-4 w-4 {icon_color}",
                                                    }
                                                }
                                                StatusBadge { color: badge_color, "{tx_type}" }
                                            }
                                        }
                                        TableCell {
                                            div { class: "max-w-40 space-y-1",
                                                p { class: "truncate text-sm font-semibold text-foreground", "{user_id}" }
                                                if let Some(target_id) = target {
                                                    p { class: "truncate text-sm text-foreground-muted", "→ {target_id}" }
                                                }
                                            }
                                        }
                                        TableCell {
                                            span {
                                                class: "whitespace-nowrap text-sm font-semibold {amount_color}",
                                                "{format_amount(&tx_type, amount)}"
                                            }
                                        }
                                        TableCell { class: "whitespace-nowrap text-sm text-foreground-muted", "{month}" }
                                        TableCell { class: "max-w-48 truncate text-sm text-foreground-muted", "{desc}" }
                                        TableCell { class: "whitespace-nowrap text-sm text-foreground-muted", "{created}" }
                                    }
                                }
                            }
                        }
                    }
                }
                {more_element}
            }
        }

        AwardPointsDialog {
            open: show_award(),
            project_id,
            on_close: move |_| show_award.set(false),
            on_success: move |msg: String| {
                award_message.set(Some((AlertVariant::Success, msg)));
                query.restart();
            },
            on_error: move |msg: String| {
                award_message.set(Some((AlertVariant::Error, msg)));
            },
        }
    }
}

#[component]
fn AwardPointsDialog(
    open: bool,
    project_id: ReadSignal<ProjectPartition>,
    on_close: EventHandler,
    on_success: EventHandler<String>,
    on_error: EventHandler<String>,
) -> Element {
    let t: ProjectsTranslate = use_translate();
    let mut user_id = use_signal(String::new);
    let mut amount = use_signal(String::new);
    let mut month = use_signal(|| chrono::Utc::now().format("%Y-%m").to_string());
    let mut description = use_signal(String::new);
    let mut submitting = use_signal(|| false);

    let on_submit = move |_| {
        let uid = user_id();
        let amt_str = amount();
        let month_val = month();
        let desc = description();
        let pid = project_id();
        spawn(async move {
            submitting.set(true);
            let amt: i64 = match amt_str.parse() {
                Ok(v) if v > 0 => v,
                _ => {
                    on_error.call(t.award_failure.to_string() + "invalid amount");
                    submitting.set(false);
                    return;
                }
            };
            let req = vec![crate::features::points::TransactPointsRequest {
                month: month_val,
                description: if desc.is_empty() { None } else { Some(desc) },
                tx: crate::features::points::Transaction::Award {
                    to: uid,
                    amount: amt,
                },
            }];
            match crate::features::points::controllers::transact_points_handler(pid, req).await {
                Ok(_) => {
                    on_close.call(());
                    user_id.set(String::new());
                    amount.set(String::new());
                    description.set(String::new());
                    on_success.call(t.award_success.to_string());
                }
                Err(e) => on_error.call(format!("{}{e}", t.award_failure)),
            }
            submitting.set(false);
        });
    };

    rsx! {
        DialogRoot {
            open,
            on_open_change: move |v: bool| {
                if !v {
                    on_close.call(());
                }
            },
            DialogContent {
                DialogTitle { {t.award_points_title} }
                DialogDescription { {t.award_points_desc} }
                div { class: "mt-4 space-y-4",
                    FormField {
                        label: t.award_user_id,
                        r#type: "text",
                        value: user_id(),
                        oninput: move |e: FormEvent| user_id.set(e.value()),
                        placeholder: t.award_user_id_placeholder.to_string(),
                    }
                    FormField {
                        label: t.award_amount,
                        r#type: "number",
                        value: amount(),
                        oninput: move |e: FormEvent| amount.set(e.value()),
                        placeholder: t.award_amount_placeholder.to_string(),
                        min: "1",
                    }
                    FormField {
                        label: t.award_month,
                        r#type: "month",
                        value: month(),
                        oninput: move |e: FormEvent| month.set(e.value()),
                    }
                    FormField {
                        label: t.description,
                        r#type: "text",
                        value: description(),
                        oninput: move |e: FormEvent| description.set(e.value()),
                        placeholder: t.award_description_placeholder.to_string(),
                    }
                }
                DialogActions {
                    Btn {
                        variant: BtnVariant::Secondary,
                        onclick: move |_| on_close.call(()),
                        {t.cancel}
                    }
                    Btn {
                        variant: BtnVariant::Primary,
                        disabled: submitting() || user_id().is_empty() || amount().is_empty(),
                        onclick: on_submit,
                        if submitting() { {t.awarding} } else { {t.award_submit} }
                    }
                }
            }
        }
    }
}

#[component]
fn SortToggle(
    newest_first: bool,
    newest_label: &'static str,
    oldest_label: &'static str,
    on_change: EventHandler<bool>,
) -> Element {
    let newest_class = if newest_first {
        "rounded-2xl border border-brand bg-brand-soft px-4 py-2 text-sm font-semibold text-brand"
    } else {
        "rounded-2xl border border-border bg-panel px-4 py-2 text-sm font-medium text-foreground-muted transition-colors hover:bg-panel-muted"
    };
    let oldest_class = if !newest_first {
        "rounded-2xl border border-brand bg-brand-soft px-4 py-2 text-sm font-semibold text-brand"
    } else {
        "rounded-2xl border border-border bg-panel px-4 py-2 text-sm font-medium text-foreground-muted transition-colors hover:bg-panel-muted"
    };

    rsx! {
        div { class: "inline-flex gap-2",
            button {
                r#type: "button",
                class: "{newest_class}",
                onclick: move |_| on_change.call(true),
                "{newest_label}"
            }
            button {
                r#type: "button",
                class: "{oldest_class}",
                onclick: move |_| on_change.call(false),
                "{oldest_label}"
            }
        }
    }
}

fn get_transaction_icon_color(tx_type: &str) -> &'static str {
    match tx_type {
        "Award" => "text-success",
        "Deduct" => "text-danger",
        "Transfer" => "text-brand",
        "Exchange" => "text-purple",
        _ => "text-foreground-muted",
    }
}

fn get_transaction_icon_path(tx_type: &str) -> &'static str {
    match tx_type {
        "Award" => "M7 7h10v10 M7 17L17 7",
        "Deduct" => "M7 7h10v10 M17 17L7 7",
        "Transfer" => "M8 3L4 7l4 4 M4 7h16 M16 21l4-4-4-4 M20 17H4",
        "Exchange" => "M17 1l4 4-4 4 M3 11V9a4 4 0 014-4h14 M7 23l-4-4 4-4 M21 13v2a4 4 0 01-4 4H3",
        _ => {
            "M12 2l3.09 6.26L22 9.27l-5 4.87L18.18 21 12 17.77 5.82 21 7 14.14l-5-4.87 6.91-1.01L12 2"
        }
    }
}

fn get_amount_color(tx_type: &str) -> &'static str {
    match tx_type {
        "Award" => "text-success",
        "Deduct" => "text-danger",
        _ => "text-foreground",
    }
}

fn format_amount(tx_type: &str, amount: i64) -> String {
    match tx_type {
        "Award" => format!("+{}", format_number(amount)),
        "Deduct" => format!("-{}", format_number(amount)),
        _ => format_number(amount),
    }
}
