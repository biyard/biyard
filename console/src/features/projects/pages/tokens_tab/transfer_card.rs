use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::common::ui::*;
use crate::common::ProjectPartition;
use crate::features::projects::i18n::ProjectsTranslate;

#[component]
pub(super) fn TokenTransferCard(
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
