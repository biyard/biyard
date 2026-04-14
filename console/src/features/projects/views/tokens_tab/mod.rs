mod contract_params;
mod distribution_slots;
mod info_item;
mod transfer_card;

use contract_params::ContractParamsPanel;
use info_item::InfoItem;

use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::common::components::dialog::*;
use crate::common::ui::*;
use crate::common::{ProjectPartition, SupportedChain, chain_display_name};
use crate::features::accounts::context::use_account_context;
use crate::features::projects::i18n::ProjectsTranslate;

#[component]
pub fn TokensTab(project_id: ReadSignal<ProjectPartition>) -> Element {
    let t: ProjectsTranslate = use_translate();
    let nav = use_navigator();
    let account_ctx = use_account_context();
    let can_write = account_ctx().can_write();
    let mut deploying = use_signal(|| false);
    let mut message = use_signal(|| None::<String>);
    let mut show_deploy_confirm = use_signal(|| false);
    let mut deploy_understood = use_signal(|| false);

    let token_result = use_loader(move || async move {
        crate::features::tokens::controllers::get_token_handler(project_id()).await
    });

    let mut token = token_result?;

    let token_opt = token();

    let is_deploying_db = token_opt
        .as_ref()
        .map(|t| t.deploying)
        .unwrap_or(false);

    use_effect(move || {
        if !is_deploying_db {
            return;
        }
        spawn(async move {
            #[cfg(feature = "web")]
            {
                gloo_timers::future::TimeoutFuture::new(5_000).await;
                token.restart();
            }
        });
    });

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
                                    if can_write {
                                        Btn {
                                            variant: BtnVariant::Primary,
                                            onclick: move |_| {
                                                nav.push(Route::TokenCreate { project_id: to_create.clone() });
                                            },
                                            {t.create_token}
                                        }
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
                    let deployed_chain = token_data.chain_id.and_then(SupportedChain::from_chain_id);
                    let to_edit = project_id();

                    rsx! {
                        if !is_deployed {
                            // --- Not deployed: unified card ---
                            SectionCard {
                                div { class: "flex flex-wrap items-center gap-3 mb-5",
                                    SectionTitle { class: "mb-0", {t.token_info} }
                                    StatusBadge { color: BadgeColor::Yellow, {t.not_deployed} }
                                }

                                div { class: "grid gap-4 sm:grid-cols-2 mt-5",
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
                                }

                                ContractParamsPanel {
                                    project_id,
                                    token: token_data.clone(),
                                }

                                if can_write {
                                    div { class: "mt-5 flex flex-wrap gap-3",
                                        Btn {
                                            variant: BtnVariant::Secondary,
                                            onclick: move |_| {
                                                nav.push(Route::TokenEdit { project_id: to_edit.clone() });
                                            },
                                            {t.edit_token}
                                        }
                                        Btn {
                                            variant: BtnVariant::Primary,
                                            disabled: deploying() || token_data.deploying || token_data.monthly_emission <= 0 || token_data.chain_id.is_none(),
                                            onclick: move |_| {
                                                deploy_understood.set(false);
                                                show_deploy_confirm.set(true);
                                            },
                                            if deploying() || token_data.deploying {
                                                {t.deploying}
                                            } else {
                                                {t.deploy_token_on_chain}
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            // --- Deployed: show contract info ---
                            SectionCard {
                                div { class: "flex flex-wrap items-center gap-3 mb-5",
                                    SectionTitle { class: "mb-0", {t.token_info} }
                                    StatusBadge { color: BadgeColor::Green,
                                        "{t.deployed} · {deployed_chain_name}"
                                    }
                                }

                                div { class: "grid gap-4 sm:grid-cols-2",
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
                                }

                                if let Some(ref addr) = token_data.contract_address {
                                    div { class: "mt-4 flex flex-col gap-4 rounded-[24px] border border-success bg-success-soft p-5",
                                        InfoItem {
                                            label: t.contract_address.to_string(),
                                            value: addr.clone(),
                                            code_like: true,
                                            copyable: true,
                                            explorer_url: deployed_chain.map(|c| c.explorer_address_url(addr)),
                                        }
                                        if let Some(ref tx) = token_data.deployment_tx_hash {
                                            InfoItem {
                                                label: t.tx_hash.to_string(),
                                                value: tx.clone(),
                                                code_like: true,
                                                copyable: true,
                                                explorer_url: deployed_chain.map(|c| c.explorer_tx_url(tx)),
                                            }
                                        }
                                    }
                                }

                                if let Some(ref treasury_addr) = token_data.treasury_contract_address {
                                    div { class: "mt-4 flex flex-col gap-4 rounded-[24px] border border-border bg-panel-muted p-5",
                                        InfoItem {
                                            label: t.treasury_contract_address.to_string(),
                                            value: treasury_addr.clone(),
                                            code_like: true,
                                            copyable: true,
                                            explorer_url: deployed_chain.map(|c| c.explorer_address_url(treasury_addr)),
                                        }
                                        if let Some(ref tx) = token_data.treasury_deployment_tx_hash {
                                            InfoItem {
                                                label: t.treasury_deployment_tx_hash.to_string(),
                                                value: tx.clone(),
                                                code_like: true,
                                                copyable: true,
                                                explorer_url: deployed_chain.map(|c| c.explorer_tx_url(tx)),
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        DialogRoot {
                            open: show_deploy_confirm(),
                            on_open_change: move |v: bool| {
                                if !v {
                                    deploy_understood.set(false);
                                }
                                show_deploy_confirm.set(v);
                            },
                            DialogContent {
                                DialogTitle {
                                    if is_token_only {
                                        {t.deploy_treasury_confirm_title}
                                    } else {
                                        {t.deploy_token_confirm_title}
                                    }
                                }
                                DialogDescription {
                                    {t.deploy_confirm_message}
                                }
                                div { class: "mt-4 rounded-[20px] border border-warning bg-warning-soft p-4 text-sm leading-6 text-foreground",
                                    p { class: "font-semibold", {t.deploy_confirm_irreversible_title} }
                                    p { class: "mt-1 text-foreground-soft", {t.deploy_confirm_irreversible_body} }
                                }
                                label { class: "mt-4 flex cursor-pointer items-start gap-3 text-sm text-foreground",
                                    input {
                                        r#type: "checkbox",
                                        checked: "{deploy_understood()}",
                                        oninput: move |e: FormEvent| {
                                            deploy_understood.set(e.value() == "true");
                                        },
                                        class: "mt-0.5 h-4 w-4 rounded border-border bg-panel text-brand focus:ring-brand",
                                    }
                                    span { {t.deploy_confirm_acknowledge} }
                                }
                                DialogActions {
                                    Btn {
                                        variant: BtnVariant::Secondary,
                                        onclick: move |_| {
                                            deploy_understood.set(false);
                                            show_deploy_confirm.set(false);
                                        },
                                        {t.cancel}
                                    }
                                    Btn {
                                        variant: BtnVariant::Primary,
                                        disabled: !deploy_understood() || deploying() || token_data.deploying,
                                        onclick: move |_| {
                                            show_deploy_confirm.set(false);
                                            deploy_understood.set(false);
                                            let pid = project_id();
                                            spawn(async move {
                                                deploying.set(true);
                                                message.set(None);
                                                let res = crate::features::tokens::controllers::deploy_token_handler(pid).await;
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
                                        if is_token_only {
                                            {t.deploy_treasury_on_chain}
                                        } else {
                                            {t.deploy_token_on_chain}
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
