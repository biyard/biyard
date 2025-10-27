import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";
import { useAuth } from "@/contexts/AuthContext";
import { useTheme } from "@/contexts/ThemeContext";
import { useDashboardPageI18n } from "../../hooks/use-dashboard-page-i18n";

export class Controller {
  constructor(
    public t: ReturnType<typeof useDashboardPageI18n>,
    public i18n: ReturnType<typeof useTranslation>["i18n"],
    public auth: ReturnType<typeof useAuth>,
    public theme: ReturnType<typeof useTheme>,
    public navigate: ReturnType<typeof useNavigate>,
  ) {}

  get account() {
    return this.auth.account;
  }

  handleSignOut = () => {
    this.auth.setAccount(null);
    this.navigate("/signin");
  };

  toggleLanguage = () => {
    const newLang = this.i18n.language === "en" ? "ko" : "en";
    this.i18n.changeLanguage(newLang);
    localStorage.setItem("language", newLang);
  };
}

export function useController() {
  const t = useDashboardPageI18n();
  const { i18n } = useTranslation();
  const auth = useAuth();
  const theme = useTheme();
  const navigate = useNavigate();

  return new Controller(t, i18n, auth, theme, navigate);
}
