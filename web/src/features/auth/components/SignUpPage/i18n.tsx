import { useTranslation } from "react-i18next";

export const SignUpPage = {
  en: {
    signUpWithEmail: "Sign up with email",
    nameRequired: "Name is required",
    passwordMismatch: "Passwords do not match",
    signUpError: "Failed to sign up. Please try again.",
    name: "Name",
    email: "Email",
    password: "Password",
    confirmPassword: "Confirm Password",
    enterName: "Enter your name",
    enterEmail: "Enter your email",
    enterPassword: "Enter your password",
    enterConfirmPassword: "Confirm your password",
    signUp: "Sign Up",
    signIn: "Sign In",
    hasAccount: "Already have an account?",
  },
  ko: {
    signUpWithEmail: "이메일로 회원가입",
    nameRequired: "이름은 필수입니다",
    passwordMismatch: "비밀번호가 일치하지 않습니다",
    signUpError: "회원가입에 실패했습니다. 다시 시도해주세요.",
    name: "이름",
    email: "이메일",
    password: "비밀번호",
    confirmPassword: "비밀번호 확인",
    enterName: "이름을 입력하세요",
    enterEmail: "이메일을 입력하세요",
    enterPassword: "비밀번호를 입력하세요",
    enterConfirmPassword: "비밀번호를 다시 입력하세요",
    signUp: "회원가입",
    signIn: "로그인",
    hasAccount: "이미 계정이 있으신가요?",
  },
};

export interface SignUpPageI18n {
  title: string;
  tagline: string;
  loading: string;

  signUpWithEmail: string;
  nameRequired: string;
  passwordMismatch: string;
  signUpError: string;
  name: string;
  email: string;
  password: string;
  confirmPassword: string;
  enterName: string;
  enterEmail: string;
  enterPassword: string;
  enterConfirmPassword: string;
  signUp: string;
  signIn: string;
  hasAccount: string;
}

export function useSignUpPageI18n(): SignUpPageI18n {
  const { t } = useTranslation();

  return {
    signUpWithEmail: t("SignUpPage:signUpWithEmail"),
    nameRequired: t("SignUpPage:nameRequired"),
    passwordMismatch: t("SignUpPage:passwordMismatch"),
    signUpError: t("SignUpPage:signUpError"),
    name: t("SignUpPage:name"),
    email: t("SignUpPage:email"),
    password: t("SignUpPage:password"),
    confirmPassword: t("SignUpPage:confirmPassword"),
    enterName: t("SignUpPage:enterName"),
    enterEmail: t("SignUpPage:enterEmail"),
    enterPassword: t("SignUpPage:enterPassword"),
    enterConfirmPassword: t("SignUpPage:enterConfirmPassword"),
    signUp: t("SignUpPage:signUp"),
    signIn: t("SignUpPage:signIn"),
    hasAccount: t("SignUpPage:hasAccount"),

    title: t("app.title"),
    tagline: t("app.tagline"),
    loading: t("common.loading"),
  };
}
