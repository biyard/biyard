import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import { app } from "./locales/app";
import { common } from "./locales/common";
import { Intro } from "@/features/index/components/vision-mission-section/i18n";
import { WhatWeDoSection } from "@/features/index/components/what-we-do-section/i18n";

const namespaces = {
  app,
  common,
  Intro,
  WhatWeDoSection,
};

const resources = {
  en: {} as Record<string, any>,
  ko: {} as Record<string, any>,
};

Object.entries(namespaces).forEach(([key, value]) => {
  resources.en[key] = value.en;
  resources.ko[key] = value.ko;
});

i18n.use(initReactI18next).init({
  resources,
  lng: localStorage.getItem("language") || "en",
  fallbackLng: "en",
  interpolation: {
    escapeValue: false,
  },
  defaultNS: "app",
});

export default i18n;
