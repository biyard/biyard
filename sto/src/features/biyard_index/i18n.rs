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
}
