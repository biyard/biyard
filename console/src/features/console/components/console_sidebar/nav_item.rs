use dioxus::prelude::*;

use crate::Route;
use crate::common::ui::{
    IconBuilding, IconCredentials, IconDashboard, IconFolderOpen, IconLock, IconMembers,
    IconSettings, IconStar, IconToken,
};

use super::SidebarOpen;

#[derive(Clone, Copy, PartialEq)]
pub(super) enum NavIcon {
    Dashboard,
    Credentials,
    Members,
    Enterprise,
    BrandOverview,
    Token,
    Points,
    Treasury,
    Settings,
}

#[component]
pub(super) fn NavItem(
    label: String,
    to: Route,
    is_active: bool,
    icon: NavIcon,
    collapsed: bool,
) -> Element {
    let mut sidebar_open_ctx = use_context::<Signal<SidebarOpen>>();
    let (base, active) = if collapsed {
        (
            "flex h-10 w-10 items-center justify-center rounded-xl text-sidebar-muted transition-colors hover:bg-white/5 hover:text-sidebar-foreground",
            "flex h-10 w-10 items-center justify-center rounded-xl bg-sidebar-panel text-sidebar-foreground",
        )
    } else {
        (
            "flex items-center gap-3 rounded-xl px-3 py-2.5 text-sm font-medium text-sidebar-muted transition-colors hover:bg-white/5 hover:text-sidebar-foreground",
            "flex items-center gap-3 rounded-xl border border-sidebar-border bg-sidebar-panel px-3 py-2.5 text-sm font-semibold text-sidebar-foreground",
        )
    };

    let class = if is_active { active } else { base };

    rsx! {
        div { onclick: move |_| sidebar_open_ctx.set(SidebarOpen(false)),
            Link { class: "{class}", to,
                NavIconView { icon }
                if !collapsed {
                    span { "{label}" }
                }
            }
        }
    }
}

#[component]
pub(super) fn NavIconView(icon: NavIcon) -> Element {
    match icon {
        NavIcon::Dashboard => rsx! {
            IconDashboard { class: "h-5 w-5" }
        },
        NavIcon::Credentials => rsx! {
            IconCredentials { class: "h-5 w-5" }
        },
        NavIcon::Members => rsx! {
            IconMembers { class: "h-5 w-5" }
        },
        NavIcon::Enterprise => rsx! {
            IconBuilding { class: "h-5 w-5" }
        },
        NavIcon::BrandOverview => rsx! {
            IconFolderOpen { class: "h-5 w-5" }
        },
        NavIcon::Token => rsx! {
            IconToken { class: "h-5 w-5" }
        },
        NavIcon::Points => rsx! {
            IconStar { class: "h-5 w-5" }
        },
        NavIcon::Treasury => rsx! {
            IconLock { class: "h-5 w-5" }
        },
        NavIcon::Settings => rsx! {
            IconSettings { class: "h-5 w-5" }
        },
    }
}
