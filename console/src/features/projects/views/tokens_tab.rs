use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::common::components::dialog::*;
use crate::common::ui::*;
use crate::common::{ProjectPartition, SupportedChain, chain_display_name};
use crate::features::projects::i18n::ProjectsTranslate;

#[component]
pub fn TokensTab(project_id: ReadSignal<ProjectPartition>) -> Element {
    let t: ProjectsTranslate = use_translate();
    let mut mint_amount = use_signal(|| "1000".to_string());
    let mut target_user_id = use_signal(|| "treasury".to_string());
    let mut description = use_signal(String::new);
    let mut minting = use_signal(|| false);
    let mut deploying = use_signal(|| false);
    let mut message = use_signal(|| None::<String>);
    let mut show_confirm = use_signal(|| false);
    let mut selected_chain = use_signal(|| SupportedChain::KaiaKairos.chain_id());

    let mut token = use_loader(move || async move {
        crate::features::tokens::controllers::get_token_handler(project_id()).await
    })?;

    let token_data = token();
    let is_deployed = token_data.contract_address.is_some();
    let deployed_chain_name = token_data
        .chain_id
        .map(|cid| chain_display_name(cid))
        .unwrap_or_default();

    rsx! {
        div { class: "space-y-6",
            if let Some(msg) = message() {
                AlertMessage { variant: AlertVariant::Info, "{msg}" }
            }

            // Token Info Card
            SectionCard {
                div { class: "flex items-center justify-between mb-4",
                    h3 { class: "text-lg font-medium text-gray-900 dark:text-white",
                        {t.token_info}
                    }
                    if is_deployed {
                        StatusBadge { color: BadgeColor::Green,
                            "{t.deployed} · {deployed_chain_name}"
                        }
                    } else {
                        StatusBadge { color: BadgeColor::Yellow,
                            {t.not_deployed}
                        }
                    }
                }
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                    StatCard {
                        label: t.token_name.to_string(),
                        value: token_data.name.clone(),
                        color: StatColor::Gray,
                    }
                    StatCard {
                        label: t.token_symbol.to_string(),
                        value: token_data.symbol.clone(),
                        color: StatColor::Gray,
                    }
                    StatCard {
                        label: t.total_supply.to_string(),
                        value: format_number(token_data.total_supply),
                        color: StatColor::Gray,
                    }
                }

                // On-chain info or deploy section
                if let Some(ref addr) = token_data.contract_address {
                    div { class: "mt-4 p-4 bg-green-50 dark:bg-green-900/20 rounded-lg",
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-3",
                            div {
                                dt { class: "text-xs font-medium text-green-600 dark:text-green-400", {t.contract_address} }
                                dd { class: "mt-1 text-sm font-mono text-green-800 dark:text-green-200 break-all", "{addr}" }
                            }
                            div {
                                dt { class: "text-xs font-medium text-green-600 dark:text-green-400", {t.chain} }
                                dd { class: "mt-1 text-sm text-green-800 dark:text-green-200", "{deployed_chain_name}" }
                            }
                        }
                        if let Some(ref tx) = token_data.deployment_tx_hash {
                            div { class: "mt-2",
                                dt { class: "text-xs font-medium text-green-600 dark:text-green-400", {t.tx_hash} }
                                dd { class: "mt-1 text-sm font-mono text-green-800 dark:text-green-200 break-all", "{tx}" }
                            }
                        }
                    }
                } else {
                    div { class: "mt-4 p-4 bg-gray-50 dark:bg-gray-700/40 rounded-lg",
                        div { class: "flex items-end gap-4",
                            div { class: "flex-1",
                                FormLabel { {t.select_chain} }
                                select {
                                    class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md dark:bg-gray-700 dark:text-white",
                                    value: "{selected_chain}",
                                    onchange: move |e: FormEvent| {
                                        if let Ok(v) = e.value().parse::<u64>() {
                                            selected_chain.set(v);
                                        }
                                    },
                                    for chain in SupportedChain::all() {
                                        option {
                                            value: "{chain.chain_id()}",
                                            "{chain.display_name()}"
                                        }
                                    }
                                }
                            }
                            button {
                                class: "px-4 py-2 text-sm font-medium text-white bg-purple-600 rounded-md hover:bg-purple-700 disabled:opacity-50",
                                disabled: deploying(),
                                onclick: move |_| {
                                    let pid = project_id();
                                    let chain = selected_chain();
                                    spawn(async move {
                                        deploying.set(true);
                                        message.set(None);
                                        let res = crate::features::tokens::controllers::deploy_token_handler(
                                            pid,
                                            chain,
                                        ).await;
                                        match res {
                                            Ok(_) => {
                                                token.restart();
                                                message.set(Some(t.deploy_success.to_string()));
                                            }
                                            Err(e) => message.set(Some(format!("{}{e}", t.deploy_failure))),
                                        }
                                        deploying.set(false);
                                    });
                                },
                                if deploying() { {t.deploying} } else { {t.deploy_token_on_chain} }
                            }
                        }
                    }
                }
            }

            // Mint Token Card
            SectionCard {
                SectionTitle { {t.token_mint} }
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
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
                    p { class: "mt-2 text-xs text-green-600 dark:text-green-400",
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

            // Mint Confirmation Dialog
            DialogRoot {
                open: show_confirm(),
                on_open_change: move |v| show_confirm.set(v),
                DialogContent {
                    DialogTitle { {t.mint_confirm_title} }
                    DialogDescription { {t.mint_confirm_message} }

                    div { class: "bg-gray-50 dark:bg-gray-700 rounded-lg p-4 mb-6 space-y-3",
                        div { class: "flex justify-between",
                            span { class: "text-sm text-gray-500 dark:text-gray-400", {t.mint_confirm_target} }
                            span { class: "text-sm font-medium text-gray-900 dark:text-white", "{target_user_id}" }
                        }
                        div { class: "flex justify-between",
                            span { class: "text-sm text-gray-500 dark:text-gray-400", {t.mint_confirm_amount} }
                            span { class: "text-sm font-medium text-gray-900 dark:text-white",
                                "{format_number(mint_amount().parse::<i64>().unwrap_or(0))} {token_data.symbol}"
                            }
                        }
                        if !description().is_empty() {
                            div { class: "flex justify-between",
                                span { class: "text-sm text-gray-500 dark:text-gray-400", {t.description} }
                                span { class: "text-sm font-medium text-gray-900 dark:text-white", "{description}" }
                            }
                        }
                        if is_deployed {
                            div { class: "pt-2 border-t border-gray-200 dark:border-gray-600",
                                p { class: "text-xs text-green-600 dark:text-green-400",
                                    "{t.on_chain} · {deployed_chain_name}"
                                }
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

