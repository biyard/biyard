use dioxus_translate::{Translator, translate};

translate! {
    ConsoleTranslate;

    dashboard: {
        en: "Overview",
        ko: "개요",
    },
    projects: {
        en: "Brands",
        ko: "브랜드",
    },
    credentials: {
        en: "API Keys",
        ko: "API 키",
    },
    members: {
        en: "Members",
        ko: "멤버",
    },
    settings: {
        en: "Settings",
        ko: "설정",
    },
    nav_section_brand: {
        en: "Brand",
        ko: "브랜드",
    },
    nav_section_enterprise: {
        en: "Enterprise",
        ko: "기업",
    },
    nav_enterprise_overview: {
        en: "Overview",
        ko: "개요",
    },
    nav_enterprise_general: {
        en: "General",
        ko: "일반",
    },
    nav_enterprise_members: {
        en: "Members",
        ko: "멤버",
    },
    nav_enterprise_api_keys: {
        en: "API Keys",
        ko: "API 키",
    },
    nav_brand_treasury: {
        en: "Treasury",
        ko: "트레저리",
    },
    open_sidebar: {
        en: "Open navigation",
        ko: "사이드바 열기",
    },
    brand_switcher_select: {
        en: "Select a brand",
        ko: "브랜드 선택",
    },
    brand_switcher_no_brands: {
        en: "No brands yet",
        ko: "브랜드 없음",
    },
    brand_switcher_view_all: {
        en: "View all brands",
        ko: "전체 브랜드 보기",
    },
    brand_switcher_create_new: {
        en: "Create new brand",
        ko: "새 브랜드 생성",
    },
    enterprise_scope_label: {
        en: "Enterprise",
        ko: "기업",
    },
    brand_scope_label: {
        en: "Brand",
        ko: "브랜드",
    },
    enterprise_overview_title: {
        en: "Enterprise Overview",
        ko: "기업 개요",
    },
    enterprise_overview_subtitle: {
        en: "Brands, API keys, and treasury posture across {enterprise}",
        ko: "{enterprise} 전반의 브랜드, API 키, 트레저리 현황",
    },
    enterprise_summary: {
        en: "Enterprise summary",
        ko: "기업 요약",
    },
    enterprise_ready_headline: {
        en: "{name}, {enterprise} is ready for operator review.",
        ko: "{name}님, {enterprise} 운영 현황을 확인하세요.",
    },
    recent_brands_title: {
        en: "Recent Brands",
        ko: "최근 브랜드",
    },
    recent_brands_desc: {
        en: "Your most recent brands and treasury posture.",
        ko: "최근 브랜드와 트레저리 상태를 확인합니다.",
    },
    some_brands_inactive: {
        en: "Some brands are inactive. Review treasury and launch settings before the next payout window.",
        ko: "비활성 상태의 브랜드가 있습니다. 다음 정산 주기 전에 트레저리와 출시 설정을 확인해주세요.",
    },
    sign_out: {
        en: "Sign Out",
        ko: "로그아웃",
    },
    biyard_console: {
        en: "Biyard Console",
        ko: "Biyard 콘솔",
    },
    tagline: {
        en: "Blockchain Token & Point Management Platform",
        ko: "블록체인 토큰 & 포인트 관리 플랫폼",
    },
    welcome: {
        en: "Welcome to Biyard Console",
        ko: "Biyard 콘솔에 오신 것을 환영합니다",
    },
    welcome_description: {
        en: "Manage your brands, credentials, and account settings.",
        ko: "브랜드, 인증정보, 계정 설정을 관리하세요.",
    },
    my_account: {
        en: "My Account",
        ko: "내 계정",
    },
    quick_actions: {
        en: "Quick Actions",
        ko: "빠른 작업",
    },
    my_projects: {
        en: "My Brands",
        ko: "내 브랜드",
    },
    my_projects_desc: {
        en: "Create and manage your blockchain brands",
        ko: "블록체인 브랜드 생성 및 관리",
    },
    api_credentials: {
        en: "API Keys",
        ko: "API 키",
    },
    api_credentials_desc: {
        en: "Manage API keys for accessing Biyard services",
        ko: "Biyard 서비스 접근용 API 키를 관리합니다",
    },
    account_settings: {
        en: "Account Settings",
        ko: "계정 설정",
    },
    account_settings_desc: {
        en: "View and manage your account",
        ko: "계정 확인 및 관리",
    },
    account_info: {
        en: "Account Information",
        ko: "계정 정보",
    },
    name: {
        en: "Name",
        ko: "이름",
    },
    email: {
        en: "Email",
        ko: "이메일",
    },
    account_id: {
        en: "Account ID",
        ko: "계정 ID",
    },
    created_at: {
        en: "Created At",
        ko: "가입일",
    },
    profile: {
        en: "Profile",
        ko: "프로필",
    },
    profile_subtitle: {
        en: "Your personal account details and preferences.",
        ko: "내 계정 정보와 개인 설정입니다.",
    },
    display_name: {
        en: "Display name",
        ko: "표시 이름",
    },
    display_name_help: {
        en: "Shown in the sidebar, menus, and anywhere your account appears.",
        ko: "사이드바, 메뉴, 그리고 계정이 표시되는 모든 곳에 사용됩니다.",
    },
    email_readonly_help: {
        en: "Email is your login identity. Contact support to change it.",
        ko: "이메일은 로그인 식별자입니다. 변경은 지원팀에 문의해 주세요.",
    },
    save_profile: {
        en: "Save profile",
        ko: "프로필 저장",
    },
    profile_saved: {
        en: "Profile updated.",
        ko: "프로필이 저장되었습니다.",
    },
    security: {
        en: "Security",
        ko: "보안",
    },
    danger_zone: {
        en: "Danger Zone",
        ko: "위험 영역",
    },
    delete_account: {
        en: "Delete Account",
        ko: "계정 삭제",
    },
    delete_account_desc: {
        en: "This action cannot be undone. All your data will be permanently deleted.",
        ko: "이 작업은 되돌릴 수 없습니다. 모든 데이터가 영구적으로 삭제됩니다.",
    },
    delete_account_confirm: {
        en: "Confirm Account Deletion",
        ko: "계정 삭제 확인",
    },
    delete_account_warning: {
        en: "This action cannot be undone. All your data will be permanently deleted.",
        ko: "이 작업은 되돌릴 수 없습니다. 모든 데이터가 영구적으로 삭제됩니다.",
    },
    cancel: {
        en: "Cancel",
        ko: "취소",
    },
    confirm_delete: {
        en: "Confirm",
        ko: "확인",
    },
    loading: {
        en: "Loading...",
        ko: "로딩 중...",
    },
    language: {
        en: "Language",
        ko: "언어",
    },
    theme: {
        en: "Theme",
        ko: "테마",
    },
    theme_dark: {
        en: "Dark",
        ko: "다크",
    },
    theme_light: {
        en: "Light",
        ko: "라이트",
    },
}
