import { useTranslation } from "react-i18next";

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

  return {
    // From app
    title: t("app:title"),
    tagline: t("app:tagline"),

    // From auth (cross-feature reference)
    name: t("auth:name"),
    email: t("auth:email"),
    signOut: t("auth:signOut"),

    // DashboardPage specific
    welcome: t("DashboardPage:welcome"),
    myAccount: t("DashboardPage:myAccount"),
    apiCredentials: t("DashboardPage:apiCredentials"),
    accountSettings: t("DashboardPage:accountSettings"),
    profile: t("DashboardPage:profile"),
    security: t("DashboardPage:security"),
    accountId: t("DashboardPage:accountId"),
    createdAt: t("DashboardPage:createdAt"),
    themeDark: t("DashboardPage:themeDark"),
    themeLight: t("DashboardPage:themeLight"),
    credentialsDescription: t("CredentialsPage:description"),
  };
}
