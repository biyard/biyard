import { useTranslation } from "react-i18next";

export const projectDetail = {
  en: {
    title: "Project Details",
    backToProjects: "Back to Projects",
    loading: "Loading project...",
    notFound: "Project not found",
    notFoundDescription: "The project you're looking for doesn't exist.",
    overview: "Overview",
    token: "Token Transactions",
    points: "Point History",
    tokenInfo: "Token",
    pointInfo: "Points",
    settings: "Settings",
    projectId: "Project ID",
    projectName: "Project Name",
    description: "Description",
    status: "Status",
    createdAt: "Created At",
    updatedAt: "Updated At",
    monthlyTokenSupply: "Monthly Token Supply",
    exchangeRatio: "Exchange Ratio",
    tokenValue: "Token Value",
    noTokens: "No tokens yet",
    noTokensDescription: "Create your first token for this project.",
    createToken: "Create Token",
    tokenName: "Token Name",
    tokenSymbol: "Symbol",
    tokenAddress: "Contract Address",
    totalSupply: "Total Supply",
    maxSupply: "Max Supply",
    tokenStatus: "Status",
    viewOnExplorer: "View on Explorer",
    noTransactions: "No transactions yet",
    noTransactionsDescription: "Point transactions will appear here.",
    transactionType: "Type",
    amount: "Amount",
    user: "User",
    targetUser: "Target User",
    month: "Month",
    totalAwarded: "Total Awarded",
    totalDeducted: "Total Deducted",
    recentTransactions: "Recent Transactions",
    noPointsYet: "No points yet",
    noPointsDescription: "Point transactions will appear here when created.",
    noTokenTransactions: "No token transactions yet",
    noTokenTransactionsDescription: "Token transactions will appear here.",
    treasury: "Treasury",
    totalTreasury: "Total Treasury",
    floorPrice: "Floor Price",
    treasuryDescription: "Treasury data will be populated from purchase transactions.",
    treasuryConnectHint: "Connect to real data by making purchases via the API.",
    users: "Users",
    userId: "User ID",
    pointsBalance: "Points Balance",
    tokenBalance: "Token Balance",
    lastActive: "Last Active",
    usersEmptyDescription: "Users will appear as they interact via your API.",
    audit: "Audit",
    date: "Date",
    action: "Action",
    auditDescription: "Description",
    auditEmptyDescription: "Audit trail records all token and point operations.",
  },
  ko: {
    title: "프로젝트 상세",
    backToProjects: "프로젝트 목록",
    loading: "프로젝트 로딩 중...",
    notFound: "프로젝트를 찾을 수 없습니다",
    notFoundDescription: "요청하신 프로젝트가 존재하지 않습니다.",
    overview: "개요",
    token: "토큰 트랜잭션",
    points: "포인트 히스토리",
    tokenInfo: "토큰",
    pointInfo: "포인트",
    settings: "설정",
    projectId: "프로젝트 ID",
    projectName: "프로젝트 이름",
    description: "설명",
    status: "상태",
    createdAt: "생성일",
    updatedAt: "수정일",
    monthlyTokenSupply: "월간 토큰 공급량",
    exchangeRatio: "교환 비율",
    tokenValue: "토큰 가치",
    noTokens: "토큰이 없습니다",
    noTokensDescription: "이 프로젝트의 첫 번째 토큰을 생성하세요.",
    createToken: "토큰 생성",
    tokenName: "토큰 이름",
    tokenSymbol: "심볼",
    tokenAddress: "컨트랙트 주소",
    totalSupply: "총 공급량",
    maxSupply: "최대 공급량",
    tokenStatus: "상태",
    viewOnExplorer: "익스플로러에서 보기",
    noTransactions: "트랜잭션이 없습니다",
    noTransactionsDescription: "포인트 트랜잭션이 여기에 표시됩니다.",
    transactionType: "유형",
    amount: "수량",
    user: "사용자",
    targetUser: "대상 사용자",
    month: "월",
    totalAwarded: "총 지급량",
    totalDeducted: "총 차감량",
    recentTransactions: "최근 트랜잭션",
    noPointsYet: "포인트가 없습니다",
    noPointsDescription: "포인트 트랜잭션이 생성되면 여기에 표시됩니다.",
    noTokenTransactions: "토큰 트랜잭션이 없습니다",
    noTokenTransactionsDescription: "토큰 트랜잭션이 여기에 표시됩니다.",
    treasury: "재무",
    totalTreasury: "총 재무",
    floorPrice: "최저가",
    treasuryDescription: "재무 데이터는 구매 트랜잭션에서 채워집니다.",
    treasuryConnectHint: "API를 통해 구매하여 실제 데이터에 연결하세요.",
    users: "사용자 목록",
    userId: "사용자 ID",
    pointsBalance: "포인트 잔액",
    tokenBalance: "토큰 잔액",
    lastActive: "마지막 활동",
    usersEmptyDescription: "사용자가 API를 통해 상호작용하면 여기에 표시됩니다.",
    audit: "감사",
    date: "날짜",
    action: "작업",
    auditDescription: "설명",
    auditEmptyDescription: "감사 추적은 모든 토큰 및 포인트 작업을 기록합니다.",
  },
};

export interface ProjectDetailI18n {
  title: string;
  backToProjects: string;
  loading: string;
  notFound: string;
  notFoundDescription: string;
  overview: string;
  token: string;
  points: string;
  tokenInfo: string;
  pointInfo: string;
  settings: string;
  projectId: string;
  projectName: string;
  description: string;
  status: string;
  createdAt: string;
  updatedAt: string;
  monthlyTokenSupply: string;
  exchangeRatio: string;
  tokenValue: string;
  noTokens: string;
  noTokensDescription: string;
  createToken: string;
  tokenName: string;
  tokenSymbol: string;
  tokenAddress: string;
  totalSupply: string;
  maxSupply: string;
  tokenStatus: string;
  viewOnExplorer: string;
  noTransactions: string;
  noTransactionsDescription: string;
  transactionType: string;
  amount: string;
  user: string;
  targetUser: string;
  month: string;
  totalAwarded: string;
  totalDeducted: string;
  recentTransactions: string;
  noPointsYet: string;
  noPointsDescription: string;
  noTokenTransactions: string;
  noTokenTransactionsDescription: string;
  treasury: string;
  totalTreasury: string;
  floorPrice: string;
  treasuryDescription: string;
  treasuryConnectHint: string;
  users: string;
  userId: string;
  pointsBalance: string;
  tokenBalance: string;
  lastActive: string;
  usersEmptyDescription: string;
  audit: string;
  date: string;
  action: string;
  auditDescription: string;
  auditEmptyDescription: string;
}

export function useProjectDetailI18n(): ProjectDetailI18n {
  const { t } = useTranslation();

  return {
    title: t("projectDetail.title"),
    backToProjects: t("projectDetail.backToProjects"),
    loading: t("projectDetail.loading"),
    notFound: t("projectDetail.notFound"),
    notFoundDescription: t("projectDetail.notFoundDescription"),
    overview: t("projectDetail.overview"),
    token: t("projectDetail.token"),
    points: t("projectDetail.points"),
    tokenInfo: t("projectDetail.tokenInfo"),
    pointInfo: t("projectDetail.pointInfo"),
    settings: t("projectDetail.settings"),
    projectId: t("projectDetail.projectId"),
    projectName: t("projectDetail.projectName"),
    description: t("projectDetail.description"),
    status: t("projectDetail.status"),
    createdAt: t("projectDetail.createdAt"),
    updatedAt: t("projectDetail.updatedAt"),
    monthlyTokenSupply: t("projectDetail.monthlyTokenSupply"),
    exchangeRatio: t("projectDetail.exchangeRatio"),
    tokenValue: t("projectDetail.tokenValue"),
    noTokens: t("projectDetail.noTokens"),
    noTokensDescription: t("projectDetail.noTokensDescription"),
    createToken: t("projectDetail.createToken"),
    tokenName: t("projectDetail.tokenName"),
    tokenSymbol: t("projectDetail.tokenSymbol"),
    tokenAddress: t("projectDetail.tokenAddress"),
    totalSupply: t("projectDetail.totalSupply"),
    maxSupply: t("projectDetail.maxSupply"),
    tokenStatus: t("projectDetail.tokenStatus"),
    viewOnExplorer: t("projectDetail.viewOnExplorer"),
    noTransactions: t("projectDetail.noTransactions"),
    noTransactionsDescription: t("projectDetail.noTransactionsDescription"),
    transactionType: t("projectDetail.transactionType"),
    amount: t("projectDetail.amount"),
    user: t("projectDetail.user"),
    targetUser: t("projectDetail.targetUser"),
    month: t("projectDetail.month"),
    totalAwarded: t("projectDetail.totalAwarded"),
    totalDeducted: t("projectDetail.totalDeducted"),
    recentTransactions: t("projectDetail.recentTransactions"),
    noPointsYet: t("projectDetail.noPointsYet"),
    noPointsDescription: t("projectDetail.noPointsDescription"),
    noTokenTransactions: t("projectDetail.noTokenTransactions"),
    noTokenTransactionsDescription: t("projectDetail.noTokenTransactionsDescription"),
    treasury: t("projectDetail.treasury"),
    totalTreasury: t("projectDetail.totalTreasury"),
    floorPrice: t("projectDetail.floorPrice"),
    treasuryDescription: t("projectDetail.treasuryDescription"),
    treasuryConnectHint: t("projectDetail.treasuryConnectHint"),
    users: t("projectDetail.users"),
    userId: t("projectDetail.userId"),
    pointsBalance: t("projectDetail.pointsBalance"),
    tokenBalance: t("projectDetail.tokenBalance"),
    lastActive: t("projectDetail.lastActive"),
    usersEmptyDescription: t("projectDetail.usersEmptyDescription"),
    audit: t("projectDetail.audit"),
    date: t("projectDetail.date"),
    action: t("projectDetail.action"),
    auditDescription: t("projectDetail.auditDescription"),
    auditEmptyDescription: t("projectDetail.auditEmptyDescription"),
  };
}
