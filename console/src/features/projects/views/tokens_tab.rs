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

#[component]
fn InfoItem(
    label: String,
    value: String,
    code_like: bool,
    #[props(default)] copyable: bool,
    #[props(default)] explorer_url: Option<String>,
) -> Element {
    rsx! {
        div {
            p { class: "text-[11px] font-semibold uppercase tracking-[0.14em] text-foreground-muted",
                "{label}"
            }
            if code_like {
                div { class: "mt-2 flex items-center gap-2",
                    code { class: "block flex-1 break-all rounded-2xl border border-border bg-panel px-3 py-2 text-sm font-medium text-foreground",
                        "{value}"
                    }
                    if copyable {
                        CopyButton { value: value.clone() }
                    }
                    if let Some(ref url) = explorer_url {
                        a {
                            href: "{url}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "inline-flex shrink-0 items-center justify-center rounded-xl border border-border bg-panel px-2.5 py-2 text-xs font-medium text-foreground-muted hover:border-brand hover:text-brand transition-colors",
                            title: "View on block explorer",
                            "↗"
                        }
                    }
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
fn DistributionSlotsEditor(
    project_id: ReadSignal<ProjectPartition>,
    on_message: EventHandler<String>,
) -> Element {
    let t: ProjectsTranslate = use_translate();

    let mut slots = use_signal(|| Vec::<(String, String)>::new());
    let mut saving = use_signal(|| false);

    let total_bps: u32 = slots()
        .iter()
        .filter_map(|(_, bps_str)| bps_str.parse::<u32>().ok())
        .sum();
    let claim_pool_pct = 100.0 - (total_bps as f64 / 100.0);

    let on_add = move |_| {
        slots.write().push((String::new(), String::new()));
    };

    let on_save = move |_| {
        let current_slots = slots();
        let pid = project_id();
        spawn(async move {
            saving.set(true);
            let slot_inputs: Vec<crate::features::tokens::controllers::DistributionSlotInput> =
                current_slots
                    .iter()
                    .filter(|(w, b)| !w.is_empty() && !b.is_empty())
                    .map(|(w, b)| {
                        let pct: f64 = b.parse().unwrap_or(0.0);
                        crate::features::tokens::controllers::DistributionSlotInput {
                            wallet: w.clone(),
                            bps: (pct * 100.0).round() as u16,
                        }
                    })
                    .collect();

            match crate::features::tokens::controllers::set_distribution_slots_handler(
                pid,
                slot_inputs,
            )
            .await
            {
                Ok(_) => on_message.call(t.slots_saved.to_string()),
                Err(e) => on_message.call(format!("{}{e}", t.slots_save_failure)),
            }
            saving.set(false);
        });
    };

    rsx! {
        div { class: "mt-4 space-y-3",
            for (i, (_wallet, _bps)) in slots().iter().enumerate() {
                {
                    let wallet_val = _wallet.clone();
                    let bps_val = _bps.clone();
                    rsx! {
                        div {
                            key: "{i}",
                            class: "flex items-end gap-3",
                            div { class: "flex-1",
                                FormField {
                                    label: t.slot_wallet,
                                    r#type: "text",
                                    value: wallet_val,
                                    oninput: move |e: FormEvent| {
                                        slots.write()[i].0 = e.value();
                                    },
                                    placeholder: "0x...".to_string(),
                                }
                            }
                            div { class: "w-24",
                                FormField {
                                    label: t.slot_bps,
                                    r#type: "number",
                                    value: bps_val,
                                    oninput: move |e: FormEvent| {
                                        slots.write()[i].1 = e.value();
                                    },
                                    placeholder: "10".to_string(),
                                    min: "0",
                                    max: "99",
                                    suffix: "%",
                                }
                            }
                            Btn {
                                variant: BtnVariant::Secondary,
                                onclick: move |_| {
                                    slots.write().remove(i);
                                },
                                {t.remove_slot}
                            }
                        }
                    }
                }
            }

            div { class: "flex items-center justify-between",
                Btn {
                    variant: BtnVariant::Secondary,
                    onclick: on_add,
                    {t.add_slot}
                }
                p { class: "text-sm text-foreground-muted",
                    "{t.claim_pool_label}: {claim_pool_pct:.1}%"
                }
            }

            if !slots().is_empty() {
                div { class: "flex justify-end",
                    Btn {
                        variant: BtnVariant::Primary,
                        disabled: saving() || total_bps >= 10000,
                        onclick: on_save,
                        if saving() { {t.saving_slots} } else { {t.save_slots} }
                    }
                }
            }
        }
    }
}

/// Returns (effective_months, cumulative_supply).
/// effective_months = month when emission drops below 1% of initial (or 0 if decay is 0 = unlimited).
/// cumulative_supply = total tokens emitted over effective_months (or 0 if unlimited).
fn compute_emission_projection(monthly_emission: u64, decay_bps: u16) -> (u32, u64) {
    if decay_bps == 0 {
        return (0, 0); // unlimited — no convergence
    }
    let threshold = (monthly_emission as u128) / 100; // 1% of initial
    let mut total: u128 = 0;
    let mut emission = monthly_emission as u128;
    let mut months: u32 = 0;
    while emission >= threshold.max(1) && months < 1200 {
        total += emission;
        months += 1;
        emission = emission * (10000 - decay_bps as u128) / 10000;
    }
    (months, total as u64)
}

fn format_with_commas(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

#[component]
fn ContractParamsPanel(
    #[allow(unused)] project_id: ReadSignal<ProjectPartition>,
    token: crate::features::tokens::TokenResponse,
) -> Element {
    let t: ProjectsTranslate = use_translate();

    let monthly_emission = token.monthly_emission.max(0) as u64;
    let decay_bps = token.decay_rate_bps;
    let (effective_months, cumulative) = compute_emission_projection(monthly_emission, decay_bps);
    let params_configured = monthly_emission > 0;
    let slots_count = token.distribution_slots.len();
    let stable_label = token
        .stable_token_address
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(|a| {
            if a.len() > 10 {
                format!("{}...{}", &a[..6], &a[a.len()-4..])
            } else {
                a.to_string()
            }
        })
        .unwrap_or_else(|| "Not set".to_string());

    rsx! {
        div { class: "mt-5 rounded-[24px] border border-border bg-panel-muted p-5",
            p { class: "text-[11px] font-semibold uppercase tracking-[0.14em] text-foreground-muted",
                {t.contract_params_title}
            }
            p { class: "mt-1 text-xs text-foreground-muted leading-5",
                {t.contract_params_desc}
            }

            if params_configured {
                div { class: "mt-4 grid gap-3 sm:grid-cols-2",
                    div { class: "flex items-center justify-between rounded-2xl border border-border bg-panel px-4 py-3",
                        span { class: "text-xs text-foreground-muted", {t.monthly_emission_label} }
                        span { class: "text-sm font-semibold text-foreground",
                            "{format_with_commas(monthly_emission)}"
                        }
                    }
                    div { class: "flex items-center justify-between rounded-2xl border border-border bg-panel px-4 py-3",
                        span { class: "text-xs text-foreground-muted", {t.decay_rate_label} }
                        span { class: "text-sm font-semibold text-foreground",
                            "{decay_bps as f64 / 100.0:.1}%"
                        }
                    }
                    if effective_months > 0 {
                        div { class: "flex items-center justify-between rounded-2xl border border-border bg-panel px-4 py-3",
                            span { class: "text-xs text-foreground-muted", {t.emission_projection_label} }
                            span { class: "text-sm font-semibold text-foreground",
                                "{format_with_commas(cumulative)} ({effective_months} {t.months_label})"
                            }
                        }
                    } else {
                        div { class: "flex items-center justify-between rounded-2xl border border-border bg-panel px-4 py-3",
                            span { class: "text-xs text-foreground-muted", {t.emission_projection_label} }
                            span { class: "text-sm font-semibold text-foreground",
                                {t.unlimited_emission}
                            }
                        }
                    }
                    div { class: "flex items-center justify-between rounded-2xl border border-border bg-panel px-4 py-3",
                        span { class: "text-xs text-foreground-muted", {t.stable_token_label} }
                        span { class: "text-sm font-semibold text-foreground",
                            "{stable_label}"
                        }
                    }
                    div { class: "flex items-center justify-between rounded-2xl border border-border bg-panel px-4 py-3",
                        span { class: "text-xs text-foreground-muted", {t.distribution_slots_setup_title} }
                        span { class: "text-sm font-semibold text-foreground",
                            "{slots_count} slots"
                        }
                    }
                }
            } else {
                div { class: "mt-4 rounded-2xl border border-warning bg-warning-soft px-4 py-3",
                    p { class: "text-sm text-foreground",
                        {t.contract_params_not_set}
                    }
                }
            }
        }
    }
}

#[component]
fn TokenTransferCard(
    project_id: ReadSignal<ProjectPartition>,
    on_message: EventHandler<String>,
) -> Element {
    let t: ProjectsTranslate = use_translate();

    let mut wallet = use_signal(String::new);
    let mut amount = use_signal(String::new);
    let mut transferring = use_signal(|| false);

    let on_transfer = move |_| {
        let w = wallet();
        let a = amount();
        let pid = project_id();
        spawn(async move {
            transferring.set(true);
            let amt: i64 = match a.parse() {
                Ok(v) if v > 0 => v,
                _ => {
                    on_message.call(format!("{}invalid amount", t.transfer_failure));
                    transferring.set(false);
                    return;
                }
            };
            match crate::features::tokens::controllers::mint_token_handler(
                pid,
                "transfer".to_string(),
                w,
                amt,
            )
            .await
            {
                Ok(resp) => {
                    let tx = resp.tx_hash.unwrap_or_default();
                    on_message.call(format!("{}{tx}", t.transfer_success));
                    wallet.set(String::new());
                    amount.set(String::new());
                }
                Err(e) => on_message.call(format!("{}{e}", t.transfer_failure)),
            }
            transferring.set(false);
        });
    };

    rsx! {
        SectionCard {
            SectionTitle { {t.transfer_token_title} }
            p { class: "text-sm text-foreground-muted", {t.transfer_token_desc} }
            div { class: "mt-4 grid gap-4 sm:grid-cols-2",
                FormField {
                    label: t.transfer_wallet_address,
                    r#type: "text",
                    value: wallet(),
                    oninput: move |e: FormEvent| wallet.set(e.value()),
                    placeholder: t.transfer_wallet_placeholder.to_string(),
                }
                FormField {
                    label: t.transfer_amount,
                    r#type: "number",
                    value: amount(),
                    oninput: move |e: FormEvent| amount.set(e.value()),
                    placeholder: t.transfer_amount_placeholder.to_string(),
                    min: "1",
                }
            }
            div { class: "mt-4 flex justify-end",
                Btn {
                    variant: BtnVariant::Primary,
                    disabled: transferring() || wallet().is_empty() || amount().is_empty(),
                    onclick: on_transfer,
                    if transferring() { {t.transferring} } else { {t.transfer_btn} }
                }
            }
        }
    }
}

