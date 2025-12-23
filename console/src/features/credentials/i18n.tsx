import { useTranslation } from "react-i18next";

export const credentials = {
  en: {
    title: "API Credentials",
  },
  ko: {
    title: "API 자격증명",
  },
};

export interface CredentialsI18n {
  title: string;
}

export function useCredentialsI18n(): CredentialsI18n {
  const { t } = useTranslation();

  return {
    title: t("credentials.title"),
  };
}
