use dioxus_translate::{Translator, translate};

translate! {
    EnterpriseTranslate;

    members_title: {
        en: "Members",
        ko: "멤버",
    },
    members_subtitle: {
        en: "Manage who has access to this enterprise.",
        ko: "이 엔터프라이즈에 접근할 수 있는 사람을 관리합니다.",
    },
    invite_member: {
        en: "Invite Member",
        ko: "멤버 초대",
    },
    invite_modal_title: {
        en: "Invite a new member",
        ko: "새 멤버 초대",
    },
    invite_modal_description: {
        en: "Generate an invite link. Share it with the person you want to add. Anyone with the link can sign up and join with the role you choose.",
        ko: "초대 링크를 생성합니다. 추가하고 싶은 사람에게 공유하세요. 링크를 가진 사람은 누구나 가입하여 선택한 역할로 합류할 수 있습니다.",
    },
    invited_email: {
        en: "Email (for reference)",
        ko: "이메일 (참고용)",
    },
    invited_email_placeholder: {
        en: "person@example.com",
        ko: "person@example.com",
    },
    role: {
        en: "Role",
        ko: "역할",
    },
    role_admin: {
        en: "Admin",
        ko: "관리자",
    },
    role_viewer: {
        en: "Viewer",
        ko: "뷰어",
    },
    role_owner: {
        en: "Owner",
        ko: "소유자",
    },
    create_invite: {
        en: "Create invite",
        ko: "초대 생성",
    },
    creating_invite: {
        en: "Creating...",
        ko: "생성 중...",
    },
    invite_link_label: {
        en: "Invite link (copy and share)",
        ko: "초대 링크 (복사하여 공유)",
    },
    copy_invite_link: {
        en: "Copy",
        ko: "복사",
    },
    copied: {
        en: "Copied",
        ko: "복사됨",
    },
    pending_invitations: {
        en: "Pending invitations",
        ko: "대기 중인 초대",
    },
    no_pending_invitations: {
        en: "No pending invitations.",
        ko: "대기 중인 초대가 없습니다.",
    },
    expires_at: {
        en: "Expires",
        ko: "만료",
    },
    revoke: {
        en: "Revoke",
        ko: "취소",
    },
    member_list: {
        en: "Member list",
        ko: "멤버 목록",
    },
    name: {
        en: "Name",
        ko: "이름",
    },
    email: {
        en: "Email",
        ko: "이메일",
    },
    joined_at: {
        en: "Joined",
        ko: "가입일",
    },
    actions: {
        en: "Actions",
        ko: "작업",
    },
    change_role: {
        en: "Change role",
        ko: "역할 변경",
    },
    remove: {
        en: "Remove",
        ko: "내보내기",
    },
    confirm_remove_title: {
        en: "Remove member",
        ko: "멤버 내보내기",
    },
    confirm_remove_message: {
        en: "This will revoke their access to the enterprise. Their account stays active but will need a new invite to re-join.",
        ko: "이 사용자의 엔터프라이즈 접근 권한을 회수합니다. 계정은 유지되지만 다시 합류하려면 새 초대가 필요합니다.",
    },
    cancel: {
        en: "Cancel",
        ko: "취소",
    },
    close: {
        en: "Close",
        ko: "닫기",
    },
    no_members: {
        en: "No members yet",
        ko: "멤버가 없습니다",
    },
    no_members_desc: {
        en: "Invite teammates to collaborate on this enterprise.",
        ko: "팀원을 초대하여 이 엔터프라이즈에서 협업하세요.",
    },

    // Invite accept page
    accept_title: {
        en: "Join Enterprise",
        ko: "엔터프라이즈 합류",
    },
    accept_subtitle: {
        en: "You have been invited to join",
        ko: "다음 엔터프라이즈에 초대되었습니다",
    },
    accept_role_label: {
        en: "You will join as",
        ko: "다음 역할로 합류합니다",
    },
    accept_create_account: {
        en: "Create account & join",
        ko: "계정 생성 후 합류",
    },
    accept_already_have_account: {
        en: "Have an account already? Sign in first, then ask the inviter to add you again.",
        ko: "이미 계정이 있나요? 먼저 로그인한 후 초대자에게 다시 추가해 달라고 요청하세요.",
    },
    invitation_invalid: {
        en: "This invitation link is invalid or expired.",
        ko: "이 초대 링크는 유효하지 않거나 만료되었습니다.",
    },

    // No enterprise empty state
    no_enterprise_title: {
        en: "You're not part of any enterprise",
        ko: "어떤 엔터프라이즈에도 속해 있지 않습니다",
    },
    no_enterprise_desc: {
        en: "Ask an enterprise admin for an invitation link, or contact support to start a new workspace.",
        ko: "엔터프라이즈 관리자에게 초대 링크를 요청하거나, 새 워크스페이스 생성을 위해 지원팀에 문의하세요.",
    },

    // Enterprise general settings
    general_title: {
        en: "General",
        ko: "일반",
    },
    general_subtitle: {
        en: "Change how this enterprise appears across the console.",
        ko: "이 기업이 콘솔 전반에서 어떻게 표시되는지 설정합니다.",
    },
    enterprise_name: {
        en: "Enterprise name",
        ko: "기업 이름",
    },
    enterprise_name_help: {
        en: "The display name shown in the sidebar, page headers, and invitations.",
        ko: "사이드바, 페이지 헤더, 초대장에 표시되는 이름입니다.",
    },
    save_changes: {
        en: "Save changes",
        ko: "변경사항 저장",
    },
    saving: {
        en: "Saving...",
        ko: "저장 중...",
    },
    settings_saved: {
        en: "Settings saved.",
        ko: "설정이 저장되었습니다.",
    },
    owner_only_edit: {
        en: "Only Owners can change enterprise settings. Ask an owner to make changes.",
        ko: "Owner만 기업 설정을 변경할 수 있습니다. Owner에게 요청해 주세요.",
    },
}
