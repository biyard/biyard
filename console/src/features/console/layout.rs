use dioxus::prelude::*;

use crate::Route;
use crate::common::ui::{AlertMessage, AlertVariant, Btn, BtnVariant, SectionCard, Spinner};
use crate::features::accounts::context::use_account_context;
use crate::features::console::components::{ConsoleSidebar, SidebarCollapsed, SidebarOpen};
use crate::features::console::i18n::ConsoleTranslate;
use dioxus_translate::use_translate;

#[component]
pub fn Layout() -> Element {
    let account_ctx = use_account_context();
    let nav = use_navigator();
    let collapsed = use_context_provider(|| Signal::new(SidebarCollapsed(false)));
    let mut sidebar_open = use_context_provider(|| Signal::new(SidebarOpen(false)));

    use_effect(move || {
        if !account_ctx().is_logged_in() {
            nav.push(Route::SignIn {});
        }
    });

    let logged_in = account_ctx().is_logged_in();
    let lg_padding_class = if collapsed().0 {
        "lg:pl-16"
    } else {
        "lg:pl-[17rem]"
    };

    rsx! {
        div { class: "min-h-screen bg-background text-foreground",
            if logged_in {
                div { class: "pointer-events-none fixed inset-0 overflow-hidden",
                    div { class: "absolute right-[-7rem] top-[-7rem] h-80 w-80 rounded-full bg-brand-soft opacity-70 blur-3xl" }
                    div { class: "absolute bottom-[-10rem] left-[22rem] h-96 w-96 rounded-full bg-info-soft opacity-60 blur-3xl" }
                }

                SuspenseBoundary {
                    fallback: move |_| rsx! {
                        SidebarShellFallback {
                            collapsed: collapsed().0,
                        }
                    },
                    ConsoleSidebar {}
                }

                div { class: "relative min-h-screen pl-0 {lg_padding_class} transition-[padding] duration-150",
                    MobileTopBar { on_open: move |_| sidebar_open.set(SidebarOpen(true)) }

                    main { class: "px-4 pb-8 pt-6 sm:px-6",
                        div { class: "mx-auto max-w-[1180px]",
                            SuspenseBoundary {
                                fallback: move |_| rsx! {
                                    div { class: "flex justify-center py-14",
                                        div { class: "flex items-center gap-3 rounded-2xl border border-border bg-panel px-5 py-3 text-sm font-medium text-foreground-muted",
                                            Spinner { class: "h-5 w-5 animate-spin" }
                                            "Loading workspace"
                                        }
                                    }
                                },
                                // Per-route error boundary: a handler in any
                                // route (e.g. a `use_loader?` that returns
                                // Forbidden for a Viewer) bubbles up here
                                // instead of tearing down the whole app
                                // shell. The sidebar and topbar stay usable
                                // so the user can navigate away.
                                ErrorBoundary {
                                    handle_error: move |_ctx: ErrorContext| rsx! {
                                        PageErrorFallback {}
                                    },
                                    Outlet::<Route> {}
                                }
                            }
                        }
                    }
                }
            } else {
                div { class: "flex min-h-screen items-center justify-center" }
            }
        }
    }
}

#[component]
fn PageErrorFallback() -> Element {
    let t: ConsoleTranslate = use_translate();

    let on_reload = move |_| {
        #[cfg(not(feature = "server"))]
        {
            if let Some(win) = web_sys::window() {
                let _ = win.location().reload();
            }
        }
    };

    rsx! {
        div { class: "py-10",
            SectionCard {
                AlertMessage {
                    variant: AlertVariant::Error,
                    {t.page_error_body}
                }
                div { class: "mt-5 flex items-center justify-between gap-3",
                    h2 { class: "font-display text-lg font-bold tracking-tight text-foreground",
                        {t.page_error_title}
                    }
                    Btn {
                        variant: BtnVariant::Primary,
                        onclick: on_reload,
                        {t.page_error_retry}
                    }
                }
            }
        }
    }
}

#[component]
fn MobileTopBar(on_open: EventHandler<MouseEvent>) -> Element {
    let t: ConsoleTranslate = use_translate();
    let account_ctx = use_account_context();
    let enterprise_name = account_ctx()
        .enterprise_name()
        .unwrap_or_else(|| "Biyard".to_string());

    rsx! {
        header { class: "sticky top-0 z-20 flex items-center gap-3 border-b border-border bg-background/95 px-4 py-3 backdrop-blur lg:hidden",
            button {
                class: "flex h-10 w-10 shrink-0 items-center justify-center rounded-xl border border-border bg-panel text-foreground transition-colors hover:bg-panel-muted",
                "aria-label": t.open_sidebar,
                onclick: move |e| on_open.call(e),
                // Hamburger icon — three stacked lines. Kept inline here
                // because it's only used in this topbar; extracting to
                // a shared icon would be premature.
                svg {
                    class: "h-5 w-5",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    line {
                        x1: "3",
                        y1: "6",
                        x2: "21",
                        y2: "6",
                    }
                    line {
                        x1: "3",
                        y1: "12",
                        x2: "21",
                        y2: "12",
                    }
                    line {
                        x1: "3",
                        y1: "18",
                        x2: "21",
                        y2: "18",
                    }
                }
            }
            div { class: "flex min-w-0 items-center gap-2",
                div { class: "flex h-8 w-8 shrink-0 items-center justify-center rounded-lg bg-brand text-sm font-bold text-brand-contrast",
                    "B"
                }
                p { class: "truncate font-display text-sm font-bold tracking-tight text-foreground",
                    "{enterprise_name}"
                }
            }
        }
    }
}

#[component]
fn SidebarShellFallback(collapsed: bool) -> Element {
    let lg_width = if collapsed {
        "lg:w-16"
    } else {
        "lg:w-[17rem]"
    };
    let lg_padding = if collapsed {
        "lg:py-4 lg:px-0"
    } else {
        "lg:px-4 lg:py-5"
    };
    let aside_class = format!(
        "fixed inset-y-0 left-0 z-40 hidden w-[17rem] flex-col border-r border-sidebar-border bg-sidebar px-4 py-5 text-sidebar-foreground lg:flex lg:translate-x-0 lg:transition-[width,padding] {lg_width} {lg_padding}"
    );

    rsx! {
        aside { class: "{aside_class}",
            if collapsed {
                div { class: "flex flex-col items-center gap-3 px-2",
                    div { class: "h-10 w-10 animate-pulse rounded-xl bg-white/10" }
                    div { class: "h-9 w-9 animate-pulse rounded-xl bg-white/6" }
                }
            } else {
                div { class: "mb-6 flex items-center justify-between gap-2 px-1",
                    div { class: "flex min-w-0 items-center gap-3",
                        div { class: "h-10 w-10 shrink-0 animate-pulse rounded-xl bg-white/10" }
                        div { class: "min-w-0 flex-1 space-y-2",
                            div { class: "h-4 w-28 animate-pulse rounded bg-white/10" }
                            div { class: "h-3 w-20 animate-pulse rounded bg-white/6" }
                        }
                    }
                    div { class: "h-8 w-8 shrink-0 animate-pulse rounded-lg bg-white/6" }
                }
            }

            div { class: if collapsed { "mt-4 flex-1 space-y-3 px-2" } else { "flex-1 space-y-3" },
                for _ in 0..6 {
                    div { class: "h-10 animate-pulse rounded-2xl bg-white/6" }
                }
            }

            div { class: if collapsed { "px-2 pt-4" } else { "pt-4" },
                div { class: "h-12 animate-pulse rounded-2xl bg-white/8" }
            }
        }
    }
}
