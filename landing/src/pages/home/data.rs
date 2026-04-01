#[allow(dead_code)]
pub(super) struct Feature {
    pub icon: &'static str,
    pub name: &'static str,
    pub description: &'static str,
}

#[allow(dead_code)]
pub(super) const FEATURES: &[Feature] = &[
    Feature {
        icon: "\u{1F3E6}",
        name: "수익 기반 트레저리",
        description: "고객의 구매 금액 일부가 자동으로 트레저리에 적립됩니다. 실제 매출이 토큰 가치를 뒷받침합니다.",
    },
    Feature {
        icon: "\u{1F9B6}",
        name: "활동 기반 리워드",
        description: "걷기, 매장 방문, 구매 등 고객의 실제 활동이 인증되면 추가 리워드가 지급됩니다.",
    },
    Feature {
        icon: "\u{1F4C8}",
        name: "성장하는 토큰 가치",
        description: "트레저리가 쌍일수록 토큰 하한가가 올라갑니다. 고객이 많을수록, 매출이 높을수록 모두가 이득입니다.",
    },
    Feature {
        icon: "\u{1FA99}",
        name: "브랜드 전용 토큰",
        description: "우리 브랜드만의 토큰을 발행하세요. 실물 매출에 연동된 토큰으로 고객 충성도를 극대화합니다.",
    },
    Feature {
        icon: "\u{1F5F3}",
        name: "DAO 커뮤니티",
        description: "토큰 보유자가 브랜드 의사결정에 참여합니다. 고객이 곧 브랜드의 주주가 됩니다.",
    },
    Feature {
        icon: "\u{1F4BB}",
        name: "5분 만에 연동",
        description: "RESTful API와 SDK로 기존 앱/POS에 쉽게 연동하세요. 블록체인 지식 없이도 시작할 수 있습니다.",
    },
];

pub(super) struct BrandStep {
    pub icon: &'static str,
    pub title: &'static str,
    pub desc: &'static str,
}

pub(super) struct BrandScenario {
    pub purchase_item: &'static str,
    pub purchase_price: i64,
    pub reward_rate: f64,
    pub reward_amount: i64,
    pub activity_type: &'static str,
    pub activity_detail: &'static str,
    pub activity_reward: i64,
    #[allow(dead_code)]
    pub monthly_activity: &'static str,
    pub monthly_reward: i64,
    pub six_month_total: i64,
}

pub(super) struct BrandStats {
    pub treasury: i64,
    pub users: i64,
    pub floor_price: f64,
    #[allow(dead_code)]
    pub retention: &'static str,
}

pub(super) struct BrandShowcase {
    pub brand: &'static str,
    pub segment: &'static str,
    pub tagline: &'static str,
    #[allow(dead_code)]
    pub hero_message: &'static str,
    pub scenario: BrandScenario,
    pub customer_quote: &'static str,
    pub customer_name: &'static str,
    pub stats: BrandStats,
    pub steps: [BrandStep; 4],
}

pub(super) const BRAND_SHOWCASES: &[BrandShowcase] = &[
    BrandShowcase {
        brand: "Shoe Brand",
        segment: "Fashion",
        tagline: "건강과 토큰을 같이 챙기세요!",
        hero_message: "신발을 사고, 신고, 걸을수록 돈을 버는 경험",
        scenario: BrandScenario {
            purchase_item: "Shoe Brand 컴포트 워커",
            purchase_price: 129000,
            reward_rate: 2.0,
            reward_amount: 2580,
            activity_type: "걷기",
            activity_detail: "하루 8,000걸음 달성 시",
            activity_reward: 80,
            monthly_activity: "월 평균 20일 활동",
            monthly_reward: 1600,
            six_month_total: 12180,
        },
        customer_quote: "편한 신발 사서 매일 출퇴근길에 신고 다니는데, 걸을수록 포인트가 쌓이고 그게 진짜 돈이 돼요. 안 신을 이유가 없죠.",
        customer_name: "김서연, 직장인",
        stats: BrandStats { treasury: 24500, users: 1250, floor_price: 0.0245, retention: "40%" },
        steps: [
            BrandStep { icon: "\u{1F6CD}\u{FE0F}", title: "신발 구매", desc: "129,000원 신발 구매 시 2,580원(2%) 적립" },
            BrandStep { icon: "\u{1F3E6}", title: "트레저리 적립", desc: "적립금이 트레저리에 누적 → 토큰 가치 상승" },
            BrandStep { icon: "\u{1F9B6}", title: "걷기 인증", desc: "매일 신발 신고 걸으면 추가 리워드 지급" },
            BrandStep { icon: "\u{1F4C8}", title: "가치 성장", desc: "고객이 많을수록 트레저리↑ 토큰 가치↑" },
        ],
    },
    BrandShowcase {
        brand: "Coffee Brand",
        segment: "F&B",
        tagline: "커피 한 잔이 자산이 되는 경험!",
        hero_message: "매일 마시는 커피가 나의 투자가 됩니다",
        scenario: BrandScenario {
            purchase_item: "시그니처 라떼",
            purchase_price: 6500,
            reward_rate: 3.0,
            reward_amount: 195,
            activity_type: "매장 방문",
            activity_detail: "주 3회 방문 스탬프 달성 시",
            activity_reward: 50,
            monthly_activity: "월 12회 방문 기준",
            monthly_reward: 200,
            six_month_total: 2370,
        },
        customer_quote: "어차피 매일 커피 마시는데, 여기서 마시면 스탬프도 찍히고 토큰도 쌓여요. 다른 카페 갈 이유가 없어요.",
        customer_name: "박준혁, 대학생",
        stats: BrandStats { treasury: 12300, users: 850, floor_price: 0.0246, retention: "35%" },
        steps: [
            BrandStep { icon: "\u{1F6CD}\u{FE0F}", title: "음료 구매", desc: "6,500원 라떼 주문 시 195원(3%) 적립" },
            BrandStep { icon: "\u{1F3E6}", title: "트레저리 적립", desc: "적립금이 트레저리에 누적 → 토큰 가치 상승" },
            BrandStep { icon: "\u{2764}\u{FE0F}", title: "방문 스탬프", desc: "주 3회 방문 달성 시 보너스 리워드 지급" },
            BrandStep { icon: "\u{1F4C8}", title: "가치 성장", desc: "단골이 많을수록 트레저리↑ 토큰 가치↑" },
        ],
    },
    BrandShowcase {
        brand: "Fashion Brand",
        segment: "Fashion",
        tagline: "입을수록 가치가 쌓이는 패션!",
        hero_message: "좋아하는 옷을 사고 입을수록 토큰이 쌓입니다",
        scenario: BrandScenario {
            purchase_item: "시그니처 자켓",
            purchase_price: 189000,
            reward_rate: 2.5,
            reward_amount: 4725,
            activity_type: "매장 방문",
            activity_detail: "월 2회 이상 매장 방문 시",
            activity_reward: 100,
            monthly_activity: "월 3회 방문 기준",
            monthly_reward: 300,
            six_month_total: 6150,
        },
        customer_quote: "좋아하는 브랜드 옷을 사면서 토큰도 쌓이니까, 다른 브랜드 살 이유가 없어졌어요.",
        customer_name: "이현우, 패션 마니아",
        stats: BrandStats { treasury: 35800, users: 2100, floor_price: 0.0179, retention: "60%" },
        steps: [
            BrandStep { icon: "\u{1F6CD}\u{FE0F}", title: "의류 구매", desc: "189,000원 자켓 구매 시 4,725원(2.5%) 적립" },
            BrandStep { icon: "\u{1F3E6}", title: "트레저리 적립", desc: "적립금이 트레저리에 누적 → 토큰 가치 상승" },
            BrandStep { icon: "\u{1F6CD}\u{FE0F}", title: "매장 방문", desc: "매장 방문 인증 시 추가 리워드 지급" },
            BrandStep { icon: "\u{1F4C8}", title: "가치 성장", desc: "러너가 많을수록 트레저리↑ 토큰 가치↑" },
        ],
    },
];

pub(super) fn console_url() -> &'static str {
    option_env!("CONSOLE_URL").unwrap_or("https://console.dev.biyard.co")
}

pub(super) fn format_won(n: i64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    let formatted: String = result.chars().rev().collect();
    format!("{}원", formatted)
}

pub(super) fn format_number(n: i64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

pub(super) fn format_usd(n: i64) -> String {
    let formatted = format_number(n);
    format!("${}", formatted)
}
