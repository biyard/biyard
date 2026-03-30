// Mock data service for launchpad features

export interface DashboardStats {
  treasury: number;
  last_revenue_krw: number;
  user_count: number;
  challenge_participants: number;
  total_tokens_issued: number;
  total_steps: number;
}

export interface TreasuryRow {
  month: string;
  revenue_krw: number;
  inflow_usd: number;
  outflow_usd: number;
  treasury_usd: number;
  floor_price: number;
}

export interface TreasuryData {
  treasury_usd: number;
  floor_price: number;
  rows: TreasuryRow[];
}

export interface Challenge {
  id: string;
  name: string;
  brand_id: string;
  start_date: string;
  end_date: string;
  steps_per_point: number;
  points_per_token: number;
  total_participants: number;
  total_steps_global: number;
}

export interface UserChallenge {
  total_steps: number;
  total_points: number;
  pending_points: number;
  tokens_earned: number;
  tier: string;
  rank: number;
  points_per_token: number;
  milestones: Milestone[];
}

export interface Milestone {
  name: string;
  steps_required: number;
  reward: number;
  completed: boolean;
  emoji: string;
}

export interface WalletData {
  total_tokens: number;
  total_pending_points: number;
  balances: Record<string, number>;
  referral_code: string;
}

export interface Transaction {
  id: string;
  type: string;
  description: string;
  amount: number;
  created_at: string;
}

export interface DAOProposal {
  id: string;
  title: string;
  description: string;
  yes_votes: number;
  no_votes: number;
  deadline: string;
  status: string;
  my_vote: string | null;
}

export interface OnboardingStep {
  label: string;
  pct: number;
}

export interface EventItem {
  icon: string;
  text: string;
  created_at: string;
}

export interface LeaderboardEntry {
  rank: number;
  name: string;
  steps: number;
  tier: string;
}

export function getDashboardStats(brandId?: string | null): DashboardStats {
  if (brandId === "le-mouton") {
    return { treasury: 24500, last_revenue_krw: 9800000, user_count: 1250, challenge_participants: 420, total_tokens_issued: 45000, total_steps: 32400 };
  }
  if (brandId === "cafe-blossom") {
    return { treasury: 12300, last_revenue_krw: 5600000, user_count: 850, challenge_participants: 280, total_tokens_issued: 22000, total_steps: 18500 };
  }
  if (brandId === "runpulse") {
    return { treasury: 35800, last_revenue_krw: 15200000, user_count: 2100, challenge_participants: 890, total_tokens_issued: 68000, total_steps: 125000 };
  }
  return { treasury: 72600, last_revenue_krw: 30600000, user_count: 4200, challenge_participants: 1590, total_tokens_issued: 135000, total_steps: 175900 };
}

export function getTreasuryData(brandId: string, stress: number = 10): TreasuryData {
  const months = ["2025-07", "2025-08", "2025-09", "2025-10", "2025-11", "2025-12", "2026-01"];
  const baseRevenues = [5200000, 6100000, 7300000, 8500000, 9200000, 9800000, 10500000];
  const sharePercent = brandId === "le-mouton" ? 3.0 : brandId === "cafe-blossom" ? 2.5 : 4.0;
  const fxRate = 1200;

  let cumulativeTreasury = 0;
  const totalSupply = brandId === "le-mouton" ? 1000000 : brandId === "cafe-blossom" ? 500000 : 2000000;

  const rows: TreasuryRow[] = months.map((month, i) => {
    const revenue = baseRevenues[i];
    const inflow = (revenue * sharePercent) / 100 / fxRate;
    const outflow = inflow * 0.05;
    cumulativeTreasury += inflow - outflow;
    return {
      month,
      revenue_krw: revenue,
      inflow_usd: inflow,
      outflow_usd: outflow,
      treasury_usd: cumulativeTreasury,
      floor_price: cumulativeTreasury / totalSupply,
    };
  });

  const lastRow = rows[rows.length - 1];
  const stressFactor = 1 - stress / 100;

  return {
    treasury_usd: lastRow.treasury_usd * stressFactor,
    floor_price: (lastRow.treasury_usd * stressFactor) / totalSupply,
    rows,
  };
}

export function getChallenges(brandId?: string | null): Challenge[] {
  const challenges: Challenge[] = [
    {
      id: "ch-1",
      name: "Le Mouton 걷기 챌린지",
      brand_id: "le-mouton",
      start_date: "2026-01-01",
      end_date: "2026-06-30",
      steps_per_point: 100,
      points_per_token: 100,
      total_participants: 420,
      total_steps_global: 12500000,
    },
    {
      id: "ch-2",
      name: "Cafe Blossom 음료 스탬프 챌린지",
      brand_id: "cafe-blossom",
      start_date: "2026-01-01",
      end_date: "2026-06-30",
      steps_per_point: 100,
      points_per_token: 100,
      total_participants: 280,
      total_steps_global: 8200000,
    },
    {
      id: "ch-3",
      name: "RunPulse 러닝 챌린지",
      brand_id: "runpulse",
      start_date: "2026-01-01",
      end_date: "2026-06-30",
      steps_per_point: 100,
      points_per_token: 100,
      total_participants: 890,
      total_steps_global: 35000000,
    },
  ];
  if (brandId) return challenges.filter((c) => c.brand_id === brandId);
  return challenges;
}

export function getUserChallenge(): UserChallenge {
  return {
    total_steps: 32400,
    total_points: 324,
    pending_points: 124,
    tokens_earned: 2,
    tier: "Silver",
    rank: 47,
    points_per_token: 100,
    milestones: [
      { name: "Bronze (5,000 걸음)", steps_required: 5000, reward: 50, completed: true, emoji: "🥉" },
      { name: "Silver (10,000 걸음)", steps_required: 10000, reward: 120, completed: true, emoji: "🥈" },
      { name: "Gold (25,000 걸음)", steps_required: 25000, reward: 300, completed: true, emoji: "🥇" },
      { name: "Platinum (50,000 걸음)", steps_required: 50000, reward: 700, completed: false, emoji: "💎" },
      { name: "Diamond (100,000 걸음)", steps_required: 100000, reward: 1500, completed: false, emoji: "👑" },
    ],
  };
}

export function getWalletData(): WalletData {
  return {
    total_tokens: 13.24,
    total_pending_points: 124,
    balances: {
      "Le Mouton (LMT)": 8.5,
      "Cafe Blossom (CBT)": 3.24,
      "RunPulse (RPT)": 1.5,
    },
    referral_code: "REF-JHPARK-8A2F",
  };
}

export function getTransactions(): Transaction[] {
  return [
    { id: "t1", type: "earned", description: "걷기 챌린지 - 8,500걸음", amount: 85, created_at: "2026-03-28T10:30:00Z" },
    { id: "t2", type: "convert", description: "포인트 → LMT 변환", amount: 2, created_at: "2026-03-27T15:20:00Z" },
    { id: "t3", type: "bonus", description: "Silver 마일스톤 달성", amount: 120, created_at: "2026-03-25T09:00:00Z" },
    { id: "t4", type: "earned", description: "걷기 챌린지 - 12,300걸음", amount: 123, created_at: "2026-03-24T11:45:00Z" },
    { id: "t5", type: "bonus", description: "회원가입 보너스", amount: 10, created_at: "2026-03-20T08:00:00Z" },
    { id: "t6", type: "convert", description: "포인트 → CBT 변환", amount: 1.24, created_at: "2026-03-19T14:30:00Z" },
    { id: "t7", type: "earned", description: "걷기 챌린지 - 5,600걸음", amount: 56, created_at: "2026-03-18T16:00:00Z" },
    { id: "t8", type: "bonus", description: "Bronze 마일스톤 달성", amount: 50, created_at: "2026-03-15T10:00:00Z" },
  ];
}

export function getDAOProposals(_brandId?: string | null): DAOProposal[] {
  const proposals: DAOProposal[] = [
    {
      id: "p1",
      title: "리워드 배수 2배 증가",
      description: "걷기 챌린지 리워드를 기존 대비 2배로 상향하는 제안입니다.",
      yes_votes: 1250,
      no_votes: 340,
      deadline: "2026-04-15T00:00:00Z",
      status: "active",
      my_vote: null,
    },
    {
      id: "p2",
      title: "자선 기부 10% 배분",
      description: "월 수익의 10%를 환경 보호 단체에 기부하는 제안입니다.",
      yes_votes: 890,
      no_votes: 560,
      deadline: "2026-04-20T00:00:00Z",
      status: "active",
      my_vote: null,
    },
    {
      id: "p3",
      title: "파트너 브랜드 추가 승인",
      description: "새로운 파트너 브랜드 'GreenWalk'를 플랫폼에 추가하는 제안입니다.",
      yes_votes: 2100,
      no_votes: 180,
      deadline: "2026-04-10T00:00:00Z",
      status: "active",
      my_vote: "yes",
    },
  ];
  return proposals;
}

export function getOnboardingStats(): OnboardingStep[] {
  return [
    { label: "회원가입 완료", pct: 1.0 },
    { label: "프로필 설정", pct: 0.82 },
    { label: "디바이스 연결", pct: 0.65 },
    { label: "첫 챌린지 참여", pct: 0.48 },
    { label: "첫 토큰 획득", pct: 0.31 },
  ];
}

export function getEvents(_brandId?: string | null): EventItem[] {
  return [
    { icon: "💰", text: "Le Mouton 매출 ₩2,500,000 발생", created_at: "2026-03-29T14:30:00Z" },
    { icon: "🚶", text: "[Le Mouton] jhpark님이 걷기 챌린지 8,500걸음 달성", created_at: "2026-03-29T10:20:00Z" },
    { icon: "🗳️", text: "[Le Mouton] '리워드 배수 2배 증가' 제안에 45명 투표", created_at: "2026-03-28T18:00:00Z" },
    { icon: "👤", text: "[Cafe Blossom] 신규 사용자 3명 가입", created_at: "2026-03-28T12:00:00Z" },
    { icon: "💰", text: "Cafe Blossom 매출 ₩1,200,000 발생", created_at: "2026-03-27T16:45:00Z" },
    { icon: "🏆", text: "[RunPulse] user42님 러닝 챌린지 Gold 마일스톤 달성", created_at: "2026-03-27T09:30:00Z" },
    { icon: "🔄", text: "RunPulse 트레저리 $580 유입", created_at: "2026-03-26T11:00:00Z" },
  ];
}

export function getLeaderboard(): LeaderboardEntry[] {
  return [
    { rank: 1, name: "김민수", steps: 268000, tier: "Diamond" },
    { rank: 2, name: "이서연", steps: 245000, tier: "Diamond" },
    { rank: 3, name: "박지훈", steps: 198000, tier: "Platinum" },
    { rank: 4, name: "최유진", steps: 185000, tier: "Platinum" },
    { rank: 5, name: "정다은", steps: 172000, tier: "Platinum" },
    { rank: 6, name: "강현우", steps: 165000, tier: "Platinum" },
    { rank: 7, name: "윤서아", steps: 158000, tier: "Gold" },
    { rank: 8, name: "임재현", steps: 148000, tier: "Gold" },
    { rank: 9, name: "한소영", steps: 142000, tier: "Gold" },
    { rank: 10, name: "오준서", steps: 138000, tier: "Gold" },
  ];
}

// Format helpers
export function formatKRW(n: number): string {
  return new Intl.NumberFormat("ko-KR").format(Math.round(n));
}

export function formatUSD(n: number): string {
  return "$" + new Intl.NumberFormat("en-US", { maximumFractionDigits: 2 }).format(n);
}

export function formatNumber(n: number): string {
  return new Intl.NumberFormat("en-US", { maximumFractionDigits: 1 }).format(n);
}

export function formatPercent(n: number): string {
  return (n * 100).toFixed(1) + "%";
}
