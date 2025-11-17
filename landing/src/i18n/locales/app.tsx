import { useTranslation } from "react-i18next";

export const app = {
  en: {
    title: "Biyard Console",
    tagline: "Blockchain Token & Point Management Platform",
  },
  ko: {
    title: "Biyard 콘솔",
    tagline: "블록체인 토큰 & 포인트 관리 플랫폼",
  },
};

export interface AppI18n {
  title: string;
  tagline: string;
}

export function useAppI18n(): AppI18n {
  const { t } = useTranslation();

  return {
    title: t("app.title"),
    tagline: t("app.tagline"),
  };
}
