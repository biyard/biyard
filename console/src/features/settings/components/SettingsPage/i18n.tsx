import { useTranslation } from "react-i18next";
import { useCommonI18n } from "@/i18n/locales/common";
import { useAuthI18n } from "../../../auth/i18n";

export const SettingsPage = {
  en: {
    accountSettings: "Account Settings",
    profile: "Profile",
    accountId: "Account ID",
    withdrawal: "Delete Account",
    withdrawalWarning:
      "This action cannot be undone. All your data will be permanently deleted.",
    confirmWithdrawal: "Confirm Account Deletion",
    editProfile: "Edit Profile",
    profileUpdated: "Profile updated",
  },
  ko: {
    accountSettings: "계정 설정",
    profile: "프로필",
    accountId: "계정 ID",
    withdrawal: "계정 삭제",
    withdrawalWarning:
      "이 작업은 되돌릴 수 없습니다. 모든 데이터가 영구적으로 삭제됩니다.",
    confirmWithdrawal: "계정 삭제 확인",
    editProfile: "프로필 편집",
    profileUpdated: "프로필이 업데이트되었습니다",
  },
};

export interface SettingsPageI18n {
  // From auth (cross-feature reference)
  name: string;
  email: string;

  // From common
  save: string;
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
  editProfile: string;
  profileUpdated: string;
}

export function useSettingsPageI18n(): SettingsPageI18n {
  const { t } = useTranslation();
  const common = useCommonI18n();
  const auth = useAuthI18n();

  return {
    // From auth (cross-feature reference)
    name: auth.name,
    email: auth.email,

    // From common
    save: common.save,
    cancel: common.cancel,
    confirm: common.confirm,
    loading: common.loading,

    // SettingsPage specific
    accountSettings: t("SettingsPage:accountSettings"),
    profile: t("SettingsPage:profile"),
    accountId: t("SettingsPage:accountId"),
    withdrawal: t("SettingsPage:withdrawal"),
    withdrawalWarning: t("SettingsPage:withdrawalWarning"),
    confirmWithdrawal: t("SettingsPage:confirmWithdrawal"),
    editProfile: t("SettingsPage:editProfile"),
    profileUpdated: t("SettingsPage:profileUpdated"),
  };
}
