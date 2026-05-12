use dioxus_translate::{Translator, translate};

translate! {
    BiyardIndexTranslate;

    eyebrow: { en: "BIYARD INDEX", ko: "BIYARD INDEX" },
    hero_title: {
        en: "Web3-native STO rating index.",
        ko: "토큰증권 시대의 Web3 기반 평가 지표.",
    },
    hero_sub: {
        en: "Biyard Index measures STOs across six quantitative Web3 axes: on-chain issuance integrity, smart-contract security, holder distribution, governance, market trust, and information freshness. Issuers apply to be rated — it is opt-in.",
        ko: "Biyard Index 는 분산원장 위 토큰증권을 평가하기 위해 설계된 Web3 전용 지표입니다. 온체인 데이터·스마트컨트랙트 보안·지갑 분포·거버넌스 등 기존 신용평가가 다루지 못하는 영역을 6개 축으로 환산해 등급을 부여합니다.",
    },
    apply_cta: { en: "Apply for rating →", ko: "평가 신청하기 →" },
    whitepaper_cta: { en: "Read whitepaper", ko: "백서 보기" },

    why_eyebrow: { en: "WHY BIYARD INDEX", ko: "WHY BIYARD INDEX" },
    why_title: { en: "Legacy ratings don't see Web3", ko: "기존 평가는 Web3 를 못 본다" },
    why_sub: {
        en: "Existing credit ratings focus on balance sheets and disclosures. Biyard Index fills the gap they cannot reach.",
        ko: "재무제표·공시 중심의 기존 신용평가는 온체인 발행 구조·컨트랙트 보안·지갑 분포를 평가하지 못합니다. 그 빈 자리를 채우는 지표입니다.",
    },
    feat_onchain_title: { en: "On-chain first", ko: "온체인 우선 평가" },
    feat_onchain_body: {
        en: "Verifiable on-chain facts (issuance, transfers, burns) are the primary input. Disclosures are supplementary.",
        ko: "분산원장에 기록된 발행 수량·소유 이전·소각 이력 등 검증 가능한 온체인 사실을 주 입력값으로 사용합니다. 공시는 보조 자료입니다.",
    },
    feat_contract_title: { en: "Smart contract analysis", ko: "스마트컨트랙트 분석" },
    feat_contract_body: {
        en: "Audit results, upgrade authority, key custody — analysed at the code level.",
        ko: "토큰 컨트랙트 감사 결과·업그레이드 권한 구조·키 관리 방식까지 평가에 반영합니다. 코드 레벨에서 평가합니다.",
    },
    feat_realtime_title: { en: "Continuous monitoring", ko: "실시간 모니터링" },
    feat_realtime_body: {
        en: "Ratings keep updating after issuance — on-chain changes and market signals feed into the score.",
        ko: "발행 후에도 온체인 변화·거래소 시장 신호를 지속적으로 추적해 등급에 반영합니다. 한 번 산출하고 끝이 아닙니다.",
    },

    method_eyebrow: { en: "METHODOLOGY", ko: "METHODOLOGY" },
    method_title: { en: "Six Web3 axes, formula disclosed", ko: "Web3 6개 축, 산출식 전면 공개" },
    method_body: {
        en: "Each axis is scored independently and weighted by token-security category, then mapped to S/A/B/C/D. Full formulas, signals, and validation are in the whitepaper.",
        ko: "온체인 데이터를 중심으로 6개 축이 독립적으로 산출되며, 카테고리별 가중치를 적용해 종합 등급 (S/A/B/C/D) 으로 환산합니다. 상세 산출식·신호 정의·검증 절차는 백서에 정리되어 있습니다.",
    },
    axis_01: { en: "Contract security", ko: "컨트랙트 보안" },
    axis_02: { en: "On-chain issuance integrity", ko: "온체인 발행 무결성" },
    axis_03: { en: "Holder distribution", ko: "지갑 분포·집중도" },
    axis_04: { en: "Market trust", ko: "유통 신뢰성" },
    axis_05: { en: "Governance", ko: "거버넌스" },
    axis_06: { en: "Information freshness", ko: "정보 신뢰성" },
}
