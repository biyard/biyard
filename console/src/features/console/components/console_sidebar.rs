use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::features::console::i18n::ConsoleTranslate;

#[component]
pub fn ConsoleSidebar() -> Element {
    let t: ConsoleTranslate = use_translate();
    let route: Route = use_route();

    rsx! {
        aside { class: "fixed top-0 left-0 h-screen w-64 bg-white dark:bg-gray-800 shadow-lg flex flex-col",
            // Logo section
            div { class: "p-6 border-b border-gray-200 dark:border-gray-700",
                h1 { class: "text-xl font-bold text-gray-900 dark:text-white",
                    "Biyard"
                }
                p { class: "text-sm text-gray-500 dark:text-gray-400 mt-1",
                    "Console"
                }
            }

            // Navigation menu
            nav { class: "flex-1 p-4 space-y-2 overflow-y-auto",
                NavItem {
                    label: "{t.dashboard}",
                    to: Route::Dashboard {},
                    is_active: matches!(route, Route::Dashboard {}),
                    icon: NavIcon::Dashboard,
                }
                NavItem {
                    label: "{t.projects}",
                    to: Route::Projects {},
                    is_active: matches!(route, Route::Projects {} | Route::ProjectDetail { .. }),
                    icon: NavIcon::Projects,
                }
                NavItem {
                    label: "{t.credentials}",
                    to: Route::Credentials {},
                    is_active: matches!(route, Route::Credentials {}),
                    icon: NavIcon::Credentials,
                }
                NavItem {
                    label: "{t.settings}",
                    to: Route::Settings {},
                    is_active: matches!(route, Route::Settings {}),
                    icon: NavIcon::Settings,
                }
            }

            // Bottom section
            div { class: "p-4 border-t border-gray-200 dark:border-gray-700 space-y-2",
                LanguageToggle {}
                ThemeToggle {}
                SignOutButton {}
            }
        }
    }
}

#[derive(Clone, PartialEq)]
enum NavIcon {
    Dashboard,
    Projects,
    Credentials,
    Settings,
}

#[component]
fn NavItem(label: String, to: Route, is_active: bool, icon: NavIcon) -> Element {
    let active_class = if is_active {
        "flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium bg-blue-50 dark:bg-blue-900/20 text-blue-600 dark:text-blue-400"
    } else {
        "flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700"
    };

    rsx! {
        Link { class: "{active_class}", to: to,
            match icon {
                NavIcon::Dashboard => rsx! {
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "20",
                        height: "20",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        rect { x: "3", y: "3", width: "7", height: "9", rx: "1" }
                        rect { x: "14", y: "3", width: "7", height: "5", rx: "1" }
                        rect { x: "14", y: "12", width: "7", height: "9", rx: "1" }
                        rect { x: "3", y: "16", width: "7", height: "5", rx: "1" }
                    }
                },
                NavIcon::Projects => rsx! {
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "20",
                        height: "20",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z" }
                    }
                },
                NavIcon::Credentials => rsx! {
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "20",
                        height: "20",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "m15.5 7.5 2.3 2.3a1 1 0 0 0 1.4 0l2.1-2.1a1 1 0 0 0 0-1.4L19 4" }
                        path { d: "M21 2l-9.6 9.6" }
                        circle { cx: "7.5", cy: "15.5", r: "5.5" }
                    }
                },
                NavIcon::Settings => rsx! {
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "20",
                        height: "20",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" }
                        circle { cx: "12", cy: "12", r: "3" }
                    }
                },
            }
            span { "{label}" }
        }
    }
}

#[component]
fn LanguageToggle() -> Element {
    let mut lang = dioxus_translate::use_language();
    let t: ConsoleTranslate = use_translate();
    let label = match lang() {
        dioxus_translate::Language::En => "EN",
        dioxus_translate::Language::Ko => "KO",
    };

    rsx! {
        button {
            class: "flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 w-full",
            onclick: move |_| {
                lang.set(lang().switch());
            },
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "20",
                height: "20",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                circle { cx: "12", cy: "12", r: "10" }
                path { d: "M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20" }
                path { d: "M2 12h20" }
            }
            span { "{t.language} ({label})" }
        }
    }
}

#[component]
fn ThemeToggle() -> Element {
    let t: ConsoleTranslate = use_translate();
    let mut is_dark = use_signal(|| false);

    #[cfg(not(feature = "server"))]
    {
        use_effect(move || {
            let mut theme =
                document::eval(r#"document.documentElement.getAttribute("data-theme")"#);
            spawn(async move {
                if let Ok(val) = theme.recv::<String>().await {
                    is_dark.set(val == "dark");
                }
            });
        });
    }

    rsx! {
        button {
            class: "flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 w-full",
            onclick: move |_| {
                let new_dark = !is_dark();
                is_dark.set(new_dark);
                #[cfg(not(feature = "server"))]
                {
                    let theme = if new_dark { "dark" } else { "light" };
                    let js = format!(
                        r#"document.documentElement.setAttribute("data-theme", "{theme}"); localStorage.setItem("theme", "{theme}");"#,
                    );
                    document::eval(&js);
                }
            },
            if is_dark() {
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "20",
                    height: "20",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" }
                }
            } else {
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "20",
                    height: "20",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    circle { cx: "12", cy: "12", r: "4" }
                    path { d: "M12 2v2" }
                    path { d: "M12 20v2" }
                    path { d: "m4.93 4.93 1.41 1.41" }
                    path { d: "m17.66 17.66 1.41 1.41" }
                    path { d: "M2 12h2" }
                    path { d: "M20 12h2" }
                    path { d: "m6.34 17.66-1.41 1.41" }
                    path { d: "m19.07 4.93-1.41 1.41" }
                }
            }
            span { {t.theme} }
        }
    }
}

#[component]
fn SignOutButton() -> Element {
    let t: ConsoleTranslate = use_translate();
    let nav = use_navigator();

    rsx! {
        button {
            class: "flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 w-full",
            onclick: move |_| {
                spawn(async move {
                    let _ = crate::features::accounts::controllers::signout_handler().await;
                    nav.push(Route::SignIn {});
                });
            },
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "20",
                height: "20",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" }
                polyline { points: "16 17 21 12 16 7" }
                line {
                    x1: "21",
                    x2: "9",
                    y1: "12",
                    y2: "12",
                }
            }
            span { {t.sign_out} }
        }
    }
}
