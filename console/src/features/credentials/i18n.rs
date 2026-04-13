use dioxus_translate::{Translator, translate};

translate! {
    CredentialsTranslate;

    title: {
        en: "API Keys",
        ko: "API 키",
    },
    description: {
        en: "Manage API keys for accessing Biyard services",
        ko: "Biyard 서비스에 접근하기 위한 API 키를 관리합니다",
    },
    subtitle_in: {
        en: "Manage API keys for {enterprise}.",
        ko: "{enterprise}의 API 키를 관리합니다.",
    },
    create_new: {
        en: "Create New API Key",
        ko: "새 API 키 생성",
    },
    name: {
        en: "Name",
        ko: "이름",
    },
    name_placeholder: {
        en: "My API Key",
        ko: "내 API 키",
    },
    api_key: {
        en: "API Key",
        ko: "API 키",
    },
    created_at: {
        en: "Created At",
        ko: "생성일",
    },
    status: {
        en: "Status",
        ko: "상태",
    },
    actions: {
        en: "Actions",
        ko: "작업",
    },
    active: {
        en: "Active",
        ko: "활성",
    },
    inactive: {
        en: "Inactive",
        ko: "비활성",
    },
    loading: {
        en: "Loading...",
        ko: "로딩 중...",
    },
    no_credentials: {
        en: "No API keys yet",
        ko: "API 키가 없습니다",
    },
    copy: {
        en: "Copy",
        ko: "복사",
    },
    cancel: {
        en: "Cancel",
        ko: "취소",
    },
    close: {
        en: "Close",
        ko: "닫기",
    },
    generate_key: {
        en: "Generate Key",
        ko: "키 생성",
    },
    key_generated: {
        en: "API Key Generated",
        ko: "API 키가 생성되었습니다",
    },
    key_generated_warning: {
        en: "Please copy this key now. You won't be able to see it again!",
        ko: "이 키는 지금 복사해 두세요. 다시는 확인할 수 없습니다.",
    },
}
