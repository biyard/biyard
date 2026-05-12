use dioxus_translate::{Translator, translate};

translate! {
    CatalogTranslate;

    // 공통
    app_title: {
        en: "Biyard STO",
        ko: "Biyard STO",
    },

    // 네비게이션
    nav_home: { en: "Home", ko: "홈" },
    nav_market: { en: "STO Market", ko: "STO 시장" },
    nav_issuers: { en: "Issuers", ko: "발행사" },
    nav_index: { en: "Index", ko: "평가지표" },
    nav_launchpad: { en: "Launchpad", ko: "런치패드" },
    nav_news: { en: "News", ko: "뉴스" },
    nav_pricing: { en: "Pricing", ko: "가격" },

    // 홈
    home_subtitle: {
        en: "Korean STO / fractional investment information platform — DART filings + Musicow catalog.",
        ko: "국내 STO·조각투자 정보 플랫폼. DART 공시 + 뮤직카우 카탈로그 통합.",
    },
    section_recent: { en: "Recently issued", ko: "최근 발행 STO" },
    section_offerings: { en: "Open / upcoming offerings", ko: "공모 진행·예정" },
    section_more: { en: "View all →", ko: "전체 보기 →" },
    section_category_scale: { en: "Issuance by category", ko: "카테고리별 발행 건수" },
    offerings_label: { en: "Sponsored · Example", ko: "증권사 제공 정보 · 예시" },
    offerings_disclaimer: {
        en: "These cards are demo data. In production, issuers/brokers provide the listings directly; Biyard does not recommend or rate them.",
        ko: "위 카드는 데모용 예시 데이터입니다. 실제 서비스에서는 발행사·증권사가 직접 제공한 공모 정보를 표시하며, Biyard 는 추천·평가하지 않습니다.",
    },
    issuer: { en: "Issuer", ko: "발행" },
    underwriter: { en: "Underwriter", ko: "인수" },
    subscription_end: { en: "Subscription end", ko: "청약" },
    raise_amount: { en: "Raise", ko: "모집" },

    // 카탈로그
    catalog_title: { en: "STO Market", ko: "STO 시장" },
    filter_all: { en: "All", ko: "전체" },
    filter_kr: { en: "🇰🇷 Korea", ko: "🇰🇷 한국" },
    filter_global: { en: "🌍 Global", ko: "🌍 해외" },
    filter_status_all: { en: "All status", ko: "모든 상태" },
    filter_issuer_all: { en: "All issuers", ko: "모든 발행사" },
    search_placeholder: { en: "Search by name / artist", ko: "자산명·아티스트 검색" },

    // 상태
    status_issued: { en: "Issued", ko: "발행완료" },
    status_filed: { en: "Filed", ko: "신고중" },
    status_withdrawn: { en: "Withdrawn", ko: "철회" },

    // 카테고리
    cat_real_estate: { en: "🏢 Real estate", ko: "🏢 부동산" },
    cat_art: { en: "🎨 Art", ko: "🎨 미술품" },
    cat_music: { en: "🎵 Music IP", ko: "🎵 음악 IP" },
    cat_livestock: { en: "🐄 Livestock", ko: "🐄 한우·축산" },
    cat_luxury: { en: "💎 Luxury", ko: "💎 명품" },
    cat_infra: { en: "⚡ Infra", ko: "⚡ 인프라" },
    cat_content: { en: "🎬 Content", ko: "🎬 콘텐츠" },

    // 테이블 헤더
    th_asset: { en: "Asset", ko: "자산명" },
    th_category: { en: "Category", ko: "카테고리" },
    th_issuer: { en: "Issuer", ko: "발행사" },
    th_status: { en: "Status", ko: "상태" },
    th_issued_at: { en: "Issued", ko: "발행일" },

    // 상세
    detail_breadcrumb_home: { en: "Home", ko: "홈" },
    detail_breadcrumb_market: { en: "STO Market", ko: "STO 시장" },
    detail_section_info: { en: "Issuance information", ko: "발행 정보" },
    detail_section_offering: { en: "Offering details", ko: "모집 정보" },
    detail_section_structure: { en: "Issuance structure", ko: "발행 구조" },
    detail_section_links: { en: "External links", ko: "외부 링크" },
    detail_section_sources: { en: "Sources", ko: "출처" },
    detail_section_issuer: { en: "Issuer", ko: "발행사" },
    detail_section_filings: { en: "Filings", ko: "공시 자료" },
    detail_external_origin: { en: "Open original ↗", ko: "원본 페이지 ↗" },
    detail_no_external: { en: "No external link", ko: "외부 링크 없음" },
    detail_filing_view_origin: { en: "View original ↗", ko: "원본 공시 ↗" },

    // 필드
    field_asset_name: { en: "Asset name", ko: "자산명" },
    field_underlying: { en: "Underlying", ko: "기초자산" },
    field_security_type: { en: "Security type", ko: "증권 종류" },
    field_issued_at: { en: "Issued at", ko: "발행일" },
    field_status: { en: "Status", ko: "상태" },
    field_artist: { en: "Artist", ko: "작가" },
    field_rights_category: { en: "Rights category", ko: "권리 종류" },
    field_trust_no: { en: "Trust no.", ko: "신탁번호" },
    field_year: { en: "Year", ko: "제작·발매" },
    field_amount: { en: "Amount raised", ko: "모집 총액" },
    field_unit_price: { en: "Unit price", ko: "1주당 발행가" },
    field_total_units: { en: "Total units", ko: "총 발행 수량" },
    field_subscription: { en: "Subscription period", ko: "청약 기간" },
    field_issuer: { en: "Issuer", ko: "발행사" },
    field_trustee: { en: "Trustee / custodian", ko: "신탁·보관기관" },
    field_role: { en: "Role", ko: "역할" },
    field_underwriter: { en: "Underwriter", ko: "인수인" },
    field_custody: { en: "Account custody", ko: "계좌관리" },

    // 로딩·에러
    loading: { en: "Loading...", ko: "로딩 중..." },
    load_failed: { en: "Failed to load", ko: "데이터 로드 실패" },
    not_found: { en: "Not found", ko: "없음" },
}
