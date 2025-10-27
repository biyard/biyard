import { useTranslation } from "react-i18next";

export interface CredentialsPageI18n {
  // From common
  loading: string;
  cancel: string;
  close: string;
  actions: string;

  // CredentialsPage specific
  title: string;
  description: string;
  createNew: string;
  noCredentials: string;
  name: string;
  apiKey: string;
  createdAt: string;
  status: string;
  active: string;
  inactive: string;
  copy: string;
  confirmRevoke: string;
  generateKey: string;
  keyGenerated: string;
  keyGeneratedWarning: string;
}

export function useCredentialsPageI18n(): CredentialsPageI18n {
  const { t } = useTranslation();

  return {
    // From common
    loading: t("common:loading"),
    cancel: t("common:cancel"),
    close: t("common:close"),
    actions: t("common:actions"),

    // CredentialsPage specific
    title: t("CredentialsPage:title"),
    description: t("CredentialsPage:description"),
    createNew: t("CredentialsPage:createNew"),
    noCredentials: t("CredentialsPage:noCredentials"),
    name: t("CredentialsPage:name"),
    apiKey: t("CredentialsPage:apiKey"),
    createdAt: t("CredentialsPage:createdAt"),
    status: t("CredentialsPage:status"),
    active: t("CredentialsPage:active"),
    inactive: t("CredentialsPage:inactive"),
    copy: t("CredentialsPage:copy"),
    confirmRevoke: t("CredentialsPage:confirmRevoke"),
    generateKey: t("CredentialsPage:generateKey"),
    keyGenerated: t("CredentialsPage:keyGenerated"),
    keyGeneratedWarning: t("CredentialsPage:keyGeneratedWarning"),
  };
}
