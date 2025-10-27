import { useAppI18n } from "@/i18n/locales/app";
import { useCommonI18n } from "@/i18n/locales/common";
import { useTranslation } from "react-i18next";
import { useAuthI18n } from "../../i18n";

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
  const app = useAppI18n();
  const common = useCommonI18n();
  const auth = useAuthI18n();

  return {
    // From app
    ...app,

    // From common
    ...common,

    // From auth (feature-level shared)
    ...auth,

    // SignInPage specific
    signInWithEmail: t("SignInPage:signInWithEmail"),
    emailRequired: t("SignInPage:emailRequired"),
    signInError: t("SignInPage:signInError"),
  };
}
