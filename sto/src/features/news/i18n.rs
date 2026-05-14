use dioxus_translate::{Translator, translate};

translate! {
    NewsTranslate;

    page_title: { en: "Filings & news", ko: "공시·뉴스" },
    page_subtitle: {
        en: "New filings registered by issuers and related news coverage, gathered in one place.",
        ko: "발행사가 새로 등록한 공시와 관련 보도를 모아 보여드립니다.",
    },
    empty_body: {
        en: "We're preparing the filings & news feed. Soon you'll see DART filings and major-outlet coverage automatically aggregated here.",
        ko: "공시·보도 피드를 준비 중입니다. 곧 금감원 전자공시(DART)와 주요 매체 보도를 자동으로 모아 보여드릴 예정입니다.",
    },
}
