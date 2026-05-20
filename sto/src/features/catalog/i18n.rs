use dioxus_translate::{Translate, Translator, translate};

/// 발행 개요 그리드 필드.
#[derive(Translate, Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverviewField {
    #[translate(en = "Asset name", ko = "자산명")]
    AssetName,
    #[translate(en = "Underlying", ko = "기초자산")]
    Underlying,
    #[translate(en = "Security type", ko = "증권 유형")]
    SecurityType,
    #[translate(en = "Filed", ko = "신고일")]
    FiledAt,
    #[translate(en = "Status", ko = "모집 상태")]
    Status,
    // Music
    #[translate(en = "Artist", ko = "작가")]
    Artist,
    #[translate(en = "Rights category", ko = "권리 유형")]
    RightsCategory,
    #[translate(en = "Trust no.", ko = "신탁계약 번호")]
    TrustNo,
    #[translate(en = "Year", ko = "제작연도")]
    Year,
    // Real estate
    #[translate(en = "Address", ko = "소재지")]
    Address,
    #[translate(en = "Building type", ko = "건물 유형")]
    BuildingType,
    #[translate(en = "Floor area", ko = "연면적")]
    FloorArea,
    #[translate(en = "Land area", ko = "대지면적")]
    LandArea,
    #[translate(en = "Floors", ko = "건물 규모")]
    Floors,
    #[translate(en = "Completed", ko = "준공일")]
    CompletionDate,
    #[translate(en = "Tenant", ko = "임차인")]
    Tenant,
    #[translate(en = "Lease term", ko = "임대기간")]
    LeaseTerm,
    #[translate(en = "Total offering", ko = "모집총액")]
    TotalOffering,
    #[translate(en = "Total units", ko = "발행 수량")]
    TotalUnitsStr,
    #[translate(en = "Unit price", ko = "1좌당 가액")]
    UnitPriceStr,
    #[translate(en = "Upfront fee", ko = "총수수료")]
    UpfrontFee,
    #[translate(en = "Dividend freq.", ko = "배당 주기")]
    DividendFrequency,
    // Art
    #[translate(en = "Artist", ko = "작가")]
    ArtArtist,
    #[translate(en = "Year created", ko = "제작연도")]
    ArtworkYear,
    #[translate(en = "Medium", ko = "매체")]
    Medium,
    #[translate(en = "Dimensions", ko = "크기")]
    Dimensions,
    // Livestock
    #[translate(en = "Farm", ko = "농장명")]
    FarmName,
    #[translate(en = "Breed", ko = "품종")]
    Breed,
    #[translate(en = "Head count", ko = "마릿수")]
    HeadCount,
}

/// 공모 조건 그리드 필드.
#[derive(Translate, Clone, Copy, Debug, PartialEq, Eq)]
pub enum OfferingField {
    #[translate(en = "Amount raised", ko = "공모총액")]
    Amount,
    #[translate(en = "Unit price", ko = "공모가")]
    UnitPrice,
    #[translate(en = "Total units", ko = "발행 수량")]
    TotalUnits,
    #[translate(en = "Subscription period", ko = "청약 기간")]
    Subscription,
}

/// 공모 구조 그리드 필드.
#[derive(Translate, Clone, Copy, Debug, PartialEq, Eq)]
pub enum IssuanceField {
    #[translate(en = "Issuer", ko = "발행사")]
    Issuer,
    #[translate(en = "Trustee", ko = "신탁업자")]
    Trustee,
    #[translate(en = "Role", ko = "역할")]
    Role,
    #[translate(en = "Underwriter", ko = "주관 증권사")]
    Underwriter,
    #[translate(en = "Account custody", ko = "계좌관리기관")]
    Custody,
}

/// 테이블 헤더 열.
#[derive(Translate, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TableColumn {
    #[translate(en = "Asset / Underlying", ko = "자산명 / 기초자산")]
    AssetUnderlying,
    #[translate(en = "Category", ko = "카테고리")]
    Category,
    #[translate(en = "Issuer", ko = "발행사")]
    Issuer,
    #[translate(en = "Status", ko = "상태")]
    Status,
    #[translate(en = "Filed", ko = "발행")]
    Filed,
}

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

    // 카탈로그 필터 추가
    filter_cat_all: { en: "All categories", ko: "전체 카테고리" },
    filter_kr_label: { en: "🇰🇷 Korea", ko: "🇰🇷 한국" },
    filter_global_label: { en: "🌍 Global", ko: "🌍 해외" },
    filter_issuer_placeholder: { en: "Issuer — all", ko: "발행사 — 전체" },
    filter_status_placeholder: { en: "Status — all", ko: "상태 — 전체" },
    filter_search_placeholder: {
        en: "Search by asset name, issuer, underlying...",
        ko: "자산명·발행사·기초자산 검색...",
    },
    filter_results_count: { en: "{n} results", ko: "검색 결과 {n}건" },

    // 홈 영역
    hero_title: { en: "Offerings — open & upcoming", ko: "공모 진행·예정" },
    hero_disclaimer_pill: { en: "Issuer-registered", ko: "발행사 등록 정보" },
    hero_disclaimer_body: {
        en: "Expected offerings registered by issuers or brokers. Subscription dates and amounts may change; final terms are confirmed on the formal DART filing.",
        ko: "발행사·증권사가 등록한 예상 공모입니다. 청약 일정·모집액은 변경될 수 있으며 발행 시점에 확정됩니다.",
    },
    section_recent_title: { en: "Recently issued STOs", ko: "최신 발행 STO" },
    section_liquidated_title: { en: "Recently liquidated / distributed", ko: "최근 청산·분배 완료" },
    section_more_arrow: { en: "View all →", ko: "전체 보기 →" },
    section_category_scale_title: { en: "Investment volume by category", ko: "카테고리별 투자 규모" },
    section_category_scale_note: {
        en: "Cumulative raise (DART filing values). STOs without disclosed raises are counted but excluded from totals.",
        ko: "누적 모집액 기준 (DART 공시 발행가). 모집액 미공시 STO 는 건수에는 포함되나 합산에서는 제외됨.",
    },
    donut_caption_raise: { en: "Cumulative raise", ko: "누적 모집액" },
    donut_chart_aria: { en: "Pie chart by category", ko: "카테고리별 파이 차트" },
    main_nav_aria: { en: "Primary navigation", ko: "주요 화면" },
    nav_market_short: { en: "Market", ko: "STO 시장" },
    nav_index_short: { en: "Index", ko: "평가지표" },
    nav_launchpad_short: { en: "Launchpad", ko: "런치패드" },
    nav_news_short: { en: "News", ko: "뉴스" },
    nav_pricing_short: { en: "Pricing", ko: "가격" },
    topbar_search_placeholder: { en: "Search STOs...", ko: "STO 검색..." },
    topbar_lang_toggle_aria: { en: "Switch language", ko: "언어 전환" },
    sponsored_pill: { en: "Sponsored · sample", ko: "증권사 등록 · 샘플" },

    // 식별 배너
    identity_index_pill: { en: "BIYARD INDEX", ko: "BIYARD INDEX" },
    identity_index_title: { en: "Web3-native STO rating", ko: "Web3 기반 STO 평가지표" },
    identity_index_body: {
        en: "Converts Web3-native trust signals — on-chain issuance integrity, contract security, wallet distribution, governance — into 6 axes that traditional credit ratings can't capture.",
        ko: "온체인 발행 무결성·컨트랙트 보안·지갑 분포·거버넌스 등 기존 신용평가가 다루지 못하는 Web3 신뢰 신호를 6개 축으로 환산해 등급을 부여합니다.",
    },
    identity_index_cta: { en: "Read the whitepaper →", ko: "백서 보기 →" },
    identity_launchpad_pill: { en: "BIYARD LAUNCHPAD", ko: "BIYARD LAUNCHPAD" },
    identity_launchpad_title: { en: "Brand token PaaS", ko: "브랜드 토큰 PaaS" },
    identity_launchpad_body: {
        en: "Issuance infrastructure for utility tokens that pair with STOs — letting issuers and brokers run their own branded token programs.",
        ko: "STO 와 결합 가능한 유틸리티 토큰 발행 인프라. 발행사·증권사가 자체 브랜드 토큰을 운영할 수 있는 PaaS 서비스를 제공합니다.",
    },
    identity_launchpad_cta: { en: "Learn more →", ko: "자세히 보기 →" },

    // 상세 페이지 보조 라벨
    detail_section_appraisal: { en: "Appraisal values", ko: "감정평가액" },
    detail_filings_title_fmt: { en: "Filings ({n})", ko: "공시 ({n})" },
    detail_no_external_short: { en: "No external link", ko: "원본 링크가 없습니다." },
    detail_external_origin_arrow: { en: "Open original ↗", ko: "원문 보기 ↗" },
    detail_filing_origin_arrow: { en: "Source ↗", ko: "원본 공시 ↗" },
    detail_map_link: { en: "View on map ↗", ko: "지도에서 보기 ↗" },

    // 표시 단위
    unit_won: { en: "KRW", ko: "원" },
    unit_seat: { en: "units", ko: "좌" },
}
