import { useTranslation } from "react-i18next";

export const settings = {
  en: {
    title: "Settings",
  },
  ko: {
    title: "설정",
  },
};

export interface SettingsI18n {
  title: string;
}

export function useSettingsI18n(): SettingsI18n {
  const { t } = useTranslation();

  return {
    title: t("settings.title"),
  };
}
