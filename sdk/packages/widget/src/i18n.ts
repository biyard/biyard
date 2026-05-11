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
  },
  ko: {
    triggerLabel: "받기",
    defaultTitle: "토큰 받기",
    defaultCta: "받기",
    claiming: "처리 중…",
    claimedTitle: "온체인에 제출되었습니다",
    reviewMonth: "기준 월",
    reviewNetwork: "네트워크",
    reviewContract: "컨트랙트",
    close: "닫기",
    attributionDefault: "Biyard 제공 ↗",
    attributionMinimal: "via Biyard",
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
