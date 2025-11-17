import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import { app } from "./locales/app";
import { common } from "./locales/common";

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
