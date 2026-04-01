use dioxus::prelude::*;

// ── Data structures ──

struct Feature {
    icon: &'static str,
    name: &'static str,
    description: &'static str,
}

const FEATURES: &[Feature] = &[
    Feature {
        icon: "\u{1F3E6}",
        name: "수익 기반 트레저리",
        description: "고객의 구매 금액 일부가 자동으로 트레저리에 적립됩니다. 실제 매출이 토큰 가치를 뒷받침합니다.",
    },
    Feature {
        icon: "\u{1F9B6}",
        name: "활동 기반 리워드",
        description: "걷기, 러닝, 매장 방문, 구매 등 고객의 실제 활동이 인증되면 추가 리워드가 지급됩니다.",
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

struct BrandStep {
    icon: &'static str,
    title: &'static str,
    desc: &'static str,
}

struct BrandScenario {
    purchase_item: &'static str,
    purchase_price: i64,
    reward_rate: f64,
    reward_amount: i64,
    activity_type: &'static str,
    activity_detail: &'static str,
    activity_reward: i64,
    monthly_activity: &'static str,
    monthly_reward: i64,
    six_month_total: i64,
}

struct BrandStats {
    treasury: i64,
    users: i64,
    floor_price: f64,
    retention: &'static str,
}

struct BrandShowcase {
    brand: &'static str,
    segment: &'static str,
    tagline: &'static str,
    hero_message: &'static str,
    scenario: BrandScenario,
    customer_quote: &'static str,
    customer_name: &'static str,
    stats: BrandStats,
    steps: [BrandStep; 4],
}

const BRAND_SHOWCASES: &[BrandShowcase] = &[
    BrandShowcase {
        brand: "Le Mouton",
        segment: "Fashion",
        tagline: "건강과 토큰을 같이 챙기세요!",
        hero_message: "신발을 사고, 신고, 걸을수록 돈을 버는 경험",
        scenario: BrandScenario {
            purchase_item: "Le Mouton 컴포트 워커",
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
        customer_quote: "편한 신발 사서 매일 출퇴근길에 신고 다니는데, 걸을수록 포인트가 쌓이고 그게 진짜 돈이 돼요. 르무통 안 신을 이유가 없죠.",
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
        brand: "Cafe Blossom",
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
        brand: "RunPulse",
        segment: "Sports Tech",
        tagline: "달릴수록 벌리는 스마트한 운동!",
        hero_message: "운동하면서 건강도 챙기고 수익도 챙기세요",
        scenario: BrandScenario {
            purchase_item: "RunPulse 스마트밴드",
            purchase_price: 89000,
            reward_rate: 2.5,
            reward_amount: 2225,
            activity_type: "러닝",
            activity_detail: "주 3회 5km 러닝 인증 시",
            activity_reward: 150,
            monthly_activity: "월 12회 러닝 기준",
            monthly_reward: 1800,
            six_month_total: 13025,
        },
        customer_quote: "밴드 차고 뛰기만 하면 자동으로 기록되고 토큰이 쌓여요. 운동 동기부여가 확실히 달라졌어요.",
        customer_name: "이현우, 마라토너",
        stats: BrandStats { treasury: 35800, users: 2100, floor_price: 0.0179, retention: "60%" },
        steps: [
            BrandStep { icon: "\u{1F6CD}\u{FE0F}", title: "디바이스 구매", desc: "89,000원 스마트밴드 구매 시 2,225원(2.5%) 적립" },
            BrandStep { icon: "\u{1F3E6}", title: "트레저리 적립", desc: "적립금이 트레저리에 누적 → 토큰 가치 상승" },
            BrandStep { icon: "\u{1F9B6}", title: "러닝 인증", desc: "GPS 연동 러닝 기록 달성 시 추가 리워드" },
            BrandStep { icon: "\u{1F4C8}", title: "가치 성장", desc: "러너가 많을수록 트레저리↑ 토큰 가치↑" },
        ],
    },
];

struct Holding {
    token: &'static str,
    brand: &'static str,
    price: f64,
    change24h: f64,
    amount: f64,
    value: f64,
}

const HOLDINGS: &[Holding] = &[
    Holding { token: "LMT", brand: "Le Mouton", price: 0.0245, change24h: 3.2, amount: 8.5, value: 0.2083 },
    Holding { token: "CBT", brand: "Cafe Blossom", price: 0.0246, change24h: 1.8, amount: 3.24, value: 0.0797 },
    Holding { token: "RPT", brand: "RunPulse", price: 0.0179, change24h: 5.4, amount: 1.5, value: 0.0269 },
];

struct Activity {
    emoji: &'static str,
    text: &'static str,
    time: &'static str,
}

const RECENT_ACTIVITY: &[Activity] = &[
    Activity { emoji: "\u{1F6CD}\u{FE0F}", text: "Le Mouton 컴포트 워커 구매 - +2,580 LMT", time: "2시간 전" },
    Activity { emoji: "\u{1F6B6}", text: "걷기 8,000걸음 달성 - +80 LMT", time: "5시간 전" },
    Activity { emoji: "\u{2615}", text: "Cafe Blossom 시그니처 라떼 - +195 CBT", time: "1일 전" },
    Activity { emoji: "\u{1F3C3}", text: "RunPulse 5km 러닝 완료 - +150 RPT", time: "2일 전" },
    Activity { emoji: "\u{1F381}", text: "주간 방문 보너스 - +50 CBT", time: "3일 전" },
];

struct GrowthStep {
    emoji: &'static str,
    title: &'static str,
    desc: &'static str,
}

const GROWTH_STEPS: &[GrowthStep] = &[
    GrowthStep { emoji: "\u{1F6CD}\u{FE0F}", title: "구매 & 활동", desc: "브랜드 제품을 구매하고 활동하면 토큰이 적립됩니다" },
    GrowthStep { emoji: "\u{1F3E6}", title: "트레저리 축적", desc: "매출의 일부가 트레저리에 쌍여 토큰 가치를 뒷받침합니다" },
    GrowthStep { emoji: "\u{1F4C8}", title: "가치 상승", desc: "트레저리가 커질수록 토큰 하한가가 올라갑니다" },
    GrowthStep { emoji: "\u{1F4B0}", title: "수익 실현", desc: "토큰의 가치가 성장하여 실제 수익으로 이어집니다" },
];

struct TokenInfo {
    rank: i32,
    token: &'static str,
    brand: &'static str,
    price: f64,
    change24h: f64,
    market_cap: i64,
    circulating: i64,
    total_supply: i64,
    floor_price: f64,
}

const TOKEN_LIST: &[TokenInfo] = &[
    TokenInfo { rank: 1, token: "LMT", brand: "Le Mouton", price: 0.0245, change24h: 3.2, market_cap: 24500, circulating: 45000, total_supply: 1000000, floor_price: 0.0245 },
    TokenInfo { rank: 2, token: "CBT", brand: "Cafe Blossom", price: 0.0246, change24h: 1.8, market_cap: 12300, circulating: 22000, total_supply: 500000, floor_price: 0.0246 },
    TokenInfo { rank: 3, token: "RPT", brand: "RunPulse", price: 0.0179, change24h: 5.4, market_cap: 35800, circulating: 68000, total_supply: 2000000, floor_price: 0.0179 },
];

struct SwapRecord {
    date: &'static str,
    from: &'static str,
    to: &'static str,
    amount: &'static str,
    rate: &'static str,
    status: &'static str,
}

const RECENT_SWAPS: &[SwapRecord] = &[
    SwapRecord { date: "2026-03-28", from: "LMT", to: "CBT", amount: "2.0", rate: "0.85", status: "Completed" },
    SwapRecord { date: "2026-03-25", from: "CBT", to: "RPT", amount: "1.5", rate: "1.41", status: "Completed" },
    SwapRecord { date: "2026-03-20", from: "RPT", to: "LMT", amount: "1.0", rate: "0.83", status: "Completed" },
];

struct DaoProposal {
    id: i32,
    brand: &'static str,
    title: &'static str,
    description: &'static str,
    yes_votes: i32,
    no_votes: i32,
    deadline: &'static str,
}

const DAO_PROPOSALS: &[DaoProposal] = &[
    DaoProposal { id: 0, brand: "Le Mouton", title: "리워드 배수 2배 증가", description: "걷기 리워드 배수를 현재 1배에서 2배로 증가시켜 고객 활동을 더욱 장려합니다.", yes_votes: 1250, no_votes: 340, deadline: "2026-04-15" },
    DaoProposal { id: 1, brand: "Cafe Blossom", title: "신메뉴 출시 기념 보너스", description: "신메뉴 출시를 기념하여 첫 주문 고객에게 3배 보너스 토큰을 지급합니다.", yes_votes: 890, no_votes: 120, deadline: "2026-04-20" },
    DaoProposal { id: 2, brand: "RunPulse", title: "마라톤 이벤트 토큰 배분", description: "마라톤 이벤트 참가자에게 특별 토큰 보상을 배분합니다.", yes_votes: 2100, no_votes: 180, deadline: "2026-04-10" },
];

// ── Helpers ──

fn format_won(n: i64) -> String {
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

fn format_number(n: i64) -> String {
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

fn format_usd(n: i64) -> String {
    let formatted = format_number(n);
    format!("${}", formatted)
}

fn get_swap_rate(from: &str, to: &str) -> f64 {
    match (from, to) {
        ("LMT", "CBT") => 0.85,
        ("LMT", "RPT") => 1.2,
        ("CBT", "LMT") => 1.18,
        ("CBT", "RPT") => 1.41,
        ("RPT", "LMT") => 0.83,
        ("RPT", "CBT") => 0.71,
        _ => 1.0,
    }
}

fn get_balance(token: &str) -> f64 {
    match token {
        "LMT" => 8.5,
        "CBT" => 3.24,
        "RPT" => 1.5,
        _ => 0.0,
    }
}

fn get_brand_name(token: &str) -> &'static str {
    match token {
        "LMT" => "Le Mouton",
        "CBT" => "Cafe Blossom",
        "RPT" => "RunPulse",
        _ => "",
    }
}

// ── CSS Animations ──

const CSS_ANIMATIONS: &str = r#"
@keyframes spinCube {
  0% { transform: rotateX(-20deg) rotateY(0deg); }
  100% { transform: rotateX(-20deg) rotateY(360deg); }
}
@keyframes floatParticle {
  0% { transform: translateY(0px); opacity: 0.4; }
  100% { transform: translateY(-12px); opacity: 1; }
}
@keyframes chartPulse {
  0% { filter: drop-shadow(0 0 15px rgba(56,189,248,0.2)); }
  50% { filter: drop-shadow(0 0 25px rgba(56,189,248,0.4)); }
  100% { filter: drop-shadow(0 0 15px rgba(56,189,248,0.2)); }
}
@keyframes pulseRed {
  0%, 100% { opacity: 0.3; transform: scale(1); }
  50% { opacity: 0.8; transform: scale(1.1); }
}
@keyframes pulseGreen {
  0%, 100% { opacity: 0.5; transform: scale(1); }
  50% { opacity: 1; transform: scale(1.05); }
}
@keyframes drawLineRed {
  0% { stroke-dashoffset: 200; }
  100% { stroke-dashoffset: 0; }
}
@keyframes drawLineGreen {
  0% { stroke-dashoffset: 200; }
  100% { stroke-dashoffset: 0; }
}
@keyframes flowDown {
  0% { transform: translateY(-10px); opacity: 0; }
  50% { opacity: 1; }
  100% { transform: translateY(10px); opacity: 0; }
}
@keyframes glowPulse {
  0%, 100% { box-shadow: 0 0 20px rgba(0,212,170,0.3); }
  50% { box-shadow: 0 0 40px rgba(0,212,170,0.6); }
}
@keyframes transformArrow {
  0%, 100% { transform: scale(1); filter: drop-shadow(0 0 10px rgba(0,212,170,0.3)); }
  50% { transform: scale(1.1); filter: drop-shadow(0 0 20px rgba(0,212,170,0.6)); }
}
@keyframes nodeFloat {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-5px); }
}
@keyframes dashFlow {
  0% { stroke-dashoffset: 20; }
  100% { stroke-dashoffset: 0; }
}
@keyframes fadeInOut {
  0%, 100% { opacity: 0.2; }
  50% { opacity: 0.7; }
}
@keyframes lineGrow {
  0% { stroke-dashoffset: 300; }
  100% { stroke-dashoffset: 0; }
}
@keyframes spinCubeLeft {
  0% { transform: rotateX(25deg) rotateZ(-15deg) rotateY(0deg); }
  100% { transform: rotateX(25deg) rotateZ(-15deg) rotateY(360deg); }
}
@keyframes spinCubeRight {
  0% { transform: rotateX(-25deg) rotateZ(12deg) rotateY(0deg); }
  100% { transform: rotateX(-25deg) rotateZ(12deg) rotateY(-360deg); }
}
@keyframes spinCubeMain {
  0% { transform: rotateX(-15deg) rotateZ(8deg) rotateY(0deg); }
  100% { transform: rotateX(-15deg) rotateZ(8deg) rotateY(360deg); }
}

/* Scroll reveal animations */
@keyframes slideUp {
  0% { opacity: 0; transform: translateY(30px); }
  100% { opacity: 1; transform: translateY(0); }
}
@keyframes typeIn {
  0% { max-width: 0; }
  100% { max-width: 100%; }
}
@keyframes blink {
  50% { border-color: transparent; }
}

.scroll-hidden {
  opacity: 0;
}
.scroll-bounce {
  animation: slideUp 0.6s ease-out forwards;
}
.scroll-fade {
  animation: slideUp 0.5s ease-out forwards;
}
.scroll-type {
  display: inline-block;
  overflow: hidden;
  white-space: nowrap;
  max-width: 0;
  border-right: 2px solid #00d4aa;
  animation: typeIn 1.5s ease-out forwards, blink 0.7s step-end 1.5s infinite alternate;
}
"#;

const SCROLL_REVEAL_JS: &str = r#"
(function initReveal() {
    function setupObserver() {
        var els = document.querySelectorAll('.reveal-bounce, .reveal-fade, .reveal-type');
        if (els.length === 0) {
            setTimeout(setupObserver, 300);
            return;
        }
        var io = new IntersectionObserver(function(entries) {
            entries.forEach(function(entry) {
                if (entry.isIntersecting) {
                    var el = entry.target;
                    if (el.classList.contains('reveal-bounce')) {
                        el.classList.remove('scroll-hidden');
                        el.classList.add('scroll-bounce');
                    } else if (el.classList.contains('reveal-fade')) {
                        el.classList.remove('scroll-hidden');
                        el.classList.add('scroll-fade');
                    } else if (el.classList.contains('reveal-type')) {
                        el.classList.add('scroll-type');
                    }
                    io.unobserve(el);
                }
            });
        }, { threshold: 0.1, rootMargin: '0px 0px -50px 0px' });

        els.forEach(function(el) {
            if (el.classList.contains('reveal-bounce') || el.classList.contains('reveal-fade')) {
                el.classList.add('scroll-hidden');
            }
            io.observe(el);
        });
    }
    if (document.readyState === 'complete') {
        setTimeout(setupObserver, 800);
    } else {
        window.addEventListener('load', function() { setTimeout(setupObserver, 800); });
    }
    // Also retry on any DOM changes (SPA navigation)
    var mo = new MutationObserver(function() {
        var fresh = document.querySelectorAll('.reveal-bounce:not(.scroll-bounce):not(.scroll-hidden), .reveal-fade:not(.scroll-fade):not(.scroll-hidden)');
        if (fresh.length > 0) { setupObserver(); }
    });
    mo.observe(document.body || document.documentElement, { childList: true, subtree: true });
})();
"#;

// ── Main Home Component ──

#[component]
pub fn Home() -> Element {
    let mut active_tab = use_signal(|| "home");

    rsx! {
        div {
            style: "min-height: 100vh; background: #0c1018; color: #e8eefc;",
            style { dangerous_inner_html: CSS_ANIMATIONS }
            script { dangerous_inner_html: SCROLL_REVEAL_JS }

            // Navigation Bar
            nav {
                style: "background: #0c1018; border-bottom: 1px solid rgba(0,212,170,0.12); padding: 12px 16px;",
                div {
                    class: "max-w-6xl mx-auto flex items-center justify-between",
                    span {
                        class: "text-xl font-extrabold",
                        style: "color: #00d4aa;",
                        "Biyard"
                    }
                    div {
                        class: "flex items-center gap-1",
                        for (key, label, icon) in [("home", "Home", "\u{1F3E0}"), ("wallet", "Wallet", "\u{1F4B3}"), ("tokens", "Tokens", "\u{1F4CA}"), ("dao", "DAO", "\u{1F5F3}")] {
                            {
                                let is_active = *active_tab.read() == key;
                                let bg = if is_active { "#141c2b" } else { "transparent" };
                                let color = if is_active { "#00d4aa" } else { "#7a8ba6" };
                                let style_str = format!("background: {}; color: {};", bg, color);
                                rsx! {
                                    button {
                                        class: "flex items-center gap-1.5 px-4 py-2 rounded-lg text-sm font-medium",
                                        style: "{style_str}",
                                        onclick: move |_| active_tab.set(key),
                                        span { "{icon}" }
                                        span { class: "hidden sm:inline", "{label}" }
                                    }
                                }
                            }
                        }
                    }
                    button {
                        class: "px-4 py-2 rounded-lg text-sm font-medium",
                        style: "border: 1px solid #00d4aa; color: #00d4aa; background: transparent;",
                        "Sign In"
                    }
                }
            }

            // Tab Content
            if *active_tab.read() == "wallet" {
                WalletSection {}
            } else if *active_tab.read() == "tokens" {
                TokensSection {}
            } else if *active_tab.read() == "dao" {
                DaoSection {}
            } else {
                HomeSection {}
            }
        }
    }
}

// ── HomeSection ──

#[component]
fn HomeSection() -> Element {
    rsx! {
        // Hero
        section {
            class: "relative overflow-hidden flex flex-col items-center justify-center px-4",
            style: "background: linear-gradient(135deg, #0c1018 0%, #0d1a24 50%, #0c1018 100%); min-height: 100vh;",
            // Background glows (large, dramatic)
            div {
                class: "absolute inset-0 pointer-events-none",
                div {
                    class: "absolute rounded-full",
                    style: "top: 15%; left: 35%; width: 500px; height: 500px; background: #6366f1; filter: blur(120px); opacity: 0.12;",
                }
                div {
                    class: "absolute rounded-full",
                    style: "top: 25%; left: 45%; width: 400px; height: 400px; background: #00d4aa; filter: blur(100px); opacity: 0.08;",
                }
                div {
                    class: "absolute rounded-full",
                    style: "bottom: 20%; right: 30%; width: 450px; height: 450px; background: #38bdf8; filter: blur(100px); opacity: 0.08;",
                }
            }

            // Main centered content
            div {
                class: "relative z-10 flex flex-col items-center pt-16 pb-12 w-full max-w-6xl mx-auto",

                // Multi-cube container
                div {
                    class: "relative mb-20",
                    style: "width: 750px; height: 500px;",

                    // Left cube (Fashion/Retail - overlapping, tilted)
                    div {
                        class: "absolute hidden lg:block",
                        style: "left: 10px; bottom: 5%; perspective: 900px; z-index: 2;",
                        // Glow
                        div { style: "position: absolute; top: 50%; left: 50%; transform: translate(-50%,-50%); width: 160px; height: 160px; background: #60a5fa; filter: blur(60px); opacity: 0.12; border-radius: 50%;" }
                        div {
                            style: "width: 200px; height: 200px; position: relative; transform-style: preserve-3d; animation: spinCubeLeft 22s linear infinite;",
                            // Front: Shoe SVG
                            div {
                                style: "transform: translateZ(100px); background: rgba(96,165,250,0.08); border: 1px solid rgba(96,165,250,0.25); position: absolute; inset: 0; border-radius: 14px; backdrop-filter: blur(4px); display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 10px;",
                                div {
                                    style: "color: #60a5fa; width: 80px; height: 50px;",
                                    dangerous_inner_html: r#"<svg viewBox="0 0 120 60"><path d="M15,45 Q15,30 25,25 Q35,20 50,18 L75,16 Q85,15 95,18 Q105,22 110,30 L112,38 L112,42 Q112,48 105,48 L20,48 Q15,48 15,45 Z" fill="none" stroke="currentColor" stroke-width="1.8" opacity="0.7"/><path d="M12,48 L115,48 Q115,55 108,55 L18,55 Q12,55 12,48 Z" fill="currentColor" opacity="0.12" stroke="currentColor" stroke-width="1.5" opacity="0.5"/><path d="M50,20 L58,17 M60,22 L68,19 M70,23 L78,20" fill="none" stroke="currentColor" stroke-width="1" opacity="0.4"/></svg>"#,
                                }
                                span { style: "font-size: 9px; font-weight: 700; color: #60a5fa; margin-top: 4px;", "Le Mouton" }
                            }
                            // Back: Coffee SVG
                            div {
                                style: "transform: rotateY(180deg) translateZ(100px); background: rgba(244,114,182,0.08); border: 1px solid rgba(244,114,182,0.25); position: absolute; inset: 0; border-radius: 14px; backdrop-filter: blur(4px); display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 10px;",
                                div {
                                    style: "color: #f472b6; width: 70px; height: 50px;",
                                    dangerous_inner_html: r#"<svg viewBox="0 0 120 60"><path d="M35,15 Q35,10 45,10 L75,10 Q85,10 85,15 L82,45 Q82,50 72,50 L48,50 Q38,50 38,45 Z" fill="none" stroke="currentColor" stroke-width="1.8" opacity="0.7"/><path d="M85,20 Q100,20 100,32 Q100,42 85,42" fill="none" stroke="currentColor" stroke-width="1.5" opacity="0.5"/><path d="M50,5 Q48,0 52,-5" fill="none" stroke="currentColor" stroke-width="1" opacity="0.3"/><path d="M60,3 Q58,-3 62,-8" fill="none" stroke="currentColor" stroke-width="1" opacity="0.25"/><path d="M70,5 Q68,0 72,-5" fill="none" stroke="currentColor" stroke-width="1" opacity="0.3"/></svg>"#,
                                }
                                span { style: "font-size: 9px; font-weight: 700; color: #f472b6; margin-top: 4px;", "Cafe Blossom" }
                            }
                            // Left side
                            div {
                                style: "transform: rotateY(-90deg) translateZ(100px); background: rgba(52,211,153,0.08); border: 1px solid rgba(52,211,153,0.2); position: absolute; inset: 0; border-radius: 14px; backdrop-filter: blur(4px); display: flex; flex-direction: column; align-items: center; justify-content: center;",
                                div {
                                    style: "color: #34d399; width: 60px; height: 50px;",
                                    dangerous_inner_html: r#"<svg viewBox="0 0 120 80"><circle cx="58" cy="12" r="7" fill="currentColor" opacity="0.6"/><path d="M52,20 L52,28 Q52,32 48,36 L40,44 L40,48 Q46,46 50,42 L56,36 L56,48 L50,62 L46,68 L50,70 L56,56 L60,48 L64,56 L70,70 L74,68 L70,62 L64,48 L64,36 L68,40 Q72,44 76,42 L80,38 L78,34 Q74,36 70,34 L64,28 L64,20 Z" fill="currentColor" opacity="0.5"/></svg>"#,
                                }
                                span { style: "font-size: 9px; font-weight: 700; color: #34d399; margin-top: 2px;", "RunPulse" }
                            }
                            // Right side
                            div {
                                style: "transform: rotateY(90deg) translateZ(100px); background: rgba(251,191,36,0.06); border: 1px solid rgba(251,191,36,0.2); position: absolute; inset: 0; border-radius: 14px; backdrop-filter: blur(4px); display: flex; flex-direction: column; align-items: center; justify-content: center;",
                                span { style: "font-size: 24px; font-weight: 800; color: #fbbf24; opacity: 0.6;", "+" }
                                span { style: "font-size: 8px; color: #7a8ba6; margin-top: 2px;", "More Brands" }
                            }
                        }
                    }

                    // Right cube (Stats/Metrics - overlapping, tilted)
                    div {
                        class: "absolute hidden lg:block",
                        style: "right: 10px; bottom: 0%; perspective: 900px; z-index: 2;",
                        // Glow
                        div { style: "position: absolute; top: 50%; left: 50%; transform: translate(-50%,-50%); width: 160px; height: 160px; background: #a78bfa; filter: blur(60px); opacity: 0.12; border-radius: 50%;" }
                        div {
                            style: "width: 200px; height: 200px; position: relative; transform-style: preserve-3d; animation: spinCubeRight 19s linear infinite;",
                            // Face 1: Rising line chart + Floor Price
                            div {
                                style: "transform: translateZ(100px); background: rgba(52,211,153,0.06); border: 1px solid rgba(52,211,153,0.2); position: absolute; inset: 0; border-radius: 14px; backdrop-filter: blur(4px); display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 15px;",
                                div {
                                    style: "width: 100%; height: 60px; color: #34d399;",
                                    dangerous_inner_html: r#"<svg viewBox='0 0 100 50' fill='none' style='width:100%;height:100%;'><path d='M5,42 L20,38 L35,35 L50,28 L65,22 L80,15 L95,8' stroke='currentColor' stroke-width='2' opacity='0.7' stroke-linecap='round'/><path d='M5,42 L20,38 L35,35 L50,28 L65,22 L80,15 L95,8 L95,48 L5,48 Z' fill='currentColor' opacity='0.08'/><circle cx='50' cy='28' r='2.5' fill='currentColor' opacity='0.5'/><circle cx='80' cy='15' r='2.5' fill='currentColor' opacity='0.6'/><circle cx='95' cy='8' r='3' fill='currentColor' opacity='0.8'/></svg>"#,
                                }
                                span { style: "font-size: 8px; color: #34d399; letter-spacing: 1px; text-transform: uppercase; margin-top: 6px;", "Floor Price" }
                            }
                            // Face 2: Stair-step bars + Treasury
                            div {
                                style: "transform: rotateY(180deg) translateZ(100px); background: rgba(96,165,250,0.06); border: 1px solid rgba(96,165,250,0.2); position: absolute; inset: 0; border-radius: 14px; backdrop-filter: blur(4px); display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 15px;",
                                div {
                                    style: "width: 100%; height: 60px; color: #60a5fa;",
                                    dangerous_inner_html: r#"<svg viewBox='0 0 100 50' fill='none' style='width:100%;height:100%;'><rect x='8' y='38' width='10' height='10' rx='2' fill='currentColor' opacity='0.15'/><rect x='22' y='32' width='10' height='16' rx='2' fill='currentColor' opacity='0.2'/><rect x='36' y='26' width='10' height='22' rx='2' fill='currentColor' opacity='0.25'/><rect x='50' y='20' width='10' height='28' rx='2' fill='currentColor' opacity='0.3'/><rect x='64' y='14' width='10' height='34' rx='2' fill='currentColor' opacity='0.4'/><rect x='78' y='6' width='10' height='42' rx='2' fill='currentColor' opacity='0.5'/></svg>"#,
                                }
                                span { style: "font-size: 8px; color: #60a5fa; letter-spacing: 1px; text-transform: uppercase; margin-top: 6px;", "Treasury" }
                            }
                            // Face 3: Growing user icons + Holders
                            div {
                                style: "transform: rotateY(90deg) translateZ(100px); background: rgba(167,139,250,0.06); border: 1px solid rgba(167,139,250,0.2); position: absolute; inset: 0; border-radius: 14px; backdrop-filter: blur(4px); display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 15px;",
                                div {
                                    style: "width: 100%; height: 60px; color: #a78bfa;",
                                    dangerous_inner_html: r#"<svg viewBox='0 0 100 50' fill='none' style='width:100%;height:100%;'><circle cx='20' cy='15' r='5' fill='currentColor' opacity='0.2'/><path d='M12,30 Q12,24 20,22 Q28,24 28,30' fill='currentColor' opacity='0.15'/><circle cx='50' cy='13' r='6' fill='currentColor' opacity='0.3'/><path d='M41,30 Q41,23 50,21 Q59,23 59,30' fill='currentColor' opacity='0.2'/><circle cx='80' cy='10' r='7' fill='currentColor' opacity='0.45'/><path d='M70,30 Q70,22 80,20 Q90,22 90,30' fill='currentColor' opacity='0.3'/><path d='M15,38 L30,36 L50,33 L70,30 L90,26' stroke='currentColor' stroke-width='1' opacity='0.3' stroke-dasharray='3,3'/><polygon points='88,24 93,27 88,29' fill='currentColor' opacity='0.3'/></svg>"#,
                                }
                                span { style: "font-size: 8px; color: #a78bfa; letter-spacing: 1px; text-transform: uppercase; margin-top: 6px;", "Holders" }
                            }
                            // Face 4: Pie/donut chart + Revenue
                            div {
                                style: "transform: rotateY(-90deg) translateZ(100px); background: rgba(244,114,182,0.06); border: 1px solid rgba(244,114,182,0.2); position: absolute; inset: 0; border-radius: 14px; backdrop-filter: blur(4px); display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 15px;",
                                div {
                                    style: "width: 100%; height: 60px; color: #f472b6;",
                                    dangerous_inner_html: r#"<svg viewBox='0 0 100 50' fill='none' style='width:100%;height:100%;'><circle cx='50' cy='25' r='18' fill='none' stroke='currentColor' stroke-width='3' opacity='0.12'/><circle cx='50' cy='25' r='18' fill='none' stroke='currentColor' stroke-width='3' opacity='0.5' stroke-dasharray='85,113' stroke-dashoffset='0' stroke-linecap='round'/><circle cx='50' cy='25' r='12' fill='none' stroke='currentColor' stroke-width='2' opacity='0.08'/><text x='50' y='28' text-anchor='middle' fill='currentColor' font-size='8' font-weight='bold' opacity='0.6'>75%</text></svg>"#,
                                }
                                span { style: "font-size: 8px; color: #f472b6; letter-spacing: 1px; text-transform: uppercase; margin-top: 6px;", "Revenue" }
                            }
                        }
                    }

                    // Main centered cube (original)
                    div {
                        class: "absolute",
                        style: "left: 50%; top: 40%; transform: translate(-50%, -50%); width: 380px; height: 380px;",
                    // Outer glow ring
                    div {
                        class: "absolute rounded-full",
                        style: "top: 50%; left: 50%; transform: translate(-50%, -50%); width: 300px; height: 300px; border: 1px solid #00d4aa; opacity: 0.2; box-shadow: 0 0 60px rgba(0,212,170,0.3), inset 0 0 60px rgba(0,212,170,0.1);",
                    }
                    // Dashed ring
                    div {
                        class: "absolute rounded-full",
                        style: "top: 50%; left: 50%; transform: translate(-50%, -50%); width: 420px; height: 420px; border: 1px dashed rgba(96,165,250,0.3); opacity: 0.15; animation: spinCube 30s linear infinite;",
                    }
                    // Center glow
                    div {
                        class: "absolute rounded-full",
                        style: "top: 50%; left: 50%; transform: translate(-50%, -50%); width: 240px; height: 240px; background: #6366f1; filter: blur(90px); opacity: 0.3;",
                    }
                    // Rotating cube (big: 200x200, translateZ 100px)
                    div {
                        class: "absolute",
                        style: "top: 50%; left: 50%; transform: translate(-50%, -50%); perspective: 1200px;",
                        div {
                            style: "width: 200px; height: 200px; position: relative; transform-style: preserve-3d; animation: spinCubeMain 15s linear infinite;",
                            // Cube faces
                            for (transform, icon, value, label, bg) in [
                                ("translateZ(100px)", "\u{1F4C8}", "$0.0245", "Floor Price", "rgba(96,165,250,0.12)"),
                                ("rotateY(180deg) translateZ(100px)", "\u{1F512}", "$72,600", "Treasury", "rgba(167,139,250,0.12)"),
                                ("rotateY(90deg) translateZ(100px)", "\u{267E}\u{FE0F}", "AUTO", "Buyback", "rgba(99,102,241,0.12)"),
                                ("rotateY(-90deg) translateZ(100px)", "\u{1F525}", "DEFLATION", "Burn", "rgba(244,114,182,0.12)"),
                            ] {
                                {
                                    let face_style = format!(
                                        "transform: {}; background: {}; border: 1px solid rgba(148,163,250,0.2); box-shadow: 0 0 30px rgba(99,102,241,0.12), inset 0 0 30px rgba(99,102,241,0.05); position: absolute; inset: 0; border-radius: 20px; backdrop-filter: blur(4px); display: flex; flex-direction: column; align-items: center; justify-content: center;",
                                        transform, bg
                                    );
                                    rsx! {
                                        div {
                                            style: "{face_style}",
                                            span {
                                                class: "text-3xl mb-1",
                                                style: "filter: drop-shadow(0 0 8px rgba(99,102,241,0.5));",
                                                "{icon}"
                                            }
                                            span {
                                                class: "text-base font-extrabold",
                                                style: "color: #a5b4fc; letter-spacing: -0.025em;",
                                                "{value}"
                                            }
                                            span {
                                                class: "text-xs font-semibold mt-1",
                                                style: "color: #7a8ba6; letter-spacing: 0.1em; text-transform: uppercase; font-size: 9px;",
                                                "{label}"
                                            }
                                        }
                                    }
                                }
                            }
                            // Top and bottom faces (Biyard logo)
                            for transform in ["rotateX(90deg) translateZ(100px)", "rotateX(-90deg) translateZ(100px)"] {
                                {
                                    let face_style = format!(
                                        "transform: {}; background: rgba(0,212,170,0.15); border: 1px solid rgba(0,212,170,0.35); position: absolute; inset: 0; border-radius: 20px; display: flex; align-items: center; justify-content: center;",
                                        transform
                                    );
                                    rsx! {
                                        div {
                                            style: "{face_style}",
                                            span {
                                                class: "text-xl font-black tracking-wider",
                                                style: "color: #00d4aa; filter: drop-shadow(0 0 12px rgba(0,212,170,0.6));",
                                                "Biyard"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    // Orbiting labels around the cube
                    for (text, top, left, delay) in [
                        ("Treasury \u{2191}", "2%", "60%", "0s"),
                        ("Burn \u{1F525}", "82%", "70%", "1.2s"),
                        ("Buyback \u{267E}\u{FE0F}", "80%", "5%", "0.6s"),
                        ("Floor \u{2191}", "5%", "2%", "1.8s"),
                    ] {
                        {
                            let label_style = format!(
                                "top: {}; left: {}; position: absolute; background: rgba(99,102,241,0.08); border: 1px solid rgba(148,163,250,0.2); border-radius: 9999px; padding: 6px 14px; font-size: 11px; font-weight: 600; color: #a5b4fc; letter-spacing: 0.5px; animation: floatParticle 3.5s ease-in-out {} infinite alternate; box-shadow: 0 0 16px rgba(99,102,241,0.1);",
                                top, left, delay
                            );
                            rsx! {
                                div { style: "{label_style}", "{text}" }
                            }
                        }
                    }
                    // Sparkle dots
                    for (top, left, dur) in [("3%", "42%", "2.5s"), ("50%", "2%", "3.2s"), ("92%", "48%", "4s"), ("30%", "95%", "2.8s"), ("65%", "95%", "3.5s"), ("15%", "85%", "2.2s")] {
                        {
                            let dot_style = format!(
                                "position: absolute; top: {}; left: {}; width: 5px; height: 5px; border-radius: 9999px; background: #00d4aa; box-shadow: 0 0 8px #00d4aa, 0 0 16px rgba(0,212,170,0.4); animation: floatParticle {} ease-in-out infinite alternate;",
                                top, left, dur
                            );
                            rsx! {
                                div { style: "{dot_style}" }
                            }
                        }
                    }
                    } // end main cube absolute
                } // end multi-cube container

                // Hero text below cube
                div {
                    class: "text-center max-w-2xl",
                    p {
                        class: "font-medium mb-4 uppercase text-sm",
                        style: "color: #00d4aa; letter-spacing: 0.1em;",
                        "Revenue-Backed Token Platform"
                    }
                    h1 {
                        class: "text-5xl md:text-6xl font-extrabold leading-tight",
                        style: "color: #e8eefc;",
                        "사고, 쓰고, 즐길수록"
                        br {}
                        span {
                            style: "background-image: linear-gradient(to right, #00d4aa, #00d4aa); -webkit-background-clip: text; background-clip: text; color: transparent;",
                            "가치가 커지는 토큰"
                        }
                    }
                    p {
                        class: "mt-6 text-xl max-w-2xl mx-auto leading-relaxed",
                        style: "color: #7a8ba6;",
                        "고객의 구매와 활동이 트레저리에 쌓이고, 그 가치가 모든 토큰 보유자에게 돌아갑니다."
                        br {}
                        "브랜드와 고객이 함께 성장하는 새로운 로열티 플랫폼."
                    }
                    div {
                        class: "mt-10 flex items-center justify-center gap-4 flex-wrap",
                        a {
                            class: "inline-flex items-center px-8 py-4 rounded-xl font-bold text-lg",
                            style: "background: #00d4aa; color: #0c1018; box-shadow: 0 10px 25px rgba(0,212,170,0.3);",
                            href: "#",
                            "도입 문의하기 →"
                        }
                        a {
                            class: "inline-flex items-center px-8 py-4 rounded-xl font-bold text-lg",
                            style: "border: 2px solid rgba(232,238,252,0.3); color: #e8eefc;",
                            href: "#",
                            "ROI 시뮬레이터"
                        }
                    }
                }
            }

        }

        // Before vs After Comparison
        {
            let left_svg = r##"<svg viewBox="0 0 280 260" fill="none" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:280px;height:auto;">
  <!-- Project hexagon node (pulsing) -->
  <polygon points="140,10 170,25 170,55 140,70 110,55 110,25" fill="rgba(239,68,68,0.12)" stroke="#ef4444" stroke-width="1.5" opacity="0.7" style="animation: pulseRed 3s ease-in-out infinite;transform-origin:140px 40px;"/>
  <text x="140" y="44" text-anchor="middle" fill="#ef4444" font-size="11" font-weight="bold">프로젝트</text>
  <text x="140" y="82" text-anchor="middle" fill="#ef4444" font-size="9" opacity="0.6">실적 없는 프로젝트</text>

  <!-- Broken connection lines (animated dash flow) -->
  <line x1="120" y1="60" x2="65" y2="100" stroke="#ef4444" stroke-width="1" stroke-dasharray="4,4" opacity="0.4" style="animation: dashFlow 2s linear infinite;"/>
  <line x1="140" y1="70" x2="140" y2="100" stroke="#ef4444" stroke-width="1" stroke-dasharray="4,4" opacity="0.35" style="animation: dashFlow 2.5s linear infinite;"/>
  <line x1="160" y1="60" x2="215" y2="100" stroke="#ef4444" stroke-width="1" stroke-dasharray="4,4" opacity="0.3" style="animation: dashFlow 3s linear infinite;"/>
  <!-- Scattered broken fragments -->
  <line x1="90" y1="82" x2="100" y2="88" stroke="#ef4444" stroke-width="0.8" opacity="0.15" stroke-dasharray="2,3"/>
  <line x1="180" y1="78" x2="192" y2="85" stroke="#ef4444" stroke-width="0.8" opacity="0.12" stroke-dasharray="2,3"/>

  <!-- Investor person shapes (fading out) -->
  <g style="animation: pulseRed 2.5s ease-in-out infinite;transform-origin:60px 115px;">
    <circle cx="60" cy="108" r="7" fill="none" stroke="#ef4444" stroke-width="1" opacity="0.4"/>
    <path d="M48,130 Q48,120 60,118 Q72,120 72,130" fill="none" stroke="#ef4444" stroke-width="1" opacity="0.3"/>
  </g>
  <g style="animation: pulseRed 3s ease-in-out 0.5s infinite;transform-origin:140px 115px;">
    <circle cx="140" cy="108" r="7" fill="none" stroke="#ef4444" stroke-width="1" opacity="0.3"/>
    <path d="M128,130 Q128,120 140,118 Q152,120 152,130" fill="none" stroke="#ef4444" stroke-width="1" opacity="0.2"/>
  </g>
  <g style="animation: pulseRed 3.5s ease-in-out 1s infinite;transform-origin:220px 115px;">
    <circle cx="220" cy="108" r="7" fill="none" stroke="#ef4444" stroke-width="1" opacity="0.2"/>
    <path d="M208,130 Q208,120 220,118 Q232,120 232,130" fill="none" stroke="#ef4444" stroke-width="1" opacity="0.12"/>
  </g>

  <!-- Label near declining investors -->
  <text x="60" y="148" text-anchor="middle" fill="#ef4444" font-size="8" opacity="0.5">피해 반복</text>

  <!-- Declining chart -->
  <line x1="30" y1="160" x2="30" y2="245" stroke="rgba(239,68,68,0.3)" stroke-width="1"/>
  <line x1="30" y1="245" x2="255" y2="245" stroke="rgba(239,68,68,0.3)" stroke-width="1"/>
  <!-- Grid lines -->
  <line x1="30" y1="185" x2="255" y2="185" stroke="rgba(239,68,68,0.06)" stroke-width="0.5"/>
  <line x1="30" y1="210" x2="255" y2="210" stroke="rgba(239,68,68,0.06)" stroke-width="0.5"/>
  <!-- Area fill under curve -->
  <path d="M30,170 Q70,172 100,182 Q140,196 180,212 Q220,228 255,240 L255,245 L30,245 Z" fill="rgba(239,68,68,0.06)"/>
  <!-- Smooth declining line (animated draw) -->
  <path d="M30,170 Q70,172 100,182 Q140,196 180,212 Q220,228 255,240" stroke="#ef4444" stroke-width="2" fill="none" stroke-linecap="round" stroke-dasharray="300" style="animation: lineGrow 4s ease-out infinite;"/>
  <!-- Down arrow indicator -->
  <path d="M245,225 L250,233 L240,233 Z" fill="#ef4444" opacity="0.4"/>
  <!-- Label near chart -->
  <text x="245" y="220" text-anchor="end" fill="#ef4444" font-size="9" opacity="0.5">가치 하락</text>
</svg>"##;

            let right_svg = r##"<svg viewBox="0 0 280 260" fill="none" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:280px;height:auto;">
  <!-- Arrow marker definition -->
  <defs>
    <marker id="arrowG" markerWidth="6" markerHeight="4" refX="5" refY="2" orient="auto">
      <path d="M0,0 L6,2 L0,4" fill="#00d4aa" opacity="0.5"/>
    </marker>
  </defs>

  <!-- Revenue document/receipt shape (floating) -->
  <g style="animation: nodeFloat 3s ease-in-out infinite;">
  <path d="M120,8 L160,8 L165,13 L165,48 L158,52 L152,48 L146,52 L140,48 L134,52 L128,48 L122,52 L115,48 L115,13 Z" fill="rgba(0,212,170,0.12)" stroke="#00d4aa" stroke-width="1.5" opacity="0.7"/>
  <line x1="125" y1="20" x2="155" y2="20" stroke="#00d4aa" stroke-width="0.8" opacity="0.3"/>
  <line x1="125" y1="27" x2="150" y2="27" stroke="#00d4aa" stroke-width="0.8" opacity="0.25"/>
  <line x1="125" y1="34" x2="145" y2="34" stroke="#00d4aa" stroke-width="0.8" opacity="0.2"/>
  <text x="140" y="46" text-anchor="middle" fill="#00d4aa" font-size="9" font-weight="bold" opacity="0.8">매출</text>
  <text x="172" y="30" text-anchor="start" fill="#00d4aa" font-size="8" opacity="0.55">실제 매출</text>
  </g>

  <!-- Arrow down to treasury (flowing) -->
  <line x1="140" y1="53" x2="140" y2="68" stroke="#00d4aa" stroke-width="1.5" opacity="0.6" marker-end="url(#arrowG)"/>

  <!-- Treasury shield/vault shape (glowing) -->
  <path d="M105,70 L175,70 L175,100 Q175,115 140,120 Q105,115 105,100 Z" fill="rgba(0,212,170,0.1)" stroke="#00d4aa" stroke-width="2" opacity="0.7" style="animation: fadeInOut 4s ease-in-out infinite;filter:drop-shadow(0 0 8px rgba(0,212,170,0.3));"/>
  <path d="M120,85 L135,85 L135,100 L120,100 Z" fill="none" stroke="#00d4aa" stroke-width="1" opacity="0.35"/>
  <path d="M145,85 L160,85 L160,100 L145,100 Z" fill="none" stroke="#00d4aa" stroke-width="1" opacity="0.35"/>
  <circle cx="140" cy="92" r="5" fill="none" stroke="#00d4aa" stroke-width="1" opacity="0.4"/>
  <circle cx="140" cy="92" r="2" fill="#00d4aa" opacity="0.25"/>
  <text x="140" y="114" text-anchor="middle" fill="#00d4aa" font-size="9" font-weight="bold" opacity="0.7">트레저리</text>
  <text x="182" y="92" text-anchor="start" fill="#00d4aa" font-size="8" opacity="0.5">온체인 트레저리</text>

  <!-- Flowing arrows to token holders (animated) -->
  <line x1="115" y1="115" x2="62" y2="142" stroke="#00d4aa" stroke-width="1.2" opacity="0.5" marker-end="url(#arrowG)" stroke-dasharray="6,4" style="animation: dashFlow 2s linear infinite;"/>
  <line x1="140" y1="120" x2="140" y2="142" stroke="#00d4aa" stroke-width="1.2" opacity="0.5" marker-end="url(#arrowG)" stroke-dasharray="6,4" style="animation: dashFlow 2.5s linear infinite;"/>
  <line x1="165" y1="115" x2="218" y2="142" stroke="#00d4aa" stroke-width="1.2" opacity="0.5" marker-end="url(#arrowG)" stroke-dasharray="6,4" style="animation: dashFlow 3s linear infinite;"/>

  <!-- Token holder person shapes (growing pulse) -->
  <g style="animation: pulseGreen 3s ease-in-out infinite;transform-origin:55px 160px;">
    <circle cx="55" cy="150" r="6" fill="none" stroke="#00d4aa" stroke-width="1.2" opacity="0.6"/>
    <path d="M44,168 Q44,160 55,158 Q66,160 66,168" fill="rgba(0,212,170,0.12)" stroke="#00d4aa" stroke-width="1" opacity="0.5"/>
    <circle cx="55" cy="150" r="10" fill="none" stroke="#00d4aa" stroke-width="0.5" opacity="0.15"/>
    <text x="55" y="180" text-anchor="middle" fill="#00d4aa" font-size="7" opacity="0.5">토큰 홀더</text>
  </g>

  <g style="animation: pulseGreen 3s ease-in-out 0.5s infinite;transform-origin:140px 158px;">
    <circle cx="140" cy="148" r="7" fill="none" stroke="#00d4aa" stroke-width="1.2" opacity="0.7"/>
    <path d="M128,168 Q128,159 140,157 Q152,159 152,168" fill="rgba(0,212,170,0.15)" stroke="#00d4aa" stroke-width="1" opacity="0.6"/>
  <circle cx="140" cy="148" r="12" fill="none" stroke="#00d4aa" stroke-width="0.5" opacity="0.18"/>
  <text x="140" y="180" text-anchor="middle" fill="#00d4aa" font-size="7" opacity="0.5">토큰 홀더</text>
  </g>

  <g style="animation: pulseGreen 3s ease-in-out 1s infinite;transform-origin:225px 157px;">
    <circle cx="225" cy="147" r="8" fill="none" stroke="#00d4aa" stroke-width="1.2" opacity="0.8"/>
    <path d="M212,168 Q212,158 225,156 Q238,158 238,168" fill="rgba(0,212,170,0.18)" stroke="#00d4aa" stroke-width="1" opacity="0.7"/>
    <circle cx="225" cy="147" r="14" fill="none" stroke="#00d4aa" stroke-width="0.5" opacity="0.2"/>
    <text x="225" y="180" text-anchor="middle" fill="#00d4aa" font-size="7" opacity="0.5">토큰 홀더</text>
  </g>

  <!-- Label near holders -->
  <text x="140" y="192" text-anchor="middle" fill="#00d4aa" font-size="8" opacity="0.55">가치 상승</text>

  <!-- Rising chart -->
  <line x1="30" y1="205" x2="30" y2="248" stroke="rgba(0,212,170,0.3)" stroke-width="1"/>
  <line x1="30" y1="248" x2="255" y2="248" stroke="rgba(0,212,170,0.3)" stroke-width="1"/>
  <line x1="30" y1="235" x2="255" y2="235" stroke="rgba(0,212,170,0.05)" stroke-width="0.5"/>
  <line x1="30" y1="222" x2="255" y2="222" stroke="rgba(0,212,170,0.05)" stroke-width="0.5"/>
  <!-- Area gradient fill -->
  <path d="M30,246 Q70,244 110,238 Q150,230 190,222 Q230,216 255,210 L255,248 L30,248 Z" fill="rgba(0,212,170,0.08)"/>
  <!-- Smooth rising line (animated draw) -->
  <path d="M30,246 Q70,244 110,238 Q150,230 190,222 Q230,216 255,210" stroke="#00d4aa" stroke-width="2" fill="none" stroke-linecap="round" stroke-dasharray="300" style="animation: lineGrow 4s ease-out infinite;"/>
  <!-- Up arrow indicator -->
  <path d="M248,215 L253,207 L258,215 Z" fill="#00d4aa" opacity="0.5"/>
  <!-- Label near chart -->
  <text x="245" y="205" text-anchor="end" fill="#00d4aa" font-size="9" opacity="0.55">우상향</text>
</svg>"##;

            let transform_svg = r##"<svg viewBox="0 0 80 80" fill="none" xmlns="http://www.w3.org/2000/svg" style="width:72px;height:72px;">
  <circle cx="40" cy="40" r="38" fill="rgba(0,212,170,0.12)" stroke="#00d4aa" stroke-width="2.5" style="animation: transformArrow 2s ease-in-out infinite;"/>
  <circle cx="40" cy="40" r="32" fill="none" stroke="#00d4aa" stroke-width="0.8" opacity="0.25" stroke-dasharray="4,3"/>
  <path d="M22 40 H54 M46 30 L56 40 L46 50" stroke="#00d4aa" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
</svg>"##;

            rsx! {
                section {
                    class: "py-20 px-4 relative overflow-hidden",
                    style: "background: #0c1018;",
                    // Amber/orange glow from top-right
                    div { class: "absolute", style: "top: -100px; right: -100px; width: 500px; height: 500px; background: radial-gradient(circle, rgba(251,191,36,0.08) 0%, transparent 70%); pointer-events: none;" }
                    div { class: "absolute", style: "bottom: -50px; left: -100px; width: 400px; height: 400px; background: radial-gradient(circle, rgba(244,114,182,0.06) 0%, transparent 70%); pointer-events: none;" }
                    div {
                        class: "max-w-6xl mx-auto relative z-10",
                        div {
                            class: "text-center mb-16 reveal-fade",
                            p {
                                class: "text-sm font-semibold tracking-widest uppercase mb-4",
                                style: "color: #00d4aa;",
                                "WHY BIYARD?"
                            }
                            h2 {
                                class: "text-3xl md:text-4xl font-bold reveal-type",
                                style: "color: #e8eefc;",
                                "기존 거래소/증시의 문제, Biyard가 해결합니다"
                            }
                        }
                        div {
                            class: "grid grid-cols-1 lg:grid-cols-[1fr_auto_1fr] gap-6 items-stretch",

                            // LEFT PANEL: 기존 거래소
                            div {
                                class: "rounded-2xl p-8 relative overflow-hidden reveal-bounce",
                                style: "background: rgba(30,30,40,0.6); backdrop-filter: blur(12px); -webkit-backdrop-filter: blur(12px); border: 1px solid rgba(239,68,68,0.15); box-shadow: 0 8px 32px rgba(0,0,0,0.3);",
                                div {
                                    class: "absolute top-4 right-4 px-3 py-1 rounded-full text-xs font-bold",
                                    style: "background: rgba(239,68,68,0.1); color: #ef4444;",
                                    "BEFORE"
                                }
                                h3 {
                                    class: "text-xl font-bold mb-6",
                                    style: "color: #7a8ba6;",
                                    "기존 거래소"
                                }
                                // SVG Diagram
                                div {
                                    class: "flex justify-center mb-6 h-64",
                                    div {
                                        dangerous_inner_html: "{left_svg}",
                                    }
                                }
                                // Bullet points
                                div {
                                    class: "space-y-2",
                                    for text in [
                                        "실적 없는 토큰이 상장되어 투자자 피해 반복",
                                        "무분별한 물량 희석으로 보유자 가치 훼손",
                                        "근거 없는 가격 변동으로 시장 신뢰 상실",
                                        "정보 비대칭 \u{2014} 내부자만 유리한 구조",
                                        "자금 흐름 불투명 \u{2014} 검증 불가능",
                                    ] {
                                        div {
                                            class: "flex items-start gap-2",
                                            div {
                                                class: "w-1.5 h-1.5 rounded-full flex-shrink-0 mt-1.5",
                                                style: "background: #ef4444; opacity: 0.6;",
                                            }
                                            p { class: "text-sm", style: "color: #7a8ba6;", "{text}" }
                                        }
                                    }
                                }
                                // Stats comparison box
                                div {
                                    class: "mt-4 rounded-lg p-3 flex justify-between text-center",
                                    style: "background: rgba(239,68,68,0.06); border: 1px solid rgba(239,68,68,0.12);",
                                    div {
                                        p { class: "text-xs", style: "color: #7a8ba6;", "평균 토큰 수명" }
                                        p { class: "text-sm font-bold", style: "color: #ef4444;", "6개월" }
                                    }
                                    div {
                                        p { class: "text-xs", style: "color: #7a8ba6;", "투자자 손실률" }
                                        p { class: "text-sm font-bold", style: "color: #ef4444;", "80%+" }
                                    }
                                }
                            }

                            // CENTER: Transform arrow
                            div {
                                class: "hidden lg:flex items-center justify-center",
                                div {
                                    class: "flex flex-col items-center gap-3",
                                    div {
                                        dangerous_inner_html: "{transform_svg}",
                                    }
                                    p {
                                        class: "text-xs font-bold tracking-widest",
                                        style: "color: #00d4aa;",
                                        "TRANSFORM"
                                    }
                                    p {
                                        class: "text-sm font-bold mt-1 text-center",
                                        style: "color: #00d4aa;",
                                        "Biyard가 해결합니다"
                                    }
                                }
                            }

                            // RIGHT PANEL: Biyard 런치패드
                            div {
                                class: "rounded-2xl p-8 relative overflow-hidden reveal-bounce",
                                style: "background: rgba(0,212,170,0.05); backdrop-filter: blur(12px); -webkit-backdrop-filter: blur(12px); border: 1px solid rgba(0,212,170,0.2); box-shadow: 0 8px 32px rgba(0,212,170,0.1), inset 0 1px 0 rgba(0,212,170,0.1);",
                                div {
                                    class: "absolute top-4 right-4 px-3 py-1 rounded-full text-xs font-bold",
                                    style: "background: rgba(0,212,170,0.15); color: #00d4aa;",
                                    "AFTER"
                                }
                                h3 {
                                    class: "text-xl font-bold mb-6",
                                    style: "color: #00d4aa;",
                                    "Biyard 런치패드"
                                }
                                // SVG Diagram
                                div {
                                    class: "flex justify-center mb-6 h-64",
                                    div {
                                        dangerous_inner_html: "{right_svg}",
                                    }
                                }
                                // Bullet points
                                div {
                                    class: "space-y-2",
                                    for text in [
                                        "실제 매출이 있는 기업만 토큰 발행 가능",
                                        "트레저리 = 매출 온체인 증명 \u{2014} 가짜 불가",
                                        "하한가 스마트 컨트랙트 \u{2014} 덤핑해도 바닥 보장",
                                        "매출이 늘면 모든 홀더 자산 가치 자동 상승",
                                        "모든 자금 흐름 온체인 공개 \u{2014} 완전한 투명성",
                                    ] {
                                        div {
                                            class: "flex items-start gap-2",
                                            div {
                                                class: "w-1.5 h-1.5 rounded-full flex-shrink-0 mt-1.5",
                                                style: "background: #00d4aa;",
                                            }
                                            p { class: "text-sm", style: "color: #e8eefc;", "{text}" }
                                        }
                                    }
                                }
                                // Stats comparison box
                                div {
                                    class: "mt-4 rounded-lg p-3 flex justify-between text-center",
                                    style: "background: rgba(0,212,170,0.06); border: 1px solid rgba(0,212,170,0.12);",
                                    div {
                                        p { class: "text-xs", style: "color: #7a8ba6;", "매출 기반 가치 보장" }
                                        p { class: "text-sm font-bold", style: "color: #00d4aa;", "100%" }
                                    }
                                    div {
                                        p { class: "text-xs", style: "color: #7a8ba6;", "온체인 투명성" }
                                        p { class: "text-sm font-bold", style: "color: #00d4aa;", "24/7" }
                                    }
                                }
                            }
                        }

                        // Bottom summary stats (compact)
                        div {
                            class: "grid grid-cols-3 gap-4 mt-10",
                            for (value, label) in [
                                ("100%", "매출 기반 가치 보장"),
                                ("0%", "스캠 토큰 상장 가능성"),
                                ("24/7", "온체인 투명성 공개"),
                            ] {
                                div {
                                    class: "text-center rounded-xl py-4 backdrop-blur-md",
                                    style: "background: rgba(20,28,43,0.5); border: 1px solid rgba(0,212,170,0.15); box-shadow: 0 4px 16px rgba(0,0,0,0.2);",
                                    p {
                                        class: "text-2xl font-extrabold",
                                        style: "color: #00d4aa;",
                                        "{value}"
                                    }
                                    p {
                                        class: "text-xs mt-1",
                                        style: "color: #7a8ba6;",
                                        "{label}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // How It Works
        section {
            class: "py-20 px-4 relative overflow-hidden",
            style: "background: #141c2b;",
            // Purple glow from left
            div { class: "absolute", style: "top: 50%; left: -150px; transform: translateY(-50%); width: 500px; height: 500px; background: radial-gradient(circle, rgba(167,139,250,0.1) 0%, transparent 70%); pointer-events: none;" }
            div { class: "absolute", style: "bottom: -100px; right: -50px; width: 400px; height: 400px; background: radial-gradient(circle, rgba(96,165,250,0.06) 0%, transparent 70%); pointer-events: none;" }
            div {
                class: "max-w-5xl mx-auto text-center mb-16 reveal-fade",
                h2 {
                    class: "text-3xl md:text-4xl font-bold mb-4 reveal-type",
                    style: "color: #e8eefc;",
                    "어떻게 작동하나요?"
                }
                p {
                    class: "text-lg max-w-2xl mx-auto",
                    style: "color: #7a8ba6;",
                    "고객이 제품을 구매하면 매출의 일부가 트레저리에 적립되고, 제품을 사용하며 활동할수록 추가 리워드를 받습니다."
                }
            }
            div {
                class: "max-w-5xl mx-auto",
                div {
                    class: "grid grid-cols-1 md:grid-cols-4 gap-6",
                    for (si, (step, title, desc, accent, rgb)) in [
                        ("01", "고객이 구매", "제품/서비스 구매 금액의 2~4%가 자동 적립", "#60a5fa", "96,165,250"),
                        ("02", "트레저리 적립", "적립금이 트레저리에 누적되어 토큰 가치를 뒷받침", "#a78bfa", "167,139,250"),
                        ("03", "활동 인증", "걷기, 러닝, 방문 등 활동이 인증되면 추가 리워드", "#fbbf24", "251,191,36"),
                        ("04", "함께 성장", "고객이 늘수록 트레저리↑ 토큰 가치↑ 모두가 이득", "#34d399", "52,211,153"),
                    ].iter().enumerate() {
                        {
                            let card_style = format!(
                                "background: rgba(10,16,26,0.6); backdrop-filter: blur(16px); -webkit-backdrop-filter: blur(16px); border: 1px solid rgba({},0.15); box-shadow: 0 12px 40px rgba(0,0,0,0.3), inset 0 1px 0 rgba(255,255,255,0.04);",
                                rgb
                            );
                            let line_style = format!("background: linear-gradient(90deg, transparent, rgba({},0.4), transparent);", rgb);
                            let step_color = format!("color: {};", accent);
                            let glow_style = format!("background: {}; filter: blur(50px); opacity: 0.12;", accent);
                            let svgs: [&str; 4] = [
                                // Card 1: Credit card connected by dotted lines to cart, data particles flowing
                                r#"<rect x="30" y="50" width="100" height="65" rx="8" fill="none" stroke="currentColor" stroke-width="1.2" opacity="0.5"/>
<line x1="42" y1="68" x2="82" y2="68" stroke="currentColor" stroke-width="1" opacity="0.3"/>
<rect x="42" y="78" width="50" height="6" rx="2" fill="none" stroke="currentColor" stroke-width="0.8" opacity="0.25"/>
<rect x="42" y="90" width="30" height="6" rx="2" fill="none" stroke="currentColor" stroke-width="0.8" opacity="0.2"/>
<circle cx="115" cy="100" r="3" fill="currentColor" opacity="0.15"/>
<line x1="132" y1="82" x2="180" y2="82" stroke="currentColor" stroke-width="1" opacity="0.3" stroke-dasharray="4,4"/>
<circle cx="140" cy="82" r="2.5" fill="currentColor" opacity="0.6" style="animation: floatParticle 2s ease-in-out infinite alternate;"/>
<circle cx="155" cy="82" r="2" fill="currentColor" opacity="0.4" style="animation: floatParticle 2.5s ease-in-out 0.3s infinite alternate;"/>
<circle cx="168" cy="82" r="1.5" fill="currentColor" opacity="0.3" style="animation: floatParticle 2s ease-in-out 0.6s infinite alternate;"/>
<rect x="190" y="55" width="55" height="60" rx="6" fill="none" stroke="currentColor" stroke-width="1.2" opacity="0.4"/>
<line x1="200" y1="75" x2="235" y2="75" stroke="currentColor" stroke-width="0.8" opacity="0.2"/>
<circle cx="217" cy="90" r="8" fill="none" stroke="currentColor" stroke-width="1" opacity="0.3"/>
<line x1="213" y1="90" x2="221" y2="90" stroke="currentColor" stroke-width="0.8" opacity="0.3"/>
<line x1="217" y1="86" x2="217" y2="94" stroke="currentColor" stroke-width="0.8" opacity="0.3"/>
<circle cx="80" cy="140" r="3" fill="currentColor" opacity="0.1"/>
<line x1="80" y1="118" x2="80" y2="136" stroke="currentColor" stroke-width="0.8" opacity="0.15" stroke-dasharray="3,3"/>
<circle cx="217" cy="120" r="3" fill="currentColor" opacity="0.1"/>
<line x1="217" y1="118" x2="217" y2="136" stroke="currentColor" stroke-width="0.8" opacity="0.15" stroke-dasharray="3,3"/>"#,
                                // Card 2: Vault/safe with stacking coins, arrows showing inflow
                                r#"<rect x="95" y="45" width="90" height="75" rx="10" fill="none" stroke="currentColor" stroke-width="1.2" opacity="0.4"/>
<rect x="105" y="37" width="70" height="14" rx="5" fill="none" stroke="currentColor" stroke-width="1" opacity="0.3"/>
<circle cx="140" cy="80" r="14" fill="none" stroke="currentColor" stroke-width="1.5" opacity="0.5"/>
<circle cx="140" cy="80" r="5" fill="currentColor" opacity="0.15"/>
<rect x="138" y="80" width="4" height="12" rx="1.5" fill="currentColor" opacity="0.25"/>
<line x1="30" y1="60" x2="90" y2="70" stroke="currentColor" stroke-width="1" opacity="0.25" stroke-dasharray="4,4"/>
<polygon points="88,68 93,72 88,74" fill="currentColor" opacity="0.3"/>
<line x1="30" y1="90" x2="90" y2="85" stroke="currentColor" stroke-width="1" opacity="0.2" stroke-dasharray="4,4"/>
<polygon points="88,83 93,86 88,88" fill="currentColor" opacity="0.25"/>
<line x1="30" y1="120" x2="90" y2="100" stroke="currentColor" stroke-width="1" opacity="0.15" stroke-dasharray="4,4"/>
<polygon points="88,98 93,101 88,103" fill="currentColor" opacity="0.2"/>
<circle cx="25" cy="58" r="4" fill="currentColor" opacity="0.12"/>
<circle cx="22" cy="90" r="3.5" fill="currentColor" opacity="0.1"/>
<circle cx="25" cy="122" r="3" fill="currentColor" opacity="0.08"/>
<ellipse cx="220" cy="100" rx="18" ry="5" fill="none" stroke="currentColor" stroke-width="1" opacity="0.2"/>
<ellipse cx="220" cy="95" rx="18" ry="5" fill="none" stroke="currentColor" stroke-width="1" opacity="0.25"/>
<ellipse cx="220" cy="90" rx="18" ry="5" fill="none" stroke="currentColor" stroke-width="1" opacity="0.3"/>
<ellipse cx="220" cy="85" rx="18" ry="5" fill="none" stroke="currentColor" stroke-width="1" opacity="0.35"/>
<line x1="188" y1="82" x2="200" y2="88" stroke="currentColor" stroke-width="0.8" opacity="0.2" stroke-dasharray="3,3"/>"#,
                                // Card 3: Path/route with checkpoint nodes, verification flow
                                r#"<circle cx="35" cy="130" r="6" fill="none" stroke="currentColor" stroke-width="1.2" opacity="0.3"/>
<circle cx="35" cy="130" r="2" fill="currentColor" opacity="0.2"/>
<path d="M42,125 Q65,100 80,105 Q95,110 105,85 Q115,60 140,65 Q165,70 175,45 Q185,20 220,30" fill="none" stroke="currentColor" stroke-width="1.2" opacity="0.4" stroke-linecap="round"/>
<circle cx="80" cy="105" r="10" fill="none" stroke="currentColor" stroke-width="1" opacity="0.25"/>
<path d="M75,105 L79,109 L86,101" fill="none" stroke="currentColor" stroke-width="1.2" opacity="0.5"/>
<circle cx="105" cy="85" r="10" fill="none" stroke="currentColor" stroke-width="1" opacity="0.3"/>
<path d="M100,85 L104,89 L111,81" fill="none" stroke="currentColor" stroke-width="1.2" opacity="0.55"/>
<circle cx="140" cy="65" r="10" fill="none" stroke="currentColor" stroke-width="1" opacity="0.35"/>
<path d="M135,65 L139,69 L146,61" fill="none" stroke="currentColor" stroke-width="1.2" opacity="0.65"/>
<circle cx="175" cy="45" r="10" fill="none" stroke="currentColor" stroke-width="1" opacity="0.4"/>
<path d="M170,45 L174,49 L181,41" fill="none" stroke="currentColor" stroke-width="1.2" opacity="0.75"/>
<circle cx="220" cy="30" r="12" fill="none" stroke="currentColor" stroke-width="1.5" opacity="0.5"/>
<path d="M214,30 L219,35 L228,25" fill="none" stroke="currentColor" stroke-width="1.5" opacity="0.8"/>
<line x1="55" y1="140" x2="250" y2="140" stroke="currentColor" stroke-width="0.8" opacity="0.1"/>
<circle cx="80" cy="140" r="2" fill="currentColor" opacity="0.08"/>
<circle cx="140" cy="140" r="2" fill="currentColor" opacity="0.08"/>
<circle cx="200" cy="140" r="2" fill="currentColor" opacity="0.08"/>"#,
                                // Card 4: Ascending chart with nodes, connected to user icons
                                r#"<line x1="30" y1="140" x2="30" y2="20" stroke="currentColor" stroke-width="0.8" opacity="0.15"/>
<line x1="30" y1="140" x2="250" y2="140" stroke="currentColor" stroke-width="0.8" opacity="0.15"/>
<line x1="30" y1="110" x2="250" y2="110" stroke="currentColor" stroke-width="0.5" opacity="0.06"/>
<line x1="30" y1="80" x2="250" y2="80" stroke="currentColor" stroke-width="0.5" opacity="0.06"/>
<line x1="30" y1="50" x2="250" y2="50" stroke="currentColor" stroke-width="0.5" opacity="0.06"/>
<path d="M40,125 L80,110 L120,90 L160,65 L200,42 L240,25" fill="none" stroke="currentColor" stroke-width="1.5" opacity="0.6" stroke-linecap="round"/>
<circle cx="40" cy="125" r="4" fill="currentColor" opacity="0.15" stroke="currentColor" stroke-width="1" opacity="0.3"/>
<circle cx="80" cy="110" r="4" fill="currentColor" opacity="0.2" stroke="currentColor" stroke-width="1" opacity="0.35"/>
<circle cx="120" cy="90" r="5" fill="currentColor" opacity="0.25" stroke="currentColor" stroke-width="1" opacity="0.4"/>
<circle cx="160" cy="65" r="5" fill="currentColor" opacity="0.3" stroke="currentColor" stroke-width="1" opacity="0.45"/>
<circle cx="200" cy="42" r="5" fill="currentColor" opacity="0.35" stroke="currentColor" stroke-width="1" opacity="0.5"/>
<circle cx="240" cy="25" r="6" fill="currentColor" opacity="0.4" stroke="currentColor" stroke-width="1.2" opacity="0.6"/>
<polygon points="240,20 248,28 243,28" fill="currentColor" opacity="0.5"/>
<circle cx="80" cy="155" r="5" fill="none" stroke="currentColor" stroke-width="0.8" opacity="0.2"/>
<circle cx="80" cy="152" r="2" fill="none" stroke="currentColor" stroke-width="0.8" opacity="0.2"/>
<circle cx="140" cy="155" r="5" fill="none" stroke="currentColor" stroke-width="0.8" opacity="0.25"/>
<circle cx="140" cy="152" r="2" fill="none" stroke="currentColor" stroke-width="0.8" opacity="0.25"/>
<circle cx="200" cy="155" r="5" fill="none" stroke="currentColor" stroke-width="0.8" opacity="0.3"/>
<circle cx="200" cy="152" r="2" fill="none" stroke="currentColor" stroke-width="0.8" opacity="0.3"/>
<line x1="80" y1="148" x2="80" y2="140" stroke="currentColor" stroke-width="0.8" opacity="0.15" stroke-dasharray="2,2"/>
<line x1="140" y1="148" x2="140" y2="140" stroke="currentColor" stroke-width="0.8" opacity="0.15" stroke-dasharray="2,2"/>
<line x1="200" y1="148" x2="200" y2="140" stroke="currentColor" stroke-width="0.8" opacity="0.15" stroke-dasharray="2,2"/>"#,
                            ];
                            let svg_html = svgs[si].to_string();
                            let svg_full = format!("<svg viewBox='0 0 280 170' style='width:100%;height:100%;'>{}</svg>", svg_html);
                            rsx! {
                                div {
                                    class: "text-center rounded-2xl relative overflow-hidden reveal-bounce",
                                    style: "{card_style}",
                                    // Top accent line
                                    div {
                                        class: "absolute top-0 left-[10%] right-[10%] h-[1px]",
                                        style: "{line_style}",
                                    }
                                    // SVG illustration (large, Galxe-style)
                                    div {
                                        class: "w-full h-48 flex items-center justify-center relative px-3 pt-4",
                                        // Glow behind
                                        div {
                                            class: "absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-32 h-32 rounded-full",
                                            style: "{glow_style}",
                                        }
                                        div {
                                            style: "color: {accent}; width: 100%; height: 100%;",
                                            dangerous_inner_html: "{svg_full}",
                                        }
                                    }
                                    // Text
                                    div {
                                        class: "px-5 pb-5",
                                        p {
                                            class: "text-xs font-bold mb-1",
                                            style: "{step_color}",
                                            "STEP {step}"
                                        }
                                        h3 {
                                            class: "text-lg font-bold mb-2",
                                            style: "color: #e8eefc;",
                                            "{title}"
                                        }
                                        p {
                                            class: "text-sm",
                                            style: "color: #7a8ba6;",
                                            "{desc}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Brand Showcase - Individual full-width sections
        {
            let brand_accents: [&str; 3] = ["#60a5fa", "#f472b6", "#34d399"];
            let brand_rgbs: [&str; 3] = ["96,165,250", "244,114,182", "52,211,153"];
            let section_bgs: [&str; 3] = ["#0c1018", "#141c2b", "#0c1018"];

            // Step SVG icons (Galxe-style thin-line)
            let step_svgs: [[&str; 4]; 3] = [
                [
                    // Le Mouton step 1: Shoe purchase
                    r#"<svg viewBox='0 0 160 120' fill='none' style='width:100%;height:100%;'><path d='M15,75 Q15,60 25,55 Q35,50 50,48 L75,46 Q85,45 95,48 Q105,52 110,60 L112,68 L112,72 Q112,78 105,78 L20,78 Q15,78 15,75 Z' fill='none' stroke='currentColor' stroke-width='1.8' opacity='0.7'/><path d='M12,78 L115,78 Q115,85 108,85 L18,85 Q12,85 12,78 Z' fill='currentColor' opacity='0.12' stroke='currentColor' stroke-width='1.5' opacity='0.5'/><path d='M50,50 L58,47 M60,52 L68,49 M70,53 L78,50' fill='none' stroke='currentColor' stroke-width='1' opacity='0.4'/><path d='M22,82 L26,82 M32,82 L36,82 M42,82 L46,82 M52,82 L56,82 M62,82 L66,82 M72,82 L76,82 M82,82 L86,82 M92,82 L96,82' fill='none' stroke='currentColor' stroke-width='0.6' opacity='0.25'/></svg>"#,
                    // Le Mouton step 2: Treasury vault
                    r#"<svg viewBox='0 0 160 120' fill='none' style='width:100%;height:100%;'><rect x='40' y='25' width='80' height='65' rx='10' fill='none' stroke='currentColor' stroke-width='1.2' opacity='0.4'/><rect x='50' y='18' width='60' height='12' rx='5' fill='none' stroke='currentColor' stroke-width='1' opacity='0.3'/><circle cx='80' cy='55' r='14' fill='none' stroke='currentColor' stroke-width='1.5' opacity='0.5'/><circle cx='80' cy='55' r='5' fill='currentColor' opacity='0.15'/><rect x='78' y='55' width='4' height='12' rx='1.5' fill='currentColor' opacity='0.25'/><path d='M15,40 Q25,38 38,42' fill='none' stroke='currentColor' stroke-width='1' opacity='0.3' stroke-dasharray='4,4'/><polygon points='36,40 40,43 36,45' fill='currentColor' opacity='0.3'/><path d='M15,60 Q25,58 38,55' fill='none' stroke='currentColor' stroke-width='1' opacity='0.25' stroke-dasharray='4,4'/><polygon points='36,53 40,56 36,58' fill='currentColor' opacity='0.25'/><ellipse cx='135' cy='70' rx='15' ry='4' fill='none' stroke='currentColor' stroke-width='1' opacity='0.25'/><ellipse cx='135' cy='66' rx='15' ry='4' fill='none' stroke='currentColor' stroke-width='1' opacity='0.3'/><ellipse cx='135' cy='62' rx='15' ry='4' fill='none' stroke='currentColor' stroke-width='1' opacity='0.35'/></svg>"#,
                    // Le Mouton step 3: Walking verification (pictogram person)
                    r#"<svg viewBox='0 0 160 120' fill='none' style='width:100%;height:100%;'><circle cx='45' cy='18' r='7' fill='currentColor' opacity='0.5'/><path d='M39,26 L39,38 Q39,42 35,46 L28,54 L28,58 Q34,56 38,52 L44,44 L44,60 L38,76 L34,82 L38,84 L44,68 L48,58 L52,68 L58,84 L62,82 L58,76 L52,60 L52,44 L56,48 Q60,52 64,50 L68,46 L66,42 Q62,44 58,42 L52,38 L52,26 Z' fill='currentColor' opacity='0.45'/><circle cx='95' cy='50' r='18' fill='none' stroke='currentColor' stroke-width='1' opacity='0.25'/><path d='M89,50 L93,54 L102,45' fill='none' stroke='currentColor' stroke-width='1.5' opacity='0.6'/><path d='M115,30 L125,30 L130,20 L135,40 L140,25 L145,35 L150,30' fill='none' stroke='currentColor' stroke-width='1' opacity='0.3'/></svg>"#,
                    // Le Mouton step 4: Growth chart
                    r#"<svg viewBox='0 0 160 120' fill='none' style='width:100%;height:100%;'><line x1='20' y1='95' x2='20' y2='15' stroke='currentColor' stroke-width='0.8' opacity='0.15'/><line x1='20' y1='95' x2='145' y2='95' stroke='currentColor' stroke-width='0.8' opacity='0.15'/><line x1='20' y1='75' x2='145' y2='75' stroke='currentColor' stroke-width='0.5' opacity='0.06'/><line x1='20' y1='55' x2='145' y2='55' stroke='currentColor' stroke-width='0.5' opacity='0.06'/><line x1='20' y1='35' x2='145' y2='35' stroke='currentColor' stroke-width='0.5' opacity='0.06'/><path d='M25,85 L50,75 L75,60 L100,42 L125,28 L140,20' fill='none' stroke='currentColor' stroke-width='1.5' opacity='0.6' stroke-linecap='round'/><circle cx='50' cy='75' r='3' fill='currentColor' opacity='0.2' stroke='currentColor' stroke-width='1'/><circle cx='75' cy='60' r='3' fill='currentColor' opacity='0.25' stroke='currentColor' stroke-width='1'/><circle cx='100' cy='42' r='4' fill='currentColor' opacity='0.3' stroke='currentColor' stroke-width='1'/><circle cx='125' cy='28' r='4' fill='currentColor' opacity='0.35' stroke='currentColor' stroke-width='1'/><circle cx='140' cy='20' r='5' fill='currentColor' opacity='0.4' stroke='currentColor' stroke-width='1.2'/><polygon points='140,16 146,22 142,22' fill='currentColor' opacity='0.5'/></svg>"#,
                ],
                [
                    // Cafe Blossom step 1: Coffee cup
                    r#"<svg viewBox='0 0 160 120' fill='none' style='width:100%;height:100%;'><path d='M45,30 Q45,25 55,25 L85,25 Q95,25 95,30 L92,60 Q92,65 82,65 L58,65 Q48,65 48,60 Z' fill='none' stroke='currentColor' stroke-width='1.8' opacity='0.7'/><ellipse cx='70' cy='27' rx='22' ry='4' fill='currentColor' opacity='0.08' stroke='currentColor' stroke-width='1' opacity='0.4'/><path d='M95,35 Q110,35 110,47 Q110,57 95,57' fill='none' stroke='currentColor' stroke-width='1.5' opacity='0.5'/><path d='M60,20 Q58,15 62,10' fill='none' stroke='currentColor' stroke-width='1' opacity='0.25'/><path d='M70,18 Q68,12 72,7' fill='none' stroke='currentColor' stroke-width='1' opacity='0.2'/><path d='M80,20 Q78,15 82,10' fill='none' stroke='currentColor' stroke-width='1' opacity='0.25'/></svg>"#,
                    // Cafe Blossom step 2: Treasury
                    r#"<svg viewBox='0 0 160 120' fill='none' style='width:100%;height:100%;'><rect x='40' y='25' width='80' height='65' rx='10' fill='none' stroke='currentColor' stroke-width='1.2' opacity='0.4'/><rect x='50' y='18' width='60' height='12' rx='5' fill='none' stroke='currentColor' stroke-width='1' opacity='0.3'/><circle cx='80' cy='55' r='14' fill='none' stroke='currentColor' stroke-width='1.5' opacity='0.5'/><circle cx='80' cy='55' r='5' fill='currentColor' opacity='0.15'/><rect x='78' y='55' width='4' height='12' rx='1.5' fill='currentColor' opacity='0.25'/></svg>"#,
                    // Cafe Blossom step 3: Stamp card
                    r#"<svg viewBox='0 0 160 120' fill='none' style='width:100%;height:100%;'><rect x='30' y='25' width='100' height='65' rx='8' fill='none' stroke='currentColor' stroke-width='1.2' opacity='0.4'/><circle cx='50' cy='48' r='8' fill='currentColor' opacity='0.15' stroke='currentColor' stroke-width='1'/><path d='M47,48 L49,50 L54,45' fill='none' stroke='currentColor' stroke-width='1.2' opacity='0.6'/><circle cx='75' cy='48' r='8' fill='currentColor' opacity='0.15' stroke='currentColor' stroke-width='1'/><path d='M72,48 L74,50 L79,45' fill='none' stroke='currentColor' stroke-width='1.2' opacity='0.6'/><circle cx='100' cy='48' r='8' fill='currentColor' opacity='0.15' stroke='currentColor' stroke-width='1'/><path d='M97,48 L99,50 L104,45' fill='none' stroke='currentColor' stroke-width='1.2' opacity='0.6'/><circle cx='50' cy='72' r='8' fill='none' stroke='currentColor' stroke-width='1' opacity='0.25' stroke-dasharray='3,3'/><circle cx='75' cy='72' r='8' fill='none' stroke='currentColor' stroke-width='1' opacity='0.25' stroke-dasharray='3,3'/><circle cx='100' cy='72' r='8' fill='none' stroke='currentColor' stroke-width='1' opacity='0.2' stroke-dasharray='3,3'/></svg>"#,
                    // Cafe Blossom step 4: Growth
                    r#"<svg viewBox='0 0 160 120' fill='none' style='width:100%;height:100%;'><line x1='20' y1='95' x2='20' y2='15' stroke='currentColor' stroke-width='0.8' opacity='0.15'/><line x1='20' y1='95' x2='145' y2='95' stroke='currentColor' stroke-width='0.8' opacity='0.15'/><path d='M25,85 L50,75 L75,60 L100,42 L125,28 L140,20' fill='none' stroke='currentColor' stroke-width='1.5' opacity='0.6' stroke-linecap='round'/><circle cx='50' cy='75' r='3' fill='currentColor' opacity='0.2' stroke='currentColor' stroke-width='1'/><circle cx='75' cy='60' r='3' fill='currentColor' opacity='0.25' stroke='currentColor' stroke-width='1'/><circle cx='100' cy='42' r='4' fill='currentColor' opacity='0.3' stroke='currentColor' stroke-width='1'/><circle cx='140' cy='20' r='5' fill='currentColor' opacity='0.4' stroke='currentColor' stroke-width='1.2'/><polygon points='140,16 146,22 142,22' fill='currentColor' opacity='0.5'/></svg>"#,
                ],
                [
                    // RunPulse step 1: Smartband purchase
                    r#"<svg viewBox='0 0 160 120' fill='none' style='width:100%;height:100%;'><rect x='55' y='15' width='50' height='85' rx='22' fill='none' stroke='currentColor' stroke-width='1.5' opacity='0.5'/><rect x='62' y='35' width='36' height='30' rx='4' fill='none' stroke='currentColor' stroke-width='1' opacity='0.4'/><circle cx='80' cy='50' r='8' fill='none' stroke='currentColor' stroke-width='1' opacity='0.3'/><path d='M76,50 L79,53 L85,47' fill='none' stroke='currentColor' stroke-width='1.2' opacity='0.5'/><line x1='65' y1='72' x2='95' y2='72' stroke='currentColor' stroke-width='0.8' opacity='0.2'/><circle cx='80' cy='80' r='2' fill='currentColor' opacity='0.2'/></svg>"#,
                    // RunPulse step 2: Treasury
                    r#"<svg viewBox='0 0 160 120' fill='none' style='width:100%;height:100%;'><rect x='40' y='25' width='80' height='65' rx='10' fill='none' stroke='currentColor' stroke-width='1.2' opacity='0.4'/><rect x='50' y='18' width='60' height='12' rx='5' fill='none' stroke='currentColor' stroke-width='1' opacity='0.3'/><circle cx='80' cy='55' r='14' fill='none' stroke='currentColor' stroke-width='1.5' opacity='0.5'/><circle cx='80' cy='55' r='5' fill='currentColor' opacity='0.15'/><rect x='78' y='55' width='4' height='12' rx='1.5' fill='currentColor' opacity='0.25'/></svg>"#,
                    // RunPulse step 3: Running person (pictogram silhouette)
                    r#"<svg viewBox='0 0 160 120' fill='none' style='width:100%;height:100%;'><circle cx='58' cy='15' r='8' fill='currentColor' opacity='0.5'/><path d='M50,24 L48,36 Q47,40 42,44 L32,52 L34,56 Q40,52 46,46 L52,38 L50,56 L42,76 L38,84 L44,86 L52,66 L58,52 L64,66 L72,86 L78,84 L70,76 L62,56 L64,38 L70,44 Q76,50 82,46 L88,40 L84,36 Q78,40 72,36 L64,28 L66,24 Z' fill='currentColor' opacity='0.45'/><circle cx='115' cy='50' r='18' fill='none' stroke='currentColor' stroke-width='1' opacity='0.25'/><path d='M109,50 L113,54 L122,45' fill='none' stroke='currentColor' stroke-width='1.5' opacity='0.6'/></svg>"#,
                    // RunPulse step 4: Growth
                    r#"<svg viewBox='0 0 160 120' fill='none' style='width:100%;height:100%;'><line x1='20' y1='95' x2='20' y2='15' stroke='currentColor' stroke-width='0.8' opacity='0.15'/><line x1='20' y1='95' x2='145' y2='95' stroke='currentColor' stroke-width='0.8' opacity='0.15'/><path d='M25,85 L50,75 L75,60 L100,42 L125,28 L140,20' fill='none' stroke='currentColor' stroke-width='1.5' opacity='0.6' stroke-linecap='round'/><circle cx='50' cy='75' r='3' fill='currentColor' opacity='0.2' stroke='currentColor' stroke-width='1'/><circle cx='75' cy='60' r='3' fill='currentColor' opacity='0.25' stroke='currentColor' stroke-width='1'/><circle cx='100' cy='42' r='4' fill='currentColor' opacity='0.3' stroke='currentColor' stroke-width='1'/><circle cx='140' cy='20' r='5' fill='currentColor' opacity='0.4' stroke='currentColor' stroke-width='1.2'/><polygon points='140,16 146,22 142,22' fill='currentColor' opacity='0.5'/></svg>"#,
                ],
            ];

            rsx! {
                // Section header
                section {
                    class: "py-12 px-4",
                    style: "background: #0c1018;",
                    div {
                        class: "max-w-6xl mx-auto text-center reveal-fade",
                        p {
                            class: "text-sm font-semibold tracking-widest uppercase mb-3",
                            style: "color: #00d4aa;",
                            "USE CASES"
                        }
                        h2 {
                            class: "text-3xl md:text-4xl font-bold mb-2 reveal-type",
                            style: "color: #e8eefc;",
                            "다양한 브랜드, 하나의 플랫폼"
                        }
                        p {
                            class: "text-base",
                            style: "color: #7a8ba6;",
                            "어떤 산업이든 매출 기반 토큰 이코노미를 구축할 수 있습니다"
                        }
                    }
                }
                // Individual brand sections
                for (bi, brand) in BRAND_SHOWCASES.iter().enumerate() {
                    {
                        let accent = brand_accents[bi];
                        let rgb = brand_rgbs[bi];
                        let bg = section_bgs[bi];
                        let section_style = format!("background: {};", bg);
                        let badge_style = format!("background: rgba({},0.15); color: {};", rgb, accent);
                        let tagline_color = format!("color: {};", accent);

                        // Pre-compute scenario strings
                        let purchase_price_str = format_won(brand.scenario.purchase_price);
                        let reward_amount_str = format_won(brand.scenario.reward_amount);
                        let reward_rate_str = format!("{}%", brand.scenario.reward_rate);
                        let activity_reward_str = format_won(brand.scenario.activity_reward);
                        let monthly_reward_str = format_won(brand.scenario.monthly_reward);
                        let six_month_str = format_won(brand.scenario.six_month_total);

                        let treasury_str = format_usd(brand.stats.treasury);
                        let users_str = format_number(brand.stats.users);
                        let floor_str = format!("{:.4}", brand.stats.floor_price);

                        // Step card styles
                        let step_card_style = format!(
                            "background: rgba(10,16,26,0.6); backdrop-filter: blur(16px); -webkit-backdrop-filter: blur(16px); border: 1px solid rgba({},0.15); box-shadow: 0 12px 40px rgba(0,0,0,0.3), inset 0 1px 0 rgba(255,255,255,0.04);",
                            rgb
                        );
                        let line_style = format!("background: linear-gradient(90deg, transparent, rgba({},0.4), transparent);", rgb);
                        let glow_style = format!("background: {}; filter: blur(50px); opacity: 0.12;", accent);
                        let step_color = format!("color: {};", accent);
                        let quote_border = format!("border-left: 3px solid {};", accent);
                        let stats_border = format!("border: 1px solid rgba({},0.18);", rgb);
                        let detail_card_style = format!(
                            "background: rgba(10,16,26,0.5); border: 1px solid rgba({},0.12); box-shadow: 0 4px 16px rgba(0,0,0,0.2);",
                            rgb
                        );
                        let detail_label_color = format!("color: {};", accent);

                        // Pre-compute step SVGs
                        let svg0 = step_svgs[bi][0].to_string();
                        let svg1 = step_svgs[bi][1].to_string();
                        let svg2 = step_svgs[bi][2].to_string();
                        let svg3 = step_svgs[bi][3].to_string();
                        let step_svg_list = [svg0, svg1, svg2, svg3];

                        rsx! {
                            section {
                                class: "py-16 px-4 relative overflow-hidden",
                                style: "{section_style}",
                                div {
                                    class: "max-w-6xl mx-auto relative z-10",
                                    // Header: brand name + badge + tagline
                                    div {
                                        class: "text-center mb-10 reveal-fade",
                                        div {
                                            class: "flex items-center justify-center gap-3 mb-2",
                                            h3 {
                                                class: "text-2xl md:text-3xl font-bold",
                                                style: "color: #e8eefc;",
                                                "{brand.brand}"
                                            }
                                            span {
                                                class: "text-xs font-medium px-3 py-1 rounded-full",
                                                style: "{badge_style}",
                                                "{brand.segment}"
                                            }
                                        }
                                        p {
                                            class: "text-lg font-semibold",
                                            style: "{tagline_color}",
                                            "{brand.tagline}"
                                        }
                                        p {
                                            class: "text-sm mt-1",
                                            style: "color: #7a8ba6;",
                                            "{brand.hero_message}"
                                        }
                                    }

                                    // 4-step reward flow cards
                                    div {
                                        class: "grid grid-cols-1 md:grid-cols-4 gap-5 mb-10",
                                        for (si, step) in brand.steps.iter().enumerate() {
                                            {
                                                let step_num = format!("0{}", si + 1);
                                                let svg_html = step_svg_list[si].clone();
                                                let step_colors: [(&str, &str); 4] = [
                                                    ("#60a5fa", "96,165,250"),
                                                    ("#a78bfa", "167,139,250"),
                                                    ("#fbbf24", "251,191,36"),
                                                    ("#34d399", "52,211,153"),
                                                ];
                                                let (s_accent, s_rgb) = step_colors[si];
                                                let s_card_style = format!(
                                                    "background: rgba(10,16,26,0.5); backdrop-filter: blur(16px); -webkit-backdrop-filter: blur(16px); border: 1px solid rgba({},0.2); box-shadow: 0 12px 40px rgba(0,0,0,0.3), 0 0 20px rgba({},0.08), inset 0 1px 0 rgba(255,255,255,0.06);",
                                                    s_rgb, s_rgb
                                                );
                                                let s_line_style = format!(
                                                    "background: linear-gradient(90deg, transparent, rgba({},0.5), transparent);",
                                                    s_rgb
                                                );
                                                let s_step_color = format!("color: {};", s_accent);
                                                rsx! {
                                                    div {
                                                        class: "text-center rounded-2xl relative overflow-hidden reveal-bounce",
                                                        style: "{s_card_style}",
                                                        div {
                                                            class: "absolute top-0 left-[10%] right-[10%] h-[2px]",
                                                            style: "{s_line_style}",
                                                        }
                                                        // SVG illustration
                                                        div {
                                                            class: "w-full h-32 flex items-center justify-center relative px-3 pt-3",
                                                            div {
                                                                class: "absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-24 h-24 rounded-full",
                                                                style: "background: {s_accent}; filter: blur(40px); opacity: 0.1;",
                                                            }
                                                            div {
                                                                style: "color: {s_accent}; width: 100%; height: 100%;",
                                                                dangerous_inner_html: "{svg_html}",
                                                            }
                                                        }
                                                        // Text
                                                        div {
                                                            class: "px-4 pb-4",
                                                            p {
                                                                class: "text-xs font-bold mb-1",
                                                                style: "{s_step_color}",
                                                                "STEP {step_num}"
                                                            }
                                                            h4 {
                                                                class: "text-base font-bold mb-1",
                                                                style: "color: #e8eefc;",
                                                                "{step.title}"
                                                            }
                                                            p {
                                                                class: "text-xs",
                                                                style: "color: #7a8ba6;",
                                                                "{step.desc}"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // 2-column layout: scenario details + quote/stats
                                    div {
                                        class: "grid grid-cols-1 lg:grid-cols-2 gap-6",

                                        // Left: Scenario details
                                        div {
                                            class: "rounded-2xl p-6",
                                            style: "{detail_card_style}",
                                            h4 {
                                                class: "text-lg font-bold mb-5",
                                                style: "color: #e8eefc;",
                                                "리워드 시나리오"
                                            }
                                            div {
                                                class: "space-y-4",
                                                // Purchase item
                                                div {
                                                    class: "flex justify-between items-center",
                                                    span { class: "text-sm", style: "color: #7a8ba6;", "구매 상품" }
                                                    span { class: "text-sm font-semibold", style: "color: #e8eefc;", "{brand.scenario.purchase_item}" }
                                                }
                                                div {
                                                    class: "flex justify-between items-center",
                                                    span { class: "text-sm", style: "color: #7a8ba6;", "구매 가격" }
                                                    span { class: "text-sm font-semibold", style: "color: #e8eefc;", "{purchase_price_str}" }
                                                }
                                                div {
                                                    class: "flex justify-between items-center",
                                                    span { class: "text-sm", style: "color: #7a8ba6;", "리워드율" }
                                                    span { class: "text-sm font-bold", style: "{detail_label_color}", "{reward_rate_str}" }
                                                }
                                                div {
                                                    class: "flex justify-between items-center",
                                                    span { class: "text-sm", style: "color: #7a8ba6;", "구매 리워드" }
                                                    span { class: "text-sm font-semibold", style: "color: #e8eefc;", "{reward_amount_str}" }
                                                }
                                                div { class: "h-px", style: "background: rgba(255,255,255,0.06);" }
                                                div {
                                                    class: "flex justify-between items-center",
                                                    span { class: "text-sm", style: "color: #7a8ba6;", "활동 유형" }
                                                    span { class: "text-sm font-semibold", style: "color: #e8eefc;", "{brand.scenario.activity_type}" }
                                                }
                                                div {
                                                    class: "flex justify-between items-center",
                                                    span { class: "text-sm", style: "color: #7a8ba6;", "활동 조건" }
                                                    span { class: "text-sm font-semibold", style: "color: #e8eefc;", "{brand.scenario.activity_detail}" }
                                                }
                                                div {
                                                    class: "flex justify-between items-center",
                                                    span { class: "text-sm", style: "color: #7a8ba6;", "활동 리워드" }
                                                    span { class: "text-sm font-semibold", style: "color: #e8eefc;", "{activity_reward_str}" }
                                                }
                                                div { class: "h-px", style: "background: rgba(255,255,255,0.06);" }
                                                div {
                                                    class: "flex justify-between items-center",
                                                    span { class: "text-sm", style: "color: #7a8ba6;", "월간 활동" }
                                                    span { class: "text-sm font-semibold", style: "color: #e8eefc;", "{brand.scenario.monthly_activity}" }
                                                }
                                                div {
                                                    class: "flex justify-between items-center",
                                                    span { class: "text-sm", style: "color: #7a8ba6;", "월간 리워드" }
                                                    span { class: "text-sm font-bold", style: "{detail_label_color}", "{monthly_reward_str}" }
                                                }
                                                div {
                                                    class: "flex justify-between items-center pt-2",
                                                    style: "border-top: 1px solid rgba(255,255,255,0.08);",
                                                    span { class: "text-sm font-bold", style: "color: #e8eefc;", "6개월 총 리워드" }
                                                    span { class: "text-lg font-extrabold", style: "{detail_label_color}", "{six_month_str}" }
                                                }
                                            }
                                        }

                                        // Right: Customer quote + Stats grid
                                        div {
                                            class: "flex flex-col gap-5",

                                            // Customer quote
                                            div {
                                                class: "rounded-2xl p-6",
                                                style: "{detail_card_style}",
                                                div {
                                                    class: "pl-4 mb-3",
                                                    style: "{quote_border}",
                                                    p {
                                                        class: "text-sm leading-relaxed italic",
                                                        style: "color: #c8d4e8;",
                                                        "\"{brand.customer_quote}\""
                                                    }
                                                }
                                                p {
                                                    class: "text-xs font-medium text-right",
                                                    style: "color: #7a8ba6;",
                                                    "- {brand.customer_name}"
                                                }
                                            }

                                            // Stats grid 2x2
                                            div {
                                                class: "grid grid-cols-2 gap-4 flex-1",
                                                div {
                                                    class: "rounded-xl p-5 reveal-bounce",
                                                    style: "{stats_border} background: rgba(10,16,26,0.5);",
                                                    p { class: "text-xs mb-3", style: "color: #7a8ba6;", "Treasury" }
                                                    p { class: "text-2xl font-extrabold mb-3", style: "color: #e8eefc;", "{treasury_str}" }
                                                    div { style: "color: #60a5fa; width: 100%; height: 40px;",
                                                        dangerous_inner_html: r#"<svg viewBox='0 0 120 40' fill='none' style='width:100%;height:100%;'><rect x='4' y='30' width='12' height='10' rx='2' fill='currentColor' opacity='0.12'/><rect x='20' y='24' width='12' height='16' rx='2' fill='currentColor' opacity='0.16'/><rect x='36' y='18' width='12' height='22' rx='2' fill='currentColor' opacity='0.22'/><rect x='52' y='13' width='12' height='27' rx='2' fill='currentColor' opacity='0.28'/><rect x='68' y='8' width='12' height='32' rx='2' fill='currentColor' opacity='0.36'/><rect x='84' y='4' width='12' height='36' rx='2' fill='currentColor' opacity='0.44'/><rect x='100' y='1' width='12' height='39' rx='2' fill='currentColor' opacity='0.55'/></svg>"#,
                                                    }
                                                    p { class: "text-xs mt-2", style: "color: #34d399;", "\u{25B2} +12.5% this month" }
                                                }
                                                div {
                                                    class: "rounded-xl p-5 reveal-bounce",
                                                    style: "{stats_border} background: rgba(10,16,26,0.5);",
                                                    p { class: "text-xs mb-3", style: "color: #7a8ba6;", "Active Users" }
                                                    p { class: "text-2xl font-extrabold mb-3", style: "color: #e8eefc;", "{users_str}" }
                                                    div { style: "color: #a78bfa; width: 100%; height: 40px;",
                                                        dangerous_inner_html: r#"<svg viewBox='0 0 120 40' fill='none' style='width:100%;height:100%;'><circle cx='15' cy='14' r='6' fill='currentColor' opacity='0.15'/><path d='M6,30 Q6,24 15,22 Q24,24 24,30' fill='currentColor' opacity='0.1'/><circle cx='42' cy='12' r='7' fill='currentColor' opacity='0.22'/><path d='M32,30 Q32,22 42,20 Q52,22 52,30' fill='currentColor' opacity='0.15'/><circle cx='72' cy='10' r='8' fill='currentColor' opacity='0.32'/><path d='M61,30 Q61,20 72,18 Q83,20 83,30' fill='currentColor' opacity='0.22'/><circle cx='105' cy='8' r='9' fill='currentColor' opacity='0.42'/><path d='M93,32 Q93,18 105,16 Q117,18 117,32' fill='currentColor' opacity='0.3'/><path d='M10,34 L40,32 L70,29 L105,25' stroke='currentColor' stroke-width='1' opacity='0.25' stroke-dasharray='3,3'/></svg>"#,
                                                    }
                                                    p { class: "text-xs mt-2", style: "color: #34d399;", "\u{25B2} +180 this week" }
                                                }
                                                div {
                                                    class: "rounded-xl p-5 reveal-bounce",
                                                    style: "{stats_border} background: rgba(10,16,26,0.5);",
                                                    p { class: "text-xs mb-3", style: "color: #7a8ba6;", "Floor Price" }
                                                    p { class: "text-2xl font-extrabold mb-3", style: "{detail_label_color}", "{floor_str}" }
                                                    div { style: "color: #34d399; width: 100%; height: 40px;",
                                                        dangerous_inner_html: r#"<svg viewBox='0 0 120 40' fill='none' style='width:100%;height:100%;'><path d='M4,36 L18,32 L32,33 L46,26 L60,28 L74,20 L88,16 L102,10 L116,4' stroke='currentColor' stroke-width='2' opacity='0.6' stroke-linecap='round'/><path d='M4,36 L18,32 L32,33 L46,26 L60,28 L74,20 L88,16 L102,10 L116,4 L116,40 L4,40 Z' fill='currentColor' opacity='0.08'/><circle cx='60' cy='28' r='2.5' fill='currentColor' opacity='0.4'/><circle cx='88' cy='16' r='2.5' fill='currentColor' opacity='0.5'/><circle cx='116' cy='4' r='3' fill='currentColor' opacity='0.7'/></svg>"#,
                                                    }
                                                    p { class: "text-xs mt-2", style: "color: #34d399;", "\u{25B2} All-time high" }
                                                }
                                                div {
                                                    class: "rounded-xl p-5 reveal-bounce",
                                                    style: "{stats_border} background: rgba(10,16,26,0.5);",
                                                    p { class: "text-xs mb-3", style: "color: #7a8ba6;", "Retention" }
                                                    p { class: "text-2xl font-extrabold mb-3", style: "color: #00d4aa;", "{brand.stats.retention}" }
                                                    div { class: "flex items-center justify-center", style: "width: 100%; height: 40px; color: #00d4aa;",
                                                        dangerous_inner_html: r#"<svg viewBox='0 0 60 40' fill='none' style='width:60px;height:40px;'><circle cx='30' cy='20' r='16' fill='none' stroke='currentColor' stroke-width='3' opacity='0.1'/><circle cx='30' cy='20' r='16' fill='none' stroke='currentColor' stroke-width='3.5' opacity='0.6' stroke-dasharray='75,100' stroke-dashoffset='-25' stroke-linecap='round'/><text x='30' y='24' text-anchor='middle' fill='currentColor' font-size='10' font-weight='bold' opacity='0.7'>UP</text></svg>"#,
                                                    }
                                                    p { class: "text-xs mt-2", style: "color: #34d399;", "vs industry avg 15%" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Why Biyard - compact 2x2 grid
        section {
            class: "py-12 px-4 relative overflow-hidden",
            style: "background: #141c2b;",
            div {
                class: "max-w-4xl mx-auto relative z-10",
                h2 {
                    class: "text-2xl md:text-3xl font-bold mb-8 text-center reveal-type",
                    style: "color: #e8eefc;",
                    "왜 Biyard를 선택해야 하나요?"
                }
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    for (wi, (title, desc)) in [
                        ("고객이 떠나지 않습니다", "구매할수록, 사용할수록 토큰이 쌓이고 가치가 올라갑니다. 리텐션 35~60% 향상."),
                        ("매출이 곧 마케팅입니다", "매출의 2~4%만 트레저리에 넣으면 고객이 알아서 홍보합니다."),
                        ("개발 없이 시작합니다", "API 키 하나면 기존 앱이나 POS에 연동됩니다. 블록체인 지식 불필요."),
                        ("투명하게 신뢰를 쌓습니다", "모든 토큰 발행과 트레저리 변동은 블록체인에 기록됩니다."),
                    ].iter().enumerate() {
                        {
                            let colors: [(&str, &str); 4] = [
                                ("#f472b6", "244,114,182"),
                                ("#60a5fa", "96,165,250"),
                                ("#a78bfa", "167,139,250"),
                                ("#34d399", "52,211,153"),
                            ];
                            let (accent, rgb) = colors[wi % 4];
                            let card_style = format!(
                                "background: rgba(10,16,26,0.5); border: 1px solid rgba({},0.18); box-shadow: 0 4px 16px rgba(0,0,0,0.2);",
                                rgb
                            );
                            let title_color = format!("color: {};", accent);
                            rsx! {
                                div {
                                    class: "rounded-xl p-5 relative overflow-hidden reveal-bounce",
                                    style: "{card_style}",
                                    h3 {
                                        class: "text-base font-bold mb-2",
                                        style: "{title_color}",
                                        "{title}"
                                    }
                                    p {
                                        class: "text-sm leading-relaxed",
                                        style: "color: #7a8ba6;",
                                        "{desc}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // CTA
        section {
            class: "py-14 px-4 text-center relative overflow-hidden",
            style: "background: #0c1018;",
            div {
                class: "relative z-10 reveal-fade",
                h2 {
                    class: "text-3xl font-extrabold mb-3",
                    style: "color: #e8eefc;",
                    "지금 시작하세요"
                }
                p {
                    class: "mb-6 max-w-xl mx-auto text-base",
                    style: "color: #7a8ba6;",
                    "고객이 사고, 쓰고, 즐길수록 함께 성장하는 토큰 이코노미. 당신의 브랜드도 가능합니다."
                }
                div {
                    class: "flex items-center justify-center gap-4 flex-wrap",
                    a {
                        class: "inline-flex items-center px-7 py-3 rounded-xl font-bold text-base shadow-lg",
                        style: "background: #00d4aa; color: #0c1018;",
                        href: "#",
                        "도입 문의하기 →"
                    }
                    a {
                        class: "inline-flex items-center px-7 py-3 rounded-xl font-bold text-base",
                        style: "border: 1px solid rgba(0,212,170,0.3); color: #00d4aa;",
                        href: "#",
                        "요금제 보기"
                    }
                }
            }
        }

        // Footer
        footer {
            class: "py-8 px-4 text-center",
            style: "background: #0c1018; border-top: 1px solid rgba(0,212,170,0.12);",
            p {
                class: "text-sm",
                style: "color: #4a5568;",
                "\u{00A9} 2026 Biyard. All rights reserved."
            }
        }
    }
}

// ── WalletSection ──

#[component]
fn WalletSection() -> Element {
    let total_value = format!("$0.49");
    let total_won = format!("\u{2248} \u{20A9}588");
    let chart_bars: [i32; 12] = [28, 32, 30, 35, 38, 36, 40, 42, 45, 48, 52, 58];

    rsx! {
        div {
            class: "py-12 px-4",
            div {
                class: "max-w-6xl mx-auto",
                // Portfolio Value Hero Card
                div {
                    class: "rounded-2xl p-8 mb-8",
                    style: "background: linear-gradient(135deg, #141c2b, #1a2435); border: 1px solid rgba(0,212,170,0.12);",
                    p { class: "text-sm mb-1", style: "color: #7a8ba6;", "Total Portfolio Value" }
                    div {
                        class: "flex items-end gap-4 mb-2",
                        h2 { class: "text-4xl font-extrabold", style: "color: #e8eefc;", "{total_value}" }
                        span { class: "text-lg mb-1", style: "color: #7a8ba6;", "{total_won}" }
                    }
                    span {
                        class: "inline-block text-xs font-medium px-3 py-1 rounded-full",
                        style: "background: rgba(0,212,170,0.15); color: #00d4aa;",
                        "+3.1% this month"
                    }
                }
                // Holdings Table
                div {
                    class: "rounded-2xl overflow-hidden mb-8",
                    style: "background: rgba(20,28,43,0.6); backdrop-filter: blur(12px); -webkit-backdrop-filter: blur(12px); border: 1px solid rgba(0,212,170,0.15); box-shadow: 0 8px 32px rgba(0,0,0,0.2), inset 0 1px 0 rgba(255,255,255,0.05);",
                    div {
                        class: "p-6 pb-4",
                        h3 { class: "text-lg font-bold", style: "color: #e8eefc;", "Holdings" }
                    }
                    div {
                        class: "overflow-x-auto",
                        table {
                            class: "w-full",
                            thead {
                                tr {
                                    style: "border-bottom: 1px solid rgba(0,212,170,0.12);",
                                    for h in ["Token", "Brand", "Price", "24h", "Holdings", "Value"] {
                                        th {
                                            class: "px-6 py-3 text-left text-xs font-medium",
                                            style: "color: #7a8ba6;",
                                            "{h}"
                                        }
                                    }
                                }
                            }
                            tbody {
                                for h in HOLDINGS.iter() {
                                    {
                                        let price_str = format!("${:.4}", h.price);
                                        let change_str = format!("+{}%", h.change24h);
                                        let value_str = format!("${:.4}", h.value);
                                        rsx! {
                                            tr {
                                                style: "border-bottom: 1px solid rgba(0,212,170,0.06);",
                                                td { class: "px-6 py-4 font-bold text-sm", style: "color: #e8eefc;", "{h.token}" }
                                                td { class: "px-6 py-4 text-sm", style: "color: #7a8ba6;", "{h.brand}" }
                                                td { class: "px-6 py-4 text-sm", style: "color: #e8eefc;", "{price_str}" }
                                                td { class: "px-6 py-4 text-sm font-medium", style: "color: #00d4aa;", "{change_str}" }
                                                td { class: "px-6 py-4 text-sm", style: "color: #e8eefc;", "{h.amount}" }
                                                td { class: "px-6 py-4 text-sm font-medium", style: "color: #e8eefc;", "{value_str}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                // Portfolio Performance Chart
                div {
                    class: "rounded-2xl p-6 mb-8",
                    style: "background: rgba(20,28,43,0.6); backdrop-filter: blur(12px); -webkit-backdrop-filter: blur(12px); border: 1px solid rgba(0,212,170,0.15); box-shadow: 0 8px 32px rgba(0,0,0,0.2), inset 0 1px 0 rgba(255,255,255,0.05);",
                    h3 { class: "text-lg font-bold mb-6", style: "color: #e8eefc;", "Portfolio Performance" }
                    div {
                        class: "flex items-end gap-2",
                        style: "height: 120px;",
                        for (i, bar) in chart_bars.iter().enumerate() {
                            {
                                let height_pct = (*bar as f64 / 58.0) * 100.0;
                                let opacity = 0.5 + (i as f64 / chart_bars.len() as f64) * 0.5;
                                let bar_style = format!(
                                    "height: {:.1}%; background: linear-gradient(to top, rgba(0,212,170,0.4), #00d4aa); opacity: {:.2}; flex: 1; border-radius: 4px 4px 0 0;",
                                    height_pct, opacity
                                );
                                rsx! {
                                    div { style: "{bar_style}" }
                                }
                            }
                        }
                    }
                    div {
                        class: "flex justify-between mt-2",
                        span { class: "text-xs", style: "color: #4a5568;", "12 months ago" }
                        span { class: "text-xs", style: "color: #4a5568;", "Now" }
                    }
                }
                // Recent Activity
                div {
                    class: "rounded-2xl p-6 mb-8",
                    style: "background: rgba(20,28,43,0.6); backdrop-filter: blur(12px); -webkit-backdrop-filter: blur(12px); border: 1px solid rgba(0,212,170,0.15); box-shadow: 0 8px 32px rgba(0,0,0,0.2), inset 0 1px 0 rgba(255,255,255,0.05);",
                    h3 { class: "text-lg font-bold mb-4", style: "color: #e8eefc;", "Recent Activity" }
                    div {
                        class: "space-y-3",
                        for a in RECENT_ACTIVITY.iter() {
                            div {
                                class: "flex items-center gap-3 p-3 rounded-xl",
                                style: "background: #1a2435;",
                                span { class: "text-xl", "{a.emoji}" }
                                div {
                                    class: "flex-1",
                                    p { class: "text-sm", style: "color: #e8eefc;", "{a.text}" }
                                }
                                span { class: "text-xs", style: "color: #4a5568;", "{a.time}" }
                            }
                        }
                    }
                }
                // How Your Token Value Grows
                div {
                    class: "rounded-2xl p-6",
                    style: "background: rgba(20,28,43,0.6); backdrop-filter: blur(12px); -webkit-backdrop-filter: blur(12px); border: 1px solid rgba(0,212,170,0.15); box-shadow: 0 8px 32px rgba(0,0,0,0.2), inset 0 1px 0 rgba(255,255,255,0.05);",
                    h3 {
                        class: "text-lg font-bold mb-6 text-center",
                        style: "color: #e8eefc;",
                        "How Your Token Value Grows"
                    }
                    div {
                        class: "grid grid-cols-1 md:grid-cols-4 gap-4",
                        for (i, s) in GROWTH_STEPS.iter().enumerate() {
                            div {
                                class: "text-center p-4 rounded-xl relative",
                                style: "background: #1a2435;",
                                if i < 3 {
                                    div {
                                        class: "hidden md:block absolute z-10",
                                        style: "top: 50%; right: -12px; transform: translateY(-50%); color: #00d4aa; font-size: 20px;",
                                        "\u{2192}"
                                    }
                                }
                                div { class: "text-3xl mb-3", "{s.emoji}" }
                                h4 { class: "font-bold text-sm mb-1", style: "color: #e8eefc;", "{s.title}" }
                                p { class: "text-xs", style: "color: #7a8ba6;", "{s.desc}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── SwapSection ──

#[component]
fn SwapSection() -> Element {
    let tokens = ["LMT", "CBT", "RPT"];
    let mut from_idx = use_signal(|| 0usize);
    let mut to_idx = use_signal(|| 1usize);
    let mut from_amount = use_signal(|| String::new());

    let from_token = tokens[*from_idx.read()];
    let to_token = tokens[*to_idx.read()];

    let numeric_from: f64 = from_amount.read().parse().unwrap_or(0.0);
    let rate = if from_token == to_token { 1.0 } else { get_swap_rate(from_token, to_token) };
    let to_amount = numeric_from * rate;
    let to_amount_str = if numeric_from > 0.0 { format!("{:.4}", to_amount) } else { String::new() };
    let rate_str = if from_token == to_token { "1".to_string() } else { format!("{:.2}", rate) };
    let from_balance = get_balance(from_token);
    let to_balance = get_balance(to_token);
    let from_avail = format!("Available: {} {}", from_balance, from_token);
    let to_avail = format!("Available: {} {}", to_balance, to_token);
    let rate_label = format!("1 {} = {} {}", from_token, rate_str, to_token);

    rsx! {
        div {
            class: "py-12 px-4",
            div {
                class: "max-w-6xl mx-auto",
                // Swap Card
                div {
                    class: "max-w-md mx-auto mb-12",
                    div {
                        class: "rounded-2xl p-6",
                        style: "background: rgba(20,28,43,0.6); backdrop-filter: blur(12px); -webkit-backdrop-filter: blur(12px); border: 1px solid rgba(0,212,170,0.15); box-shadow: 0 8px 32px rgba(0,0,0,0.2), inset 0 1px 0 rgba(255,255,255,0.05);",
                        h3 { class: "text-lg font-bold mb-6 text-center", style: "color: #e8eefc;", "Swap Tokens" }
                        // From
                        div {
                            class: "mb-1",
                            p { class: "text-xs font-medium mb-2", style: "color: #7a8ba6;", "From" }
                            div {
                                class: "flex gap-2",
                                select {
                                    class: "rounded-xl px-4 py-3 text-sm font-bold outline-none cursor-pointer",
                                    style: "background: #1a2435; color: #e8eefc; border: 1px solid rgba(0,212,170,0.12);",
                                    value: "{from_idx}",
                                    onchange: move |e| {
                                        let val: usize = e.value().parse().unwrap_or(0);
                                        from_idx.set(val);
                                        if val == *to_idx.read() {
                                            let other = if val == 0 { 1 } else { 0 };
                                            to_idx.set(other);
                                        }
                                    },
                                    for (i, t) in tokens.iter().enumerate() {
                                        option { value: "{i}", "{t}" }
                                    }
                                }
                                input {
                                    r#type: "number",
                                    class: "flex-1 rounded-xl px-4 py-3 text-sm outline-none text-right",
                                    style: "background: #1a2435; color: #e8eefc; border: 1px solid rgba(0,212,170,0.12);",
                                    placeholder: "0.00",
                                    value: "{from_amount}",
                                    oninput: move |e| from_amount.set(e.value()),
                                }
                            }
                            p { class: "text-xs mt-1", style: "color: #4a5568;", "{from_avail}" }
                        }
                        // Swap direction button
                        div {
                            class: "flex justify-center my-3",
                            button {
                                class: "w-10 h-10 rounded-full flex items-center justify-center",
                                style: "background: #1a2435; border: 1px solid rgba(0,212,170,0.12); color: #00d4aa;",
                                onclick: move |_| {
                                    let tmp = *from_idx.read();
                                    from_idx.set(*to_idx.read());
                                    to_idx.set(tmp);
                                },
                                "\u{21C5}"
                            }
                        }
                        // To
                        div {
                            class: "mb-4",
                            p { class: "text-xs font-medium mb-2", style: "color: #7a8ba6;", "To" }
                            div {
                                class: "flex gap-2",
                                select {
                                    class: "rounded-xl px-4 py-3 text-sm font-bold outline-none cursor-pointer",
                                    style: "background: #1a2435; color: #e8eefc; border: 1px solid rgba(0,212,170,0.12);",
                                    value: "{to_idx}",
                                    onchange: move |e| {
                                        let val: usize = e.value().parse().unwrap_or(1);
                                        to_idx.set(val);
                                        if val == *from_idx.read() {
                                            let other = if val == 0 { 1 } else { 0 };
                                            from_idx.set(other);
                                        }
                                    },
                                    for (i, t) in tokens.iter().enumerate() {
                                        option { value: "{i}", "{t}" }
                                    }
                                }
                                input {
                                    r#type: "text",
                                    class: "flex-1 rounded-xl px-4 py-3 text-sm outline-none text-right",
                                    style: "background: #1a2435; color: #7a8ba6; border: 1px solid rgba(0,212,170,0.12);",
                                    readonly: true,
                                    value: "{to_amount_str}",
                                    placeholder: "0.00",
                                }
                            }
                            p { class: "text-xs mt-1", style: "color: #4a5568;", "{to_avail}" }
                        }
                        // Rate
                        div {
                            class: "text-center mb-4",
                            p { class: "text-xs", style: "color: #7a8ba6;", "{rate_label}" }
                        }
                        // Swap button
                        button {
                            class: "w-full py-3 rounded-xl font-bold text-sm",
                            style: "background: #00d4aa; color: #0c1018;",
                            "Swap"
                        }
                    }
                }
                // My Balances Row
                div {
                    class: "grid grid-cols-1 md:grid-cols-3 gap-4 mb-8",
                    for t in tokens.iter() {
                        {
                            let bal = get_balance(t);
                            let brand_name = get_brand_name(t);
                            rsx! {
                                div {
                                    class: "rounded-xl p-5 text-center",
                                    style: "background: rgba(20,28,43,0.6); backdrop-filter: blur(12px); -webkit-backdrop-filter: blur(12px); border: 1px solid rgba(0,212,170,0.15); box-shadow: 0 8px 32px rgba(0,0,0,0.2), inset 0 1px 0 rgba(255,255,255,0.05);",
                                    p { class: "text-xs mb-1", style: "color: #7a8ba6;", "{brand_name}" }
                                    p { class: "text-2xl font-bold", style: "color: #e8eefc;", "{bal}" }
                                    p { class: "text-xs font-medium", style: "color: #00d4aa;", "{t}" }
                                }
                            }
                        }
                    }
                }
                // Recent Swaps Table
                div {
                    class: "rounded-2xl overflow-hidden",
                    style: "background: rgba(20,28,43,0.6); backdrop-filter: blur(12px); -webkit-backdrop-filter: blur(12px); border: 1px solid rgba(0,212,170,0.15); box-shadow: 0 8px 32px rgba(0,0,0,0.2), inset 0 1px 0 rgba(255,255,255,0.05);",
                    div {
                        class: "p-6 pb-4",
                        h3 { class: "text-lg font-bold", style: "color: #e8eefc;", "Recent Swaps" }
                    }
                    div {
                        class: "overflow-x-auto",
                        table {
                            class: "w-full",
                            thead {
                                tr {
                                    style: "border-bottom: 1px solid rgba(0,212,170,0.12);",
                                    for h in ["Date", "From", "To", "Amount", "Rate", "Status"] {
                                        th {
                                            class: "px-6 py-3 text-left text-xs font-medium",
                                            style: "color: #7a8ba6;",
                                            "{h}"
                                        }
                                    }
                                }
                            }
                            tbody {
                                for s in RECENT_SWAPS.iter() {
                                    tr {
                                        style: "border-bottom: 1px solid rgba(0,212,170,0.06);",
                                        td { class: "px-6 py-4 text-sm", style: "color: #7a8ba6;", "{s.date}" }
                                        td { class: "px-6 py-4 text-sm font-medium", style: "color: #e8eefc;", "{s.from}" }
                                        td { class: "px-6 py-4 text-sm font-medium", style: "color: #e8eefc;", "{s.to}" }
                                        td { class: "px-6 py-4 text-sm", style: "color: #e8eefc;", "{s.amount}" }
                                        td { class: "px-6 py-4 text-sm", style: "color: #7a8ba6;", "{s.rate}" }
                                        td {
                                            class: "px-6 py-4 text-sm",
                                            span {
                                                class: "inline-block px-2 py-0.5 rounded-full text-xs font-medium",
                                                style: "background: rgba(0,212,170,0.15); color: #00d4aa;",
                                                "{s.status}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── TokensSection ──

#[component]
fn TokensSection() -> Element {
    let total_market_cap = format_usd(72600);
    let avg_floor = format!("$0.0223");

    rsx! {
        div {
            class: "py-12 px-4",
            div {
                class: "max-w-6xl mx-auto",
                // Header
                div {
                    class: "mb-8",
                    h2 { class: "text-3xl font-bold mb-2", style: "color: #e8eefc;", "Token Market" }
                    p { style: "color: #7a8ba6;", "All revenue-backed tokens on Biyard" }
                }
                // Market Stats
                div {
                    class: "grid grid-cols-1 md:grid-cols-3 gap-4 mb-8",
                    for (label, value) in [("Total Market Cap", total_market_cap.as_str()), ("Total Tokens", "3"), ("Avg Floor Price", avg_floor.as_str())] {
                        div {
                            class: "rounded-xl p-5 text-center",
                            style: "background: rgba(20,28,43,0.6); backdrop-filter: blur(12px); -webkit-backdrop-filter: blur(12px); border: 1px solid rgba(0,212,170,0.15); box-shadow: 0 8px 32px rgba(0,0,0,0.2), inset 0 1px 0 rgba(255,255,255,0.05);",
                            p { class: "text-xs mb-1", style: "color: #7a8ba6;", "{label}" }
                            p { class: "text-2xl font-bold", style: "color: #e8eefc;", "{value}" }
                        }
                    }
                }
                // Token List Table
                div {
                    class: "rounded-2xl overflow-hidden",
                    style: "background: rgba(20,28,43,0.6); backdrop-filter: blur(12px); -webkit-backdrop-filter: blur(12px); border: 1px solid rgba(0,212,170,0.15); box-shadow: 0 8px 32px rgba(0,0,0,0.2), inset 0 1px 0 rgba(255,255,255,0.05);",
                    div {
                        class: "overflow-x-auto",
                        table {
                            class: "w-full",
                            thead {
                                tr {
                                    style: "border-bottom: 1px solid rgba(0,212,170,0.12);",
                                    for h in ["#", "Token", "Brand", "Price", "24h Change", "Market Cap", "Circulating Supply", "Floor Price"] {
                                        th {
                                            class: "px-6 py-4 text-left text-xs font-medium",
                                            style: "color: #7a8ba6;",
                                            "{h}"
                                        }
                                    }
                                }
                            }
                            tbody {
                                for t in TOKEN_LIST.iter() {
                                    {
                                        let price_str = format!("${:.4}", t.price);
                                        let change_str = format!("+{}%", t.change24h);
                                        let mcap_str = format_usd(t.market_cap);
                                        let circ_str = format!("{} / {}", format_number(t.circulating), format_number(t.total_supply));
                                        let floor_str = format!("${:.4}", t.floor_price);
                                        rsx! {
                                            tr {
                                                style: "border-bottom: 1px solid rgba(0,212,170,0.06);",
                                                td { class: "px-6 py-4 text-sm", style: "color: #7a8ba6;", "{t.rank}" }
                                                td { class: "px-6 py-4 text-sm font-bold", style: "color: #e8eefc;", "{t.token}" }
                                                td { class: "px-6 py-4 text-sm", style: "color: #7a8ba6;", "{t.brand}" }
                                                td { class: "px-6 py-4 text-sm", style: "color: #e8eefc;", "{price_str}" }
                                                td { class: "px-6 py-4 text-sm font-medium", style: "color: #00d4aa;", "{change_str}" }
                                                td { class: "px-6 py-4 text-sm", style: "color: #e8eefc;", "{mcap_str}" }
                                                td { class: "px-6 py-4 text-sm", style: "color: #7a8ba6;", "{circ_str}" }
                                                td { class: "px-6 py-4 text-sm", style: "color: #e8eefc;", "{floor_str}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── DaoSection ──

#[component]
fn DaoSection() -> Element {
    let mut votes = use_signal(|| {
        let mut m = std::collections::HashMap::<i32, String>::new();
        m.insert(2, "yes".to_string());
        m
    });

    rsx! {
        div {
            class: "py-12 px-4",
            div {
                class: "max-w-4xl mx-auto",
                // Header
                div {
                    class: "mb-8 text-center",
                    h2 { class: "text-3xl font-bold mb-2", style: "color: #e8eefc;", "DAO Governance" }
                    p { style: "color: #7a8ba6;", "토큰 보유자로서 브랜드의 미래를 결정하세요" }
                }
                // Proposal Cards
                div {
                    class: "space-y-6",
                    for p in DAO_PROPOSALS.iter() {
                        {
                            let total = p.yes_votes + p.no_votes;
                            let yes_pct = if total > 0 { (p.yes_votes as f64 / total as f64 * 100.0).round() as i32 } else { 0 };
                            let no_pct = 100 - yes_pct;
                            let yes_width = format!("width: {}%; background: #00d4aa; height: 100%; border-radius: 9999px;", yes_pct);
                            let no_width = format!("width: {}%; background: #ef4444; height: 100%; border-radius: 9999px;", no_pct);
                            let yes_label = format!("Yes ({})", format_number(p.yes_votes as i64));
                            let no_label = format!("No ({})", format_number(p.no_votes as i64));
                            let yes_pct_str = format!("{}%", yes_pct);
                            let no_pct_str = format!("{}%", no_pct);
                            let user_vote = votes.read().get(&p.id).cloned();
                            let pid_yes = p.id;
                            let pid_no = p.id;

                            rsx! {
                                div {
                                    class: "rounded-2xl p-6",
                                    style: "background: rgba(20,28,43,0.6); backdrop-filter: blur(12px); -webkit-backdrop-filter: blur(12px); border: 1px solid rgba(0,212,170,0.15); box-shadow: 0 8px 32px rgba(0,0,0,0.2), inset 0 1px 0 rgba(255,255,255,0.05);",
                                    // Brand badge
                                    span {
                                        class: "inline-block text-xs font-bold px-3 py-1 rounded-full mb-3",
                                        style: "background: #00d4aa; color: #0c1018;",
                                        "{p.brand}"
                                    }
                                    h3 { class: "text-xl font-bold mb-2", style: "color: #e8eefc;", "{p.title}" }
                                    p { class: "text-sm mb-5", style: "color: #7a8ba6;", "{p.description}" }
                                    // Progress bars
                                    div {
                                        class: "mb-4 space-y-3",
                                        // Yes bar
                                        div {
                                            div {
                                                class: "flex justify-between text-xs mb-1",
                                                span { style: "color: #00d4aa;", "{yes_label}" }
                                                span { style: "color: #00d4aa;", "{yes_pct_str}" }
                                            }
                                            div {
                                                class: "w-full h-3 rounded-full overflow-hidden",
                                                style: "background: #1a2435;",
                                                div { style: "{yes_width}" }
                                            }
                                        }
                                        // No bar
                                        div {
                                            div {
                                                class: "flex justify-between text-xs mb-1",
                                                span { style: "color: #ef4444;", "{no_label}" }
                                                span { style: "color: #ef4444;", "{no_pct_str}" }
                                            }
                                            div {
                                                class: "w-full h-3 rounded-full overflow-hidden",
                                                style: "background: #1a2435;",
                                                div { style: "{no_width}" }
                                            }
                                        }
                                    }
                                    // Vote buttons / indicator
                                    div {
                                        class: "flex items-center justify-between",
                                        div {
                                            class: "flex gap-2",
                                            if let Some(vote) = user_vote {
                                                {
                                                    let vote_text = if vote == "yes" { "투표 완료 (Yes)" } else { "투표 완료 (No)" };
                                                    rsx! {
                                                        span {
                                                            class: "inline-flex items-center gap-1 text-sm font-medium px-4 py-2 rounded-xl",
                                                            style: "background: rgba(0,212,170,0.15); color: #00d4aa;",
                                                            "\u{2705} {vote_text}"
                                                        }
                                                    }
                                                }
                                            } else {
                                                button {
                                                    class: "px-5 py-2 rounded-xl text-sm font-bold",
                                                    style: "background: #00d4aa; color: #0c1018;",
                                                    onclick: move |_| {
                                                        votes.write().insert(pid_yes, "yes".to_string());
                                                    },
                                                    "Yes"
                                                }
                                                button {
                                                    class: "px-5 py-2 rounded-xl text-sm font-bold",
                                                    style: "background: rgba(239,68,68,0.2); color: #ef4444; border: 1px solid rgba(239,68,68,0.3);",
                                                    onclick: move |_| {
                                                        votes.write().insert(pid_no, "no".to_string());
                                                    },
                                                    "No"
                                                }
                                            }
                                        }
                                        span {
                                            class: "text-xs",
                                            style: "color: #4a5568;",
                                            "Deadline: {p.deadline}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
