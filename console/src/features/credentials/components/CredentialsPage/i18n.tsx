import { useTranslation } from "react-i18next";
import { useCommonI18n } from "@/i18n/locales/common";

export const CredentialsPage = {
  en: {
    title: "API Credentials",
    description: "Manage your API credentials for accessing Biyard services",
    createNew: "Create New Credential",
    noCredentials: "No credentials yet",
    name: "Name",
    apiKey: "API Key",
    createdAt: "Created At",
    status: "Status",
    active: "Active",
    inactive: "Inactive",
    copy: "Copy",
    confirmRevoke: "Are you sure you want to revoke this credential?",
    generateKey: "Generate Key",
    keyGenerated: "API Key Generated",
    keyGeneratedWarning:
      "Please copy this key now. You won't be able to see it again!",
  },
  ko: {
    title: "API 인증 정보",
    description: "Biyard 서비스 접근을 위한 API 인증 정보를 관리합니다",
    createNew: "새 인증 정보 생성",
    noCredentials: "인증 정보가 없습니다",
    name: "이름",
    apiKey: "API 키",
    createdAt: "생성일",
    status: "상태",
    active: "활성",
    inactive: "비활성",
    copy: "복사",
    confirmRevoke: "이 인증 정보를 취소하시겠습니까?",
    generateKey: "키 생성",
    keyGenerated: "API 키가 생성되었습니다",
    keyGeneratedWarning: "지금 이 키를 복사하세요. 다시 볼 수 없습니다!",
  },
};

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
  const common = useCommonI18n();

  return {
    // From common
    loading: common.loading,
    cancel: common.cancel,
    close: common.close,
    actions: common.actions,

    // CredentialsPage specific
    title: t("CredentialsPage.title"),
    description: t("CredentialsPage.description"),
    createNew: t("CredentialsPage.createNew"),
    noCredentials: t("CredentialsPage.noCredentials"),
    name: t("CredentialsPage.name"),
    apiKey: t("CredentialsPage.apiKey"),
    createdAt: t("CredentialsPage.createdAt"),
    status: t("CredentialsPage.status"),
    active: t("CredentialsPage.active"),
    inactive: t("CredentialsPage.inactive"),
    copy: t("CredentialsPage.copy"),
    confirmRevoke: t("CredentialsPage.confirmRevoke"),
    generateKey: t("CredentialsPage.generateKey"),
    keyGenerated: t("CredentialsPage.keyGenerated"),
    keyGeneratedWarning: t("CredentialsPage.keyGeneratedWarning"),
  };
}
