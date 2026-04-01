import { useState } from "react";
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
  Wallet,
  ArrowDownUp,
  ListOrdered,
  Home,
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
      "편한 신발 사서 매일 출퇴근길에 신고 다니는데, 걸수록 포인트가 쌓이고 그게 진짜 돈이 돼요. 르무통 안 신을 이유가 없죠.",
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

/* ========================================================================
   Tab type
   ======================================================================== */
type TabKey = "home" | "wallet" | "swap" | "tokens" | "dao";

/* ========================================================================
   HomeSection  (original landing page, untouched)
   ======================================================================== */
function HomeSection() {
  return (
    <>
      {/* ── Hero ── */}
      <section
        className="relative overflow-hidden py-24 px-4"
        style={{ background: "linear-gradient(135deg, #0c1018 0%, #0d1a24 50%, #0c1018 100%)" }}
      >
        <div className="absolute inset-0 opacity-10">
          <div className="absolute top-20 left-1/4 w-72 h-72 rounded-full blur-3xl" style={{ background: "#00d4aa" }} />
          <div className="absolute bottom-10 right-1/4 w-96 h-96 rounded-full blur-3xl" style={{ background: "#00d4aa" }} />
        </div>

        <div className="relative z-10 max-w-6xl mx-auto grid grid-cols-1 lg:grid-cols-[1fr_auto_1fr] items-center gap-8">
          {/* Left decoration - 3D product cube */}
          <div className="hidden lg:flex items-center justify-end">
            <div className="relative w-80 h-80">
              {/* Outer glow ring */}
              <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-52 h-52 rounded-full opacity-20" style={{ border: "1px solid #00d4aa", boxShadow: "0 0 40px rgba(0,212,170,0.3), inset 0 0 40px rgba(0,212,170,0.1)" }} />
              <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-64 h-64 rounded-full opacity-10" style={{ border: "1px dashed #00d4aa", animation: "spinCube 30s linear infinite" }} />
              {/* Center glow */}
              <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-44 h-44 rounded-full blur-[70px] opacity-35" style={{ background: "#00d4aa" }} />
              {/* Rotating cube with product emojis */}
              <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2" style={{ perspective: "800px" }}>
                <div style={{
                  width: "140px", height: "140px", position: "relative", transformStyle: "preserve-3d",
                  animation: "spinCube 15s linear infinite",
                }}>
                  {[
                    { transform: "translateZ(70px)", icon: "📈", value: "$0.0245", label: "Floor Price", bg: "rgba(0,212,170,0.18)" },
                    { transform: "rotateY(180deg) translateZ(70px)", icon: "🔒", value: "$72,600", label: "Treasury", bg: "rgba(0,212,170,0.14)" },
                    { transform: "rotateY(90deg) translateZ(70px)", icon: "♾️", value: "AUTO", label: "Buyback", bg: "rgba(0,212,170,0.16)" },
                    { transform: "rotateY(-90deg) translateZ(70px)", icon: "🔥", value: "DEFLATION", label: "Burn", bg: "rgba(0,212,170,0.12)" },
                    { transform: "rotateX(90deg) translateZ(70px)", icon: "", value: "B", label: "BIYARD", bg: "rgba(0,212,170,0.20)" },
                    { transform: "rotateX(-90deg) translateZ(70px)", icon: "", value: "B", label: "BIYARD", bg: "rgba(0,212,170,0.10)" },
                  ].map((face, i) => (
                    <div key={i} className="absolute inset-0 rounded-2xl backdrop-blur-sm flex flex-col items-center justify-center" style={{
                      transform: face.transform, background: face.bg,
                      border: "1px solid rgba(0,212,170,0.35)",
                      boxShadow: "0 0 20px rgba(0,212,170,0.1), inset 0 0 20px rgba(0,212,170,0.05)",
                    }}>
                      {face.icon ? (
                        <>
                          <span className="text-2xl mb-0.5" style={{ filter: "drop-shadow(0 0 6px rgba(0,212,170,0.5))" }}>{face.icon}</span>
                          <span className="text-sm font-extrabold tracking-tight" style={{ color: "#00d4aa" }}>{face.value}</span>
                          <span className="text-[8px] font-semibold tracking-widest uppercase mt-0.5" style={{ color: "#7a8ba6" }}>{face.label}</span>
                        </>
                      ) : (
                        <span className="text-4xl font-black" style={{ color: "#00d4aa", filter: "drop-shadow(0 0 10px rgba(0,212,170,0.6))" }}>{face.value}</span>
                      )}
                    </div>
                  ))}
                </div>
              </div>
              {/* Orbiting mechanism labels */}
              {[
                { text: "Treasury ↑", top: "5%", left: "55%", delay: "0s" },
                { text: "Burn 🔥", top: "78%", left: "65%", delay: "1.2s" },
                { text: "Buyback ♾️", top: "75%", left: "8%", delay: "0.6s" },
                { text: "Floor ↑", top: "10%", left: "5%", delay: "1.8s" },
              ].map((p, i) => (
                <div key={i} className="absolute rounded-full px-3 py-1" style={{
                  top: p.top, left: p.left,
                  background: "rgba(0,212,170,0.08)", border: "1px solid rgba(0,212,170,0.2)",
                  fontSize: "10px", fontWeight: 600, color: "#00d4aa", letterSpacing: "0.5px",
                  animation: `floatParticle 3.5s ease-in-out ${p.delay} infinite alternate`,
                  boxShadow: "0 0 12px rgba(0,212,170,0.08)",
                }}>
                  {p.text}
                </div>
              ))}
              {/* Sparkle dots */}
              {[
                { top: "5%", left: "45%", d: "2.5s" },
                { top: "55%", left: "5%", d: "3.2s" },
                { top: "90%", left: "50%", d: "4s" },
                { top: "35%", left: "92%", d: "2.8s" },
              ].map((s, i) => (
                <div key={i} className="absolute w-1 h-1 rounded-full" style={{
                  top: s.top, left: s.left, background: "#00d4aa",
                  boxShadow: "0 0 6px #00d4aa, 0 0 12px rgba(0,212,170,0.4)",
                  animation: `floatParticle ${s.d} ease-in-out infinite alternate`,
                }} />
              ))}
            </div>
          </div>

          {/* Center - Hero content */}
          <div className="text-center max-w-2xl">
            <p className="font-medium mb-4 tracking-wide uppercase text-sm" style={{ color: "#00d4aa" }}>
              Revenue-Backed Token Platform
            </p>
            <h1 className="text-5xl md:text-6xl font-extrabold leading-tight" style={{ color: "#e8eefc" }}>
              사고, 쓰고, 즐길수록
              <br />
              <span className="text-transparent bg-clip-text" style={{ backgroundImage: "linear-gradient(to right, #00d4aa, #00d4aa)" }}>
                가치가 커지는 토큰
              </span>
            </h1>
            <p className="mt-6 text-xl max-w-2xl mx-auto leading-relaxed" style={{ color: "#7a8ba6" }}>
              고객의 구매와 활동이 트레저리에 쌓이고, 그 가치가 모든 토큰 보유자에게 돌아갑니다.
              <br />
              브랜드와 고객이 함께 성장하는 새로운 로열티 플랫폼.
            </p>
            <div className="mt-10 flex items-center justify-center gap-4 flex-wrap">
              <Link
                to="/signup"
                className="inline-flex items-center px-8 py-4 rounded-xl font-bold text-lg transition-colors shadow-lg"
                style={{ background: "#00d4aa", color: "#0c1018", boxShadow: "0 10px 25px rgba(0,212,170,0.3)" }}
              >
                무료로 시작하기
                <ArrowRight className="ml-2 h-5 w-5" />
              </Link>
              <Link
                to="/roi-simulator"
                className="inline-flex items-center px-8 py-4 rounded-xl font-bold text-lg transition-colors"
                style={{ border: "2px solid rgba(232,238,252,0.3)", color: "#e8eefc" }}
              >
                ROI 시뮬레이터
              </Link>
            </div>
          </div>

          {/* Right decoration - Mini chart + floating elements */}
          <div className="hidden lg:flex items-center justify-start">
            <div className="relative w-72 h-72">
              {/* Glow */}
              <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-36 h-36 rounded-full blur-[50px] opacity-30" style={{ background: "#00d4aa" }} />
              {/* Chart visualization */}
              <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-48">
                {/* SVG line chart */}
                <svg viewBox="0 0 200 100" className="w-full" style={{ filter: "drop-shadow(0 0 8px rgba(0,212,170,0.4))" }}>
                  <defs>
                    <linearGradient id="chartFill" x1="0" y1="0" x2="0" y2="1">
                      <stop offset="0%" stopColor="#00d4aa" stopOpacity="0.3" />
                      <stop offset="100%" stopColor="#00d4aa" stopOpacity="0" />
                    </linearGradient>
                  </defs>
                  <path d="M0,80 L20,72 L40,75 L60,58 L80,62 L100,45 L120,48 L140,32 L160,28 L180,18 L200,10 L200,100 L0,100 Z" fill="url(#chartFill)" />
                  <path d="M0,80 L20,72 L40,75 L60,58 L80,62 L100,45 L120,48 L140,32 L160,28 L180,18 L200,10" fill="none" stroke="#00d4aa" strokeWidth="2" strokeLinecap="round" />
                  {/* Data points */}
                  {[[0,80],[60,58],[100,45],[140,32],[200,10]].map(([x,y],i) => (
                    <circle key={i} cx={x} cy={y} r="3" fill="#00d4aa" style={{ filter: "drop-shadow(0 0 4px #00d4aa)" }} />
                  ))}
                </svg>
                <div className="flex justify-between mt-1 px-1">
                  <span className="text-[9px]" style={{ color: "#4a5568" }}>2025</span>
                  <span className="text-[9px] font-semibold" style={{ color: "#00d4aa" }}>▲ Floor Price</span>
                  <span className="text-[9px]" style={{ color: "#4a5568" }}>2026</span>
                </div>
              </div>
              {/* Orbiting nodes */}
              {[0,1,2,3,4,5].map((i) => (
                <div key={i} className="absolute w-2.5 h-2.5 rounded-full" style={{
                  background: "#00d4aa",
                  boxShadow: "0 0 10px rgba(0,212,170,0.6)",
                  top: `${50 + 42 * Math.sin((i * Math.PI * 2) / 6)}%`,
                  left: `${50 + 42 * Math.cos((i * Math.PI * 2) / 6)}%`,
                  animation: `floatParticle ${3 + i * 0.5}s ease-in-out ${i * 0.3}s infinite alternate`,
                  opacity: 0.4 + i * 0.1,
                }} />
              ))}
              {/* Connecting lines (SVG) */}
              <svg className="absolute inset-0 w-full h-full" style={{ opacity: 0.15 }}>
                {[0,1,2].map((i) => (
                  <line key={i}
                    x1={`${50 + 42 * Math.cos((i * Math.PI * 2) / 6)}%`}
                    y1={`${50 + 42 * Math.sin((i * Math.PI * 2) / 6)}%`}
                    x2={`${50 + 42 * Math.cos(((i + 3) * Math.PI * 2) / 6)}%`}
                    y2={`${50 + 42 * Math.sin(((i + 3) * Math.PI * 2) / 6)}%`}
                    stroke="#00d4aa" strokeWidth="1"
                  />
                ))}
              </svg>
            </div>
          </div>
        </div>
      </section>

      {/* ── How It Works (공통 구조) ── */}
      <section className="py-20 px-4" style={{ background: "#141c2b" }}>
        <div className="max-w-5xl mx-auto text-center mb-16">
          <h2 className="text-3xl md:text-4xl font-bold mb-4" style={{ color: "#e8eefc" }}>
            어떻게 작동하나요?
          </h2>
          <p className="text-lg max-w-2xl mx-auto" style={{ color: "#7a8ba6" }}>
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
                  <div
                    className="w-16 h-16 rounded-2xl flex items-center justify-center mx-auto mb-4 shadow-lg"
                    style={{ background: "#00d4aa", color: "#000000" }}
                  >
                    <Icon className="h-7 w-7" />
                  </div>
                  <p className="text-xs font-bold mb-1" style={{ color: "#00d4aa" }}>
                    STEP {item.step}
                  </p>
                  <h3 className="text-lg font-bold mb-2" style={{ color: "#e8eefc" }}>
                    {item.title}
                  </h3>
                  <p className="text-sm" style={{ color: "#7a8ba6" }}>
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
          className="py-20 px-4"
          style={{ background: idx % 2 === 0 ? "#0c1018" : "#141c2b" }}
        >
          <div className="max-w-6xl mx-auto">
            {/* Brand Header */}
            <div className="text-center mb-12">
              <span
                className="inline-block text-xs font-medium px-3 py-1 rounded-full mb-3"
                style={{ background: "rgba(0,212,170,0.15)", color: "#00d4aa" }}
              >
                {brand.segment}
              </span>
              <h2 className="text-3xl md:text-4xl font-bold mb-2" style={{ color: "#e8eefc" }}>
                {brand.brand}
              </h2>
              <p className="text-2xl font-bold" style={{ color: "#00d4aa" }}>
                "{brand.tagline}"
              </p>
              <p className="mt-2 text-lg" style={{ color: "#7a8ba6" }}>
                {brand.heroMessage}
              </p>
            </div>

            {/* Reward Flow */}
            <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-12">
              {brand.steps.map((step, si) => {
                const StepIcon = step.icon;
                return (
                  <div
                    key={si}
                    className="rounded-xl p-5 text-center relative"
                    style={{ background: "#1a2435" }}
                  >
                    {si < brand.steps.length - 1 && (
                      <div className="hidden md:block absolute top-1/2 -right-3 transform -translate-y-1/2 z-10">
                        <ArrowRight className="h-5 w-5" style={{ color: "#00d4aa" }} />
                      </div>
                    )}
                    <div
                      className="w-12 h-12 rounded-xl flex items-center justify-center mx-auto mb-3 shadow"
                      style={{ background: "#141c2b", color: "#00d4aa" }}
                    >
                      <StepIcon className="h-6 w-6" />
                    </div>
                    <h4 className="font-bold text-sm mb-1" style={{ color: "#e8eefc" }}>
                      {step.title}
                    </h4>
                    <p className="text-xs" style={{ color: "#7a8ba6" }}>
                      {step.desc}
                    </p>
                  </div>
                );
              })}
            </div>

            {/* Scenario + Quote */}
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-8 mb-12">
              {/* Detailed Scenario */}
              <div
                className="rounded-2xl shadow-lg p-8"
                style={{ background: "#141c2b", border: "1px solid rgba(0,212,170,0.12)" }}
              >
                <h3 className="text-lg font-bold mb-6" style={{ color: "#e8eefc" }}>
                  구체적인 시나리오
                </h3>
                <div className="space-y-4">
                  <div className="flex items-start gap-3">
                    <CheckCircle className="h-5 w-5 mt-0.5 flex-shrink-0" style={{ color: "#00d4aa" }} />
                    <div>
                      <p className="font-medium" style={{ color: "#e8eefc" }}>
                        {brand.scenario.purchaseItem} 구매
                      </p>
                      <p className="text-sm" style={{ color: "#7a8ba6" }}>
                        {formatWon(brand.scenario.purchasePrice)}의 {brand.scenario.rewardRate}% →{" "}
                        <span className="font-bold" style={{ color: "#00d4aa" }}>
                          {formatWon(brand.scenario.rewardAmount)} 적립
                        </span>
                      </p>
                    </div>
                  </div>
                  <div className="flex items-start gap-3">
                    <CheckCircle className="h-5 w-5 mt-0.5 flex-shrink-0" style={{ color: "#00d4aa" }} />
                    <div>
                      <p className="font-medium" style={{ color: "#e8eefc" }}>
                        {brand.scenario.activityType} 리워드
                      </p>
                      <p className="text-sm" style={{ color: "#7a8ba6" }}>
                        {brand.scenario.activityDetail} →{" "}
                        <span className="font-bold" style={{ color: "#00d4aa" }}>
                          +{brand.scenario.activityReward} 포인트
                        </span>
                      </p>
                    </div>
                  </div>
                  <div className="flex items-start gap-3">
                    <CheckCircle className="h-5 w-5 mt-0.5 flex-shrink-0" style={{ color: "#00d4aa" }} />
                    <div>
                      <p className="font-medium" style={{ color: "#e8eefc" }}>
                        월간 활동 리워드
                      </p>
                      <p className="text-sm" style={{ color: "#7a8ba6" }}>
                        {brand.scenario.monthlyActivity} →{" "}
                        <span className="font-bold" style={{ color: "#00d4aa" }}>
                          월 +{formatNumber(brand.scenario.monthlyReward)} 포인트
                        </span>
                      </p>
                    </div>
                  </div>
                  <div
                    className="mt-6 p-4 rounded-xl"
                    style={{ background: "#1a2435", border: "1px solid rgba(0,212,170,0.12)" }}
                  >
                    <p className="text-sm" style={{ color: "#7a8ba6" }}>6개월 예상 총 적립</p>
                    <p className="text-3xl font-extrabold" style={{ color: "#00d4aa" }}>
                      {formatWon(brand.scenario.sixMonthTotal)}
                    </p>
                    <p className="text-xs mt-1" style={{ color: "#4a5568" }}>
                      구매 적립 + 활동 리워드 합산
                    </p>
                  </div>
                </div>
              </div>

              {/* Customer Quote + Stats */}
              <div className="flex flex-col gap-6">
                {/* Quote */}
                <div
                  className="rounded-2xl p-8 flex-1"
                  style={{
                    background: "linear-gradient(135deg, #141c2b 0%, #1a2435 100%)",
                    borderLeft: "4px solid #00d4aa",
                    color: "#e8eefc",
                  }}
                >
                  <p className="text-lg leading-relaxed mb-6" style={{ opacity: 0.95 }}>
                    "{brand.customerQuote}"
                  </p>
                  <p className="text-sm font-medium" style={{ opacity: 0.8 }}>
                    — {brand.customerName}
                  </p>
                </div>

                {/* Stats */}
                <div className="grid grid-cols-2 gap-4">
                  <div
                    className="rounded-xl p-4 shadow text-center"
                    style={{ background: "#141c2b", border: "1px solid rgba(0,212,170,0.12)" }}
                  >
                    <p className="text-xs" style={{ color: "#7a8ba6" }}>트레저리</p>
                    <p className="text-xl font-bold" style={{ color: "#e8eefc" }}>
                      {formatUSD(brand.stats.treasury)}
                    </p>
                  </div>
                  <div
                    className="rounded-xl p-4 shadow text-center"
                    style={{ background: "#141c2b", border: "1px solid rgba(0,212,170,0.12)" }}
                  >
                    <p className="text-xs" style={{ color: "#7a8ba6" }}>활성 유저</p>
                    <p className="text-xl font-bold" style={{ color: "#e8eefc" }}>
                      {formatNumber(brand.stats.users)}명
                    </p>
                  </div>
                  <div
                    className="rounded-xl p-4 shadow text-center"
                    style={{ background: "#141c2b", border: "1px solid rgba(0,212,170,0.12)" }}
                  >
                    <p className="text-xs" style={{ color: "#7a8ba6" }}>토큰 하한가</p>
                    <p className="text-xl font-bold" style={{ color: "#e8eefc" }}>
                      {formatUSD(brand.stats.floorPrice)}
                    </p>
                  </div>
                  <div
                    className="rounded-xl p-4 shadow text-center"
                    style={{ background: "#141c2b", border: "1px solid rgba(0,212,170,0.12)" }}
                  >
                    <p className="text-xs" style={{ color: "#7a8ba6" }}>리텐션 향상</p>
                    <p className="text-xl font-bold" style={{ color: "#00d4aa" }}>
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
      <section className="py-20 px-4" style={{ background: "#0c1018" }}>
        <div className="max-w-6xl mx-auto">
          <div className="text-center mb-16">
            <h2 className="text-3xl md:text-4xl font-bold mb-4" style={{ color: "#e8eefc" }}>
              기업을 위한 올인원 플랫폼
            </h2>
            <p className="text-lg" style={{ color: "#7a8ba6" }}>
              블록체인 지식 없이도 토큰 이코노미를 구축할 수 있습니다
            </p>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {features.map((feature) => {
              const Icon = feature.icon;
              return (
                <div
                  key={feature.name}
                  className="rounded-xl p-6 hover:shadow-lg transition-shadow"
                  style={{ background: "#141c2b", border: "1px solid rgba(0,212,170,0.12)" }}
                >
                  <div
                    className="w-12 h-12 rounded-xl flex items-center justify-center mb-4"
                    style={{ background: "rgba(0,212,170,0.15)" }}
                  >
                    <Icon className="w-6 h-6" style={{ color: "#00d4aa" }} />
                  </div>
                  <h3 className="text-lg font-bold" style={{ color: "#e8eefc" }}>
                    {feature.name}
                  </h3>
                  <p className="mt-2 text-sm leading-relaxed" style={{ color: "#7a8ba6" }}>
                    {feature.description}
                  </p>
                </div>
              );
            })}
          </div>
        </div>
      </section>

      {/* ── For Businesses ── */}
      <section className="py-20 px-4" style={{ background: "#141c2b" }}>
        <div className="max-w-5xl mx-auto">
          <div className="text-center mb-16">
            <h2 className="text-3xl md:text-4xl font-bold mb-4" style={{ color: "#e8eefc" }}>
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
                className="rounded-xl p-8 shadow"
                style={{ background: "#1a2435", border: "1px solid rgba(0,212,170,0.12)" }}
              >
                <h3 className="text-xl font-bold mb-3" style={{ color: "#e8eefc" }}>
                  {item.title}
                </h3>
                <p className="leading-relaxed" style={{ color: "#7a8ba6" }}>
                  {item.desc}
                </p>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* ── CTA ── */}
      <section
        className="py-24 px-4 text-center relative overflow-hidden"
        style={{ background: "linear-gradient(135deg, #00d4aa 0%, #00856b 100%)" }}
      >
        <div className="absolute inset-0 opacity-10">
          <div className="absolute top-10 right-1/4 w-80 h-80 rounded-full blur-3xl" style={{ background: "#ffffff" }} />
        </div>
        <div className="relative z-10">
          <h2 className="text-4xl font-extrabold mb-4" style={{ color: "#ffffff" }}>
            지금 시작하세요
          </h2>
          <p className="mb-10 max-w-xl mx-auto text-lg" style={{ color: "rgba(255,255,255,0.85)" }}>
            고객이 사고, 쓰고, 즐길수록 함께 성장하는 토큰 이코노미.
            <br />
            당신의 브랜드도 가능합니다.
          </p>
          <div className="flex items-center justify-center gap-4 flex-wrap">
            <Link
              to="/signup"
              className="inline-flex items-center px-8 py-4 rounded-xl font-bold text-lg transition-colors shadow-lg"
              style={{ background: "#ffffff", color: "#00856b" }}
            >
              무료로 시작하기
              <ArrowRight className="ml-2 h-5 w-5" />
            </Link>
            <Link
              to="/pricing"
              className="inline-flex items-center px-8 py-4 rounded-xl font-bold text-lg transition-colors"
              style={{ border: "2px solid rgba(255,255,255,0.4)", color: "#ffffff" }}
            >
              요금제 보기
            </Link>
          </div>
        </div>
      </section>
    </>
  );
}

/* ========================================================================
   WalletSection
   ======================================================================== */
function WalletSection() {
  const holdings = [
    { token: "LMT", brand: "Le Mouton", price: 0.0245, change24h: 3.2, amount: 8.5, value: 0.2083 },
    { token: "CBT", brand: "Cafe Blossom", price: 0.0246, change24h: 1.8, amount: 3.24, value: 0.0797 },
    { token: "RPT", brand: "RunPulse", price: 0.0179, change24h: 5.4, amount: 1.5, value: 0.0269 },
  ];

  const totalValue = 0.49;
  const totalWon = 588;

  const chartBars = [28, 32, 30, 35, 38, 36, 40, 42, 45, 48, 52, 58];

  const recentActivity = [
    { emoji: "\uD83D\uDECD\uFE0F", text: "Le Mouton 컴포트 워커 구매 - +2,580 LMT", time: "2시간 전" },
    { emoji: "\uD83D\uDEB6", text: "걷기 8,000걸음 달성 - +80 LMT", time: "5시간 전" },
    { emoji: "\u2615", text: "Cafe Blossom 시그니처 라떼 - +195 CBT", time: "1일 전" },
    { emoji: "\uD83C\uDFC3", text: "RunPulse 5km 러닝 완료 - +150 RPT", time: "2일 전" },
    { emoji: "\uD83C\uDF81", text: "주간 방문 보너스 - +50 CBT", time: "3일 전" },
  ];

  const growthSteps = [
    { emoji: "\uD83D\uDECD\uFE0F", title: "구매 & 활동", desc: "브랜드 제품을 구매하고 활동하면 토큰이 적립됩니다" },
    { emoji: "\uD83C\uDFE6", title: "트레저리 축적", desc: "매출의 일부가 트레저리에 쌓여 토큰 가치를 뒷받침합니다" },
    { emoji: "\uD83D\uDCC8", title: "가치 상승", desc: "트레저리가 커질수록 토큰 하한가가 올라갑니다" },
    { emoji: "\uD83D\uDCB0", title: "수익 실현", desc: "토큰의 가치가 성장하여 실제 수익으로 이어집니다" },
  ];

  return (
    <div className="py-12 px-4">
      <div className="max-w-6xl mx-auto">
        {/* Portfolio Value Hero Card */}
        <div
          className="rounded-2xl p-8 mb-8"
          style={{
            background: "linear-gradient(135deg, #141c2b, #1a2435)",
            border: "1px solid rgba(0,212,170,0.12)",
          }}
        >
          <p className="text-sm mb-1" style={{ color: "#7a8ba6" }}>Total Portfolio Value</p>
          <div className="flex items-end gap-4 mb-2">
            <h2 className="text-4xl font-extrabold" style={{ color: "#e8eefc" }}>${totalValue.toFixed(2)}</h2>
            <span className="text-lg mb-1" style={{ color: "#7a8ba6" }}>{"\u2248"} {"\u20A9"}{totalWon.toLocaleString()}</span>
          </div>
          <span
            className="inline-block text-xs font-medium px-3 py-1 rounded-full"
            style={{ background: "rgba(0,212,170,0.15)", color: "#00d4aa" }}
          >
            +3.1% this month
          </span>
        </div>

        {/* Holdings Table */}
        <div
          className="rounded-2xl overflow-hidden mb-8"
          style={{ background: "#141c2b", border: "1px solid rgba(0,212,170,0.12)" }}
        >
          <div className="p-6 pb-4">
            <h3 className="text-lg font-bold" style={{ color: "#e8eefc" }}>Holdings</h3>
          </div>
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr style={{ borderBottom: "1px solid rgba(0,212,170,0.12)" }}>
                  {["Token", "Brand", "Price", "24h", "Holdings", "Value"].map((h) => (
                    <th key={h} className="px-6 py-3 text-left text-xs font-medium" style={{ color: "#7a8ba6" }}>
                      {h}
                    </th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {holdings.map((h) => (
                  <tr
                    key={h.token}
                    style={{ borderBottom: "1px solid rgba(0,212,170,0.06)" }}
                  >
                    <td className="px-6 py-4 font-bold text-sm" style={{ color: "#e8eefc" }}>{h.token}</td>
                    <td className="px-6 py-4 text-sm" style={{ color: "#7a8ba6" }}>{h.brand}</td>
                    <td className="px-6 py-4 text-sm" style={{ color: "#e8eefc" }}>${h.price.toFixed(4)}</td>
                    <td className="px-6 py-4 text-sm font-medium" style={{ color: "#00d4aa" }}>+{h.change24h}%</td>
                    <td className="px-6 py-4 text-sm" style={{ color: "#e8eefc" }}>{h.amount}</td>
                    <td className="px-6 py-4 text-sm font-medium" style={{ color: "#e8eefc" }}>${h.value.toFixed(4)}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>

        {/* Price Chart */}
        <div
          className="rounded-2xl p-6 mb-8"
          style={{ background: "#141c2b", border: "1px solid rgba(0,212,170,0.12)" }}
        >
          <h3 className="text-lg font-bold mb-6" style={{ color: "#e8eefc" }}>Portfolio Performance</h3>
          <div className="flex items-end gap-2" style={{ height: 120 }}>
            {chartBars.map((bar, i) => (
              <div
                key={i}
                className="flex-1 rounded-t"
                style={{
                  height: `${(bar / 58) * 100}%`,
                  background: `linear-gradient(to top, rgba(0,212,170,0.4), #00d4aa)`,
                  opacity: 0.5 + (i / chartBars.length) * 0.5,
                }}
              />
            ))}
          </div>
          <div className="flex justify-between mt-2">
            <span className="text-xs" style={{ color: "#4a5568" }}>12 months ago</span>
            <span className="text-xs" style={{ color: "#4a5568" }}>Now</span>
          </div>
        </div>

        {/* Recent Activity */}
        <div
          className="rounded-2xl p-6 mb-8"
          style={{ background: "#141c2b", border: "1px solid rgba(0,212,170,0.12)" }}
        >
          <h3 className="text-lg font-bold mb-4" style={{ color: "#e8eefc" }}>Recent Activity</h3>
          <div className="space-y-3">
            {recentActivity.map((a, i) => (
              <div
                key={i}
                className="flex items-center gap-3 p-3 rounded-xl"
                style={{ background: "#1a2435" }}
              >
                <span className="text-xl">{a.emoji}</span>
                <div className="flex-1">
                  <p className="text-sm" style={{ color: "#e8eefc" }}>{a.text}</p>
                </div>
                <span className="text-xs" style={{ color: "#4a5568" }}>{a.time}</span>
              </div>
            ))}
          </div>
        </div>

        {/* How Your Token Value Grows */}
        <div
          className="rounded-2xl p-6"
          style={{ background: "#141c2b", border: "1px solid rgba(0,212,170,0.12)" }}
        >
          <h3 className="text-lg font-bold mb-6 text-center" style={{ color: "#e8eefc" }}>
            How Your Token Value Grows
          </h3>
          <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
            {growthSteps.map((s, i) => (
              <div key={i} className="text-center p-4 rounded-xl relative" style={{ background: "#1a2435" }}>
                {i < growthSteps.length - 1 && (
                  <div className="hidden md:block absolute top-1/2 -right-3 transform -translate-y-1/2 z-10">
                    <ArrowRight className="h-5 w-5" style={{ color: "#00d4aa" }} />
                  </div>
                )}
                <div className="text-3xl mb-3">{s.emoji}</div>
                <h4 className="font-bold text-sm mb-1" style={{ color: "#e8eefc" }}>{s.title}</h4>
                <p className="text-xs" style={{ color: "#7a8ba6" }}>{s.desc}</p>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}

/* ========================================================================
   SwapSection
   ======================================================================== */
function SwapSection() {
  const tokens = ["LMT", "CBT", "RPT"] as const;
  type Token = (typeof tokens)[number];

  const rates: Record<string, Record<string, number>> = {
    LMT: { CBT: 0.85, RPT: 1.2 },
    CBT: { LMT: 1.18, RPT: 1.41 },
    RPT: { LMT: 0.83, CBT: 0.71 },
  };

  const balances: Record<string, number> = { LMT: 8.5, CBT: 3.24, RPT: 1.5 };
  const brandNames: Record<string, string> = { LMT: "Le Mouton", CBT: "Cafe Blossom", RPT: "RunPulse" };

  const [fromToken, setFromToken] = useState<Token>("LMT");
  const [toToken, setToToken] = useState<Token>("CBT");
  const [fromAmount, setFromAmount] = useState<string>("");

  const numericFrom = parseFloat(fromAmount) || 0;
  const rate = fromToken === toToken ? 1 : (rates[fromToken]?.[toToken] ?? 0);
  const toAmount = numericFrom * rate;

  const recentSwaps = [
    { date: "2026-03-28", from: "LMT", to: "CBT", amount: "2.0", rate: "0.85", status: "Completed" },
    { date: "2026-03-25", from: "CBT", to: "RPT", amount: "1.5", rate: "1.41", status: "Completed" },
    { date: "2026-03-20", from: "RPT", to: "LMT", amount: "1.0", rate: "0.83", status: "Completed" },
  ];

  return (
    <div className="py-12 px-4">
      <div className="max-w-6xl mx-auto">
        {/* Swap Card */}
        <div className="max-w-md mx-auto mb-12">
          <div
            className="rounded-2xl p-6"
            style={{ background: "#141c2b", border: "1px solid rgba(0,212,170,0.12)" }}
          >
            <h3 className="text-lg font-bold mb-6 text-center" style={{ color: "#e8eefc" }}>Swap Tokens</h3>

            {/* From */}
            <div className="mb-1">
              <p className="text-xs font-medium mb-2" style={{ color: "#7a8ba6" }}>From</p>
              <div className="flex gap-2">
                <select
                  value={fromToken}
                  onChange={(e) => {
                    const val = e.target.value as Token;
                    setFromToken(val);
                    if (val === toToken) {
                      const other = tokens.find((t) => t !== val);
                      if (other) setToToken(other);
                    }
                  }}
                  className="rounded-xl px-4 py-3 text-sm font-bold outline-none cursor-pointer"
                  style={{ background: "#1a2435", color: "#e8eefc", border: "1px solid rgba(0,212,170,0.12)" }}
                >
                  {tokens.map((t) => (
                    <option key={t} value={t}>{t}</option>
                  ))}
                </select>
                <input
                  type="number"
                  placeholder="0.00"
                  value={fromAmount}
                  onChange={(e) => setFromAmount(e.target.value)}
                  className="flex-1 rounded-xl px-4 py-3 text-sm outline-none text-right"
                  style={{ background: "#1a2435", color: "#e8eefc", border: "1px solid rgba(0,212,170,0.12)" }}
                />
              </div>
              <p className="text-xs mt-1" style={{ color: "#4a5568" }}>
                Available: {balances[fromToken]} {fromToken}
              </p>
            </div>

            {/* Swap button */}
            <div className="flex justify-center my-3">
              <button
                onClick={() => {
                  const tmp = fromToken;
                  setFromToken(toToken);
                  setToToken(tmp);
                }}
                className="w-10 h-10 rounded-full flex items-center justify-center transition-transform hover:rotate-180"
                style={{ background: "#1a2435", border: "1px solid rgba(0,212,170,0.12)", color: "#00d4aa" }}
              >
                <ArrowDownUp className="h-4 w-4" />
              </button>
            </div>

            {/* To */}
            <div className="mb-4">
              <p className="text-xs font-medium mb-2" style={{ color: "#7a8ba6" }}>To</p>
              <div className="flex gap-2">
                <select
                  value={toToken}
                  onChange={(e) => {
                    const val = e.target.value as Token;
                    setToToken(val);
                    if (val === fromToken) {
                      const other = tokens.find((t) => t !== val);
                      if (other) setFromToken(other);
                    }
                  }}
                  className="rounded-xl px-4 py-3 text-sm font-bold outline-none cursor-pointer"
                  style={{ background: "#1a2435", color: "#e8eefc", border: "1px solid rgba(0,212,170,0.12)" }}
                >
                  {tokens.map((t) => (
                    <option key={t} value={t}>{t}</option>
                  ))}
                </select>
                <input
                  type="text"
                  readOnly
                  value={numericFrom > 0 ? toAmount.toFixed(4) : ""}
                  placeholder="0.00"
                  className="flex-1 rounded-xl px-4 py-3 text-sm outline-none text-right"
                  style={{ background: "#1a2435", color: "#7a8ba6", border: "1px solid rgba(0,212,170,0.12)" }}
                />
              </div>
              <p className="text-xs mt-1" style={{ color: "#4a5568" }}>
                Available: {balances[toToken]} {toToken}
              </p>
            </div>

            {/* Exchange rate */}
            <div className="text-center mb-4">
              <p className="text-xs" style={{ color: "#7a8ba6" }}>
                1 {fromToken} = {fromToken === toToken ? "1" : rate.toFixed(2)} {toToken}
              </p>
            </div>

            {/* Swap action */}
            <button
              className="w-full py-3 rounded-xl font-bold text-sm transition-colors"
              style={{ background: "#00d4aa", color: "#0c1018" }}
            >
              Swap
            </button>
          </div>
        </div>

        {/* My Balances Row */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-8">
          {tokens.map((t) => (
            <div
              key={t}
              className="rounded-xl p-5 text-center"
              style={{ background: "#141c2b", border: "1px solid rgba(0,212,170,0.12)" }}
            >
              <p className="text-xs mb-1" style={{ color: "#7a8ba6" }}>{brandNames[t]}</p>
              <p className="text-2xl font-bold" style={{ color: "#e8eefc" }}>{balances[t]}</p>
              <p className="text-xs font-medium" style={{ color: "#00d4aa" }}>{t}</p>
            </div>
          ))}
        </div>

        {/* Recent Swaps Table */}
        <div
          className="rounded-2xl overflow-hidden"
          style={{ background: "#141c2b", border: "1px solid rgba(0,212,170,0.12)" }}
        >
          <div className="p-6 pb-4">
            <h3 className="text-lg font-bold" style={{ color: "#e8eefc" }}>Recent Swaps</h3>
          </div>
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr style={{ borderBottom: "1px solid rgba(0,212,170,0.12)" }}>
                  {["Date", "From", "To", "Amount", "Rate", "Status"].map((h) => (
                    <th key={h} className="px-6 py-3 text-left text-xs font-medium" style={{ color: "#7a8ba6" }}>
                      {h}
                    </th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {recentSwaps.map((s, i) => (
                  <tr key={i} style={{ borderBottom: "1px solid rgba(0,212,170,0.06)" }}>
                    <td className="px-6 py-4 text-sm" style={{ color: "#7a8ba6" }}>{s.date}</td>
                    <td className="px-6 py-4 text-sm font-medium" style={{ color: "#e8eefc" }}>{s.from}</td>
                    <td className="px-6 py-4 text-sm font-medium" style={{ color: "#e8eefc" }}>{s.to}</td>
                    <td className="px-6 py-4 text-sm" style={{ color: "#e8eefc" }}>{s.amount}</td>
                    <td className="px-6 py-4 text-sm" style={{ color: "#7a8ba6" }}>{s.rate}</td>
                    <td className="px-6 py-4 text-sm">
                      <span
                        className="inline-block px-2 py-0.5 rounded-full text-xs font-medium"
                        style={{ background: "rgba(0,212,170,0.15)", color: "#00d4aa" }}
                      >
                        {s.status}
                      </span>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  );
}

/* ========================================================================
   TokensSection
   ======================================================================== */
function TokensSection() {
  const tokenList = [
    {
      rank: 1,
      token: "LMT",
      brand: "Le Mouton",
      price: 0.0245,
      change24h: 3.2,
      marketCap: 24500,
      circulating: 45000,
      totalSupply: 1000000,
      floorPrice: 0.0245,
    },
    {
      rank: 2,
      token: "CBT",
      brand: "Cafe Blossom",
      price: 0.0246,
      change24h: 1.8,
      marketCap: 12300,
      circulating: 22000,
      totalSupply: 500000,
      floorPrice: 0.0246,
    },
    {
      rank: 3,
      token: "RPT",
      brand: "RunPulse",
      price: 0.0179,
      change24h: 5.4,
      marketCap: 35800,
      circulating: 68000,
      totalSupply: 2000000,
      floorPrice: 0.0179,
    },
  ];

  const totalMarketCap = 72600;
  const totalTokens = 3;
  const avgFloorPrice = 0.0223;

  return (
    <div className="py-12 px-4">
      <div className="max-w-6xl mx-auto">
        {/* Header */}
        <div className="mb-8">
          <h2 className="text-3xl font-bold mb-2" style={{ color: "#e8eefc" }}>Token Market</h2>
          <p style={{ color: "#7a8ba6" }}>All revenue-backed tokens on Biyard</p>
        </div>

        {/* Market Stats Row */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-8">
          {[
            { label: "Total Market Cap", value: formatUSD(totalMarketCap) },
            { label: "Total Tokens", value: totalTokens.toString() },
            { label: "Avg Floor Price", value: `$${avgFloorPrice.toFixed(4)}` },
          ].map((kpi) => (
            <div
              key={kpi.label}
              className="rounded-xl p-5 text-center"
              style={{ background: "#141c2b", border: "1px solid rgba(0,212,170,0.12)" }}
            >
              <p className="text-xs mb-1" style={{ color: "#7a8ba6" }}>{kpi.label}</p>
              <p className="text-2xl font-bold" style={{ color: "#e8eefc" }}>{kpi.value}</p>
            </div>
          ))}
        </div>

        {/* Token List Table */}
        <div
          className="rounded-2xl overflow-hidden"
          style={{ background: "#141c2b", border: "1px solid rgba(0,212,170,0.12)" }}
        >
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr style={{ borderBottom: "1px solid rgba(0,212,170,0.12)" }}>
                  {["#", "Token", "Brand", "Price", "24h Change", "Market Cap", "Circulating Supply", "Floor Price"].map((h) => (
                    <th key={h} className="px-6 py-4 text-left text-xs font-medium" style={{ color: "#7a8ba6" }}>
                      {h}
                    </th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {tokenList.map((t) => (
                  <tr
                    key={t.token}
                    className="transition-colors"
                    style={{ borderBottom: "1px solid rgba(0,212,170,0.06)" }}
                    onMouseEnter={(e) => { (e.currentTarget as HTMLElement).style.background = "#1a2435"; }}
                    onMouseLeave={(e) => { (e.currentTarget as HTMLElement).style.background = "transparent"; }}
                  >
                    <td className="px-6 py-4 text-sm" style={{ color: "#7a8ba6" }}>{t.rank}</td>
                    <td className="px-6 py-4 text-sm font-bold" style={{ color: "#e8eefc" }}>{t.token}</td>
                    <td className="px-6 py-4 text-sm" style={{ color: "#7a8ba6" }}>{t.brand}</td>
                    <td className="px-6 py-4 text-sm" style={{ color: "#e8eefc" }}>${t.price.toFixed(4)}</td>
                    <td className="px-6 py-4 text-sm font-medium" style={{ color: "#00d4aa" }}>+{t.change24h}%</td>
                    <td className="px-6 py-4 text-sm" style={{ color: "#e8eefc" }}>{formatUSD(t.marketCap)}</td>
                    <td className="px-6 py-4 text-sm" style={{ color: "#7a8ba6" }}>
                      {formatNumber(t.circulating)} / {formatNumber(t.totalSupply)}
                    </td>
                    <td className="px-6 py-4 text-sm" style={{ color: "#e8eefc" }}>${t.floorPrice.toFixed(4)}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  );
}

/* ========================================================================
   DAOSection
   ======================================================================== */
function DAOSection() {
  const [votes, setVotes] = useState<Record<number, "yes" | "no" | null>>({
    0: null,
    1: null,
    2: "yes", // already voted
  });

  const proposals = [
    {
      id: 0,
      brand: "Le Mouton",
      title: "\uB9AC\uC6CC\uB4DC \uBC30\uC218 2\uBC30 \uC99D\uAC00",
      description: "\uAC78\uAE30 \uB9AC\uC6CC\uB4DC \uBC30\uC218\uB97C \uD604\uC7AC 1\uBC30\uC5D0\uC11C 2\uBC30\uB85C \uC99D\uAC00\uC2DC\uCF1C \uACE0\uAC1D \uD65C\uB3D9\uC744 \uB354\uC6B1 \uC7A5\uB824\uD569\uB2C8\uB2E4.",
      yesVotes: 1250,
      noVotes: 340,
      deadline: "2026-04-15",
    },
    {
      id: 1,
      brand: "Cafe Blossom",
      title: "\uC2E0\uBA54\uB274 \uCD9C\uC2DC \uAE30\uB150 \uBCF4\uB108\uC2A4",
      description: "\uC2E0\uBA54\uB274 \uCD9C\uC2DC\uB97C \uAE30\uB150\uD558\uC5EC \uCCAB \uC8FC\uBB38 \uACE0\uAC1D\uC5D0\uAC8C 3\uBC30 \uBCF4\uB108\uC2A4 \uD1A0\uD070\uC744 \uC9C0\uAE09\uD569\uB2C8\uB2E4.",
      yesVotes: 890,
      noVotes: 120,
      deadline: "2026-04-20",
    },
    {
      id: 2,
      brand: "RunPulse",
      title: "\uB9C8\uB77C\uD1A4 \uC774\uBCA4\uD2B8 \uD1A0\uD070 \uBC30\uBD84",
      description: "\uB9C8\uB77C\uD1A4 \uC774\uBCA4\uD2B8 \uCC38\uAC00\uC790\uC5D0\uAC8C \uD2B9\uBCC4 \uD1A0\uD070 \uBCF4\uC0C1\uC744 \uBC30\uBD84\uD569\uB2C8\uB2E4.",
      yesVotes: 2100,
      noVotes: 180,
      deadline: "2026-04-10",
    },
  ];

  const handleVote = (proposalId: number, vote: "yes" | "no") => {
    if (votes[proposalId] !== null) return;
    setVotes((prev) => ({ ...prev, [proposalId]: vote }));
  };

  return (
    <div className="py-12 px-4">
      <div className="max-w-4xl mx-auto">
        {/* Header */}
        <div className="mb-8 text-center">
          <h2 className="text-3xl font-bold mb-2" style={{ color: "#e8eefc" }}>DAO Governance</h2>
          <p style={{ color: "#7a8ba6" }}>{"\uD1A0\uD070 \uBCF4\uC720\uC790\uB85C\uC11C \uBE0C\uB79C\uB4DC\uC758 \uBBF8\uB798\uB97C \uACB0\uC815\uD558\uC138\uC694"}</p>
        </div>

        {/* Proposal Cards */}
        <div className="space-y-6">
          {proposals.map((p) => {
            const totalVotes = p.yesVotes + p.noVotes;
            const yesPercent = Math.round((p.yesVotes / totalVotes) * 100);
            const noPercent = 100 - yesPercent;
            const userVote = votes[p.id];

            return (
              <div
                key={p.id}
                className="rounded-2xl p-6"
                style={{ background: "#141c2b", border: "1px solid rgba(0,212,170,0.12)" }}
              >
                {/* Brand badge */}
                <span
                  className="inline-block text-xs font-bold px-3 py-1 rounded-full mb-3"
                  style={{ background: "#00d4aa", color: "#0c1018" }}
                >
                  {p.brand}
                </span>

                <h3 className="text-xl font-bold mb-2" style={{ color: "#e8eefc" }}>{p.title}</h3>
                <p className="text-sm mb-5" style={{ color: "#7a8ba6" }}>{p.description}</p>

                {/* Progress bars */}
                <div className="mb-4 space-y-3">
                  {/* Yes bar */}
                  <div>
                    <div className="flex justify-between text-xs mb-1">
                      <span style={{ color: "#00d4aa" }}>Yes ({formatNumber(p.yesVotes)})</span>
                      <span style={{ color: "#00d4aa" }}>{yesPercent}%</span>
                    </div>
                    <div className="w-full h-3 rounded-full overflow-hidden" style={{ background: "#1a2435" }}>
                      <div
                        className="h-full rounded-full transition-all"
                        style={{ width: `${yesPercent}%`, background: "#00d4aa" }}
                      />
                    </div>
                  </div>
                  {/* No bar */}
                  <div>
                    <div className="flex justify-between text-xs mb-1">
                      <span style={{ color: "#ef4444" }}>No ({formatNumber(p.noVotes)})</span>
                      <span style={{ color: "#ef4444" }}>{noPercent}%</span>
                    </div>
                    <div className="w-full h-3 rounded-full overflow-hidden" style={{ background: "#1a2435" }}>
                      <div
                        className="h-full rounded-full transition-all"
                        style={{ width: `${noPercent}%`, background: "#ef4444" }}
                      />
                    </div>
                  </div>
                </div>

                {/* Vote buttons / indicator */}
                <div className="flex items-center justify-between">
                  <div className="flex gap-2">
                    {userVote !== null ? (
                      <span
                        className="inline-flex items-center gap-1 text-sm font-medium px-4 py-2 rounded-xl"
                        style={{
                          background: "rgba(0,212,170,0.15)",
                          color: "#00d4aa",
                        }}
                      >
                        <CheckCircle className="h-4 w-4" />
                        {"\uD22C\uD45C \uC644\uB8CC"} ({userVote === "yes" ? "Yes" : "No"})
                      </span>
                    ) : (
                      <>
                        <button
                          onClick={() => handleVote(p.id, "yes")}
                          className="px-5 py-2 rounded-xl text-sm font-bold transition-colors"
                          style={{ background: "#00d4aa", color: "#0c1018" }}
                        >
                          Yes
                        </button>
                        <button
                          onClick={() => handleVote(p.id, "no")}
                          className="px-5 py-2 rounded-xl text-sm font-bold transition-colors"
                          style={{ background: "rgba(239,68,68,0.2)", color: "#ef4444", border: "1px solid rgba(239,68,68,0.3)" }}
                        >
                          No
                        </button>
                      </>
                    )}
                  </div>
                  <span className="text-xs" style={{ color: "#4a5568" }}>
                    Deadline: {p.deadline}
                  </span>
                </div>
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
}

/* ========================================================================
   DemoPage  (main export with tab navigation)
   ======================================================================== */
const cssAnimations = `
@keyframes spinCube {
  0% { transform: rotateX(-20deg) rotateY(0deg); }
  100% { transform: rotateX(-20deg) rotateY(360deg); }
}
@keyframes floatParticle {
  0% { transform: translateY(0px); opacity: 0.4; }
  100% { transform: translateY(-12px); opacity: 1; }
}
`;

export function DemoPage() {
  const [activeTab, setActiveTab] = useState<TabKey>("home");

  const tabs: { key: TabKey; label: string; icon: React.ComponentType<{ className?: string }> }[] = [
    { key: "home", label: "Home", icon: Home },
    { key: "wallet", label: "Wallet", icon: Wallet },
    { key: "swap", label: "Swap", icon: ArrowDownUp },
    { key: "tokens", label: "Tokens", icon: ListOrdered },
    { key: "dao", label: "DAO", icon: Vote },
  ];

  return (
    <div className="min-h-screen" style={{ background: "#0c1018", color: "#e8eefc" }}>
      <style>{cssAnimations}</style>
      {/* ── Navigation Bar ── */}
      <nav
        className="px-4 py-3"
        style={{ background: "#0c1018", borderBottom: "1px solid rgba(0,212,170,0.12)" }}
      >
        <div className="max-w-6xl mx-auto flex items-center justify-between">
          {/* Left: Brand */}
          <span className="text-xl font-extrabold" style={{ color: "#00d4aa" }}>
            Biyard
          </span>

          {/* Center: Tabs */}
          <div className="flex items-center gap-1">
            {tabs.map((tab) => {
              const Icon = tab.icon;
              const isActive = activeTab === tab.key;
              return (
                <button
                  key={tab.key}
                  onClick={() => setActiveTab(tab.key)}
                  className="flex items-center gap-1.5 px-4 py-2 rounded-lg text-sm font-medium transition-colors"
                  style={{
                    background: isActive ? "#141c2b" : "transparent",
                    color: isActive ? "#00d4aa" : "#7a8ba6",
                  }}
                >
                  <Icon className="h-4 w-4" />
                  <span className="hidden sm:inline">{tab.label}</span>
                </button>
              );
            })}
          </div>

          {/* Right: Sign In */}
          <button
            className="px-4 py-2 rounded-lg text-sm font-medium transition-colors"
            style={{ border: "1px solid #00d4aa", color: "#00d4aa", background: "transparent" }}
          >
            Sign In
          </button>
        </div>
      </nav>

      {/* ── Tab Content ── */}
      {activeTab === "home" && <HomeSection />}
      {activeTab === "wallet" && <WalletSection />}
      {activeTab === "swap" && <SwapSection />}
      {activeTab === "tokens" && <TokensSection />}
      {activeTab === "dao" && <DAOSection />}
    </div>
  );
}
