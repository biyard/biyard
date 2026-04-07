use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::common::Result;
use crate::common::components::dialog::*;
use crate::common::ui::*;
use crate::common::{ProjectPartition, SupportedChain, chain_display_name};
use crate::features::projects::i18n::ProjectsTranslate;
use crate::features::tokens::TokenResponse;

#[component]
pub fn TokensTab(project_id: ReadSignal<ProjectPartition>) -> Element {
    let t: ProjectsTranslate = use_translate();
    let nav = use_navigator();
    let mut mint_amount = use_signal(|| "1000".to_string());
    let mut target_user_id = use_signal(|| "treasury".to_string());
    let mut description = use_signal(String::new);
    let mut minting = use_signal(|| false);
    let mut deploying = use_signal(|| false);
    let mut message = use_signal(|| None::<String>);
    let mut show_confirm = use_signal(|| false);
    let mut selected_chain = use_signal(|| SupportedChain::KaiaKairos.chain_id());

    let mut token = use_loader(move || async move {
        let result: Result<Option<TokenResponse>> = Ok(
            crate::features::tokens::controllers::get_token_handler(project_id())
                .await
                .ok(),
        );
        result
    })?;

    let token_opt = token();

    rsx! {
        div { class: "space-y-6",
            if let Some(msg) = message() {
                AlertMessage { variant: AlertVariant::Info, "{msg}" }
            }

            match token_opt {
                None => {
                    let to_create = project_id();
                    rsx! {
                        SectionCard {
                            EmptyState {
                                icon: rsx! { IconToken {} },
                                title: t.no_token.to_string(),
                                description: t.no_token_desc.to_string(),
                                actions: rsx! {
                                    Btn {
                                        variant: BtnVariant::Primary,
                                        onclick: move |_| {
                                            nav.push(Route::TokenCreate { project_id: to_create.clone() });
                                        },
                                        {t.create_token}
                                    }
                                },
                            }
                        }
                    }
                }
                Some(token_data) => {
                    let has_token_contract = token_data.contract_address.is_some();
                    let has_treasury_contract = token_data.treasury_contract_address.is_some();
                    let is_deployed = has_token_contract && has_treasury_contract;
                    let is_token_only = has_token_contract && !has_treasury_contract;
                    let deployed_chain_name = token_data
                        .chain_id
                        .map(chain_display_name)
                        .unwrap_or_default();
                    let deployment_chain_id = token_data.chain_id.unwrap_or(selected_chain());
                    let to_edit = project_id();

                    rsx! {
                        SectionCard {
                            div { class: "flex flex-col gap-6 xl:flex-row xl:items-start xl:justify-between",
                                div { class: "space-y-5 xl:max-w-2xl",
                                    div { class: "flex flex-wrap items-center gap-3",
                                        SectionTitle { class: "mb-0", {t.token_info} }
                                        if is_deployed {
                                            StatusBadge { color: BadgeColor::Green,
                                                "{t.deployed} · {deployed_chain_name}"
                                            }
                                        } else if has_token_contract {
                                            StatusBadge { color: BadgeColor::Yellow,
                                                {t.token_only}
                                            }
                                        } else {
                                            StatusBadge { color: BadgeColor::Yellow,
                                                {t.not_deployed}
                                            }
                                        }
                                    }
                                    div { class: "grid gap-4 sm:grid-cols-3",
                                        StatCard {
                                            label: t.token_name.to_string(),
                                            value: token_data.name.clone(),
                                            color: StatColor::Gray,
                                        }
                                        StatCard {
                                            label: t.token_symbol.to_string(),
                                            value: token_data.symbol.clone(),
                                            color: StatColor::Blue,
                                        }
                                        StatCard {
                                            label: t.total_supply.to_string(),
                                            value: format_number(token_data.total_supply),
                                            color: StatColor::Purple,
                                        }
                                    }

                                    if !has_token_contract {
                                        div {
                                            Btn {
                                                variant: BtnVariant::Secondary,
                                                onclick: move |_| {
                                                    nav.push(Route::TokenEdit { project_id: to_edit.clone() });
                                                },
                                                {t.edit_token}
                                            }
                                        }
                                    }

                                    if let Some(ref addr) = token_data.contract_address {
                                        div { class: "rounded-[24px] border border-success bg-success-soft p-5",
                                            div { class: "grid gap-4 md:grid-cols-2",
                                                InfoItem {
                                                    label: t.contract_address.to_string(),
                                                    value: addr.clone(),
                                                    code_like: true,
                                                }
                                                InfoItem {
                                                    label: t.chain.to_string(),
                                                    value: deployed_chain_name.clone(),
                                                    code_like: false,
                                                }
                                            }
                                            if let Some(ref tx) = token_data.deployment_tx_hash {
                                                div { class: "mt-4",
                                                    InfoItem {
                                                        label: t.tx_hash.to_string(),
                                                        value: tx.clone(),
                                                        code_like: true,
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    if let Some(ref treasury_addr) = token_data.treasury_contract_address {
                                        div { class: "rounded-[24px] border border-border bg-panel-muted p-5",
                                            div { class: "grid gap-4 md:grid-cols-2",
                                                InfoItem {
                                                    label: t.treasury_contract_address.to_string(),
                                                    value: treasury_addr.clone(),
                                                    code_like: true,
                                                }
                                                InfoItem {
                                                    label: t.stable_token_address.to_string(),
                                                    value: token_data
                                                        .stable_token_address
                                                        .clone()
                                                        .unwrap_or_else(|| "-".to_string()),
                                                    code_like: true,
                                                }
                                                InfoItem {
                                                    label: t.treasury_reserve_rate.to_string(),
                                                    value: format!("{:.2}%", token_data.treasury_reserve_bps as f64 / 100.0),
                                                    code_like: false,
                                                }
                                                InfoItem {
                                                    label: t.chain.to_string(),
                                                    value: deployed_chain_name.clone(),
                                                    code_like: false,
                                                }
                                            }
                                            if let Some(ref tx) = token_data.treasury_deployment_tx_hash {
                                                div { class: "mt-4",
                                                    InfoItem {
                                                        label: t.treasury_deployment_tx_hash.to_string(),
                                                        value: tx.clone(),
                                                        code_like: true,
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                if !is_deployed {
                                    div { class: "w-full rounded-[24px] border border-border bg-panel-muted p-5 xl:max-w-sm",
                                        p { class: "text-[11px] font-semibold uppercase tracking-[0.14em] text-foreground-muted",
                                            {t.select_chain}
                                        }
                                        // Native <select> shows a browser-default chevron on
                                        // the right; `appearance-none` hides it so the custom
                                        // SVG chevron below can take over. `pr-11` leaves
                                        // room for that icon without text ever touching it.
                                        div { class: "relative mt-3",
                                            select {
                                                class: "block w-full appearance-none rounded-2xl border border-border bg-panel pl-4 pr-11 py-3 text-sm font-medium text-foreground focus:border-brand focus:outline-none focus:ring-2 focus:ring-brand disabled:cursor-not-allowed disabled:opacity-60",
                                                value: "{deployment_chain_id}",
                                                disabled: has_token_contract,
                                                onchange: move |e: FormEvent| {
                                                    if let Ok(v) = e.value().parse::<u64>() {
                                                        selected_chain.set(v);
                                                    }
                                                },
                                                for chain in SupportedChain::all() {
                                                    option { value: "{chain.chain_id()}", "{chain.display_name()}" }
                                                }
                                            }
                                            span { class: "pointer-events-none absolute inset-y-0 right-3 flex items-center text-foreground-muted",
                                                IconChevronDown { class: "h-4 w-4" }
                                            }
                                        }
                                        p { class: "mt-3 text-sm leading-6 text-foreground-muted",
                                            if is_token_only {
                                                {t.complete_treasury_note}
                                            } else {
                                                {t.deploy_stack_note}
                                            }
                                        }
                                        Btn {
                                            variant: BtnVariant::Primary,
                                            disabled: deploying(),
                                            class: "mt-5 w-full justify-center",
                                            onclick: move |_| {
                                                let pid = project_id();
                                                let chain = if has_token_contract {
                                                    token_data.chain_id.unwrap_or(selected_chain())
                                                } else {
                                                    selected_chain()
                                                };
                                                spawn(async move {
                                                    deploying.set(true);
                                                    message.set(None);
                                                    let res = crate::features::tokens::controllers::deploy_token_handler(pid, chain).await;
                                                    match res {
                                                        Ok(_) => {
                                                            token.restart();
                                                            message.set(Some(
                                                                if is_token_only {
                                                                    t.treasury_deploy_success.to_string()
                                                                } else {
                                                                    t.deploy_success.to_string()
                                                                }
                                                            ));
                                                        }
                                                        Err(e) => message.set(Some(format!("{}{e}", t.deploy_failure))),
                                                    }
                                                    deploying.set(false);
                                                });
                                            },
                                            if deploying() {
                                                {t.deploying}
                                            } else if is_token_only {
                                                {t.deploy_treasury_on_chain}
                                            } else {
                                                {t.deploy_token_on_chain}
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        SectionCard {
                            SectionTitle { {t.token_mint} }
                            div { class: "grid gap-4 md:grid-cols-3",
                                FormField {
                                    label: t.target_user_id,
                                    r#type: "text",
                                    value: target_user_id(),
                                    oninput: move |e: FormEvent| target_user_id.set(e.value()),
                                    placeholder: "treasury".to_string(),
                                }
                                FormField {
                                    label: t.mint_amount,
                                    r#type: "number",
                                    value: mint_amount(),
                                    oninput: move |e: FormEvent| mint_amount.set(e.value()),
                                    min: "1",
                                }
                                FormField {
                                    label: t.description,
                                    r#type: "text",
                                    value: description(),
                                    oninput: move |e: FormEvent| description.set(e.value()),
                                    placeholder: t.mint_description_placeholder.to_string(),
                                }
                            }

                            if is_deployed {
                                p { class: "mt-3 text-sm font-medium text-success",
                                    "{t.on_chain} · {deployed_chain_name}"
                                }
                            }

                            div { class: "mt-6 flex justify-end",
                                Btn {
                                    variant: BtnVariant::Primary,
                                    disabled: minting(),
                                    onclick: move |_| {
                                        let target = target_user_id();
                                        let amount = mint_amount().parse::<i64>().unwrap_or(0);

                                        if target.trim().is_empty() || amount <= 0 {
                                            message.set(Some(t.validation_error.to_string()));
                                            return;
                                        }

                                        show_confirm.set(true);
                                    },
                                    if minting() { {t.minting} } else { {t.token_mint} }
                                }
                            }
                        }

                        DialogRoot {
                            open: show_confirm(),
                            on_open_change: move |v| show_confirm.set(v),
                            DialogContent {
                                DialogTitle { {t.mint_confirm_title} }
                                DialogDescription { {t.mint_confirm_message} }

                                div { class: "rounded-[24px] border border-border bg-panel-muted p-5",
                                    div { class: "space-y-3",
                                        ConfirmLine {
                                            label: t.mint_confirm_target.to_string(),
                                            value: target_user_id(),
                                        }
                                        ConfirmLine {
                                            label: t.mint_confirm_amount.to_string(),
                                            value: format!(
                                                "{} {}",
                                                format_number(mint_amount().parse::<i64>().unwrap_or(0)),
                                                token_data.symbol
                                            ),
                                        }
                                        if !description().is_empty() {
                                            ConfirmLine {
                                                label: t.description.to_string(),
                                                value: description(),
                                            }
                                        }
                                    }
                                    if is_deployed {
                                        p { class: "mt-4 border-t border-border pt-4 text-sm font-medium text-success",
                                            "{t.on_chain} · {deployed_chain_name}"
                                        }
                                    }
                                }

                                DialogActions {
                                    Btn {
                                        variant: BtnVariant::Secondary,
                                        onclick: move |_| show_confirm.set(false),
                                        {t.cancel}
                                    }
                                    Btn {
                                        variant: BtnVariant::Primary,
                                        onclick: move |_| {
                                            show_confirm.set(false);
                                            let pid = project_id();
                                            let target = target_user_id();
                                            let amount = mint_amount().parse::<i64>().unwrap_or(0);
                                            let desc = {
                                                let d = description();
                                                if d.is_empty() { None } else { Some(d) }
                                            };

                                            spawn(async move {
                                                minting.set(true);
                                                message.set(None);
                                                let res = crate::features::tokens::controllers::mint_token_handler(
                                                    pid,
                                                    target,
                                                    amount,
                                                    desc,
                                                )
                                                .await;
                                                match res {
                                                    Ok(resp) => {
                                                        token.restart();
                                                        let mut msg = t.mint_success.to_string();
                                                        if let Some(tx) = resp.tx_hash {
                                                            msg.push_str(&format!(" Tx: {tx}"));
                                                        }
                                                        message.set(Some(msg));
                                                    }
                                                    Err(e) => message.set(Some(format!("{}{e}", t.mint_failure))),
                                                }
                                                minting.set(false);
                                            });
                                        },
                                        {t.confirm}
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

#[component]
fn InfoItem(label: String, value: String, code_like: bool) -> Element {
    rsx! {
        div {
            p { class: "text-[11px] font-semibold uppercase tracking-[0.14em] text-foreground-muted",
                "{label}"
            }
            if code_like {
                code { class: "mt-2 block break-all rounded-2xl border border-border bg-panel px-3 py-2 text-sm font-medium text-foreground",
                    "{value}"
                }
            } else {
                p { class: "mt-2 text-sm font-semibold text-foreground",
                    "{value}"
                }
            }
        }
    }
}

#[component]
fn ConfirmLine(label: String, value: String) -> Element {
    rsx! {
        div { class: "flex items-center justify-between gap-4",
            span { class: "text-sm text-foreground-muted", "{label}" }
            span { class: "text-sm font-semibold text-foreground", "{value}" }
        }
    }
}
