import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import en from "./locales/en.json";
import ko from "./locales/ko.json";
import { SignInPage } from "@/features/auth/components/SignInPage/i18n";
import { SignUpPage } from "@/features/auth/components/SignUpPage/i18n";
import { DashboardPage } from "@/features/dashboard/components/DashboardPage/i18n";
import { SettingsPage } from "@/features/settings/components/SettingsPage/i18n";
import { CredentialsPage } from "@/features/credentials/components/CredentialsPage/i18n";

export const resources: {
  en: Record<string, unknown>;
  ko: Record<string, unknown>;
} = {
  en: {},
  ko: {},
};

const app = {
  en: {
    title: "Biyard Console",
    tagline: "Blockchain Token & Point Management Platform",
  },
  ko: {
    title: "Biyard 콘솔",
    tagline: "블록체인 토큰 & 포인트 관리 플랫폼",
  },
};

const common = {
  en: {
    save: "Save",
    cancel: "Cancel",
    delete: "Delete",
    confirm: "Confirm",
    close: "Close",
    loading: "Loading...",
    error: "Error",
    success: "Success",
    warning: "Warning",
    info: "Info",
    actions: "Actions",
  },
  ko: {
    save: "저장",
    cancel: "취소",
    delete: "삭제",
    confirm: "확인",
    close: "닫기",
    loading: "로딩 중...",
    error: "오류",
    success: "성공",
    warning: "경고",
    info: "정보",
    actions: "작업",
  },
};

Object.entries({
  app,
  common,
  SignInPage,
  SignUpPage,
  DashboardPage,
  SettingsPage,
  CredentialsPage,
}).forEach(([key, value]) => {
  resources.en[key] = value.en;
  resources.ko[key] = value.ko;
});

i18n.use(initReactI18next).init({
  resources: {
    en: { translation: en },
    ko: { translation: ko },
  },
  lng: localStorage.getItem("language") || "en",
  fallbackLng: "en",
  interpolation: {
    escapeValue: false,
  },
});

export default i18n;
