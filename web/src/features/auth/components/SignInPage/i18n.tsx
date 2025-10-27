import { useTranslation } from "react-i18next";

export const SignInPage = {
  en: {
    signInError: "Failed to sign in. Please check your credentials.",
    emailRequired: "Email is required",
    signInWithEmail: "Sign in with email",
    email: "Email",
    enterEmail: "Enter your email",
    password: "Password",
    enterPassword: "Enter your password",
    enterName: "Enter your name",
    signIn: "Sign In",
    signUp: "Sign Up",
    noAccount: "Don't have an account?",
  },
  ko: {
    emailRequired: "이메일은 필수입니다",
    signInError: "로그인에 실패했습니다. 인증 정보를 확인해주세요.",
    signInWithEmail: "이메일로 로그인",
    email: "이메일",
    enterEmail: "이메일을 입력하세요",
    enterPassword: "비밀번호를 입력하세요",
    enterName: "이름을 입력하세요",
    enterConfirmPassword: "비밀번호를 다시 입력하세요",
    signIn: "로그인",
    signUp: "회원가입",
    password: "비밀번호",
    noAccount: "계정이 없으신가요?",
  },
};

export interface SignInPageI18n {
  title: string;
  tagline: string;

  loading: string;

  signIn: string;
  emailRequired: string;
  signInError: string;
  signInWithEmail: string;
  enterEmail: string;
  enterPassword: string;
  enterName: string;
  enterConfirmPassword: string;
  signUp: string;
  email: string;
  password: string;
  noAccount: string;
}

export function useSignInPageI18n(): SignInPageI18n {
  const { t } = useTranslation();

  return {
    signIn: t("SignInPage:signIn"),
    emailRequired: t("SignInPage:emailRequired"),
    signInError: t("SignInPage:signInError"),
    signInWithEmail: t("SignInPage:signInWithEmail"),
    enterEmail: t("SignInPage:enterEmail"),
    enterPassword: t("SignInPage:enterPassword"),
    enterName: t("SignInPage:enterName"),
    enterConfirmPassword: t("SignInPage:enterConfirmPassword"),
    signUp: t("SignInPage:signUp"),
    email: t("SignInPage:email"),
    password: t("SignInPage:password"),
    noAccount: t("SignInPage:noAccount"),

    title: t("app.title"),
    tagline: t("app.tagline"),
    loading: t("common.loading"),
  };
}
