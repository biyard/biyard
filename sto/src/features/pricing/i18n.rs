use dioxus_translate::{Translator, translate};

translate! {
    PricingTranslate;

    page_title: { en: "Free for individuals, licensed for organisations", ko: "개인은 무료, 기업은 라이선스" },
    page_subtitle: {
        en: "Filing-based information is free for investors; data and features are offered to issuers, trustees, brokers, and other institutional customers under a license.",
        ko: "투자자에게는 공시 기반 정보를 무료로 공개하고, 발행사·신탁사·증권사 등 기관 고객에게는 라이선스 형태로 데이터와 기능을 제공합니다.",
    },

    track_individual_badge: { en: "Individual", ko: "개인" },
    track_individual_title: { en: "Individual investors · free", ko: "개인 투자자 · 무료" },
    track_individual_desc: {
        en: "All information needed to find and compare offerings is free and open. No payment or sign-up required.",
        ko: "공모 자산을 찾고 비교하는 데 필요한 정보는 모두 무료로 공개합니다. 별도 결제나 가입 없이 누구나 사용할 수 있습니다.",
    },
    track_individual_feat1: {
        en: "Search offerings · filter by category and issuer",
        ko: "공모 자산 검색·카테고리·발행사별 필터",
    },
    track_individual_feat2: {
        en: "Detailed view of issuance structure and filings",
        ko: "발행 구조·공시 자료 상세 보기",
    },
    track_individual_feat3: {
        en: "Direct links to external filings and issuer pages",
        ko: "외부 원문 공시·발행사 페이지 바로가기",
    },
    track_individual_feat4: {
        en: "Biyard Index lookup (after official launch)",
        ko: "Biyard 신뢰지수 조회 (정식 출시 후)",
    },

    track_org_badge: { en: "Organisation", ko: "기관" },
    track_org_title: { en: "Organisation license · contact us", ko: "기관 라이선스 · 문의" },
    track_org_desc: {
        en: "A data + tooling bundle for issuers, brokers, and asset managers. Read demand signals before issuance and embed information into your own channels.",
        ko: "발행사·증권사·운용사를 위한 데이터·도구 묶음입니다. 발행 전 시장 반응을 미리 보고, 자체 채널에 정보를 임베드할 수 있습니다.",
    },
    track_org_feat1: {
        en: "Investor interest & search-trend data",
        ko: "투자자 관심도·검색 트렌드 데이터",
    },
    track_org_feat2: {
        en: "Biyard Launchpad brand-token API",
        ko: "Biyard Launchpad 브랜드 토큰 API",
    },
    track_org_feat3: {
        en: "Priority Biyard Index assessment",
        ko: "Biyard 신뢰지수 평가 우선 적용",
    },
    track_org_feat4: {
        en: "Featured offering slots on the main screen (advertising)",
        ko: "메인 화면 공모 노출 슬롯 (광고)",
    },
}
