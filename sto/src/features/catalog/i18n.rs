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
    nav_market: { en: "Listings", ko: "공모 자산" },
    nav_issuers: { en: "Issuers", ko: "발행사" },
    nav_index: { en: "Index", ko: "신뢰지수" },
    nav_launchpad: { en: "Launchpad", ko: "런치패드" },
    nav_news: { en: "News", ko: "공시·뉴스" },
    nav_pricing: { en: "Pricing", ko: "요금제" },

    // 홈
    home_subtitle: {
        en: "Korean STO / fractional investment information platform — DART filings + Musicow catalog.",
        ko: "국내 조각투자·STO 자산을 한곳에서. 금감원 전자공시(DART)와 발행사 자료를 정리해 보여드립니다.",
    },
    section_recent: { en: "Recently issued", ko: "최근 공모 발행" },
    section_offerings: { en: "Open offerings", ko: "진행 중인 공모" },
    section_more: { en: "View all →", ko: "전체 보기" },
    section_category_scale: { en: "By underlying asset", ko: "기초자산별 발행 현황" },
    offerings_label: { en: "Sponsored · Sample", ko: "증권사 등록 · 샘플" },
    offerings_disclaimer: {
        en: "These cards are samples. In production, issuers and brokers register their own offerings; Biyard does not recommend or rate any of them.",
        ko: "위 카드는 샘플 화면입니다. 실제 운영 시에는 발행사·증권사가 직접 등록한 공모 정보가 노출되며, 본 서비스는 별도의 추천이나 등급을 부여하지 않습니다.",
    },
    issuer: { en: "Issuer", ko: "발행" },
    underwriter: { en: "Underwriter", ko: "주관" },
    subscription_end: { en: "Subscription end", ko: "청약 마감" },
    raise_amount: { en: "Raise", ko: "공모 규모" },

    // 카탈로그
    catalog_title: { en: "Listings", ko: "공모 자산" },
    catalog_subtitle: {
        en: "Browse all listed STOs by category, region, status, and issuer.",
        ko: "발행된 모든 조각투자·STO 자산을 카테고리·지역·상태·발행사로 살펴보세요.",
    },
    filter_all: { en: "All", ko: "전체" },
    filter_kr: { en: "🇰🇷 Korea", ko: "🇰🇷 국내" },
    filter_global: { en: "🌍 Global", ko: "🌍 해외" },
    filter_status_all: { en: "All status", ko: "전체 상태" },
    filter_issuer_all: { en: "All issuers", ko: "전체 발행사" },
    search_placeholder: { en: "Search by name or underlying", ko: "종목명·기초자산 검색" },
    page_size_label: { en: "Per page", ko: "표시 개수" },
    results_count: { en: "Results", ko: "검색 결과" },
    showing: { en: "showing", ko: "표시" },
    unit_count: { en: "items", ko: "건" },

    // 상태 (모집 단계 기준 — 한국 자본시장 관행)
    status_issued: { en: "Subscribed", ko: "모집 완료" },
    status_filed: { en: "Open", ko: "모집 진행" },
    status_withdrawn: { en: "Withdrawn", ko: "모집 철회" },

    // 카테고리
    cat_real_estate: { en: "🏢 Real estate", ko: "🏢 부동산" },
    cat_art: { en: "🎨 Art", ko: "🎨 미술품" },
    cat_music: { en: "🎵 Music IP", ko: "🎵 음악 저작권" },
    cat_livestock: { en: "🐄 Livestock", ko: "🐄 축산" },
    cat_luxury: { en: "💎 Luxury", ko: "💎 명품" },
    cat_infra: { en: "⚡ Infra", ko: "⚡ 인프라" },
    cat_content: { en: "🎬 Content", ko: "🎬 콘텐츠 IP" },

    // 테이블 헤더
    th_asset: { en: "Asset", ko: "종목" },
    th_underlying: { en: "Underlying", ko: "기초자산" },
    th_category: { en: "Category", ko: "유형" },
    th_issuer: { en: "Issuer", ko: "발행사" },
    th_status: { en: "Status", ko: "모집 상태" },
    th_issued_at: { en: "Filed", ko: "신고일" },

    // 상세
    detail_breadcrumb_home: { en: "Home", ko: "홈" },
    detail_breadcrumb_market: { en: "Listings", ko: "공모 자산" },
    detail_section_info: { en: "Issuance information", ko: "발행 개요" },
    detail_section_offering: { en: "Offering details", ko: "공모 조건" },
    detail_section_structure: { en: "Issuance structure", ko: "공모 구조" },
    detail_section_links: { en: "External links", ko: "공식 페이지" },
    detail_section_sources: { en: "Sources", ko: "근거 자료" },
    detail_section_issuer: { en: "Issuer", ko: "발행사" },
    detail_section_filings: { en: "Filings", ko: "관련 공시" },
    detail_external_origin: { en: "Open original ↗", ko: "원문 보기" },
    detail_no_external: { en: "No external link", ko: "등록된 외부 링크가 없습니다" },
    detail_filing_view_origin: { en: "View original ↗", ko: "공시 원문 보기" },

    // 필드
    field_asset_name: { en: "Asset name", ko: "종목명" },
    field_underlying: { en: "Underlying", ko: "기초자산" },
    field_security_type: { en: "Security type", ko: "증권 유형" },
    field_issued_at: { en: "Filed", ko: "신고일" },
    field_status: { en: "Status", ko: "모집 상태" },
    field_artist: { en: "Artist", ko: "작가" },
    field_rights_category: { en: "Rights category", ko: "권리 유형" },
    field_trust_no: { en: "Trust no.", ko: "신탁계약 번호" },
    field_year: { en: "Year", ko: "제작연도" },
    field_amount: { en: "Amount raised", ko: "공모총액" },
    field_unit_price: { en: "Unit price", ko: "공모가" },
    field_total_units: { en: "Total units", ko: "발행 수량" },
    field_subscription: { en: "Subscription period", ko: "청약 기간" },
    field_issuer: { en: "Issuer", ko: "발행사" },
    field_trustee: { en: "Trustee", ko: "신탁업자" },
    field_role: { en: "Role", ko: "역할" },
    field_underwriter: { en: "Underwriter", ko: "주관 증권사" },
    field_custody: { en: "Account custody", ko: "계좌관리기관" },

    // 로딩·에러
    loading: { en: "Loading...", ko: "불러오는 중" },
    load_failed: { en: "Failed to load", ko: "정보를 불러오지 못했습니다" },
    not_found: { en: "—", ko: "—" },
}
