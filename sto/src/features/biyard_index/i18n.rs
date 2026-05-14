use dioxus_translate::{Translator, translate};

translate! {
    BiyardIndexTranslate;

    eyebrow: { en: "BIYARD INDEX", ko: "BIYARD INDEX" },
    hero_title: {
        en: "Web3-native STO rating index.",
        ko: "토큰증권 시대의\n새로운 평가 기준.",
    },
    hero_sub: {
        en: "Biyard Index measures STOs across six quantitative Web3 axes: on-chain issuance integrity, smart-contract security, holder distribution, governance, market trust, and information freshness. Issuers apply to be rated — it is opt-in.",
        ko: "분산원장 위에서 발행되는 토큰증권을 6가지 정량 지표로 평가합니다. 발행 무결성·컨트랙트 안전성·홀더 분포·거버넌스·유통 신뢰도·정보 갱신까지, 기존 신용평가가 다루기 어려운 영역을 함께 살펴봅니다.",
    },
    apply_cta: { en: "Apply for rating →", ko: "평가 신청하기" },
    whitepaper_cta: { en: "Read whitepaper", ko: "백서 보기" },

    why_eyebrow: { en: "WHY BIYARD INDEX", ko: "왜 필요한가" },
    why_title: { en: "Legacy ratings don't see Web3", ko: "기존 신용평가는 토큰증권을 평가하기 어렵습니다" },
    why_sub: {
        en: "Existing credit ratings focus on balance sheets and disclosures. Biyard Index fills the gap they cannot reach.",
        ko: "재무제표·공시 중심의 기존 평가 방식은 분산원장 위 토큰의 무결성이나 지갑 집중도 같은 정보를 다루지 못합니다. Biyard Index 가 그 부분을 보완합니다.",
    },
    feat_onchain_title: { en: "On-chain first", ko: "온체인 데이터 기반" },
    feat_onchain_body: {
        en: "Verifiable on-chain facts (issuance, transfers, burns) are the primary input. Disclosures are supplementary.",
        ko: "분산원장에 기록된 발행·이전·소각 이력 등 검증 가능한 사실을 우선 사용합니다. 공시 자료는 보조 자료로 활용합니다.",
    },
    feat_contract_title: { en: "Smart contract analysis", ko: "스마트컨트랙트 분석" },
    feat_contract_body: {
        en: "Audit results, upgrade authority, key custody — analysed at the code level.",
        ko: "컨트랙트 감사 결과, 업그레이드 권한, 키 관리 방식까지 코드 수준에서 확인합니다.",
    },
    feat_realtime_title: { en: "Continuous monitoring", ko: "지속 모니터링" },
    feat_realtime_body: {
        en: "Ratings keep updating after issuance — on-chain changes and market signals feed into the score.",
        ko: "발행 이후에도 온체인 변동과 시장 신호를 지속적으로 반영해 점수를 갱신합니다.",
    },

    method_eyebrow: { en: "METHODOLOGY", ko: "평가 방식" },
    method_title: { en: "Six Web3 axes, formula disclosed", ko: "6가지 정량 지표와 공개된 산출식" },
    method_body: {
        en: "Each axis is scored independently and weighted by token-security category, then mapped to S/A/B/C/D. Full formulas, signals, and validation are in the whitepaper.",
        ko: "각 지표는 독립적으로 산출하고, 자산 유형별 가중치를 적용해 S·A·B·C·D 등급으로 환산합니다. 산출식과 검증 절차는 백서에서 모두 공개합니다.",
    },
    axis_01: { en: "Contract security", ko: "컨트랙트 보안" },
    axis_02: { en: "On-chain issuance integrity", ko: "발행 무결성" },
    axis_03: { en: "Holder distribution", ko: "홀더 분포" },
    axis_04: { en: "Market trust", ko: "유통 신뢰도" },
    axis_05: { en: "Governance", ko: "거버넌스" },
    axis_06: { en: "Information freshness", ko: "정보 갱신" },

    preview_score_caption: {
        en: "Security tokens · top 18%",
        ko: "토큰증권 부문 · 상위 18%",
    },
    preview_score_pill: {
        en: "PREVIEW · not a real score",
        ko: "PREVIEW · 실제 산출 아님",
    },

    // Whitepaper
    wp_breadcrumb_root: { en: "Biyard Index", ko: "Biyard Index" },
    wp_breadcrumb_doc: { en: "Whitepaper", ko: "백서" },
    wp_coming_soon_title: { en: "Whitepaper coming soon", ko: "백서 추후 공개" },
    wp_coming_soon_desc: {
        en: "The Biyard Index whitepaper is being prepared and will be released soon.",
        ko: "Biyard Index 백서는 준비 중이며 곧 공개될 예정입니다.",
    },
    wp_version_pill: { en: "v0.1 · DRAFT", ko: "v0.1 · DRAFT" },
    wp_draft_meta: { en: "Drafting · 2026.05", ko: "작성 중 · 2026.05" },
    wp_title: { en: "Biyard Index whitepaper", ko: "Biyard Index 백서" },
    wp_intro: {
        en: "This document describes Biyard Index's design principles, axis definitions, scoring formulas, and validation procedures. Biyard Index is a Web3-native indicator for security tokens that quantifies on-chain issuance integrity, smart-contract security, holder distribution, governance, and other dimensions that traditional credit ratings cannot capture.",
        ko: "본 문서는 Biyard Index 의 설계 원칙·축 정의·산출식·검증 절차를 정리한 기술 문서입니다. Biyard Index 는 토큰증권 (Security Token) 을 평가하기 위해 설계된 Web3 전용 지표이며, 기존 신용평가가 다루지 못하는 온체인 발행 무결성·스마트컨트랙트 보안·지갑 분포·거버넌스 등을 정량 축으로 환산합니다.",
    },
    wp_sec1_title: { en: "1. Overview", ko: "1. 개요" },
    wp_sec1_body_lead: {
        en: "Biyard Index quantifies the trust profile of distributed-ledger-based security tokens. Issuer balance sheets and disclosures stay with traditional credit ratings; Biyard Index covers ",
        ko: "Biyard Index 는 분산원장 기반 토큰증권의 신뢰도를 정량적으로 평가하는 표준 지표입니다. 발행사 재무제표·증권신고서 같은 전통 정보는 기존 신용평가의 영역이며, Biyard Index 는 이들이 다루지 못하는 ",
    },
    wp_sec1_body_emph: {
        en: "Web3-native trust signals",
        ko: "Web3 고유 신뢰 신호",
    },
    wp_sec1_body_tail: {
        en: " they cannot reach.",
        ko: " 를 측정합니다.",
    },
    wp_sec2_title: { en: "2. Six evaluation axes", ko: "2. 6개 평가 축" },
    wp_axis1_name: { en: "Contract security", ko: "컨트랙트 보안" },
    wp_axis1_body: {
        en: " — token contract audit history, vulnerability findings, upgrade authority, key management.",
        ko: " — 토큰 컨트랙트 감사 이력·취약점 발견·업그레이드 권한·키 관리",
    },
    wp_axis2_name: { en: "On-chain issuance integrity", ko: "온체인 발행 무결성" },
    wp_axis2_body: {
        en: " — issued supply matches trust assets 1:1, arbitrary mint/burn authority, event-log consistency.",
        ko: " — 발행 수량 = 신탁 자산 1:1 일치, 임의 발행·소각 권한, 이벤트 로그 일관성",
    },
    wp_axis3_name: { en: "Holder distribution", ko: "지갑 분포·집중도" },
    wp_axis3_body: {
        en: " — top-N wallet share, issuer/operator self-holdings.",
        ko: " — 상위 N 지갑 비중, 발행사·운영사 자체 보유 비중",
    },
    wp_axis4_name: { en: "Market trust", ko: "유통 신뢰성" },
    wp_axis4_body: {
        en: " — exchange volume, bid-ask spread, wash-trading signals.",
        ko: " — 거래소 거래량·호가 스프레드·Wash trading 시그널",
    },
    wp_axis5_name: { en: "Governance", ko: "거버넌스" },
    wp_axis5_body: {
        en: " — upgrade procedure, multi-sig, timelocks, operator independence.",
        ko: " — 업그레이드 절차·멀티시그·타임락·운영 주체 독립성",
    },
    wp_axis6_name: { en: "Information freshness", ko: "정보 신뢰성" },
    wp_axis6_body: {
        en: " — update timeliness, dispute-response cycle, cross-channel consistency.",
        ko: " — 정보 갱신 적시성·이의 제기 응답·외부 채널 일관성",
    },
    wp_sec3_title: { en: "3. Grade conversion", ko: "3. 등급 환산" },
    wp_sec3_body: {
        en: "Composite scores are converted to five grades based on within-type percentile rank.",
        ko: "종합 점수는 유형 내 백분위를 기준으로 다섯 단계 등급으로 환산됩니다.",
    },
    wp_th_grade: { en: "Grade", ko: "등급" },
    wp_th_percentile: { en: "Percentile", ko: "백분위" },
    wp_th_desc: { en: "Description", ko: "설명" },
    wp_grade_s_pct: { en: "Top 5%", ko: "상위 5%" },
    wp_grade_s_desc: { en: "Clearly above average across all axes", ko: "모든 축에서 평균을 크게 상회" },
    wp_grade_a_pct: { en: "Top 5–25%", ko: "상위 5~25%" },
    wp_grade_a_desc: { en: "Above average across most axes", ko: "대부분 축에서 평균 상회" },
    wp_grade_b_pct: { en: "Top 25–60%", ko: "상위 25~60%" },
    wp_grade_b_desc: { en: "Around the type average", ko: "유형 평균 수준" },
    wp_grade_c_pct: { en: "Top 60–85%", ko: "상위 60~85%" },
    wp_grade_c_desc: { en: "Below average on some axes", ko: "일부 축에서 평균 미달" },
    wp_grade_d_pct: { en: "Bottom 15%", ko: "하위 15%" },
    wp_grade_d_desc: { en: "Significantly below average on most axes", ko: "다수 축에서 평균 크게 미달" },
    wp_sec4_title: { en: "4. Disclaimer", ko: "4. 면책 조항" },
    wp_sec4_lead: {
        en: "Biyard Index is ",
        ko: "Biyard Index 는 자본시장법상 ",
    },
    wp_sec4_emph: {
        en: "not a credit rating agency, investment advisory, or financial investment business",
        ko: "신용평가업·투자자문업·금융투자업이 아닙니다",
    },
    wp_sec4_tail: {
        en: " under Korea's Capital Markets Act. Grades and scores published here are informational, based on on-chain data and disclosures; they are not buy recommendations or price targets for any specific security token. Investment decisions remain the user's responsibility.",
        ko: ". 본 지표의 등급·점수는 온체인 데이터 및 공시 자료 기반 정보 제공 목적이며, 특정 토큰증권에 대한 투자 권유·매수 추천·목표가 제시가 아닙니다. 투자 결정은 사용자 본인의 책임입니다.",
    },
    wp_back_to_product: { en: "← Back to product page", ko: "← 제품 페이지로" },
}
