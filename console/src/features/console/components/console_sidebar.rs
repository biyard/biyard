use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::Route;
use crate::app::ThemeIsDark;
use crate::common::ProjectPartition;
use crate::common::ui::{
    IconBuilding, IconChevronDown, IconChevronLeft, IconChevronRight, IconCredentials,
    IconDashboard, IconFolderOpen, IconGlobe, IconLock, IconLogout, IconMembers, IconMoon,
    IconPlus, IconProjects, IconSettings, IconStar, IconSun, IconToken,
};
use crate::features::accounts::context::use_account_context;
use crate::features::console::i18n::ConsoleTranslate;
use crate::features::projects::i18n::ProjectsTranslate;

/// Shared sidebar collapsed state. Wrapped in a newtype so multiple
/// `Signal<bool>` contexts don't collide.
///
/// This is a **desktop preference** only — at `lg:` and above the
/// sidebar can be collapsed to an icon rail. Below `lg:` the sidebar
/// is a drawer and always renders in its full expanded form when open,
/// regardless of this value.
///
/// In-memory only: the preference resets to expanded on every page
/// load. We intentionally do not persist this to localStorage/cookies
/// because the collapse state is a low-stakes session preference and
/// SSR-time persistence isn't worth the hydration flicker.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarCollapsed(pub bool);

/// Mobile drawer open state. Below `lg:` the sidebar is an off-canvas
/// drawer that slides in from the left when this is true. At `lg:`
/// and above this value is effectively ignored — the sidebar is
/// permanently visible via `lg:translate-x-0`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SidebarOpen(pub bool);

#[component]
pub fn ConsoleSidebar() -> Element {
    let t: ConsoleTranslate = use_translate();
    let projects_t: ProjectsTranslate = use_translate();
    let route: Route = use_route();
    let account_ctx = use_account_context();
    let account = account_ctx().account.clone();
    let account_name = account
        .as_ref()
        .map(|acc| acc.name.clone())
        .unwrap_or_else(|| "Account".to_string());
    let account_email = account
        .as_ref()
        .map(|acc| acc.email.clone())
        .unwrap_or_default();
    let initials = account_name
        .split_whitespace()
        .filter_map(|part| part.chars().next())
        .take(2)
        .collect::<String>()
        .to_uppercase();

    // Enterprise display — primary identity at the top of the sidebar.
    // Falls back to "Biyard" only when the enterprise has not yet loaded
    // (e.g. during the very first request of a fresh session).
    let enterprise_name = account_ctx()
        .enterprise_name()
        .unwrap_or_else(|| "Biyard".to_string());
    let enterprise_initial = enterprise_name
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "B".to_string());
    let mut menu_open = use_signal(|| false);
    let mut brand_switcher_open = use_signal(|| false);

    let mut collapsed_ctx = use_context::<Signal<SidebarCollapsed>>();
    let mut sidebar_open_ctx = use_context::<Signal<SidebarOpen>>();
    let sidebar_open = sidebar_open_ctx().0;
    // When the mobile drawer is open we always want the full expanded
    // sidebar content, regardless of the user's persisted desktop
    // collapse preference. On desktop `sidebar_open` stays false so
    // this collapses back to the raw `collapsed` value.
    let collapsed = collapsed_ctx().0 && !sidebar_open;

    let toggle_collapsed = move |_| {
        let next = !collapsed_ctx().0;
        collapsed_ctx.set(SidebarCollapsed(next));
    };

    let close_drawer = move |_| sidebar_open_ctx.set(SidebarOpen(false));

    // Load brands for the switcher dropdown. Wrapped in Ok() so the
    // layout never suspends on transient errors — the sidebar should
    // render even if the brand list is empty.
    let brands_loader = use_loader(move || async move {
        Ok::<_, crate::common::Error>(
            crate::features::projects::controllers::list_projects_handler(50, None)
                .await
                .unwrap_or_default(),
        )
    })?;
    let brands_list = brands_loader();
    let brands_items = brands_list.items.clone();

    // Resolve the current project id from the route for brand-scoped
    // sub-nav and switcher highlighting. All project-scoped variants
    // contribute, including editors and legacy redirects.
    let current_project_id: Option<ProjectPartition> = match &route {
        Route::ProjectDetail { project_id }
        | Route::ProjectPoints { project_id }
        | Route::ProjectSettings { project_id }
        | Route::ProjectToken { project_id }
        | Route::ProjectTreasury { project_id }
        | Route::ProjectEdit { project_id }
        | Route::TokenCreate { project_id }
        | Route::TokenEdit { project_id }
        | Route::ProjectIndexRedirect { project_id }
        | Route::LegacyTokenCreateRedirect { project_id }
        | Route::LegacyTokenEditRedirect { project_id } => Some(project_id.clone()),
        _ => None,
    };
    let in_brand_scope = current_project_id.is_some();
    let current_brand_name = current_project_id
        .as_ref()
        .and_then(|pid| {
            let id_str = pid.to_string();
            brands_items
                .iter()
                .find(|p| p.id == id_str)
                .map(|p| p.name.clone())
        })
        .unwrap_or_default();

    // Sidebar visual shell. On mobile (< lg) it is a drawer:
    //   - Always rendered at full drawer width (`w-[17rem]`) regardless
    //     of `collapsed` (see `effective_collapsed` above).
    //   - Translated off-screen by default, slid in when `sidebar_open`.
    // At `lg:` and above it is the permanent rail: `lg:translate-x-0`
    // overrides the mobile translate, and the width flips between
    // `lg:w-16` and `lg:w-[17rem]` based on the user's desktop
    // `collapsed` preference.
    let mobile_transform = if sidebar_open {
        "translate-x-0"
    } else {
        "-translate-x-full"
    };
    // Use the raw (persisted) collapsed preference for the desktop
    // width class, not `effective_collapsed`, because the latter is
    // forced to false whenever the mobile drawer is open.
    let lg_width = if collapsed_ctx().0 {
        "lg:w-16"
    } else {
        "lg:w-[17rem]"
    };
    // Padding on mobile is always the expanded form so the drawer
    // content is roomy. At `lg:` it follows `effective_collapsed`.
    let lg_padding = if collapsed {
        "lg:py-4 lg:px-0"
    } else {
        "lg:px-4 lg:py-5"
    };
    let aside_class = format!(
        "fixed inset-y-0 left-0 z-40 flex w-[17rem] flex-col border-r border-sidebar-border bg-sidebar px-4 py-5 text-sidebar-foreground transition-transform duration-200 {mobile_transform} lg:translate-x-0 lg:transition-[width,padding] {lg_width} {lg_padding}"
    );

    rsx! {
        // Backdrop for the mobile drawer. Hidden on desktop via
        // `lg:hidden`; invisible unless the drawer is open. Click to
        // dismiss.
        if sidebar_open {
            div {
                class: "fixed inset-0 z-30 bg-black/60 backdrop-blur-sm lg:hidden",
                "aria-hidden": "true",
                onclick: close_drawer,
            }
        }

        aside { class: "{aside_class}",

            // ── Header: enterprise identity + collapse toggle ──────
            // The primary label is the current enterprise name. Product
            // branding ("Biyard Console") is kept as a subdued subtitle
            // so users always know which workspace they're looking at.
            if collapsed {
                div { class: "flex flex-col items-center gap-3 px-2",
                    div {
                        class: "flex h-10 w-10 items-center justify-center rounded-xl bg-brand text-base font-bold text-brand-contrast",
                        title: "{enterprise_name}",
                        "{enterprise_initial}"
                    }
                    // Expand toggle is a desktop preference; `hidden lg:flex`
                    // keeps it off-screen on mobile where the drawer has
                    // its own close affordance (backdrop click).
                    button {
                        class: "hidden h-9 w-9 items-center justify-center rounded-xl border border-sidebar-border text-sidebar-muted transition-colors hover:bg-white/5 hover:text-sidebar-foreground lg:flex",
                        "aria-label": "Expand sidebar",
                        onclick: toggle_collapsed,
                        IconChevronRight { class: "h-4 w-4" }
                    }
                }
            } else {
                div { class: "mb-6 flex items-center justify-between gap-2 px-1",
                    div { class: "flex min-w-0 items-center gap-3",
                        div { class: "flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-brand text-base font-bold text-brand-contrast",
                            "{enterprise_initial}"
                        }
                        div { class: "min-w-0",
                            p {
                                class: "truncate font-display text-base font-bold tracking-tight text-sidebar-foreground",
                                title: "{enterprise_name}",
                                "{enterprise_name}"
                            }
                            p { class: "text-[10px] font-semibold uppercase tracking-[0.16em] text-sidebar-muted",
                                "Biyard Console"
                            }
                        }
                    }
                    // Mobile: close drawer button (X, lg:hidden).
                    button {
                        class: "flex h-8 w-8 shrink-0 items-center justify-center rounded-lg text-sidebar-muted transition-colors hover:bg-white/5 hover:text-sidebar-foreground lg:hidden",
                        "aria-label": "Close sidebar",
                        onclick: close_drawer,
                        IconChevronLeft { class: "h-4 w-4" }
                    }
                    // Desktop: collapse toggle (hidden lg:flex).
                    button {
                        class: "hidden h-8 w-8 shrink-0 items-center justify-center rounded-lg text-sidebar-muted transition-colors hover:bg-white/5 hover:text-sidebar-foreground lg:flex",
                        "aria-label": "Collapse sidebar",
                        onclick: toggle_collapsed,
                        IconChevronLeft { class: "h-4 w-4" }
                    }
                }
            }

            // ── Navigation ─────────────────────────────────────────
            // BRAND (contextual): switcher + 4 sub-pages only when a
            //   brand is currently selected via the URL.
            // ENTERPRISE (fixed): always visible, never changes.
            nav { class: if collapsed { "flex-1 space-y-4 overflow-y-auto px-2 mt-4" } else { "flex-1 space-y-5 overflow-y-auto" },

                // ── BRAND section ──
                div { class: "space-y-1",
                    if !collapsed {
                        p { class: "px-2 pt-1 pb-2 text-[10px] font-semibold uppercase tracking-[0.18em] text-sidebar-muted",
                            {t.nav_section_brand}
                        }

                        // Brand switcher button (expanded mode only)
                        div { class: "relative",
                            if brand_switcher_open() {
                                BrandSwitcherMenu {
                                    brands: brands_items.clone(),
                                    current_project_id: current_project_id.as_ref().map(|p| p.to_string()),
                                    on_close: move |_| brand_switcher_open.set(false),
                                }
                            }
                            button {
                                class: if in_brand_scope { "flex w-full items-center gap-2.5 rounded-2xl border border-brand/40 bg-brand-soft py-2.5 pl-3 pr-3 text-left text-sm font-semibold text-sidebar-foreground transition-colors hover:bg-brand-soft" } else { "flex w-full items-center gap-2.5 rounded-2xl border border-sidebar-border bg-sidebar-panel py-2.5 pl-3 pr-3 text-left text-sm font-medium text-sidebar-muted transition-colors hover:bg-white/5 hover:text-sidebar-foreground" },
                                onclick: move |_| brand_switcher_open.set(!brand_switcher_open()),
                                div { class: if in_brand_scope { "flex h-8 w-8 shrink-0 items-center justify-center rounded-lg bg-brand text-sm font-bold text-brand-contrast" } else { "flex h-8 w-8 shrink-0 items-center justify-center rounded-lg bg-sidebar-panel text-sidebar-muted" },
                                    if in_brand_scope {
                                        "{current_brand_name.chars().next().unwrap_or('B')}"
                                    } else {
                                        IconProjects { class: "h-4 w-4" }
                                    }
                                }
                                div { class: "min-w-0 flex-1 pr-1",
                                    if in_brand_scope {
                                        p { class: "truncate text-sm font-semibold text-sidebar-foreground",
                                            "{current_brand_name}"
                                        }
                                    } else {
                                        p { class: "truncate text-sm font-medium text-sidebar-muted",
                                            {t.brand_switcher_select}
                                        }
                                    }
                                }
                                span { class: "flex h-6 w-6 shrink-0 items-center justify-center rounded-md text-sidebar-muted",
                                    IconChevronDown { class: "h-4 w-4" }
                                }
                            }
                        }
                    }

                    // Brand sub-nav: only visible in brand scope.
                    if let Some(pid) = current_project_id.clone() {
                        div { class: if collapsed { "space-y-1" } else { "mt-2 space-y-1 pl-1" },
                            NavItem {
                                label: projects_t.overview.to_string(),
                                to: Route::ProjectDetail {
                                    project_id: pid.clone(),
                                },
                                is_active: matches!(route, Route::ProjectDetail { .. }),
                                icon: NavIcon::BrandOverview,
                                collapsed,
                            }
                            NavItem {
                                label: projects_t.tokens.to_string(),
                                to: Route::ProjectToken {
                                    project_id: pid.clone(),
                                },
                                is_active: matches!(
                                    route,
                                    Route::ProjectToken { .. }
                                    | Route::TokenCreate { .. }
                                    | Route::TokenEdit { .. }
                                    | Route::LegacyTokenCreateRedirect { .. }
                                    | Route::LegacyTokenEditRedirect { .. }
                                ),
                                icon: NavIcon::Token,
                                collapsed,
                            }
                            NavItem {
                                label: projects_t.points.to_string(),
                                to: Route::ProjectPoints {
                                    project_id: pid.clone(),
                                },
                                is_active: matches!(route, Route::ProjectPoints { .. }),
                                icon: NavIcon::Points,
                                collapsed,
                            }
                            NavItem {
                                label: t.nav_brand_treasury.to_string(),
                                to: Route::ProjectTreasury {
                                    project_id: pid.clone(),
                                },
                                is_active: matches!(route, Route::ProjectTreasury { .. }),
                                icon: NavIcon::Treasury,
                                collapsed,
                            }
                            NavItem {
                                label: projects_t.settings_tab.to_string(),
                                to: Route::ProjectSettings {
                                    project_id: pid,
                                },
                                is_active: matches!(route, Route::ProjectSettings { .. } | Route::ProjectEdit { .. }),
                                icon: NavIcon::Settings,
                                collapsed,
                            }
                        }
                    }
                }

                // ── ENTERPRISE section ──
                div { class: "space-y-1",
                    if !collapsed {
                        p { class: "px-2 pt-1 pb-2 text-[10px] font-semibold uppercase tracking-[0.18em] text-sidebar-muted",
                            {t.nav_section_enterprise}
                        }
                    }
                    NavItem {
                        label: t.nav_enterprise_overview.to_string(),
                        to: Route::Dashboard {},
                        is_active: matches!(route, Route::Dashboard {}),
                        icon: NavIcon::Dashboard,
                        collapsed,
                    }
                    NavItem {
                        label: t.nav_enterprise_general.to_string(),
                        to: Route::EnterpriseGeneralPage {},
                        is_active: matches!(route, Route::EnterpriseGeneralPage {}),
                        icon: NavIcon::Enterprise,
                        collapsed,
                    }
                    NavItem {
                        label: t.nav_enterprise_members.to_string(),
                        to: Route::MembersPage {},
                        is_active: matches!(route, Route::MembersPage {} | Route::LegacyMembersRedirect {}),
                        icon: NavIcon::Members,
                        collapsed,
                    }
                    NavItem {
                        label: t.nav_enterprise_api_keys.to_string(),
                        to: Route::Credentials {},
                        is_active: matches!(route, Route::Credentials {} | Route::LegacyCredentialsRedirect {}),
                        icon: NavIcon::Credentials,
                        collapsed,
                    }
                }
            }

            // ── Account card ───────────────────────────────────────
            div { class: if collapsed { "mt-4 px-1" } else { "relative mt-3" },
                if !collapsed && menu_open() {
                    AccountMenu { on_close: move |_| menu_open.set(false) }
                }

                if collapsed {
                    div { class: "flex justify-center",
                        button {
                            class: "flex h-10 w-10 items-center justify-center rounded-xl bg-brand text-sm font-bold text-brand-contrast",
                            "aria-label": "Account",
                            onclick: move |_| menu_open.set(!menu_open()),
                            if initials.is_empty() {
                                "BY"
                            } else {
                                "{initials}"
                            }
                        }
                    }
                } else {
                    div { class: "rounded-2xl border border-sidebar-border bg-sidebar-panel p-2.5",
                        div { class: "flex items-center gap-3",
                            div { class: "flex h-9 w-9 shrink-0 items-center justify-center rounded-xl bg-brand text-sm font-bold text-brand-contrast",
                                if initials.is_empty() {
                                    "BY"
                                } else {
                                    "{initials}"
                                }
                            }
                            div { class: "min-w-0 flex-1",
                                p { class: "truncate text-sm font-semibold text-sidebar-foreground",
                                    "{account_name}"
                                }
                                if !account_email.is_empty() {
                                    p { class: "truncate text-xs text-sidebar-muted",
                                        "{account_email}"
                                    }
                                }
                            }
                            button {
                                class: "flex h-8 w-8 items-center justify-center rounded-lg text-sidebar-muted transition-colors hover:bg-white/5 hover:text-sidebar-foreground",
                                "aria-label": "Open account menu",
                                "aria-expanded": menu_open(),
                                onclick: move |_| menu_open.set(!menu_open()),
                                svg {
                                    class: "h-4 w-4",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    circle { cx: "12", cy: "5", r: "1" }
                                    circle { cx: "12", cy: "12", r: "1" }
                                    circle { cx: "12", cy: "19", r: "1" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Brand switcher dropdown: list of brands + "view all" / "create new".
///
/// Clicking a brand navigates to `/projects/:id/overview` preserving
/// the user's "current brand" intent. The menu closes on navigation.
#[component]
fn BrandSwitcherMenu(
    brands: Vec<crate::features::projects::ProjectResponse>,
    current_project_id: Option<String>,
    on_close: EventHandler<()>,
) -> Element {
    let nav = use_navigator();
    let t: ConsoleTranslate = use_translate();
    let mut sidebar_open_ctx = use_context::<Signal<SidebarOpen>>();
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

#[derive(Clone, Copy, PartialEq)]
enum NavIcon {
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
fn NavItem(label: String, to: Route, is_active: bool, icon: NavIcon, collapsed: bool) -> Element {
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

    // Wrap the Link in a div with onclick that closes the mobile
    // drawer. Event bubbles up from the Link's internal <a>, so the
    // router still navigates; the side effect of closing the drawer
    // runs in addition. On desktop `sidebar_open` is already false
    // so this is a no-op.
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
fn NavIconView(icon: NavIcon) -> Element {
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

#[component]
fn AccountMenu(on_close: EventHandler<()>) -> Element {
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
fn MenuAction(
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

    // Theme signal is owned by the root `App` component (`ThemeIsDark`
    // provider) so it survives every remount of this drop-down. The
    // previous attempts (`use_signal` local + `use_root_context`) both
    // either lost state or panicked with `ValueDroppedError` when the
    // menu was closed and reopened.
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
