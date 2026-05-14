use dioxus_translate::{Translator, translate};

translate! {
    IssuersTranslate;

    page_title: { en: "Issuers", ko: "발행사" },
    page_subtitle: {
        en: "Browse Korean tokenized-securities (STO) issuers. Click an issuer to see its issued assets.",
        ko: "국내 조각투자·STO 발행사를 한눈에 확인하세요. 발행사를 누르면 해당 발행사가 발행한 자산 목록을 볼 수 있습니다.",
    },
    breadcrumb: { en: "Issuers", ko: "발행사" },

    section_assets: { en: "Issued assets", ko: "발행 자산" },
    count_unit: { en: "items", ko: "건" },
    empty_assets: { en: "No assets registered yet.", ko: "아직 등록된 발행 자산이 없습니다." },

    sandbox_label: { en: "Sandbox", ko: "샌드박스" },

    load_failed: { en: "Failed to load.", ko: "정보를 불러오지 못했습니다" },
    back_to_list: { en: "← Back to issuers", ko: "← 발행사 목록으로" },
    loading: { en: "Loading", ko: "불러오는 중" },
}
