use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::common::{ProjectPartition, SupportedChain};
use crate::common::ui::*;
use crate::features::accounts::context::use_account_context;
use crate::features::console::i18n::ConsoleTranslate;
use crate::features::projects::i18n::ProjectsTranslate;
use crate::features::tokens::TokenResponse;

#[derive(Clone, PartialEq)]
pub enum TokenEditorMode {
    Create,
    Edit { token: TokenResponse },
}

#[component]
pub fn TokenCreate(project_id: ReadSignal<ProjectPartition>) -> Element {
    let t: ProjectsTranslate = use_translate();
    let console_t: ConsoleTranslate = use_translate();
    let nav = use_navigator();
    let account_ctx = use_account_context();

    let existing_result = use_loader(move || async move {
        crate::features::tokens::controllers::get_token_handler(project_id()).await
    });
    let project_result = use_loader(move || async move {
        crate::features::projects::controllers::get_project_handler(project_id()).await
    });

    // Viewers cannot create tokens — bounce them to the brand detail.
    // Registered before the `?` below so the hook count stays stable.
    let can_write = account_ctx().can_write();
    use_effect(move || {
        if !can_write {
            nav.replace(Route::ProjectDetail { project_id: project_id() });
        }
    });

    let existing = existing_result?;
    let project = project_result?;

    if !can_write {
        return rsx! {};
    }

    let pid_back = project_id();
    let token_opt = existing();
    let brand_name = project().name.clone();

    if let Some(token) = token_opt {
        let to_detail = pid_back.clone();
        let to_edit = pid_back.clone();
        let brand_name_1 = brand_name.clone();
        return rsx! {
            div { class: "space-y-8",
                PageHeader {
                    title: t.token_already_exists.to_string(),
                    subtitle: t.token_already_exists_subtitle.to_string(),
                    scope: PageScope::Brand {
                        name: brand_name_1,
                    },
                    workspace_label: console_t.enterprise_scope_label.to_string(),
                    brand_label: console_t.brand_scope_label.to_string(),
                    actions: rsx! {
                        Btn {
                            variant: BtnVariant::Secondary,
                            onclick: move |_| {
                                nav.push(Route::ProjectDetail {
                                    project_id: to_detail.clone(),
                                });
                            },
                            {t.back_to_brand}
                        }
                    },
                }

                SectionCard {
                    SectionTitle { {t.token_info} }
                    div { class: "grid gap-4 sm:grid-cols-2",
                        StatCard {
                            label: t.token_name.to_string(),
                            value: token.name.clone(),
                            color: StatColor::Gray,
                        }
                        StatCard {
                            label: t.token_symbol.to_string(),
                            value: token.symbol.clone(),
                            color: StatColor::Blue,
                        }
                    }

                    div { class: "mt-6 flex justify-end",
                        if token.contract_address.is_none() {
                            Btn {
                                variant: BtnVariant::Primary,
                                onclick: move |_| {
                                    nav.push(Route::TokenEdit {
                                        project_id: to_edit.clone(),
                                    });
                                },
                                {t.edit_token}
                            }
                        }
                    }
                }
            }
        };
    }

    rsx! {
        div { class: "space-y-8",
            PageHeader {
                title: t.create_token.to_string(),
                subtitle: t.create_token_subtitle.to_string(),
                scope: PageScope::Brand {
                    name: brand_name.clone(),
                },
                workspace_label: console_t.enterprise_scope_label.to_string(),
                brand_label: console_t.brand_scope_label.to_string(),
                actions: rsx! {
                    Btn {
                        variant: BtnVariant::Secondary,
                        onclick: move |_| {
                            nav.push(Route::ProjectDetail {
                                project_id: pid_back.clone(),
                            });
                        },
                        {t.skip_for_now}
                    }
                },
            }

            TokenEditorCard { project_id, mode: TokenEditorMode::Create }
        }
    }
}

#[component]
pub fn TokenEdit(project_id: ReadSignal<ProjectPartition>) -> Element {
    let t: ProjectsTranslate = use_translate();
    let console_t: ConsoleTranslate = use_translate();
    let nav = use_navigator();
    let account_ctx = use_account_context();

    let token_result = use_loader(move || async move {
        crate::features::tokens::controllers::get_token_handler(project_id()).await
    });
    let project_result = use_loader(move || async move {
        crate::features::projects::controllers::get_project_handler(project_id()).await
    });

    // Viewers cannot edit tokens — bounce them to the brand detail.
    let can_write = account_ctx().can_write();
    use_effect(move || {
        if !can_write {
            nav.replace(Route::ProjectDetail { project_id: project_id() });
        }
    });

    let token = token_result?;
    let project = project_result?;

    if !can_write {
        return rsx! {};
    }

    let pid_back = project_id();
    let brand_name = project().name.clone();

    // No token configured yet — TokenEdit was navigated to directly. Send
    // the user to the create flow rather than rendering an empty form.
    let Some(token_data) = token() else {
        let to_create = pid_back.clone();
        return rsx! {
            div { class: "space-y-8",
                PageHeader {
                    title: t.create_token.to_string(),
                    subtitle: t.create_token_subtitle.to_string(),
                    scope: PageScope::Brand {
                        name: brand_name.clone(),
                    },
                    workspace_label: console_t.enterprise_scope_label.to_string(),
                    brand_label: console_t.brand_scope_label.to_string(),
                }
                SectionCard {
                    EmptyState {
                        icon: rsx! {
                            IconToken {}
                        },
                        title: t.no_token.to_string(),
                        description: t.no_token_desc.to_string(),
                        actions: rsx! {
                            Btn {
                                variant: BtnVariant::Primary,
                                onclick: move |_| {
                                    nav.push(Route::TokenCreate {
                                        project_id: to_create.clone(),
                                    });
                                },
                                {t.create_token}
                            }
                        },
                    }
                }
            }
        };
    };

    if token_data.contract_address.is_some() {
        let to_detail = pid_back.clone();
        let brand_name_1 = brand_name.clone();
        return rsx! {
            div { class: "space-y-8",
                PageHeader {
                    title: t.token_locked_title.to_string(),
                    subtitle: t.token_locked_subtitle.to_string(),
                    scope: PageScope::Brand {
                        name: brand_name_1,
                    },
                    workspace_label: console_t.enterprise_scope_label.to_string(),
                    brand_label: console_t.brand_scope_label.to_string(),
                    actions: rsx! {
                        Btn {
                            variant: BtnVariant::Secondary,
                            onclick: move |_| {
                                nav.push(Route::ProjectDetail {
                                    project_id: to_detail.clone(),
                                });
                            },
                            {t.back_to_brand}
                        }
                    },
                }
            }
        };
    }

    rsx! {
        div { class: "space-y-8",
            PageHeader {
                title: t.edit_token.to_string(),
                subtitle: t.edit_token_subtitle.to_string(),
                scope: PageScope::Brand {
                    name: brand_name.clone(),
                },
                workspace_label: console_t.enterprise_scope_label.to_string(),
                brand_label: console_t.brand_scope_label.to_string(),
                actions: rsx! {
                    Btn {
                        variant: BtnVariant::Secondary,
                        onclick: move |_| {
                            nav.push(Route::ProjectDetail {
                                project_id: pid_back.clone(),
                            });
                        },
                        {t.back_to_brand}
                    }
                },
            }

            TokenEditorCard {
                project_id,
                mode: TokenEditorMode::Edit {
                    token: token_data,
                },
            }
        }
    }
}

#[component]
pub fn TokenEditorCard(project_id: ReadSignal<ProjectPartition>, mode: TokenEditorMode) -> Element {
    let t: ProjectsTranslate = use_translate();
    let nav = use_navigator();
    let is_create = matches!(&mode, TokenEditorMode::Create);

    let existing_token = match &mode {
        TokenEditorMode::Create => None,
        TokenEditorMode::Edit { token } => Some(token.clone()),
    };

    let seed_name = existing_token
        .as_ref()
        .map(|t| t.name.clone())
        .unwrap_or_default();
    let seed_symbol = existing_token
        .as_ref()
        .map(|t| t.symbol.clone())
        .unwrap_or_default();
    let seed_decimals = existing_token
        .as_ref()
        .map(|t| t.decimals.to_string())
        .unwrap_or_else(|| "18".to_string());
    let seed_description = existing_token
        .as_ref()
        .and_then(|t| t.description.clone())
        .unwrap_or_default();

    let seed_decay_rate = existing_token
        .as_ref()
        .map(|t| (t.decay_rate_bps as f64 / 100.0).to_string())
        .unwrap_or_else(|| "5".to_string());
    let seed_stable_token = existing_token
        .as_ref()
        .and_then(|t| t.stable_token_address.clone())
        .unwrap_or_else(|| "0xd077a400968890eacc75cdc901f0356c943e4fdb".to_string());
    let seed_slots: Vec<(String, String)> = existing_token
        .as_ref()
        .map(|t| {
            t.distribution_slots
                .iter()
                .map(|s| (s.wallet.clone(), (s.bps as f64 / 100.0).to_string()))
                .collect()
        })
        .unwrap_or_default();

    let mut name = use_signal(move || seed_name.clone());
    let mut symbol = use_signal(move || seed_symbol.clone());
    let mut decimals = use_signal(move || seed_decimals.clone());
    let seed_monthly_emission = existing_token
        .as_ref()
        .map(|t| t.monthly_emission.to_string())
        .unwrap_or_else(|| "1000000".to_string());

    let mut description = use_signal(move || seed_description.clone());
    let mut monthly_emission = use_signal(move || seed_monthly_emission.clone());
    let mut decay_rate = use_signal(move || seed_decay_rate.clone());
    let mut start_month = use_signal(|| chrono::Utc::now().format("%Y-%m").to_string());
    let mut selected_chain = use_signal(|| {
        existing_token
            .as_ref()
            .and_then(|t| t.chain_id)
            .unwrap_or_else(|| SupportedChain::KaiaKairos.chain_id())
    });
    let mut stable_token = use_signal(move || seed_stable_token.clone());
    let mut dist_slots = use_signal(move || seed_slots.clone());
    let mut message = use_signal(|| None::<(AlertVariant, String)>);
    let mut loading = use_signal(|| false);

    let title = if is_create {
        t.create_token.to_string()
    } else {
        t.edit_token.to_string()
    };
    let helper_text = if is_create {
        t.token_create_helper
    } else {
        t.token_edit_helper
    };

    let preview_name_is_placeholder = name().trim().is_empty();
    let preview_name = if preview_name_is_placeholder {
        t.token_name.to_string()
    } else {
        name()
    };
    let preview_symbol_is_placeholder = symbol().trim().is_empty();
    let preview_symbol = if preview_symbol_is_placeholder {
        "TKN".to_string()
    } else {
        symbol()
    };

    let save_failure = t.save_failure.to_string();
    let token_saved = t.token_saved.to_string();
    let required_fields_msg = t.token_required_fields.to_string();

    rsx! {
        SectionCard {
            div { class: "mb-6",
                SectionTitle { "{title}" }
                p { class: "mt-1 text-sm leading-6 text-foreground-muted", "{helper_text}" }
            }

            if let Some((variant, text)) = message() {
                div { class: "mb-5",
                    AlertMessage { variant, "{text}" }
                }
            }

            div { class: "grid gap-6 lg:grid-cols-[minmax(0,1.15fr)_minmax(0,0.85fr)]",
                div { class: "space-y-6",
                    div { class: "grid gap-4 md:grid-cols-2",
                        FormField {
                            label: t.token_name,
                            value: name(),
                            oninput: move |e: FormEvent| name.set(e.value()),
                            placeholder: t.token_name_placeholder.to_string(),
                            required: true,
                        }
                        FormField {
                            label: t.token_symbol,
                            value: symbol(),
                            oninput: move |e: FormEvent| symbol.set(e.value()),
                            placeholder: t.symbol_placeholder.to_string(),
                            maxlength: "10",
                            required: true,
                        }
                        FormField {
                            label: t.token_decimals,
                            r#type: "number",
                            value: decimals(),
                            oninput: move |e: FormEvent| decimals.set(e.value()),
                            placeholder: t.decimals_placeholder.to_string(),
                            min: "0",
                            max: "18",
                            required: true,
                        }
                        div { class: "md:col-span-2",
                            FormField {
                                label: t.description,
                                value: description(),
                                oninput: move |e: FormEvent| description.set(e.value()),
                                placeholder: t.description_placeholder.to_string(),
                            }
                        }
                        // monthly_emission is hardcoded to 1,000,000 — will be refactored later.
                        div {
                            FormField {
                                label: t.monthly_emission_label,
                                r#type: "number",
                                value: monthly_emission(),
                                oninput: move |e: FormEvent| monthly_emission.set(e.value()),
                                placeholder: "1000000".to_string(),
                                min: "1",
                                required: true,
                            }
                        }
                        div {
                            FormField {
                                label: t.decay_rate_label,
                                r#type: "number",
                                value: decay_rate(),
                                oninput: move |e: FormEvent| decay_rate.set(e.value()),
                                placeholder: t.decay_rate_placeholder.to_string(),
                                min: "0",
                                max: "99",
                                suffix: "%",
                            }
                            p { class: "mt-2 text-xs font-medium text-foreground-muted",
                                {t.decay_rate_help}
                            }
                        }
                        div {
                            p { class: "mb-1 text-xs font-semibold text-foreground-muted",
                                {t.select_chain}
                            }
                            div { class: "relative",
                                select {
                                    class: "block w-full appearance-none rounded-2xl border border-border bg-panel pl-4 pr-11 py-3 text-sm font-medium text-foreground focus:border-brand focus:outline-none focus:ring-2 focus:ring-brand",
                                    value: "{selected_chain()}",
                                    onchange: move |e: FormEvent| {
                                        if let Ok(v) = e.value().parse::<u64>() {
                                            selected_chain.set(v);
                                            if let Some(chain) = SupportedChain::from_chain_id(v) {
                                                if let Some(first) = chain.stable_token_options().first() {
                                                    stable_token.set(first.address.to_string());
                                                }
                                            }
                                        }
                                    },
                                    for chain in SupportedChain::visible() {
                                        option { value: "{chain.chain_id()}", "{chain.display_name()}" }
                                    }
                                }
                                span { class: "pointer-events-none absolute inset-y-0 right-3 flex items-center text-foreground-muted",
                                    IconChevronDown { class: "h-4 w-4" }
                                }
                            }
                        }
                        div {
                            p { class: "mb-1 text-xs font-semibold text-foreground-muted",
                                {t.stable_token_label}
                            }
                            {
                                let chain_opt = SupportedChain::from_chain_id(selected_chain());
                                let options = chain_opt.map(|c| c.stable_token_options()).unwrap_or_default();
                                rsx! {
                                    div { class: "relative",
                                        select {
                                            class: "block w-full appearance-none rounded-2xl border border-border bg-panel pl-4 pr-11 py-3 text-sm font-medium text-foreground focus:border-brand focus:outline-none focus:ring-2 focus:ring-brand",
                                            value: "{stable_token()}",
                                            onchange: move |e: FormEvent| stable_token.set(e.value()),
                                            for opt in options.iter() {
                                                option { value: "{opt.address}", "{opt.name} ({opt.symbol})" }
                                            }
                                        }
                                        span { class: "pointer-events-none absolute inset-y-0 right-3 flex items-center text-foreground-muted",
                                            IconChevronDown { class: "h-4 w-4" }
                                        }
                                    }
                                }
                            }
                            p { class: "mt-2 text-xs font-medium text-foreground-muted",
                                {t.stable_token_help}
                            }
                        }
                        {
                            let is_testnet = SupportedChain::from_chain_id(selected_chain())
                                .map(|c| c.is_testnet())
                                .unwrap_or(false);
                            rsx! {
                                div {
                                    FormField {
                                        label: t.start_month_label,
                                        r#type: "month",
                                        value: start_month(),
                                        oninput: move |e: FormEvent| start_month.set(e.value()),
                                        disabled: !is_testnet,
                                    }
                                    p { class: "mt-2 text-xs font-medium text-foreground-muted",
                                        if is_testnet {
                                            {t.start_month_help}
                                        } else {
                                            {t.start_month_mainnet_help}
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div { class: "mt-6",
                        p { class: "text-sm font-semibold text-foreground",
                            {t.distribution_slots_setup_title}
                        }
                        p { class: "mt-1 text-xs text-foreground-muted",
                            {t.distribution_slots_setup_desc}
                        }
                        div { class: "mt-3 space-y-3",
                            {
                                let total_bps: u32 = dist_slots()
                                    .iter()
                                    .filter_map(|(_, b)| b.parse::<f64>().ok())
                                    .map(|p| (p * 100.0).round() as u32)
                                    .sum();
                                let user_pool_pct = 100.0 - (total_bps as f64 / 100.0);
                                rsx! {
                                    div { class: "flex items-center gap-3 rounded-2xl border border-brand/30 bg-brand-soft px-4 py-3",
                                        div { class: "flex-1",
                                            p { class: "text-xs font-semibold text-foreground-muted", {t.user_claim_pool} }
                                        }
                                        div { class: "w-24 text-right",
                                            p { class: "text-sm font-semibold text-brand", "{user_pool_pct:.1}%" }
                                        }
                                    }
                                }
                            }

                            for (i , (_wallet , _bps)) in dist_slots().iter().enumerate() {
                                {
                                    let wallet_val = _wallet.clone();
                                    let bps_val = _bps.clone();
                                    rsx! {
                                        div { key: "{i}", class: "flex items-end gap-3",
                                            div { class: "flex-1",
                                                FormField {
                                                    label: t.slot_wallet,
                                                    r#type: "text",
                                                    value: wallet_val,
                                                    oninput: move |e: FormEvent| {
                                                        dist_slots.write()[i].0 = e.value();
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
                                                        dist_slots.write()[i].1 = e.value();
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
                                                    dist_slots.write().remove(i);
                                                },
                                                {t.remove_slot}
                                            }
                                        }
                                    }
                                }
                            }
                            Btn {
                                variant: BtnVariant::Secondary,
                                onclick: move |_| {
                                    dist_slots.write().push((String::new(), String::new()));
                                },
                                {t.add_slot}
                            }
                        }
                    }

                    div { class: "flex flex-col gap-3 pt-2 sm:flex-row sm:justify-end",
                        Btn {
                            variant: BtnVariant::Secondary,
                            disabled: loading(),
                            onclick: move |_| {
                                nav.push(Route::ProjectDetail {
                                    project_id: project_id(),
                                });
                            },
                            {t.cancel}
                        }
                        Btn {
                            variant: BtnVariant::Primary,
                            class: "sm:min-w-40",
                            disabled: loading(),
                            onclick: move |_| {
                                let pid = project_id();
                                let mode = mode.clone();
                                let nav = nav.clone();
                                let token_saved = token_saved.clone();
                                let save_failure = save_failure.clone();
                                let required_fields_msg = required_fields_msg.clone();
                                let name_val = name().trim().to_string();
                                let symbol_val = symbol().trim().to_string();
                                let decimals_input = decimals().trim().to_string();
                                let desc_val = {
                                    let value = description();
                                    if value.trim().is_empty() { None } else { Some(value) }
                                };
                                let emission_val: i64 = monthly_emission().trim().parse().unwrap_or(1_000_000);
                                let start_month_val = start_month();
                                let decay_val: u16 = {
                                    let pct: f64 = decay_rate().trim().parse().unwrap_or(5.0);
                                    (pct * 100.0).round() as u16
                                };
                                let stable_val = stable_token();
                                let chain_val = selected_chain();
                                let slot_entries: Vec<crate::features::tokens::DistributionSlotEntry> = dist_slots()
                                    .iter()
                                    .filter(|(w, b)| !w.trim().is_empty() && !b.trim().is_empty())
                                    .map(|(w, b)| {
                                        let pct: f64 = b.parse().unwrap_or(0.0);
                                        crate::features::tokens::DistributionSlotEntry {
                                            wallet: w.clone(),
                                            bps: (pct * 100.0).round() as u16,
                                        }
                                    })
                                    .collect();

                                if name_val.is_empty()
                                    || symbol_val.is_empty()
                                    || decimals_input.is_empty()
                                {
                                    message.set(Some((AlertVariant::Error, required_fields_msg)));
                                    return;
                                }

                                let Ok(decimals_val) = decimals_input.parse::<u8>() else {
                                    message.set(Some((AlertVariant::Error, required_fields_msg)));
                                    return;
                                };

                                spawn(async move {
                                    loading.set(true);
                                    message.set(None);

                                    let result = match mode {
                                        TokenEditorMode::Create => {
                                            crate::features::tokens::controllers::create_token_handler(
                                                    pid.clone(),
                                                    name_val,
                                                    symbol_val,
                                                    decimals_val,
                                                    desc_val,
                                                    emission_val,
                                                    decay_val,
                                                    slot_entries.clone(),
                                                    if stable_val.is_empty() {
                                                        None
                                                    } else {
                                                        Some(stable_val.clone())
                                                    },
                                                    Some(chain_val),
                                                    if start_month_val.is_empty() { None } else { Some(start_month_val.clone()) },
                                                )
                                                .await
                                                .map(|_| ())
                                        }
                                        TokenEditorMode::Edit { .. } => {
                                            crate::features::tokens::controllers::update_token_handler(
                                                    pid.clone(),
                                                    Some(name_val),
                                                    Some(symbol_val),
                                                    Some(decimals_val),
                                                    desc_val,
                                                    Some(emission_val),
                                                    Some(decay_val),
                                                    Some(slot_entries.clone()),
                                                    if stable_val.is_empty() {
                                                        None
                                                    } else {
                                                        Some(stable_val.clone())
                                                    },
                                                    Some(chain_val),
                                                )
                                                .await
                                                .map(|_| ())
                                        }
                                    };
                                    match result {
                                        Ok(_) => {
                                            message.set(Some((AlertVariant::Success, token_saved.clone())));
                                            nav.push(Route::ProjectToken {
                                                project_id: pid,
                                            });
                                        }
                                        Err(error) => {
                                            message
                                                .set(
                                                    Some((AlertVariant::Error, format!("{save_failure}{error}"))),
                                                );
                                        }
                                    }
                                    loading.set(false);
                                });
                            },
                            if loading() {
                                if is_create {
                                    {t.creating}
                                } else {
                                    {t.saving}
                                }
                            } else if is_create {
                                {t.create_token}
                            } else {
                                {t.save_token}
                            }
                        }
                    }
                }

                div { class: "space-y-4",
                    div { class: "rounded-[24px] border border-border bg-panel-muted p-5",
                        p { class: "text-[11px] font-semibold uppercase tracking-[0.14em] text-foreground-muted",
                            {t.live_preview}
                        }
                        div { class: "mt-4 space-y-3",
                            div { class: "rounded-2xl border border-border bg-panel px-4 py-3",
                                p { class: "text-[11px] font-semibold uppercase tracking-[0.12em] text-foreground-muted",
                                    {t.token_name}
                                }
                                p { class: if preview_name_is_placeholder { "mt-2 text-lg font-semibold italic text-foreground-muted" } else { "mt-2 text-lg font-semibold text-foreground" },
                                    "{preview_name}"
                                }
                            }
                            div { class: "rounded-2xl border border-border bg-panel px-4 py-3",
                                p { class: "text-[11px] font-semibold uppercase tracking-[0.12em] text-foreground-muted",
                                    {t.token_symbol}
                                }
                                p { class: if preview_symbol_is_placeholder { "mt-2 text-lg font-semibold italic text-foreground-muted" } else { "mt-2 text-lg font-semibold text-foreground" },
                                    "{preview_symbol}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
