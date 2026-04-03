use dioxus_translate::{Translator, translate};

// Navigation
translate! {
    NavTranslate;

    about: {
        en: "About",
        ko: "About",
    },
    solution: {
        en: "Solution",
        ko: "Solution",
    },
    showcase: {
        en: "Showcase",
        ko: "Showcase",
    },
    faq: {
        en: "FAQ",
        ko: "FAQ",
    },
    console: {
        en: "Console",
        ko: "Console",
    },
}

// Hero section
translate! {
    HeroTranslate;

    badge: {
        en: "Biyard Launchpad",
        ko: "Biyard Launchpad",
    },
    headline_1: {
        en: "Revenue becomes ",
        ko: "매출이 ",
    },
    headline_accent: {
        en: "the value of tokens",
        ko: "토큰의 가치가 되는",
    },
    headline_2: {
        en: "We're building that era.",
        ko: "시대를 만듭니다.",
    },
    subtitle: {
        en: "With a token economy linked to real revenue, customers become shareholders and brands grow together. Biyard provides the most transparent decentralized infrastructure connecting businesses and consumers.",
        ko: "실제 매출에 연동된 토큰 이코노미로, 고객은 주주가 되고 브랜드는 함께 성장합니다. Biyard는 기업과 소비자를 연결하는 가장 투명한 분산형 인프라를 제공합니다.",
    },
    cta_console: {
        en: "Get Started with Console",
        ko: "Console 시작하기",
    },
    cta_learn: {
        en: "Learn More",
        ko: "자세히 알아보기",
    },
}

// About section
translate! {
    AboutTranslate;

    section_label: {
        en: "The Problem",
        ko: "The Problem",
    },
    heading_1: {
        en: "of the token market ",
        ko: "토큰 시장의 ",
    },
    heading_accent: {
        en: "80% fail",
        ko: "80%는 실패",
    },
    heading_2: {
        en: ".",
        ko: "합니다.",
    },
    avg_token_life: {
        en: "Average lifespan of existing tokens",
        ko: "기존 토큰의 평균 수명",
    },
    problem_desc: {
        en: "Tokens without track records, unfounded prices, opaque fund flows. Investors suffer repeatedly, and market trust has collapsed.",
        ko: "실적 없는 토큰, 근거 없는 가격, 투명하지 않은 자금 흐름. 투자자는 반복적으로 피해를 입고, 시장 신뢰는 무너졌습니다.",
    },
    before_title: {
        en: "Existing Exchanges",
        ko: "기존 거래소",
    },
    before_1: {
        en: "Token listing without track record → Repeated investor losses",
        ko: "실적 없는 토큰 상장 → 투자자 피해 반복",
    },
    before_2: {
        en: "Indiscriminate dilution → Holder value erosion",
        ko: "무분별한 물량 희석 → 보유자 가치 훼손",
    },
    before_3: {
        en: "Information asymmetry → Only insiders benefit",
        ko: "정보 비대칭 → 내부자만 유리한 구조",
    },
    before_4: {
        en: "Opaque fund flows → Impossible to verify",
        ko: "자금 흐름 불투명 → 검증 불가능",
    },
    after_title: {
        en: "Biyard Launchpad",
        ko: "Biyard 런치패드",
    },
    before_diagram: {
        en: "No-track-record project",
        ko: "실적 없는 프로젝트",
    },
    after_diagram: {
        en: "Revenue-based company",
        ko: "실제 매출 기반 기업",
    },
    after_1: {
        en: "Only companies with revenue can issue tokens",
        ko: "매출이 있는 기업만 토큰 발행 가능",
    },
    after_2: {
        en: "Treasury = on-chain revenue proof. No faking",
        ko: "트레저리 = 온체인 매출 증명. 가짜 불가",
    },
    after_3: {
        en: "Floor price smart contract guarantees bottom",
        ko: "하한가 스마트 컨트랙트로 바닥 보장",
    },
    after_4: {
        en: "All fund flows publicly on-chain",
        ko: "모든 자금 흐름 온체인 공개",
    },
    how_it_works: {
        en: "How It Works",
        ko: "How It Works",
    },
    how_heading_1: {
        en: "How is Biyard ",
        ko: "Biyard는 어떻게 ",
    },
    how_heading_accent: {
        en: "different?",
        ko: "다를까요?",
    },
    step_1_icon: {
        en: "\u{1F6D2}",
        ko: "\u{1F6D2}",
    },
    step_1_title: {
        en: "Customer Purchase",
        ko: "고객이 구매",
    },
    step_1_desc: {
        en: "When a customer purchases a product/service, 2~4% of the payment is automatically deposited.",
        ko: "기업의 상품/서비스를 구매하면 결제 금액의 2~4%가 자동으로 적립됩니다.",
    },
    step_2_title: {
        en: "Treasury Deposit",
        ko: "트레저리 적립",
    },
    step_2_desc: {
        en: "The deposited amount accumulates in the on-chain treasury. This becomes the foundation of token value.",
        ko: "적립된 금액이 온체인 트레저리에 누적됩니다. 이것이 토큰 가치의 근간이 됩니다.",
    },
    step_3_title: {
        en: "Activity Rewards",
        ko: "활동 리워드",
    },
    step_3_desc: {
        en: "Additional token rewards are given for verified activities like walking, store visits, and social sharing.",
        ko: "걷기, 매장 방문, SNS 공유 등 활동 인증 시 추가 토큰 리워드가 지급됩니다.",
    },
    step_4_title: {
        en: "Value Growth",
        ko: "가치 상승",
    },
    step_4_desc: {
        en: "As revenue grows, the treasury builds up and the floor price rises. All holders' assets grow together.",
        ko: "매출이 늘수록 트레저리가 쌓이고, 하한가가 올라갑니다. 모든 홀더의 자산이 함께 성장합니다.",
    },
    formula_label: {
        en: "Core Formula",
        ko: "핵심 공식",
    },
    formula_desc: {
        en: "As long as revenue continues, the token's floor price is mathematically guaranteed. If someone sells below the floor price, the treasury automatically buys back and burns, so the floor price never drops.",
        ko: "매출이 지속되는 한 토큰 가치의 바닥이 수학적으로 보장됩니다. 누군가 하한가 이하로 매도하면 트레저리가 자동 매수 후 소각하여 하한가는 절대 하락하지 않습니다.",
    },
}

// Solution section
translate! {
    SolutionTranslate;

    section_label: {
        en: "Core Innovation",
        ko: "Core Innovation",
    },
    heading_1: {
        en: "Purchases become ",
        ko: "구매가 곧 ",
    },
    heading_accent: {
        en: "investments",
        ko: "투자",
    },
    heading_2: {
        en: ".",
        ko: "가 됩니다.",
    },
    card_1_label: {
        en: "Revenue-Linked",
        ko: "Revenue-Linked",
    },
    card_1_title: {
        en: "Revenue-Linked Deposits",
        ko: "매출 연동 적립",
    },
    card_1_desc: {
        en: "A portion of purchase amount is automatically deposited into the on-chain treasury. Real revenue is the token's fundamental.",
        ko: "구매 금액의 일부가 자동으로 온체인 트레저리에 적립됩니다. 실제 매출이 토큰의 펀더멘털입니다.",
    },
    card_2_label: {
        en: "Floor Guarantee",
        ko: "Floor Guarantee",
    },
    card_2_title: {
        en: "Floor Price Guarantee",
        ko: "하한가 보장",
    },
    card_2_desc: {
        en: "Smart contracts mathematically defend the floor price. An innovative structure where the bottom is guaranteed even during dumps.",
        ko: "스마트 컨트랙트가 수학적으로 하한가를 방어합니다. 덤핑해도 바닥이 보장되는 혁신적 구조입니다.",
    },
    card_3_label: {
        en: "Value Growth",
        ko: "Value Growth",
    },
    card_3_title: {
        en: "Value Growth",
        ko: "가치 성장",
    },
    card_3_desc: {
        en: "As revenue grows, the treasury builds up and the floor price rises. All holders' asset value grows together.",
        ko: "매출이 늘면 트레저리가 쌓이고 하한가가 상승합니다. 모든 홀더의 자산 가치가 함께 올라갑니다.",
    },
}

// Why Biyard section
translate! {
    WhyBiyardTranslate;

    heading_1: {
        en: "Not spending, but ",
        ko: "소모가 아닌 ",
    },
    heading_accent: {
        en: "circulation",
        ko: "순환",
    },
    heading_2: {
        en: ",",
        ko: "으로,",
    },
    heading_3: {
        en: "Changing the structure of marketing.",
        ko: "마케팅의 구조를 바꿉니다.",
    },
    before_label: {
        en: "Traditional Marketing Vicious Cycle",
        ko: "기존 마케팅 악순환",
    },
    before_ad_spend: {
        en: "Ad spend",
        ko: "광고비",
    },
    before_ad_amount: {
        en: "₩100M spent",
        ko: "1억 지출",
    },
    before_acquisition: {
        en: "Customer acquisition",
        ko: "고객 유치",
    },
    before_temporary: {
        en: "Temporary",
        ko: "일시적",
    },
    before_money_out: {
        en: "Money drains out",
        ko: "돈이 빠져나감",
    },
    before_churn: {
        en: "Customer churn",
        ko: "고객 이탈",
    },
    before_no_return: {
        en: "No return visits",
        ko: "재방문 X",
    },
    before_repeat: {
        en: "More ads",
        ko: "또 광고",
    },
    before_repeat_spend: {
        en: "Repeat spending",
        ko: "반복 지출",
    },
    before_summary: {
        en: "Endless vicious cycle",
        ko: "끝없는 악순환",
    },
    after_label: {
        en: "Biyard Virtuous Cycle",
        ko: "Biyard 선순환",
    },
    after_revenue: {
        en: "Revenue generated",
        ko: "매출 발생",
    },
    after_deposit: {
        en: "2-4% deposited",
        ko: "2-4% 적립",
    },
    after_value_up: {
        en: "Token value",
        ko: "토큰 가치",
    },
    after_auto_rise: {
        en: "Auto increase",
        ko: "자동 상승",
    },
    after_center: {
        en: "Value circulates",
        ko: "가치가 순환한다",
    },
    after_advocacy: {
        en: "Customer advocacy",
        ko: "고객 홍보",
    },
    after_word_of_mouth: {
        en: "Word of mouth",
        ko: "입소문 확산",
    },
    after_new_customers: {
        en: "New customers",
        ko: "신규 고객",
    },
    after_organic: {
        en: "Organic inflow",
        ko: "자연 유입",
    },
    after_summary: {
        en: "Virtuous cycle",
        ko: "선순환",
    },
    bridge_1: {
        en: "Biyard Launchpad is the infrastructure that makes this virtuous cycle possible. Companies convert existing ad spend into token rewards, and customers accumulate digital assets with real value through purchases and activities.",
        ko: "Biyard Launchpad는 이 선순환을 가능하게 하는 인프라입니다. 기업은 기존 광고비를 토큰 리워드로 전환하고, 고객은 구매와 활동을 통해 실제 가치가 있는 디지털 자산을 적립합니다.",
    },
    bridge_2: {
        en: "Revenue accumulated in the treasury can be verified by anyone on-chain, and smart contracts automatically defend the floor price. Because token value is linked to actual revenue, the floor is guaranteed as long as revenue continues.",
        ko: "트레저리에 쌓인 매출은 온체인에서 누구나 검증할 수 있고, 스마트 컨트랙트가 하한가를 자동으로 방어합니다. 토큰의 가치가 실제 매출에 연동되기 때문에, 매출이 지속되는 한 가치의 바닥이 보장됩니다.",
    },
    bridge_3: {
        en: "No blockchain knowledge needed — integrate with a single API. Any industry with revenue can build a token economy.",
        ko: "블록체인 지식이 없어도 API 하나로 연동 가능하며, 어떤 업종이든 매출이 있다면 토큰 이코노미를 구축할 수 있습니다.",
    },
}

// Showcase section
translate! {
    ShowcaseTranslate;

    section_label: {
        en: "Use Cases",
        ko: "Use Cases",
    },
    section_title_1: {
        en: "Many Brands,",
        ko: "다양한 브랜드,",
    },
    section_title_2: {
        en: "One Platform.",
        ko: "하나의 플랫폼.",
    },
    six_month_reward: {
        en: "6-Month Reward",
        ko: "6개월 리워드",
    },
    why_examples_1: {
        en: "Why these brands as ",
        ko: "왜 이 브랜드들을 ",
    },
    why_examples_accent: {
        en: "examples",
        ko: "예시",
    },
    why_examples_2: {
        en: "?",
        ko: "로 들었을까요?",
    },
    why_examples_desc_1: {
        en: "Shoes, coffee, fashion — these three brands represent industries where customers repeatedly purchase and engage daily. Biyard's token economy transforms these everyday consumption activities into asset accumulation.",
        ko: "신발, 커피, 패션 — 이 세 브랜드는 고객이 매일 반복적으로 구매하고 활동하는 대표적인 업종입니다. Biyard의 토큰 이코노미는 이런 일상적 소비 행위를 자산 축적으로 전환합니다.",
    },
    why_examples_desc_2_prefix: {
        en: "But these three are just the beginning. ",
        ko: "하지만 이 세 가지는 시작일 뿐입니다. ",
    },
    why_examples_desc_2_highlight: {
        en: "Any business that generates revenue",
        ko: "매출이 발생하는 모든 비즈니스",
    },
    why_examples_desc_2_suffix: {
        en: " is eligible. Healthcare, education, logistics, food, subscription services — no industry restrictions. Integrate with your existing POS or app with a single RESTful API.",
        ko: "가 대상입니다. 헬스케어, 교육, 물류, 식품, 구독 서비스 등 업종에 제한이 없으며, RESTful API 하나로 기존 POS나 앱에 연동할 수 있습니다.",
    },
    industry_healthcare: { en: "Healthcare", ko: "헬스케어" },
    industry_education: { en: "Education", ko: "교육" },
    industry_logistics: { en: "Logistics", ko: "물류" },
    industry_food: { en: "Food", ko: "식품" },
    industry_subscription: { en: "Subscription", ko: "구독서비스" },
    industry_entertainment: { en: "Entertainment", ko: "엔터테인먼트" },
    industry_beauty: { en: "Beauty", ko: "뷰티" },
    industry_travel: { en: "Travel", ko: "여행" },
}

// FAQ section
translate! {
    FaqTranslate;

    section_title: {
        en: "Frequently Asked Questions",
        ko: "자주 묻는 질문",
    },
    q1: {
        en: "How is token value guaranteed?",
        ko: "토큰 가치는 어떻게 보장되나요?",
    },
    a1: {
        en: "All tokens are linked to the actual revenue of real companies. A portion of the company's revenue is deposited into the treasury, which forms the floor price of the token. Projects without revenue cannot issue tokens, and as long as revenue continues, the token's floor price is guaranteed.",
        ko: "모든 토큰은 실제 기업의 매출에 연동됩니다. 기업의 매출 일부가 트레저리에 적립되고, 이 트레저리가 토큰 가치의 하한선을 형성합니다. 매출이 없는 프로젝트는 토큰을 발행할 수 없으며, 매출이 지속되는 한 토큰 가치의 바닥이 보장됩니다.",
    },
    q2: {
        en: "How is the floor price maintained?",
        ko: "하한가(Floor Price)는 어떻게 유지되나요?",
    },
    a2: {
        en: "A smart contract automatically defends the floor price. If someone sells below the floor, the treasury buys back and burns automatically.",
        ko: "스마트 컨트랙트가 자동으로 하한가를 방어합니다. 누군가 토큰을 하한가 이하로 매도하면, 트레저리가 자동으로 해당 토큰을 하한가에 매수(Buyback)하고 소각(Burn)합니다.",
    },
    q3: {
        en: "What happens during a mass sell-off?",
        ko: "대량 매도(덤핑)가 발생하면 어떻게 되나요?",
    },
    a3: {
        en: "Even if 90% is sold at once, the floor price holds. The treasury buys all sell volume and burns it.",
        ko: "전체 물량의 90%가 한번에 매도되어도 하한가는 유지됩니다. 트레저리가 모든 매도 물량을 하한가에 매수하고 소각하기 때문입니다.",
    },
    q4: {
        en: "How is it different from existing exchanges?",
        ko: "기존 토큰 거래소와 무엇이 다른가요?",
    },
    a4: {
        en: "Existing exchanges allow tokens without earnings to be listed. Biyard only allows companies with actual revenue, all flows are public on-chain.",
        ko: "기존 거래소는 실적 없는 토큰도 상장이 가능하고, 가치 근거 없는 가격 변동이 발생합니다. Biyard는 실제 매출이 있는 기업만 토큰을 발행할 수 있고, 모든 자금 흐름이 온체인에 공개됩니다.",
    },
    q5: {
        en: "What types of companies can join?",
        ko: "어떤 기업이 입점할 수 있나요?",
    },
    a5: {
        en: "Any company that generates revenue. No industry restrictions. Integrate with just an API — no blockchain expertise required.",
        ko: "매출이 발생하는 모든 기업이 대상입니다. 신발, 커피, 의류, 식품, 헬스케어 등 업종에 제한이 없습니다. API 연동만으로 기존 POS나 앱에 토큰 이코노미를 붙일 수 있어, 블록체인 전문 인력 없이도 시작할 수 있습니다.",
    },
    q6: {
        en: "What benefits do token holders get?",
        ko: "토큰 보유자는 어떤 혜택이 있나요?",
    },
    a6: {
        en: "Token holders are essentially brand shareholders. Revenue grows the treasury and token value. Through the DAO, holders can participate in brand decisions.",
        ko: "토큰 보유자는 사실상 해당 브랜드의 주주와 같습니다. 매출이 늘면 트레저리가 쌓이고 토큰 가치가 올라갑니다. 또한 DAO를 통해 브랜드의 리워드 정책, 이벤트 등 의사결정에 직접 참여할 수 있습니다.",
    },
}

// CTA section
translate! {
    CtaTranslate;

    heading_1: {
        en: "Revenue-based token economy,",
        ko: "매출 기반 토큰 경제,",
    },
    heading_accent: {
        en: "get started now.",
        ko: "지금 시작하세요.",
    },
    subtitle: {
        en: "With a revenue-based token economy, brands and customers grow together. Integrate in 5 minutes with no blockchain knowledge.",
        ko: "매출 기반 토큰 이코노미로 브랜드와 고객이 함께 성장합니다. 블록체인 지식 없이도 5분 만에 연동.",
    },
    cta_console: {
        en: "Go to Console →",
        ko: "Console로 이동하기 →",
    },
    cta_pricing: {
        en: "View Pricing",
        ko: "요금제 보기",
    },
}

// Footer section
translate! {
    FooterTranslate;

    description: {
        en: "Revenue-backed token infrastructure for brands. Building trust through transparency.",
        ko: "브랜드를 위한 매출 기반 토큰 인프라. 투명성으로 신뢰를 구축합니다.",
    },
}
