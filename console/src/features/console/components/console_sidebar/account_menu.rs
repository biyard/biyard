use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::app::ThemeIsDark;
use crate::common::ui::{IconGlobe, IconLogout, IconMoon, IconSettings, IconSun};
use crate::features::accounts::context::use_account_context;
use crate::features::console::i18n::ConsoleTranslate;

#[component]
pub(super) fn AccountMenu(on_close: EventHandler<()>) -> Element {
    let t: ConsoleTranslate = use_translate();
    let nav = use_navigator();

    rsx! {
        div { class: "absolute bottom-full left-0 right-0 z-10 mb-2 rounded-2xl border border-sidebar-border bg-sidebar-panel p-1.5",
            MenuAction {
                label: t.profile.to_string(),
                onclick: move |_| {
                    on_close.call(());
                    nav.push(Route::Settings {});
                },
                IconSettings { class: "h-4 w-4" }
            }

            LanguageMenuAction { on_close: move |_| on_close.call(()) }
            ThemeMenuAction { on_close: move |_| on_close.call(()) }

            div { class: "my-1 border-t border-sidebar-border" }

            SignOutButton { on_close: move |_| on_close.call(()) }
        }
    }
}

#[component]
pub(super) fn MenuAction(
    label: String,
    onclick: EventHandler<MouseEvent>,
    #[props(optional)] value: Option<String>,
    #[props(default = false)] danger: bool,
    children: Element,
) -> Element {
    let class = if danger {
        "flex w-full items-center gap-2.5 rounded-xl px-3 py-2 text-sm font-semibold text-danger transition-colors hover:bg-danger-soft"
    } else {
        "flex w-full items-center gap-2.5 rounded-xl px-3 py-2 text-sm font-medium text-sidebar-muted transition-colors hover:bg-white/5 hover:text-sidebar-foreground"
    };

    rsx! {
        button { class: "{class}", onclick: move |event| onclick.call(event),
            {children}
            span { class: "flex-1 text-left", "{label}" }
            if let Some(value) = value {
                span { class: "rounded-full bg-white/5 px-2 py-0.5 text-[10px] font-semibold uppercase tracking-[0.12em] text-sidebar-foreground",
                    "{value}"
                }
            }
        }
    }
}

#[component]
fn LanguageMenuAction(on_close: EventHandler<()>) -> Element {
    let mut lang = dioxus_translate::use_language();
    let t: ConsoleTranslate = use_translate();
    let label = match lang() {
        dioxus_translate::Language::En => "EN".to_string(),
        dioxus_translate::Language::Ko => "KO".to_string(),
    };

    rsx! {
        MenuAction {
            label: t.language.to_string(),
            value: Some(label),
            onclick: move |_| {
                lang.set(lang().switch());
                on_close.call(());
            },
            IconGlobe { class: "h-4 w-4" }
        }
    }
}

#[component]
fn ThemeMenuAction(on_close: EventHandler<()>) -> Element {
    let t: ConsoleTranslate = use_translate();

    let ThemeIsDark(mut is_dark) = use_context::<ThemeIsDark>();

    rsx! {
        MenuAction {
            label: t.theme.to_string(),
            value: Some(if is_dark() { t.theme_dark.to_string() } else { t.theme_light.to_string() }),
            onclick: move |_| {
                let new_dark = !is_dark();
                is_dark.set(new_dark);
                #[cfg(not(feature = "server"))]
                {
                    let theme = if new_dark { "dark" } else { "light" };
                    let js = format!(
                        r#"document.documentElement.setAttribute("data-theme", "{theme}");
                                           localStorage.setItem("theme", "{theme}");
                                           document.cookie = "theme={theme}; path=/; max-age=31536000; samesite=lax";"#,
                    );
                    document::eval(&js);
                }
                on_close.call(());
            },
            if is_dark() {
                IconMoon { class: "h-4 w-4" }
            } else {
                IconSun { class: "h-4 w-4" }
            }
        }
    }
}

#[component]
fn SignOutButton(on_close: EventHandler<()>) -> Element {
    let t: ConsoleTranslate = use_translate();
    let nav = use_navigator();
    let mut account_ctx = use_account_context();

    let on_signout = move |_| {
        spawn(async move {
            let _ = crate::features::accounts::controllers::signout_handler().await;
            {
                let mut w = account_ctx.write();
                w.account = None;
                w.current_enterprise = None;
            }
            nav.push(Route::SignIn {});
            on_close.call(());
        });
    };

    rsx! {
        MenuAction {
            label: t.sign_out.to_string(),
            danger: true,
            onclick: on_signout,
            IconLogout { class: "h-4 w-4" }
        }
    }
}
