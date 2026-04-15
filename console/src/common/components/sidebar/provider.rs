use super::context::*;
use super::types::*;
use dioxus::core::use_drop;
use dioxus::prelude::*;
use dioxus_primitives::merge_attributes;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::use_controlled;

const SIDEBAR_WIDTH: &str = "16rem";
const SIDEBAR_WIDTH_MOBILE: &str = "18rem";
const SIDEBAR_WIDTH_ICON: &str = "3rem";
const SIDEBAR_KEYBOARD_SHORTCUT: &str = "b";

#[component]
pub fn SidebarProvider(
    #[props(default = true)] default_open: bool,
    #[props(default)] open: ReadSignal<Option<bool>>,
    #[props(default)] on_open_change: Callback<bool>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let is_mobile = use_is_mobile();
    let side = use_signal(|| SidebarSide::Left);
    let open_mobile = use_signal(|| false);

    let (open, set_open) = use_controlled(open, default_open, on_open_change);

    let state = use_memo(move || {
        if open() {
            SidebarState::Expanded
        } else {
            SidebarState::Collapsed
        }
    });

    let ctx = SidebarCtx {
        state,
        side,
        is_mobile,
        open,
        set_open,
        open_mobile,
    };

    use_context_provider(|| ctx);

    use_effect(move || {
        spawn(async move {
            let js_code = format!(
                r#"
                function sidebarKeyHandler(event) {{
                    if (event.key === '{SIDEBAR_KEYBOARD_SHORTCUT}' && (event.metaKey || event.ctrlKey)) {{
                        event.preventDefault();
                        dioxus.send(true);
                    }}
                }}
                window.__sidebarKeyHandler = sidebarKeyHandler;
                window.addEventListener('keydown', window.__sidebarKeyHandler);
                "#
            );
            let mut eval = document::eval(&js_code);

            loop {
                if eval.recv::<bool>().await.is_ok() {
                    ctx.toggle();
                }
            }
        });
    });

    use_drop(|| {
        _ = document::eval(
            r#"
                window.removeEventListener('keydown', window.__sidebarKeyHandler);
                delete window.__sidebarKeyHandler;
                "#,
        );
    });

    let sidebar_style = format!(
        r#"
        --sidebar-width: {SIDEBAR_WIDTH};
        --sidebar-width-mobile: {SIDEBAR_WIDTH_MOBILE};
        --sidebar-width-icon: {SIDEBAR_WIDTH_ICON}
        "#
    );

    let base = attributes!(div {
        class: "sidebar-wrapper flex overflow-hidden w-full h-svh min-h-svh",
        "data-slot": "sidebar-wrapper",
        style: sidebar_style,
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { ..merged,{children} }
    }
}
