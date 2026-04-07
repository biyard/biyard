use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::common::OrganizationRole;
use crate::common::components::dialog::*;
use crate::common::ui::*;
use crate::features::console::i18n::ConsoleTranslate;
use crate::features::enterprises::EnterpriseTranslate;
use crate::features::enterprises::MemberResponse;

use super::invite_modal::InviteModal;

/// Owner / Admin / Viewer view of all current members + pending invites.
/// Mutation actions (invite, change role, remove) are gated by role
/// inside this component to keep the layout consistent for everyone.
#[component]
pub fn MembersPage() -> Element {
    let t: EnterpriseTranslate = use_translate();
    let console_t: ConsoleTranslate = use_translate();

    let mut show_invite = use_signal(|| false);
    let mut show_remove_confirm = use_signal(|| None::<MemberResponse>);
    let mut error = use_signal(|| None::<String>);
    let mut success = use_signal(|| None::<String>);

    let mut members = use_loader(move || async move {
        crate::features::enterprises::controllers::list_members_handler().await
    })?;
    let mut invitations = use_loader(move || async move {
        crate::features::enterprises::controllers::list_invitations_handler().await
    })?;

    let members_data = members();
    let invitations_data = invitations();
    let owner_count = members_data
        .iter()
        .filter(|m| matches!(m.role, OrganizationRole::Owner))
        .count();

    rsx! {
        div { class: "space-y-8",
            PageHeader {
                title: t.members_title.to_string(),
                subtitle: t.members_subtitle.to_string(),
                scope: PageScope::Workspace,
                workspace_label: console_t.enterprise_scope_label.to_string(),
                brand_label: console_t.brand_scope_label.to_string(),
                actions: rsx! {
                    Btn {
                        variant: BtnVariant::Primary,
                        onclick: move |_| show_invite.set(true),
                        {t.invite_member}
                    }
                },
            }

            if let Some(msg) = error() {
                AlertMessage { variant: AlertVariant::Error, "{msg}" }
            }
            if let Some(msg) = success() {
                AlertMessage { variant: AlertVariant::Success, "{msg}" }
            }

            // Pending invitations table.
            SectionCard {
                SectionTitle { {t.pending_invitations} }
                if invitations_data.is_empty() {
                    p { class: "text-sm text-foreground-muted", {t.no_pending_invitations} }
                } else {
                    DataTable {
                        TableHead {
                            TableHeadCell { {t.email} }
                            TableHeadCell { {t.role} }
                            TableHeadCell { {t.expires_at} }
                            TableHeadCell { {t.actions} }
                        }
                        TableBody {
                            for inv in invitations_data.iter() {
                                {
                                    let token = inv.token.clone();
                                    let email = inv.invited_email.clone();
                                    let role_label = role_label(inv.role, &t);
                                    let expires = format_timestamp(inv.expires_at);
                                    rsx! {
                                        tr { class: "border-b border-border",
                                            TableCell { "{email}" }
                                            TableCell { "{role_label}" }
                                            TableCell { "{expires}" }
                                            TableCell {
                                                Btn {
                                                    variant: BtnVariant::Secondary,
                                                    onclick: move |_| {
                                                        let token = token.clone();
                                                        spawn(async move {
                                                            let res = crate::features::enterprises::controllers::revoke_invitation_handler(token).await;
                                                            match res {
                                                                Ok(_) => invitations.restart(),
                                                                Err(e) => error.set(Some(e.to_string())),
                                                            }
                                                        });
                                                    },
                                                    {t.revoke}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Members table.
            SectionCard {
                SectionTitle { {t.member_list} }
                if members_data.is_empty() {
                    EmptyState {
                        icon: rsx! { IconMembers { class: "h-8 w-8" } },
                        title: t.no_members.to_string(),
                        description: t.no_members_desc.to_string(),
                    }
                } else {
                    DataTable {
                        TableHead {
                            TableHeadCell { {t.name} }
                            TableHeadCell { {t.email} }
                            TableHeadCell { {t.role} }
                            TableHeadCell { {t.joined_at} }
                            TableHeadCell { {t.actions} }
                        }
                        TableBody {
                            for member in members_data.iter() {
                                {
                                    let m = member.clone();
                                    let role_label = role_label(member.role, &t);
                                    let joined = format_timestamp(member.joined_at);
                                    let is_last_owner = matches!(member.role, OrganizationRole::Owner) && owner_count <= 1;
                                    rsx! {
                                        tr { class: "border-b border-border",
                                            TableCell { "{member.name}" }
                                            TableCell { "{member.email}" }
                                            TableCell {
                                                StatusBadge {
                                                    color: role_badge_color(member.role),
                                                    "{role_label}"
                                                }
                                            }
                                            TableCell { "{joined}" }
                                            TableCell {
                                                if is_last_owner {
                                                    span { class: "text-xs text-foreground-muted",
                                                        "Last owner"
                                                    }
                                                } else {
                                                    Btn {
                                                        variant: BtnVariant::Danger,
                                                        onclick: move |_| show_remove_confirm.set(Some(m.clone())),
                                                        {t.remove}
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        InviteModal {
            open: show_invite(),
            on_close: move |_| show_invite.set(false),
            on_success: move |_| {
                show_invite.set(false);
                invitations.restart();
                success.set(Some("Invitation created.".to_string()));
            },
        }

        // Remove member confirmation dialog.
        DialogRoot {
            open: show_remove_confirm().is_some(),
            on_open_change: move |v: bool| {
                if !v {
                    show_remove_confirm.set(None);
                }
            },
            DialogContent {
                DialogTitle { {t.confirm_remove_title} }
                DialogDescription { {t.confirm_remove_message} }
                if let Some(target) = show_remove_confirm() {
                    p { class: "mt-3 rounded-2xl border border-border bg-panel-muted px-4 py-3 text-sm font-semibold text-foreground",
                        "{target.name} ({target.email})"
                    }
                }
                DialogActions {
                    Btn {
                        variant: BtnVariant::Secondary,
                        onclick: move |_| show_remove_confirm.set(None),
                        {t.cancel}
                    }
                    Btn {
                        variant: BtnVariant::Danger,
                        onclick: move |_| {
                            if let Some(target) = show_remove_confirm() {
                                let account_id = target.account_id.clone();
                                show_remove_confirm.set(None);
                                spawn(async move {
                                    let res = crate::features::enterprises::controllers::remove_member_handler(account_id).await;
                                    match res {
                                        Ok(_) => {
                                            members.restart();
                                            success.set(Some("Member removed.".to_string()));
                                        }
                                        Err(e) => error.set(Some(e.to_string())),
                                    }
                                });
                            }
                        },
                        {t.remove}
                    }
                }
            }
        }
    }
}

fn role_label(role: OrganizationRole, t: &EnterpriseTranslate) -> &'static str {
    match role {
        OrganizationRole::Owner => t.role_owner,
        OrganizationRole::Admin => t.role_admin,
        OrganizationRole::Viewer => t.role_viewer,
    }
}

fn role_badge_color(role: OrganizationRole) -> BadgeColor {
    match role {
        OrganizationRole::Owner => BadgeColor::Purple,
        OrganizationRole::Admin => BadgeColor::Blue,
        OrganizationRole::Viewer => BadgeColor::Gray,
    }
}
