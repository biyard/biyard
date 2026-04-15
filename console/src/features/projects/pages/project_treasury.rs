use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::common::{ProjectPartition, SupportedChain};
use crate::common::ui::*;
use crate::features::accounts::context::use_account_context;
use crate::features::projects::i18n::ProjectsTranslate;
use crate::features::projects::pages::{FloorPriceSimulatorDialog, SalesLogDialog};
use crate::features::projects::{ProjectResponse, TreasuryStatusResponse};

/// `/projects/:project_id/treasury` — brand treasury page.
///
/// Layout:
/// 1. Project-level reserve config summary
/// 2. Live on-chain treasury snapshot (or "not deployed" notice)
/// 3. Two launcher buttons:
///    - **Floor Price Simulator**: client-side what-if tool
///    - **Sales Log Manager**: real server-backed sales ledger
#[component]
pub fn ProjectTreasury(project_id: ReadSignal<ProjectPartition>) -> Element {
    let t: ProjectsTranslate = use_translate();
    let account_ctx = use_account_context();
    let can_write = account_ctx().can_write();

    let project_loader_result = use_loader(move || async move {
        crate::features::projects::controllers::get_project_handler(project_id()).await
    });
    let status_loader_result = use_loader(move || async move {
        crate::features::projects::controllers::get_treasury_status_handler(project_id()).await
    });
    let mut show_simulator = use_signal(|| false);
    let mut show_sales_log = use_signal(|| false);
    let mut deposit_message = use_signal(|| None::<String>);

    let project_loader = project_loader_result?;
    let mut status_loader = status_loader_result?;
    let project: ProjectResponse = project_loader();
    let status: TreasuryStatusResponse = status_loader();

    let reserve_rate_pct = (project.treasury_reserve_rate * 100.0).round();
    let initial_reserve_rate = project.treasury_reserve_rate;

    rsx! {
        div { class: "space-y-6",
            SectionCard {
                SectionTitle { {t.treasury_overview} }
                div { class: "grid gap-4 sm:grid-cols-2",
                    StatCard {
                        color: StatColor::Gray,
                        label: t.treasury_reserve_rate.to_string(),
                        value: format!("{reserve_rate_pct}%"),
                    }
                    StatCard {
                        color: StatColor::Gray,
                        label: t.monthly_supply.to_string(),
                        value: format_number(project.monthly_token_supply),
                    }
                }

                // Dialog launchers — sit on the overview card so they're
                // always reachable regardless of on-chain status.
                div { class: "mt-5 flex flex-wrap gap-3",
                    Btn {
                        variant: BtnVariant::Primary,
                        onclick: move |_| show_simulator.set(true),
                        {t.open_simulator}
                    }
                    Btn {
                        variant: BtnVariant::Secondary,
                        onclick: move |_| show_sales_log.set(true),
                        {t.open_sales_log}
                    }
                }
            }

            if let Some(msg) = deposit_message() {
                AlertMessage { variant: AlertVariant::Info, "{msg}" }
            }

            if status.deployed {
                TreasuryOnChainPanel { status: status.clone() }

                if can_write && status.stable_mintable {
                    DepositBusdtCard {
                        project_id,
                        on_success: move |msg: String| {
                            deposit_message.set(Some(msg));
                            spawn(async move {
                                #[cfg(feature = "web")]
                                gloo_timers::future::TimeoutFuture::new(2_000).await;
                                status_loader.restart();
                            });
                        },
                        on_error: move |msg: String| {
                            deposit_message.set(Some(msg));
                        },
                    }

                    TestDappCard { project_id, status: status.clone() }
                }
            } else {
                AlertMessage { variant: AlertVariant::Info, {t.treasury_not_deployed} }
            }

            // Dialogs — kept at the bottom of the view so their state
            // doesn't shift the layout of the main panels.
            FloorPriceSimulatorDialog {
                open: show_simulator(),
                on_close: move |_| show_simulator.set(false),
                initial_reserve_rate,
                monthly_token_supply: project.monthly_token_supply,
            }
            SalesLogDialog {
                open: show_sales_log(),
                on_close: move |_| show_sales_log.set(false),
                project_id: project_id(),
            }
        }
    }
}

#[component]
fn TreasuryOnChainPanel(status: TreasuryStatusResponse) -> Element {
    let t: ProjectsTranslate = use_translate();

    let treasury_display =
        format_token_amount(&status.treasury_balance_raw, status.stable_decimals);
    let circulating_display =
        format_token_amount(&status.circulating_supply_raw, status.token_decimals);
    let total_display = format_token_amount(&status.total_supply_raw, status.token_decimals);
    let treasury_held_display =
        format_token_amount(&status.treasury_held_tokens_raw, status.token_decimals);
    let floor_display =
        format_floor_price_raw(&status.floor_price_raw_1e18, status.stable_decimals);

    let stable_symbol = status.stable_symbol.clone();
    let token_symbol = status.token_symbol.clone();

    let treasury_value = format!("{treasury_display} {stable_symbol}");
    let circulating_value = format!("{circulating_display} {token_symbol}");
    let total_value = format!("{total_display} {token_symbol}");
    let held_value = format!("{treasury_held_display} {token_symbol}");
    let floor_value = format!("{floor_display} {stable_symbol}");

    rsx! {
        SectionCard {
            SectionTitle { {t.treasury_onchain_title} }

            div { class: "grid gap-4 sm:grid-cols-2 lg:grid-cols-3",
                StatCard {
                    color: StatColor::Emerald,
                    label: t.treasury_onchain_balance.to_string(),
                    value: treasury_value,
                }
                StatCard {
                    color: StatColor::Blue,
                    label: t.treasury_onchain_held_tokens.to_string(),
                    value: held_value,
                }
                StatCard {
                    color: StatColor::Indigo,
                    label: t.treasury_onchain_floor.to_string(),
                    value: floor_value,
                }
                StatCard {
                    color: StatColor::Gray,
                    label: t.treasury_onchain_circulating.to_string(),
                    value: circulating_value,
                }
                StatCard {
                    color: StatColor::Gray,
                    label: t.treasury_onchain_total_supply.to_string(),
                    value: total_value,
                }
            }

            {
                let explorer = status.chain_id.and_then(SupportedChain::from_chain_id);
                rsx! {
                    if let Some(addr) = status.treasury_contract_address.as_deref() {
                        div { class: "mt-4 flex flex-wrap items-center gap-2 text-xs text-foreground-muted",
                            span { class: "font-semibold", {t.treasury_contract_address_label} }
                            if let Some(chain) = explorer {
                                a {
                                    href: chain.explorer_address_url(addr),
                                    target: "_blank",
                                    class: "break-all text-brand hover:underline",
                                    "{addr}"
                                }
                            } else {
                                code { class: "break-all", "{addr}" }
                            }
                            CopyButton { value: addr.to_string(), size: CopyButtonSize::Sm }
                        }
                    }
                    if let Some(addr) = status.brand_token_address.as_deref() {
                        div { class: "mt-2 flex flex-wrap items-center gap-2 text-xs text-foreground-muted",
                            span { class: "font-semibold", {t.brand_token_address_label} }
                            if let Some(chain) = explorer {
                                a {
                                    href: chain.explorer_address_url(addr),
                                    target: "_blank",
                                    class: "break-all text-brand hover:underline",
                                    "{addr}"
                                }
                            } else {
                                code { class: "break-all", "{addr}" }
                            }
                            CopyButton { value: addr.to_string(), size: CopyButtonSize::Sm }
                        }
                    }
                }
            }
        }
    }
}

/// Format a raw token amount (as decimal string) into a human-readable
/// value respecting the contract's `decimals`. Falls back to the raw
/// string on parse errors rather than throwing.
fn format_token_amount(raw: &str, decimals: u8) -> String {
    let Ok(value) = raw.parse::<u128>() else {
        return raw.to_string();
    };

    if decimals == 0 {
        return format_u128_with_commas(value);
    }

    let divisor = 10u128.pow(decimals as u32);
    let whole = value / divisor;
    let frac = value % divisor;

    if frac == 0 {
        return format_u128_with_commas(whole);
    }

    let frac_str = format!("{:0width$}", frac, width = decimals as usize);
    let display_frac: String = frac_str.chars().take(4).collect();
    let display_frac = display_frac.trim_end_matches('0');

    if display_frac.is_empty() {
        format_u128_with_commas(whole)
    } else {
        format!("{}.{}", format_u128_with_commas(whole), display_frac)
    }
}

fn format_floor_price_raw(raw: &str, stable_decimals: u8) -> String {
    let Ok(value) = raw.parse::<u128>() else {
        return raw.to_string();
    };
    if value == 0 {
        return "0".to_string();
    }

    // getPrice() returns stableBal_raw * 1e18 / circ_raw.
    // To get human-readable price in stable units, divide by 10^stable_decimals.
    let divisor = 10u128.pow(stable_decimals as u32);
    let whole = value / divisor;
    let frac = value % divisor;

    if frac == 0 {
        return format_u128_with_commas(whole);
    }

    let display_decimals = stable_decimals.max(2).min(6) as usize;
    let frac_str = format!("{:0width$}", frac, width = stable_decimals as usize);
    let display_frac: String = frac_str.chars().take(display_decimals).collect();
    let display_frac = display_frac.trim_end_matches('0');

    if display_frac.is_empty() {
        format_u128_with_commas(whole)
    } else {
        format!("{}.{}", format_u128_with_commas(whole), display_frac)
    }
}

fn format_u128_with_commas(value: u128) -> String {
    let s = value.to_string();
    let mut out = String::new();
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    out.chars().rev().collect()
}

#[component]
fn DepositBusdtCard(
    project_id: ReadSignal<ProjectPartition>,
    on_success: EventHandler<String>,
    on_error: EventHandler<String>,
) -> Element {
    let t: ProjectsTranslate = use_translate();

    let mut amount = use_signal(String::new);
    let mut depositing = use_signal(|| false);

    let on_deposit = move |_| {
        let a = amount();
        let pid = project_id();
        spawn(async move {
            depositing.set(true);
            let amt: i64 = match a.parse() {
                Ok(v) if v > 0 => v,
                _ => {
                    on_error.call(format!("{}invalid amount", t.deposit_failure));
                    depositing.set(false);
                    return;
                }
            };
            match crate::features::tokens::controllers::deposit_treasury_handler(pid, amt).await {
                Ok(resp) => {
                    on_success.call(format!("{}{}", t.deposit_success, resp.tx_hash));
                    amount.set(String::new());
                }
                Err(e) => on_error.call(format!("{}{e}", t.deposit_failure)),
            }
            depositing.set(false);
        });
    };

    rsx! {
        SectionCard {
            SectionTitle { {t.deposit_busdt_title} }
            p { class: "text-sm text-foreground-muted", {t.deposit_busdt_desc} }
            div { class: "mt-4 max-w-sm",
                FormField {
                    label: t.deposit_amount,
                    r#type: "number",
                    value: amount(),
                    oninput: move |e: FormEvent| amount.set(e.value()),
                    placeholder: t.deposit_amount_placeholder.to_string(),
                    min: "1",
                }
            }
            div { class: "mt-4 flex justify-end",
                Btn {
                    variant: BtnVariant::Primary,
                    disabled: depositing() || amount().is_empty(),
                    onclick: on_deposit,
                    if depositing() { {t.depositing} } else { {t.deposit_btn} }
                }
            }
        }
    }
}

#[component]
fn TestDappCard(
    project_id: ReadSignal<ProjectPartition>,
    status: TreasuryStatusResponse,
) -> Element {
    let t: ProjectsTranslate = use_translate();
    let pid = project_id();
    let encoded_pid = pid.to_string().replace('#', "%23");
    let token_addr = status.brand_token_address.as_deref().unwrap_or_default();
    let treasury_addr = status.treasury_contract_address.as_deref().unwrap_or_default();
    let exchange_url = format!("/dapp/exchange?project={encoded_pid}");
    let buyback_url = format!("/dapp/buyback?token={token_addr}&treasury={treasury_addr}");

    rsx! {
        SectionCard {
            SectionTitle { {t.test_dapp_title} }
            p { class: "text-sm text-foreground-muted", {t.test_dapp_desc} }
            div { class: "mt-4 flex flex-wrap gap-3",
                a {
                    href: exchange_url,
                    target: "_blank",
                    class: "inline-flex items-center gap-2 rounded-2xl border border-warning bg-warning-soft px-5 py-3 text-sm font-semibold text-foreground transition-colors hover:bg-warning",
                    {t.test_exchange_btn}
                }
                a {
                    href: buyback_url,
                    target: "_blank",
                    class: "inline-flex items-center gap-2 rounded-2xl border border-warning bg-warning-soft px-5 py-3 text-sm font-semibold text-foreground transition-colors hover:bg-warning",
                    {t.test_buyback_btn}
                }
            }
        }
    }
}
