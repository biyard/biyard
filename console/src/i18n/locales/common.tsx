import { useTranslation } from "react-i18next";

export const common = {
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

export interface CommonI18n {
  save: string;
  cancel: string;
  delete: string;
  confirm: string;
  close: string;
  loading: string;
  error: string;
  success: string;
  warning: string;
  info: string;
  actions: string;
}

export function useCommonI18n(): CommonI18n {
  const { t } = useTranslation();

  return {
    save: t("common.save"),
    cancel: t("common.cancel"),
    delete: t("common.delete"),
    confirm: t("common.confirm"),
    close: t("common.close"),
    loading: t("common.loading"),
    error: t("common.error"),
    success: t("common.success"),
    warning: t("common.warning"),
    info: t("common.info"),
    actions: t("common.actions"),
  };
}
