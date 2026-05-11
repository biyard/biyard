/**
 * Minimal i18n for the widget. We hand-mirror two locales (en / ko) here
 * rather than pulling in a runtime i18n library — keeps the CDN bundle small
 * and partner sites don't need a library.
 *
 * Resolution order:
 *   1. explicit `lang` attribute on the widget
 *   2. host page's `<html lang>` (if it starts with `ko`)
 *   3. fallback to English
 */

export type Locale = "en" | "ko";

export interface WidgetStrings {
  triggerLabel: string;
  defaultTitle: string;
  defaultCta: string;
  claiming: string;
  claimedTitle: string;
  reviewMonth: string;
  reviewNetwork: string;
  reviewContract: string;
  close: string;
  attributionDefault: string;
  attributionMinimal: string;
  // Shared
  loading: string;
  empty: string;
  refresh: string;
  loadMore: string;
  // Balance widget
  balanceTitle: string;
  pointsLabel: string;
  tokensLabel: string;
  connectWalletHint: string;
  // Transactions widget
  transactionsTitle: string;
  txTypeAward: string;
  txTypeDeduct: string;
  txTypeTransfer: string;
  txTypeExchange: string;
  // Monthly summary widget
  summaryTitle: string;
  summaryEarned: string;
  summarySpent: string;
  summaryBalance: string;
  summaryExchanged: string;
}

const STRINGS: Record<Locale, WidgetStrings> = {
  en: {
    triggerLabel: "Claim",
    defaultTitle: "Claim tokens",
    defaultCta: "Claim",
    claiming: "Claiming…",
    claimedTitle: "Claim submitted on-chain",
    reviewMonth: "Month",
    reviewNetwork: "Network",
    reviewContract: "Contract",
    close: "Close",
    attributionDefault: "Secured by Biyard ↗",
    attributionMinimal: "via Biyard",
    loading: "Loading…",
    empty: "Nothing to show yet.",
    refresh: "Refresh",
    loadMore: "Load more",
    balanceTitle: "Balance",
    pointsLabel: "Points",
    tokensLabel: "Tokens",
    connectWalletHint: "Connect your wallet to see on-chain balance.",
    transactionsTitle: "Activity",
    txTypeAward: "Earned",
    txTypeDeduct: "Spent",
    txTypeTransfer: "Transfer",
    txTypeExchange: "Claimed",
    summaryTitle: "Monthly summary",
    summaryEarned: "Earned",
    summarySpent: "Spent",
    summaryBalance: "Balance",
    summaryExchanged: "Claimed",
  },
  ko: {
    triggerLabel: "클레임",
    defaultTitle: "토큰 클레임",
    defaultCta: "클레임하기",
    claiming: "처리 중…",
    claimedTitle: "온체인에 제출되었습니다",
    reviewMonth: "기준 월",
    reviewNetwork: "네트워크",
    reviewContract: "컨트랙트",
    close: "닫기",
    attributionDefault: "Biyard 제공 ↗",
    attributionMinimal: "via Biyard",
    loading: "불러오는 중…",
    empty: "내역이 없습니다.",
    refresh: "새로고침",
    loadMore: "더 보기",
    balanceTitle: "잔액",
    pointsLabel: "포인트",
    tokensLabel: "토큰",
    connectWalletHint: "온체인 잔액을 보려면 지갑을 연결하세요.",
    transactionsTitle: "내역",
    txTypeAward: "적립",
    txTypeDeduct: "차감",
    txTypeTransfer: "이체",
    txTypeExchange: "클레임",
    summaryTitle: "월별 요약",
    summaryEarned: "적립",
    summarySpent: "사용",
    summaryBalance: "잔액",
    summaryExchanged: "클레임 완료",
  },
};

export function resolveLocale(explicit: string | null): Locale {
  if (explicit) {
    const norm = explicit.toLowerCase();
    if (norm === "ko" || norm.startsWith("ko-")) return "ko";
    if (norm === "en" || norm.startsWith("en-")) return "en";
  }
  if (typeof document !== "undefined") {
    const docLang = document.documentElement.lang?.toLowerCase() ?? "";
    if (docLang.startsWith("ko")) return "ko";
  }
  if (typeof navigator !== "undefined") {
    const nav = navigator.language?.toLowerCase() ?? "";
    if (nav.startsWith("ko")) return "ko";
  }
  return "en";
}

export function strings(locale: Locale): WidgetStrings {
  return STRINGS[locale];
}
