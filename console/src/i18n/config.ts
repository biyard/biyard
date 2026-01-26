import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import { auth } from "@/features/auth/i18n";
import { credentials } from "@/features/credentials/i18n";
import { dashboard } from "@/features/dashboard/i18n";
import { settings } from "@/features/settings/i18n";
import { projects } from "@/features/projects/i18n";
import { app } from "./locales/app";
import { common } from "./locales/common";
import { SignInPage } from "@/features/auth/components/SignInPage/i18n";
import { SignUpPage } from "@/features/auth/components/SignUpPage/i18n";
import { CredentialsPage } from "@/features/credentials/components/CredentialsPage/i18n";
import { DashboardPage } from "@/features/dashboard/components/DashboardPage/i18n";
import { SettingsPage } from "@/features/settings/components/SettingsPage/i18n";
import { ProjectsPage } from "@/features/projects/components/ProjectsPage/i18n";
import { projectDetail } from "@/features/projects/components/ProjectDetailPage/i18n";

export const resources: {
  en: Record<string, unknown>;
  ko: Record<string, unknown>;
} = {
  en: {},
  ko: {},
};

Object.entries({
  // Common i18n
  app,
  common,

  // Feature-level i18n
  auth,
  credentials,
  dashboard,
  settings,
  projects,

  // Component-level i18n
  SignInPage,
  SignUpPage,
  CredentialsPage,
  DashboardPage,
  SettingsPage,
  ProjectsPage,
  projectDetail,
}).forEach(([key, value]) => {
  resources.en[key] = value.en;
  resources.ko[key] = value.ko;
});

i18n.use(initReactI18next).init({
  resources: {
    en: { translation: resources.en },
    ko: { translation: resources.ko },
  },
  lng: localStorage.getItem("language") || "en",
  fallbackLng: "en",
  interpolation: {
    escapeValue: false,
  },
});

export default i18n;
