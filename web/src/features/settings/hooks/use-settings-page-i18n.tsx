import { useTranslation } from "react-i18next";

export interface SettingsPageI18n {
  // From auth (cross-feature reference)
  name: string;
  email: string;

  // From common
  cancel: string;
  confirm: string;
  loading: string;

  // SettingsPage specific
  accountSettings: string;
  profile: string;
  accountId: string;
  withdrawal: string;
  withdrawalWarning: string;
  confirmWithdrawal: string;
}

export function useSettingsPageI18n(): SettingsPageI18n {
  const { t } = useTranslation();

  return {
    // From auth (cross-feature reference)
    name: t("auth:name"),
    email: t("auth:email"),

    // From common
    cancel: t("common:cancel"),
    confirm: t("common:confirm"),
    loading: t("common:loading"),

    // SettingsPage specific
    accountSettings: t("SettingsPage:accountSettings"),
    profile: t("SettingsPage:profile"),
    accountId: t("SettingsPage:accountId"),
    withdrawal: t("SettingsPage:withdrawal"),
    withdrawalWarning: t("SettingsPage:withdrawalWarning"),
    confirmWithdrawal: t("SettingsPage:confirmWithdrawal"),
  };
}
