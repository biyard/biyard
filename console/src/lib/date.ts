import i18n from "@/i18n/config";

/**
 * Converts milliseconds timestamp from backend to Date object
 */
export function fromMillis(millis: number): Date {
  return new Date(millis);
}

/**
 * Formats a date with time according to the current language setting
 * @param date - Date object or milliseconds timestamp
 * @returns Formatted date string (e.g., "2024/01/15 14:30" for ko, "01/15/2024 14:30" for en)
 */
export function formatDateTime(date: Date | number): string {
  const d = typeof date === "number" ? fromMillis(date) : date;
  const year = d.getFullYear();
  const month = String(d.getMonth() + 1).padStart(2, "0");
  const day = String(d.getDate()).padStart(2, "0");
  const hour = String(d.getHours()).padStart(2, "0");
  const minute = String(d.getMinutes()).padStart(2, "0");

  if (i18n.language === "ko") {
    return `${year}/${month}/${day} ${hour}:${minute}`;
  }
  return `${month}/${day}/${year} ${hour}:${minute}`;
}

/**
 * Gets the current month in YYYY-MM format
 * @returns Current month string (e.g., "2024-01")
 */
export function getCurrentMonth(): string {
  const now = new Date();
  const year = now.getFullYear();
  const month = String(now.getMonth() + 1).padStart(2, "0");
  return `${year}-${month}`;
}

/**
 * Formats a relative time (e.g., "2 hours ago", "3 days ago")
 * @param date - Date object or milliseconds timestamp
 * @returns Relative time string
 */
export function formatRelativeTime(date: Date | number): string {
  const d = typeof date === "number" ? fromMillis(date) : date;
  const locale = i18n.language === "ko" ? "ko-KR" : "en-US";
  const now = new Date();
  const diffMs = now.getTime() - d.getTime();
  const diffSecs = Math.floor(diffMs / 1000);
  const diffMins = Math.floor(diffSecs / 60);
  const diffHours = Math.floor(diffMins / 60);
  const diffDays = Math.floor(diffHours / 24);

  const rtf = new Intl.RelativeTimeFormat(locale, { numeric: "auto" });

  if (diffDays > 0) {
    return rtf.format(-diffDays, "day");
  } else if (diffHours > 0) {
    return rtf.format(-diffHours, "hour");
  } else if (diffMins > 0) {
    return rtf.format(-diffMins, "minute");
  } else {
    return rtf.format(-diffSecs, "second");
  }
}
