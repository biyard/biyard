import { useTranslation } from "react-i18next";

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

  return {
    // From app
    title: t("app:title"),
    tagline: t("app:tagline"),

    // From common
    loading: t("common:loading"),

    // From auth (feature-level shared)
    name: t("auth:name"),
    email: t("auth:email"),
    password: t("auth:password"),
    confirmPassword: t("auth:confirmPassword"),
    enterName: t("auth:enterName"),
    enterEmail: t("auth:enterEmail"),
    enterPassword: t("auth:enterPassword"),
    enterConfirmPassword: t("auth:enterConfirmPassword"),
    signIn: t("auth:signIn"),
    signUp: t("auth:signUp"),
    hasAccount: t("auth:hasAccount"),

    // SignUpPage specific
    signUpWithEmail: t("SignUpPage:signUpWithEmail"),
    nameRequired: t("SignUpPage:nameRequired"),
    passwordMismatch: t("SignUpPage:passwordMismatch"),
    signUpError: t("SignUpPage:signUpError"),
  };
}
