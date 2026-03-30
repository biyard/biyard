import { useTranslation } from "react-i18next";
import { useAppI18n } from "@/i18n/locales/app";
import { useCommonI18n } from "@/i18n/locales/common";
import { useAuthI18n } from "../../i18n";

export const SignUpPage = {
  en: {
    signUpWithEmail: "Sign up with email",
    nameRequired: "Name is required",
    passwordMismatch: "Passwords do not match",
    signUpError: "Failed to sign up. Please try again.",
  },
  ko: {
    signUpWithEmail: "이메일로 회원가입",
    nameRequired: "이름은 필수입니다",
    passwordMismatch: "비밀번호가 일치하지 않습니다",
    signUpError: "회원가입에 실패했습니다. 다시 시도해주세요.",
  },
};

export interface SignUpPageI18n {
  // From app
  title: string;
  tagline: string;

  // From common
  loading: string;

  // From auth (feature-level shared)
  name: string;
  email: string;
  password: string;
  confirmPassword: string;
  enterName: string;
  enterEmail: string;
  enterPassword: string;
  enterConfirmPassword: string;
  signIn: string;
  signUp: string;
  hasAccount: string;

  // SignUpPage specific
  signUpWithEmail: string;
  nameRequired: string;
  passwordMismatch: string;
  signUpError: string;
}

export function useSignUpPageI18n(): SignUpPageI18n {
  const { t } = useTranslation();
  const app = useAppI18n();
  const common = useCommonI18n();
  const auth = useAuthI18n();

  return {
    // From app
    title: app.title,
    tagline: app.tagline,

    // From common
    loading: common.loading,

    // From auth (feature-level shared)
    name: auth.name,
    email: auth.email,
    password: auth.password,
    confirmPassword: auth.confirmPassword,
    enterName: auth.enterName,
    enterEmail: auth.enterEmail,
    enterPassword: auth.enterPassword,
    enterConfirmPassword: auth.enterConfirmPassword,
    signIn: auth.signIn,
    signUp: auth.signUp,
    hasAccount: auth.hasAccount,

    // SignUpPage specific
    signUpWithEmail: t("SignUpPage.signUpWithEmail"),
    nameRequired: t("SignUpPage.nameRequired"),
    passwordMismatch: t("SignUpPage.passwordMismatch"),
    signUpError: t("SignUpPage.signUpError"),
  };
}
