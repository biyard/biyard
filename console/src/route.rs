use crate::common::ProjectPartition;
use crate::features::accounts::pages::{InviteAccept, SignIn, SignUp};
use crate::features::console::layout::Layout as ConsoleLayout;
use crate::features::console::pages::{ApiDocs, Dashboard, Settings};
use crate::features::credentials::pages::Credentials;
use crate::features::enterprises::pages::{EnterpriseGeneralPage, MembersPage};
use crate::features::projects::pages::{
    ProjectCreate, ProjectDetail, ProjectDetailLayout, ProjectEdit, ProjectPoints, ProjectSettings,
    ProjectToken, ProjectTreasury, Projects, TokenCreate, TokenEdit,
};
use dioxus::prelude::*;

// NOTE: URL shape follows the IA locked in CLAUDE.md:
//   - Enterprise scope: /enterprise/...
//   - Personal scope:   /account/...
//   - Brand scope:      /projects/:project_id/... (canonical code name)
//   - Token is 1:1 per project, so the URL segment is singular: /token/...
//
// Rust variant names are preserved where possible to minimize churn on
// `Route::Foo {}` call sites. URL paths are what moved.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // Public pages (no sidebar, no auth)
    #[route("/docs/api")]
    ApiDocs {},

    // Auth (no sidebar, public)
    #[route("/signin")]
    SignIn {},
    #[route("/signup")]
    SignUp {},
    #[route("/invite/:token")]
    InviteAccept { token: String },

    // Console (sidebar + auth guard)
    #[layout(ConsoleLayout)]
        // ── Index / legacy redirects ────────────────────────────
        #[route("/")]
        IndexRedirect {},
        #[route("/dashboard")]
        LegacyDashboardRedirect {},
        #[route("/settings")]
        LegacySettingsRedirect {},
        #[route("/credentials")]
        LegacyCredentialsRedirect {},
        #[route("/enterprise/members")]
        LegacyMembersRedirect {},

        // ── Enterprise scope ────────────────────────────────────
        #[route("/enterprise/overview")]
        Dashboard {},
        #[route("/enterprise/settings/general")]
        EnterpriseGeneralPage {},
        #[route("/enterprise/settings/members")]
        MembersPage {},
        #[route("/enterprise/settings/api-keys")]
        Credentials {},

        // ── Personal scope ──────────────────────────────────────
        #[route("/account/profile")]
        Settings {},

        // ── Brand (project) listing & creation ──────────────────
        #[route("/projects")]
        Projects {},
        #[route("/projects/new")]
        ProjectCreate {},

        // ── Brand full-page editors (no detail layout) ──────────
        #[route("/projects/:project_id/edit")]
        ProjectEdit { project_id: ProjectPartition },
        #[route("/projects/:project_id/token/new")]
        TokenCreate { project_id: ProjectPartition },
        #[route("/projects/:project_id/token/edit")]
        TokenEdit { project_id: ProjectPartition },

        // ── Legacy plural token URLs → redirect to singular ─────
        #[route("/projects/:project_id/tokens/new")]
        LegacyTokenCreateRedirect { project_id: ProjectPartition },
        #[route("/projects/:project_id/tokens/edit")]
        LegacyTokenEditRedirect { project_id: ProjectPartition },

        // ── Bare project URL → redirect to overview ─────────────
        #[route("/projects/:project_id")]
        ProjectIndexRedirect { project_id: ProjectPartition },

        // ── Brand detail pages (shared header + tab nav) ────────
        #[nest("/projects/:project_id")]
            #[layout(ProjectDetailLayout)]
                #[route("/overview")]
                ProjectDetail { project_id: ProjectPartition },
                #[route("/token")]
                ProjectToken { project_id: ProjectPartition },
                #[route("/points")]
                ProjectPoints { project_id: ProjectPartition },
                #[route("/treasury")]
                ProjectTreasury { project_id: ProjectPartition },
                #[route("/settings")]
                ProjectSettings { project_id: ProjectPartition },
            #[end_layout]
        #[end_nest]
    #[end_layout]

    #[route("/:..rest")]
    PageNotFound { rest: Vec<String> },
}

/// `/` lands on the enterprise overview (was `/dashboard`).
#[component]
fn IndexRedirect() -> Element {
    let nav = use_navigator();
    use_effect(move || {
        nav.replace(Route::Dashboard {});
    });
    rsx! {}
}

/// Legacy `/dashboard` → `/enterprise/overview`.
#[component]
fn LegacyDashboardRedirect() -> Element {
    let nav = use_navigator();
    use_effect(move || {
        nav.replace(Route::Dashboard {});
    });
    rsx! {}
}

/// Legacy `/settings` → `/account/profile`.
#[component]
fn LegacySettingsRedirect() -> Element {
    let nav = use_navigator();
    use_effect(move || {
        nav.replace(Route::Settings {});
    });
    rsx! {}
}

/// Legacy `/credentials` → `/enterprise/settings/api-keys`.
#[component]
fn LegacyCredentialsRedirect() -> Element {
    let nav = use_navigator();
    use_effect(move || {
        nav.replace(Route::Credentials {});
    });
    rsx! {}
}

/// Legacy `/enterprise/members` → `/enterprise/settings/members`.
#[component]
fn LegacyMembersRedirect() -> Element {
    let nav = use_navigator();
    use_effect(move || {
        nav.replace(Route::MembersPage {});
    });
    rsx! {}
}

/// Bare `/projects/:project_id` → `/projects/:project_id/overview`.
#[component]
fn ProjectIndexRedirect(project_id: ReadSignal<ProjectPartition>) -> Element {
    let nav = use_navigator();
    use_effect(move || {
        nav.replace(Route::ProjectDetail {
            project_id: project_id(),
        });
    });
    rsx! {}
}

/// Legacy plural token URL `/projects/:project_id/tokens/new`.
#[component]
fn LegacyTokenCreateRedirect(project_id: ReadSignal<ProjectPartition>) -> Element {
    let nav = use_navigator();
    use_effect(move || {
        nav.replace(Route::TokenCreate {
            project_id: project_id(),
        });
    });
    rsx! {}
}

/// Legacy plural token URL `/projects/:project_id/tokens/edit`.
#[component]
fn LegacyTokenEditRedirect(project_id: ReadSignal<ProjectPartition>) -> Element {
    let nav = use_navigator();
    use_effect(move || {
        nav.replace(Route::TokenEdit {
            project_id: project_id(),
        });
    });
    rsx! {}
}

#[component]
fn PageNotFound(rest: Vec<String>) -> Element {
    rsx! {
        div { class: "flex items-center justify-center min-h-screen",
            h1 { class: "text-2xl", "Page not found" }
        }
    }
}
