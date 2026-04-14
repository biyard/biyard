use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::common::ProjectPartition;
use crate::common::ui::{IconFolderOpen, IconPlus};
use crate::features::accounts::context::use_account_context;
use crate::features::console::i18n::ConsoleTranslate;

use super::SidebarOpen;

#[component]
pub(super) fn BrandSwitcherMenu(
    brands: Vec<crate::features::projects::ProjectResponse>,
    current_project_id: Option<String>,
    on_close: EventHandler<()>,
) -> Element {
    let nav = use_navigator();
    let t: ConsoleTranslate = use_translate();
    let mut sidebar_open_ctx = use_context::<Signal<SidebarOpen>>();
    let can_write = use_account_context()().can_write();
    let items: Vec<(String, String)> = brands
        .iter()
        .map(|p| (p.id.clone(), p.name.clone()))
        .collect();

    rsx! {
        div { class: "absolute top-full left-0 right-0 z-20 mt-2 max-h-[22rem] overflow-y-auto rounded-2xl border border-sidebar-border bg-sidebar-panel p-1.5 shadow-[0_18px_40px_rgba(15,23,42,0.25)]",
            if items.is_empty() {
                p { class: "px-3 py-3 text-xs text-sidebar-muted", {t.brand_switcher_no_brands} }
            } else {
                for (id , name) in items.iter() {
                    {
                        let id = id.clone();
                        let name = name.clone();
                        let is_current = current_project_id.as_deref() == Some(id.as_str());
                        let item_class = if is_current {
                            "flex w-full items-center justify-between gap-3 rounded-xl bg-white/5 px-3 py-2 text-left text-sm font-semibold text-sidebar-foreground"
                        } else {
                            "flex w-full items-center justify-between gap-3 rounded-xl px-3 py-2 text-left text-sm font-medium text-sidebar-muted transition-colors hover:bg-white/5 hover:text-sidebar-foreground"
                        };
                        rsx! {
                            button {
                                class: "{item_class}",
                                onclick: {
                                    let id_for_click = id.clone();
                                    move |_| {
                                        on_close.call(());
                                        sidebar_open_ctx.set(SidebarOpen(false));
                                        nav.push(Route::ProjectDetail {
                                            project_id: ProjectPartition::from(id_for_click.clone()),
                                        });
                                    }
                                },
                                span { class: "truncate", "{name}" }
                                if is_current {
                                    span { class: "shrink-0 rounded-full bg-brand-soft px-2 py-0.5 text-[10px] font-semibold uppercase tracking-[0.14em] text-brand",
                                        "Current"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "my-1 border-t border-sidebar-border" }
            button {
                class: "flex w-full items-center gap-2 rounded-xl px-3 py-2 text-left text-sm font-medium text-sidebar-muted transition-colors hover:bg-white/5 hover:text-sidebar-foreground",
                onclick: move |_| {
                    on_close.call(());
                    sidebar_open_ctx.set(SidebarOpen(false));
                    nav.push(Route::Projects {});
                },
                IconFolderOpen { class: "h-4 w-4" }
                span { {t.brand_switcher_view_all} }
            }
            if can_write {
                button {
                    class: "flex w-full items-center gap-2 rounded-xl px-3 py-2 text-left text-sm font-semibold text-brand transition-colors hover:bg-white/5",
                    onclick: move |_| {
                        on_close.call(());
                        sidebar_open_ctx.set(SidebarOpen(false));
                        nav.push(Route::ProjectCreate {});
                    },
                    IconPlus { class: "h-4 w-4" }
                    span { {t.brand_switcher_create_new} }
                }
            }
        }
    }
}
