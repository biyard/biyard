import { useTranslation } from "react-i18next";
import { useAppI18n } from "@/i18n/locales/app";
import { useAuthI18n } from "../../../auth/i18n";

export const DashboardPage = {
  en: {
    welcome: "Welcome to Biyard Console",
    myAccount: "My Account",
    apiCredentials: "API Credentials",
    accountSettings: "Account Settings",
    profile: "Profile",
    security: "Security",
    accountId: "Account ID",
    createdAt: "Created At",
    themeDark: "Dark Mode",
    themeLight: "Light Mode",
  },
  ko: {
    welcome: "Biyard 콘솔에 오신 것을 환영합니다",
    myAccount: "내 계정",
    apiCredentials: "API 인증 정보",
    accountSettings: "계정 설정",
    profile: "프로필",
    security: "보안",
    accountId: "계정 ID",
    createdAt: "생성일",
    themeDark: "다크 모드",
    themeLight: "라이트 모드",
  },
};

export interface DashboardPageI18n {
  // From app
  title: string;
  tagline: string;

  // From auth (cross-feature reference)
  name: string;
  email: string;
  signOut: string;

  // DashboardPage specific
  welcome: string;
  myAccount: string;
  apiCredentials: string;
  accountSettings: string;
  profile: string;
  security: string;
  accountId: string;
  createdAt: string;
  themeDark: string;
  themeLight: string;
  credentialsDescription: string;
}

export function useDashboardPageI18n(): DashboardPageI18n {
  const { t } = useTranslation();
  const app = useAppI18n();
  const auth = useAuthI18n();

  return {
    // From app
    title: app.title,
    tagline: app.tagline,

    // From auth (cross-feature reference)
    name: auth.name,
    email: auth.email,
    signOut: auth.signOut,

    // DashboardPage specific
    welcome: t("DashboardPage.welcome"),
    myAccount: t("DashboardPage.myAccount"),
    apiCredentials: t("DashboardPage.apiCredentials"),
    accountSettings: t("DashboardPage.accountSettings"),
    profile: t("DashboardPage.profile"),
    security: t("DashboardPage.security"),
    accountId: t("DashboardPage.accountId"),
    createdAt: t("DashboardPage.createdAt"),
    themeDark: t("DashboardPage.themeDark"),
    themeLight: t("DashboardPage.themeLight"),
    credentialsDescription: t("CredentialsPage.description"),
  };
}
