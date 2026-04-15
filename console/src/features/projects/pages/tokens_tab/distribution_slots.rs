use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::common::ui::*;
use crate::common::ProjectPartition;
use crate::features::projects::i18n::ProjectsTranslate;

#[component]
pub(super) fn DistributionSlotsEditor(
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
