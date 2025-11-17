import { useTranslation } from "react-i18next";

export const auth = {
  en: {
    // Common Auth fields used by both SignIn and SignUp
    name: "Name",
    email: "Email",
    password: "Password",
    confirmPassword: "Confirm Password",
    enterName: "Enter your name",
    enterEmail: "Enter your email",
    enterPassword: "Enter your password",
    enterConfirmPassword: "Confirm your password",
    signIn: "Sign In",
    signUp: "Sign Up",
    signOut: "Sign Out",
    noAccount: "Don't have an account?",
    hasAccount: "Already have an account?",
  },
  ko: {
    // Common Auth fields used by both SignIn and SignUp
    name: "이름",
    email: "이메일",
    password: "비밀번호",
    confirmPassword: "비밀번호 확인",
    enterName: "이름을 입력하세요",
    enterEmail: "이메일을 입력하세요",
    enterPassword: "비밀번호를 입력하세요",
    enterConfirmPassword: "비밀번호를 다시 입력하세요",
    signIn: "로그인",
    signUp: "회원가입",
    signOut: "로그아웃",
    noAccount: "계정이 없으신가요?",
    hasAccount: "이미 계정이 있으신가요?",
  },
};

export interface AuthI18n {
  // Common Auth fields used by both SignIn and SignUp
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
  signOut: string;
  noAccount: string;
  hasAccount: string;
}

export function useAuthI18n(): AuthI18n {
  const { t } = useTranslation();

  return {
    // Common Auth fields used by both SignIn and SignUp
    name: t("auth.name"),
    email: t("auth.email"),
    password: t("auth.password"),
    confirmPassword: t("auth.confirmPassword"),
    enterName: t("auth.enterName"),
    enterEmail: t("auth.enterEmail"),
    enterPassword: t("auth.enterPassword"),
    enterConfirmPassword: t("auth.enterConfirmPassword"),
    signIn: t("auth.signIn"),
    signUp: t("auth.signUp"),
    signOut: t("auth.signOut"),
    noAccount: t("auth.noAccount"),
    hasAccount: t("auth.hasAccount"),
  };
}
