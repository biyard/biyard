import { useTranslation } from "react-i18next";

export const SignInPage = {
  en: {
    signInWithEmail: "Sign in with email",
    emailRequired: "Email is required",
    signInError: "Failed to sign in. Please check your credentials.",
  },
  ko: {
    signInWithEmail: "이메일로 로그인",
    emailRequired: "이메일은 필수입니다",
    signInError: "로그인에 실패했습니다. 인증 정보를 확인해주세요.",
  },
};

export interface SignInPageI18n {
  // From app
  title: string;
  tagline: string;

  // From common
  loading: string;

  // From auth (feature-level shared)
  email: string;
  password: string;
  enterEmail: string;
  enterPassword: string;
  signIn: string;
  signUp: string;
  noAccount: string;

  // SignInPage specific
  signInWithEmail: string;
  emailRequired: string;
  signInError: string;
}

export function useSignInPageI18n(): SignInPageI18n {
  const { t } = useTranslation();

  return {
    // From app
    title: t("app:title"),
    tagline: t("app:tagline"),

    // From common
    loading: t("common:loading"),

    // From auth (feature-level shared)
    email: t("auth:email"),
    password: t("auth:password"),
    enterEmail: t("auth:enterEmail"),
    enterPassword: t("auth:enterPassword"),
    signIn: t("auth:signIn"),
    signUp: t("auth:signUp"),
    noAccount: t("auth:noAccount"),

    // SignInPage specific
    signInWithEmail: t("SignInPage:signInWithEmail"),
    emailRequired: t("SignInPage:emailRequired"),
    signInError: t("SignInPage:signInError"),
  };
}
