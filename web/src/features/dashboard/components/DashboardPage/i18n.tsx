import { useTranslation } from "react-i18next";

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
    signOut: "Sign Out",
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
    signOut: "로그아웃",
    themeDark: "다크 모드",
    themeLight: "라이트 모드",
  },
};

export interface DashboardPageI18n {
  title: string;
  tagline: string;

  welcome: string;
  myAccount: string;
  apiCredentials: string;
  accountSettings: string;
  profile: string;
  security: string;
  accountId: string;
  createdAt: string;
  signOut: string;
  themeDark: string;
  themeLight: string;

  name: string;
  email: string;

  credentialsDescription: string;
}

export function useDashboardPageI18n(): DashboardPageI18n {
  const { t } = useTranslation();

  return {
    welcome: t("DashboardPage:welcome"),
    myAccount: t("DashboardPage:myAccount"),
    apiCredentials: t("DashboardPage:apiCredentials"),
    accountSettings: t("DashboardPage:accountSettings"),
    profile: t("DashboardPage:profile"),
    security: t("DashboardPage:security"),
    accountId: t("DashboardPage:accountId"),
    createdAt: t("DashboardPage:createdAt"),
    signOut: t("DashboardPage:signOut"),
    themeDark: t("DashboardPage:themeDark"),
    themeLight: t("DashboardPage:themeLight"),

    title: t("app.title"),
    tagline: t("app.tagline"),

    name: t("SignInPage:name") || "Name",
    email: t("SignInPage:email") || "Email",

    credentialsDescription: t("credentials.description") || "Manage your API credentials",
  };
}
