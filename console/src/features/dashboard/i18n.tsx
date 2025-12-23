import { useTranslation } from "react-i18next";

export const dashboard = {
  en: {
    title: "Dashboard",
    themeDark: "Dark Mode",
    themeLight: "Light Mode",
    signOut: "Sign Out",
  },
  ko: {
    title: "대시보드",
    themeDark: "다크 모드",
    themeLight: "라이트 모드",
    signOut: "로그아웃",
  },
};

export interface DashboardI18n {
  title: string;
  themeDark: string;
  themeLight: string;
  signOut: string;
}

export function useDashboardI18n(): DashboardI18n {
  const { t } = useTranslation();

  return {
    title: t("dashboard.title"),
    themeDark: t("dashboard.themeDark"),
    themeLight: t("dashboard.themeLight"),
    signOut: t("dashboard.signOut"),
  };
}
