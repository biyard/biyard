import { Link } from "react-router-dom";
import {
  Landmark,
  Footprints,
  Vote,
  Coins,
  Code,
  ShoppingBag,
  TrendingUp,
  Heart,
  ArrowRight,
  CheckCircle,
  Repeat,
} from "lucide-react";
import { formatUSD, formatNumber } from "@/lib/mock-data";

const features = [
  {
    icon: Landmark,
    name: "수익 기반 트레저리",
    description:
      "고객의 구매 금액 일부가 자동으로 트레저리에 적립됩니다. 실제 매출이 토큰 가치를 뒷받침합니다.",
  },
  {
    icon: Footprints,
    name: "활동 기반 리워드",
    description:
      "걷기, 러닝, 매장 방문, 구매 등 고객의 실제 활동이 인증되면 추가 리워드가 지급됩니다.",
  },
  {
    icon: TrendingUp,
    name: "성장하는 토큰 가치",
    description:
      "트레저리가 쌓일수록 토큰 하한가가 올라갑니다. 고객이 많을수록, 매출이 높을수록 모두가 이득입니다.",
  },
  {
    icon: Coins,
    name: "브랜드 전용 토큰",
    description:
      "우리 브랜드만의 토큰을 발행하세요. 실물 매출에 연동된 토큰으로 고객 충성도를 극대화합니다.",
  },
  {
    icon: Vote,
    name: "DAO 커뮤니티",
    description:
      "토큰 보유자가 브랜드 의사결정에 참여합니다. 고객이 곧 브랜드의 주주가 됩니다.",
  },
  {
    icon: Code,
    name: "5분 만에 연동",
    description:
      "RESTful API와 SDK로 기존 앱/POS에 쉽게 연동하세요. 블록체인 지식 없이도 시작할 수 있습니다.",
  },
];

const brandShowcases = [
  {
    brand: "Le Mouton",
    segment: "Fashion",
    tagline: "건강과 토큰을 같이 챙기세요!",
    heroMessage: "신발을 사고, 신고, 걸을수록 돈을 버는 경험",
    scenario: {
      purchaseItem: "Le Mouton 컴포트 워커",
      purchasePrice: 129000,
      rewardRate: 2,
      rewardAmount: 2580,
      activityType: "걷기",
      activityDetail: "하루 8,000걸음 달성 시",
      activityReward: 80,
      monthlyActivity: "월 평균 20일 활동",
      monthlyReward: 1600,
      sixMonthTotal: 12180,
    },
    customerQuote:
      "편한 신발 사서 매일 출퇴근길에 신고 다니는데, 걸을수록 포인트가 쌓이고 그게 진짜 돈이 돼요. 르무통 안 신을 이유가 없죠.",
    customerName: "김서연, 직장인",
    stats: { treasury: 24500, users: 1250, floorPrice: 0.0245, retention: "40%" },
    color: "blue",
    bgGradient: "from-blue-600 to-blue-800",
    lightBg: "bg-blue-50 dark:bg-blue-900/20",
    accentText: "text-blue-600 dark:text-blue-400",
    badgeBg: "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200",
    steps: [
      { icon: ShoppingBag, title: "신발 구매", desc: "129,000원 신발 구매 시 2,580원(2%) 적립" },
      { icon: Landmark, title: "트레저리 적립", desc: "적립금이 트레저리에 누적 → 토큰 가치 상승" },
      { icon: Footprints, title: "걷기 인증", desc: "매일 신발 신고 걸으면 추가 리워드 지급" },
      { icon: TrendingUp, title: "가치 성장", desc: "고객이 많을수록 트레저리↑ 토큰 가치↑" },
    ],
  },
  {
    brand: "Cafe Blossom",
    segment: "F&B",
    tagline: "커피 한 잔이 자산이 되는 경험!",
    heroMessage: "매일 마시는 커피가 나의 투자가 됩니다",
    scenario: {
      purchaseItem: "시그니처 라떼",
      purchasePrice: 6500,
      rewardRate: 3,
      rewardAmount: 195,
      activityType: "매장 방문",
      activityDetail: "주 3회 방문 스탬프 달성 시",
      activityReward: 50,
      monthlyActivity: "월 12회 방문 기준",
      monthlyReward: 200,
      sixMonthTotal: 2370,
    },
    customerQuote:
      "어차피 매일 커피 마시는데, 여기서 마시면 스탬프도 찍히고 토큰도 쌓여요. 다른 카페 갈 이유가 없어요.",
    customerName: "박준혁, 대학생",
    stats: { treasury: 12300, users: 850, floorPrice: 0.0246, retention: "35%" },
    color: "emerald",
    bgGradient: "from-emerald-600 to-emerald-800",
    lightBg: "bg-emerald-50 dark:bg-emerald-900/20",
    accentText: "text-emerald-600 dark:text-emerald-400",
    badgeBg: "bg-emerald-100 text-emerald-800 dark:bg-emerald-900 dark:text-emerald-200",
    steps: [
      { icon: ShoppingBag, title: "음료 구매", desc: "6,500원 라떼 주문 시 195원(3%) 적립" },
      { icon: Landmark, title: "트레저리 적립", desc: "적립금이 트레저리에 누적 → 토큰 가치 상승" },
      { icon: Heart, title: "방문 스탬프", desc: "주 3회 방문 달성 시 보너스 리워드 지급" },
      { icon: TrendingUp, title: "가치 성장", desc: "단골이 많을수록 트레저리↑ 토큰 가치↑" },
    ],
  },
  {
    brand: "RunPulse",
    segment: "Sports Tech",
    tagline: "달릴수록 벌리는 스마트한 운동!",
    heroMessage: "운동하면서 건강도 챙기고 수익도 챙기세요",
    scenario: {
      purchaseItem: "RunPulse 스마트밴드",
      purchasePrice: 89000,
      rewardRate: 2.5,
      rewardAmount: 2225,
      activityType: "러닝",
      activityDetail: "주 3회 5km 러닝 인증 시",
      activityReward: 150,
      monthlyActivity: "월 12회 러닝 기준",
      monthlyReward: 1800,
      sixMonthTotal: 13025,
    },
    customerQuote:
      "밴드 차고 뛰기만 하면 자동으로 기록되고 토큰이 쌓여요. 운동 동기부여가 확실히 달라졌어요.",
    customerName: "이현우, 마라토너",
    stats: { treasury: 35800, users: 2100, floorPrice: 0.0179, retention: "60%" },
    color: "purple",
    bgGradient: "from-purple-600 to-purple-800",
    lightBg: "bg-purple-50 dark:bg-purple-900/20",
    accentText: "text-purple-600 dark:text-purple-400",
    badgeBg: "bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-200",
    steps: [
      { icon: ShoppingBag, title: "디바이스 구매", desc: "89,000원 스마트밴드 구매 시 2,225원(2.5%) 적립" },
      { icon: Landmark, title: "트레저리 적립", desc: "적립금이 트레저리에 누적 → 토큰 가치 상승" },
      { icon: Footprints, title: "러닝 인증", desc: "GPS 연동 러닝 기록 달성 시 추가 리워드" },
      { icon: TrendingUp, title: "가치 성장", desc: "러너가 많을수록 트레저리↑ 토큰 가치↑" },
    ],
  },
];

function formatWon(n: number): string {
  return new Intl.NumberFormat("ko-KR").format(n) + "원";
}

export function DemoPage() {
  return (
    <div className="min-h-screen bg-white dark:bg-gray-900">
      {/* ── Hero ── */}
      <section className="relative overflow-hidden py-24 px-4 text-center bg-gradient-to-br from-gray-900 via-blue-900 to-gray-900">
        <div className="absolute inset-0 opacity-10">
          <div className="absolute top-20 left-1/4 w-72 h-72 bg-blue-500 rounded-full blur-3xl" />
          <div className="absolute bottom-10 right-1/4 w-96 h-96 bg-purple-500 rounded-full blur-3xl" />
        </div>
        <div className="relative z-10 max-w-4xl mx-auto">
          <p className="text-blue-300 font-medium mb-4 tracking-wide uppercase text-sm">
            Revenue-Backed Token Platform
          </p>
          <h1 className="text-5xl md:text-6xl font-extrabold text-white leading-tight">
            사고, 쓰고, 즐길수록
            <br />
            <span className="text-transparent bg-clip-text bg-gradient-to-r from-blue-400 to-emerald-400">
              가치가 커지는 토큰
            </span>
          </h1>
          <p className="mt-6 text-xl text-gray-300 max-w-2xl mx-auto leading-relaxed">
            고객의 구매와 활동이 트레저리에 쌓이고, 그 가치가 모든 토큰 보유자에게 돌아갑니다.
            <br />
            브랜드와 고객이 함께 성장하는 새로운 로열티 플랫폼.
          </p>
          <div className="mt-10 flex items-center justify-center gap-4 flex-wrap">
            <Link
              to="/signup"
              className="inline-flex items-center px-8 py-4 rounded-xl bg-blue-600 text-white font-bold text-lg hover:bg-blue-500 transition-colors shadow-lg shadow-blue-600/30"
            >
              무료로 시작하기
              <ArrowRight className="ml-2 h-5 w-5" />
            </Link>
            <Link
              to="/roi-simulator"
              className="inline-flex items-center px-8 py-4 rounded-xl border-2 border-white/30 text-white font-bold text-lg hover:bg-white/10 transition-colors"
            >
              ROI 시뮬레이터
            </Link>
          </div>
        </div>
      </section>

      {/* ── How It Works (공통 구조) ── */}
      <section className="py-20 px-4 bg-gray-50 dark:bg-gray-800">
        <div className="max-w-5xl mx-auto text-center mb-16">
          <h2 className="text-3xl md:text-4xl font-bold text-gray-900 dark:text-white mb-4">
            어떻게 작동하나요?
          </h2>
          <p className="text-lg text-gray-600 dark:text-gray-400 max-w-2xl mx-auto">
            고객이 제품을 구매하면 매출의 일부가 트레저리에 적립되고,
            제품을 사용하며 활동할수록 추가 리워드를 받습니다.
          </p>
        </div>

        <div className="max-w-4xl mx-auto">
          <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
            {[
              {
                step: "01",
                icon: ShoppingBag,
                title: "고객이 구매",
                desc: "제품/서비스 구매 금액의 2~4%가 자동 적립",
              },
              {
                step: "02",
                icon: Landmark,
                title: "트레저리 적립",
                desc: "적립금이 트레저리에 누적되어 토큰 가치를 뒷받침",
              },
              {
                step: "03",
                icon: Repeat,
                title: "활동 인증",
                desc: "걷기, 러닝, 방문 등 활동이 인증되면 추가 리워드",
              },
              {
                step: "04",
                icon: TrendingUp,
                title: "함께 성장",
                desc: "고객이 늘수록 트레저리↑ 토큰 가치↑ 모두가 이득",
              },
            ].map((item) => {
              const Icon = item.icon;
              return (
                <div key={item.step} className="text-center">
                  <div className="w-16 h-16 rounded-2xl bg-blue-600 text-white flex items-center justify-center mx-auto mb-4 shadow-lg">
                    <Icon className="h-7 w-7" />
                  </div>
                  <p className="text-xs font-bold text-blue-600 dark:text-blue-400 mb-1">
                    STEP {item.step}
                  </p>
                  <h3 className="text-lg font-bold text-gray-900 dark:text-white mb-2">
                    {item.title}
                  </h3>
                  <p className="text-sm text-gray-600 dark:text-gray-400">
                    {item.desc}
                  </p>
                </div>
              );
            })}
          </div>
        </div>
      </section>

      {/* ── Brand Showcases ── */}
      {brandShowcases.map((brand, idx) => (
        <section
          key={brand.brand}
          className={`py-20 px-4 ${idx % 2 === 0 ? "bg-white dark:bg-gray-900" : "bg-gray-50 dark:bg-gray-800"}`}
        >
          <div className="max-w-6xl mx-auto">
            {/* Brand Header */}
            <div className="text-center mb-12">
              <span className={`inline-block text-xs font-medium px-3 py-1 rounded-full ${brand.badgeBg} mb-3`}>
                {brand.segment}
              </span>
              <h2 className="text-3xl md:text-4xl font-bold text-gray-900 dark:text-white mb-2">
                {brand.brand}
              </h2>
              <p className={`text-2xl font-bold ${brand.accentText}`}>
                "{brand.tagline}"
              </p>
              <p className="mt-2 text-lg text-gray-600 dark:text-gray-400">
                {brand.heroMessage}
              </p>
            </div>

            {/* Reward Flow */}
            <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-12">
              {brand.steps.map((step, si) => {
                const StepIcon = step.icon;
                return (
                  <div key={si} className={`${brand.lightBg} rounded-xl p-5 text-center relative`}>
                    {si < brand.steps.length - 1 && (
                      <div className="hidden md:block absolute top-1/2 -right-3 transform -translate-y-1/2 z-10">
                        <ArrowRight className={`h-5 w-5 ${brand.accentText}`} />
                      </div>
                    )}
                    <div className={`w-12 h-12 rounded-xl ${brand.accentText} bg-white dark:bg-gray-800 flex items-center justify-center mx-auto mb-3 shadow`}>
                      <StepIcon className="h-6 w-6" />
                    </div>
                    <h4 className="font-bold text-gray-900 dark:text-white text-sm mb-1">
                      {step.title}
                    </h4>
                    <p className="text-xs text-gray-600 dark:text-gray-400">
                      {step.desc}
                    </p>
                  </div>
                );
              })}
            </div>

            {/* Scenario + Quote */}
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-8 mb-12">
              {/* Detailed Scenario */}
              <div className="bg-white dark:bg-gray-800 rounded-2xl shadow-lg p-8 border border-gray-200 dark:border-gray-700">
                <h3 className="text-lg font-bold text-gray-900 dark:text-white mb-6">
                  구체적인 시나리오
                </h3>
                <div className="space-y-4">
                  <div className="flex items-start gap-3">
                    <CheckCircle className={`h-5 w-5 mt-0.5 flex-shrink-0 ${brand.accentText}`} />
                    <div>
                      <p className="font-medium text-gray-900 dark:text-white">
                        {brand.scenario.purchaseItem} 구매
                      </p>
                      <p className="text-sm text-gray-500 dark:text-gray-400">
                        {formatWon(brand.scenario.purchasePrice)}의 {brand.scenario.rewardRate}% →{" "}
                        <span className={`font-bold ${brand.accentText}`}>
                          {formatWon(brand.scenario.rewardAmount)} 적립
                        </span>
                      </p>
                    </div>
                  </div>
                  <div className="flex items-start gap-3">
                    <CheckCircle className={`h-5 w-5 mt-0.5 flex-shrink-0 ${brand.accentText}`} />
                    <div>
                      <p className="font-medium text-gray-900 dark:text-white">
                        {brand.scenario.activityType} 리워드
                      </p>
                      <p className="text-sm text-gray-500 dark:text-gray-400">
                        {brand.scenario.activityDetail} →{" "}
                        <span className={`font-bold ${brand.accentText}`}>
                          +{brand.scenario.activityReward} 포인트
                        </span>
                      </p>
                    </div>
                  </div>
                  <div className="flex items-start gap-3">
                    <CheckCircle className={`h-5 w-5 mt-0.5 flex-shrink-0 ${brand.accentText}`} />
                    <div>
                      <p className="font-medium text-gray-900 dark:text-white">
                        월간 활동 리워드
                      </p>
                      <p className="text-sm text-gray-500 dark:text-gray-400">
                        {brand.scenario.monthlyActivity} →{" "}
                        <span className={`font-bold ${brand.accentText}`}>
                          월 +{formatNumber(brand.scenario.monthlyReward)} 포인트
                        </span>
                      </p>
                    </div>
                  </div>
                  <div className="mt-6 p-4 rounded-xl bg-gray-50 dark:bg-gray-900 border border-gray-200 dark:border-gray-700">
                    <p className="text-sm text-gray-500 dark:text-gray-400">6개월 예상 총 적립</p>
                    <p className={`text-3xl font-extrabold ${brand.accentText}`}>
                      {formatWon(brand.scenario.sixMonthTotal)}
                    </p>
                    <p className="text-xs text-gray-400 dark:text-gray-500 mt-1">
                      구매 적립 + 활동 리워드 합산
                    </p>
                  </div>
                </div>
              </div>

              {/* Customer Quote + Stats */}
              <div className="flex flex-col gap-6">
                {/* Quote */}
                <div className={`rounded-2xl p-8 bg-gradient-to-br ${brand.bgGradient} text-white flex-1`}>
                  <p className="text-lg leading-relaxed mb-6 opacity-95">
                    "{brand.customerQuote}"
                  </p>
                  <p className="text-sm font-medium opacity-80">
                    — {brand.customerName}
                  </p>
                </div>

                {/* Stats */}
                <div className="grid grid-cols-2 gap-4">
                  <div className="bg-white dark:bg-gray-800 rounded-xl p-4 shadow border border-gray-200 dark:border-gray-700 text-center">
                    <p className="text-xs text-gray-500 dark:text-gray-400">트레저리</p>
                    <p className="text-xl font-bold text-gray-900 dark:text-white">
                      {formatUSD(brand.stats.treasury)}
                    </p>
                  </div>
                  <div className="bg-white dark:bg-gray-800 rounded-xl p-4 shadow border border-gray-200 dark:border-gray-700 text-center">
                    <p className="text-xs text-gray-500 dark:text-gray-400">활성 유저</p>
                    <p className="text-xl font-bold text-gray-900 dark:text-white">
                      {formatNumber(brand.stats.users)}명
                    </p>
                  </div>
                  <div className="bg-white dark:bg-gray-800 rounded-xl p-4 shadow border border-gray-200 dark:border-gray-700 text-center">
                    <p className="text-xs text-gray-500 dark:text-gray-400">토큰 하한가</p>
                    <p className="text-xl font-bold text-gray-900 dark:text-white">
                      {formatUSD(brand.stats.floorPrice)}
                    </p>
                  </div>
                  <div className="bg-white dark:bg-gray-800 rounded-xl p-4 shadow border border-gray-200 dark:border-gray-700 text-center">
                    <p className="text-xs text-gray-500 dark:text-gray-400">리텐션 향상</p>
                    <p className={`text-xl font-bold ${brand.accentText}`}>
                      +{brand.stats.retention}
                    </p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </section>
      ))}

      {/* ── Platform Features ── */}
      <section className="py-20 px-4 bg-white dark:bg-gray-900">
        <div className="max-w-6xl mx-auto">
          <div className="text-center mb-16">
            <h2 className="text-3xl md:text-4xl font-bold text-gray-900 dark:text-white mb-4">
              기업을 위한 올인원 플랫폼
            </h2>
            <p className="text-lg text-gray-600 dark:text-gray-400">
              블록체인 지식 없이도 토큰 이코노미를 구축할 수 있습니다
            </p>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {features.map((feature) => {
              const Icon = feature.icon;
              return (
                <div
                  key={feature.name}
                  className="bg-gray-50 dark:bg-gray-800 rounded-xl p-6 hover:shadow-lg transition-shadow border border-gray-100 dark:border-gray-700"
                >
                  <div className="w-12 h-12 rounded-xl bg-blue-100 dark:bg-blue-900/50 flex items-center justify-center mb-4">
                    <Icon className="w-6 h-6 text-blue-600 dark:text-blue-400" />
                  </div>
                  <h3 className="text-lg font-bold text-gray-900 dark:text-white">
                    {feature.name}
                  </h3>
                  <p className="mt-2 text-sm text-gray-600 dark:text-gray-400 leading-relaxed">
                    {feature.description}
                  </p>
                </div>
              );
            })}
          </div>
        </div>
      </section>

      {/* ── For Businesses ── */}
      <section className="py-20 px-4 bg-gray-50 dark:bg-gray-800">
        <div className="max-w-5xl mx-auto">
          <div className="text-center mb-16">
            <h2 className="text-3xl md:text-4xl font-bold text-gray-900 dark:text-white mb-4">
              왜 Biyard를 선택해야 하나요?
            </h2>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
            {[
              {
                title: "고객이 떠나지 않습니다",
                desc: "구매할수록, 사용할수록 토큰이 쌓이고 가치가 올라갑니다. 고객은 '안 쓸 이유'가 없어집니다. 기존 로열티 프로그램 대비 리텐션 35~60% 향상.",
              },
              {
                title: "매출이 곧 마케팅입니다",
                desc: "매출의 2~4%만 트레저리에 넣으면 고객이 알아서 홍보합니다. '이 브랜드 쓰면 돈 번다'는 입소문은 어떤 광고보다 강력합니다.",
              },
              {
                title: "개발 없이 시작합니다",
                desc: "API 키 하나면 기존 앱이나 POS에 연동됩니다. 블록체인 전문 인력이 필요 없습니다. 온보딩부터 운영까지 콘솔에서 원스톱으로.",
              },
              {
                title: "투명하게 신뢰를 쌓습니다",
                desc: "모든 토큰 발행과 트레저리 변동은 블록체인에 기록됩니다. 고객은 언제든 확인할 수 있고, 이 투명성이 브랜드 신뢰도를 높입니다.",
              },
            ].map((item) => (
              <div
                key={item.title}
                className="bg-white dark:bg-gray-900 rounded-xl p-8 shadow border border-gray-200 dark:border-gray-700"
              >
                <h3 className="text-xl font-bold text-gray-900 dark:text-white mb-3">
                  {item.title}
                </h3>
                <p className="text-gray-600 dark:text-gray-400 leading-relaxed">
                  {item.desc}
                </p>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* ── CTA ── */}
      <section className="py-24 px-4 bg-gradient-to-br from-blue-600 via-blue-700 to-purple-700 text-center relative overflow-hidden">
        <div className="absolute inset-0 opacity-10">
          <div className="absolute top-10 right-1/4 w-80 h-80 bg-white rounded-full blur-3xl" />
        </div>
        <div className="relative z-10">
          <h2 className="text-4xl font-extrabold text-white mb-4">
            지금 시작하세요
          </h2>
          <p className="text-blue-100 mb-10 max-w-xl mx-auto text-lg">
            고객이 사고, 쓰고, 즐길수록 함께 성장하는 토큰 이코노미.
            <br />
            당신의 브랜드도 가능합니다.
          </p>
          <div className="flex items-center justify-center gap-4 flex-wrap">
            <Link
              to="/signup"
              className="inline-flex items-center px-8 py-4 rounded-xl bg-white text-blue-700 font-bold text-lg hover:bg-blue-50 transition-colors shadow-lg"
            >
              무료로 시작하기
              <ArrowRight className="ml-2 h-5 w-5" />
            </Link>
            <Link
              to="/pricing"
              className="inline-flex items-center px-8 py-4 rounded-xl border-2 border-white/40 text-white font-bold text-lg hover:bg-white/10 transition-colors"
            >
              요금제 보기
            </Link>
          </div>
        </div>
      </section>
    </div>
  );
}
