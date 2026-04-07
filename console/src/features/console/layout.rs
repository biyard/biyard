use dioxus::prelude::*;

use crate::Route;
use crate::common::ui::Spinner;
use crate::features::accounts::context::use_account_context;
use crate::features::console::components::{ConsoleSidebar, SidebarCollapsed, SidebarOpen};
use crate::features::console::i18n::ConsoleTranslate;
use dioxus_translate::use_translate;

#[component]
pub fn Layout() -> Element {
    let account_ctx = use_account_context();
    let nav = use_navigator();

    if !account_ctx().is_logged_in() {
        nav.push(Route::SignIn {});
        return rsx! {};
    }

    // Shared sidebar state:
    //  - `SidebarCollapsed` is the persisted desktop preference (icon
    //    rail vs full expanded). Only takes effect at `lg:` and above.
    //  - `SidebarOpen` is the ephemeral mobile drawer state. Defaults
    //    closed; toggled by the topbar hamburger. Ignored at `lg:` and
    //    above because the sidebar is permanently visible there.
    let collapsed = use_context_provider(|| Signal::new(SidebarCollapsed(false)));
    let mut sidebar_open = use_context_provider(|| Signal::new(SidebarOpen(false)));

    // Main content left padding:
    //  - Mobile (< lg): 0, because the sidebar is an overlay drawer.
    //  - `lg:` and up: reserve the sidebar rail width (16px icon or
    //    272px expanded), matching the persisted `collapsed` value.
    let lg_padding_class = if collapsed().0 { "lg:pl-16" } else { "lg:pl-[17rem]" };

    rsx! {
        div { class: "min-h-screen bg-background text-foreground",
            div { class: "pointer-events-none fixed inset-0 overflow-hidden",
                div { class: "absolute right-[-7rem] top-[-7rem] h-80 w-80 rounded-full bg-brand-soft opacity-70 blur-3xl" }
                div { class: "absolute bottom-[-10rem] left-[22rem] h-96 w-96 rounded-full bg-info-soft opacity-60 blur-3xl" }
            }

            ConsoleSidebar {}

            div { class: "relative min-h-screen pl-0 {lg_padding_class} transition-[padding] duration-150",
                // Mobile-only topbar: hamburger + compact logo. Hidden
                // at `lg:` and above because the permanent sidebar
                // already gives the user a way to navigate there.
                MobileTopBar {
                    on_open: move |_| sidebar_open.set(SidebarOpen(true)),
                }

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
                            Outlet::<Route> {}
                        }
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
        header {
            class: "sticky top-0 z-20 flex items-center gap-3 border-b border-border bg-background/95 px-4 py-3 backdrop-blur lg:hidden",
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
                    line { x1: "3", y1: "6", x2: "21", y2: "6" }
                    line { x1: "3", y1: "12", x2: "21", y2: "12" }
                    line { x1: "3", y1: "18", x2: "21", y2: "18" }
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
