use dioxus::prelude::*;
use dioxus_translate::{Translator, translate, use_translate};

use super::{IconCheck, IconCopy};

translate! {
    CopyButtonTranslate;

    copy: {
        en: "Copy",
        ko: "복사",
    },
    copied: {
        en: "Copied",
        ko: "복사됨",
    },
}

#[derive(Copy, Clone, PartialEq, Default)]
pub enum CopyButtonSize {
    #[default]
    Md,
    Sm,
}

impl CopyButtonSize {
    fn button_classes(&self) -> &'static str {
        match self {
            CopyButtonSize::Md => {
                "inline-flex h-9 w-9 items-center justify-center rounded-2xl border border-border bg-panel text-foreground-muted transition-colors hover:bg-panel-strong hover:text-foreground"
            }
            CopyButtonSize::Sm => {
                "inline-flex h-7 w-7 items-center justify-center rounded-xl border border-border bg-panel text-foreground-muted transition-colors hover:bg-panel-strong hover:text-foreground"
            }
        }
    }

    fn icon_check_class(&self) -> &'static str {
        match self {
            CopyButtonSize::Md => "h-4 w-4 text-success",
            CopyButtonSize::Sm => "h-3.5 w-3.5 text-success",
        }
    }

    fn icon_copy_class(&self) -> &'static str {
        match self {
            CopyButtonSize::Md => "h-4 w-4",
            CopyButtonSize::Sm => "h-3.5 w-3.5",
        }
    }
}

/// A copy-to-clipboard button that shows a checkmark for ~2 seconds after
/// success. Each instance manages its own "copied" state, so multiple copy
/// buttons on the same page never interfere with each other.
#[component]
pub fn CopyButton(value: String, #[props(default)] size: CopyButtonSize) -> Element {
    let t: CopyButtonTranslate = use_translate();
    let mut copied = use_signal(|| false);

    let label = if copied() { t.copied } else { t.copy };
    let button_class = size.button_classes();
    let icon_check_class = size.icon_check_class();
    let icon_copy_class = size.icon_copy_class();

    let on_click = move |_| {
        let value = value.clone();
        copy_to_clipboard(&value);
        copied.set(true);
        #[cfg(not(feature = "server"))]
        {
            spawn(async move {
                let mut eval = document::eval(
                    "await new Promise(r => setTimeout(r, 2000)); dioxus.send(true);",
                );
                let _ = eval.recv::<bool>().await;
                copied.set(false);
            });
        }
    };

    rsx! {
        button {
            r#type: "button",
            class: button_class,
            title: "{label}",
            "aria-label": "{label}",
            onclick: on_click,
            if copied() {
                IconCheck { class: icon_check_class }
            } else {
                IconCopy { class: icon_copy_class }
            }
        }
    }
}

fn copy_to_clipboard(_text: &str) {
    #[cfg(not(feature = "server"))]
    {
        let escaped = _text.replace('\\', "\\\\").replace('\'', "\\'");
        let js = format!("navigator.clipboard.writeText('{}')", escaped);
        document::eval(&js);
    }
}
