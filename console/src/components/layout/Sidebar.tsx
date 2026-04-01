import { Link, useLocation, useNavigate } from "react-router-dom";
import {
  LayoutDashboard,
  FolderKanban,
  Key,
  Settings,
  Sun,
  Moon,
  Globe,
  LogOut
} from "lucide-react";
import { useTheme } from "../../contexts/ThemeContext";
import { useAuth } from "../../contexts/AuthContext";
import { useTranslation } from "react-i18next";
import { useSignout } from "../../features/auth/hooks/use-signout";

interface NavItem {
  to: string;
  icon: typeof LayoutDashboard;
  labelKey: string;
}

export function Sidebar() {
  const location = useLocation();
  const navigate = useNavigate();
  const { theme, toggleTheme } = useTheme();
  const { setAccount } = useAuth();
  const { t, i18n } = useTranslation();
  const signoutMutation = useSignout();

  const navItems: NavItem[] = [
    { to: "/dashboard", icon: LayoutDashboard, labelKey: "dashboard.title" },
    { to: "/projects", icon: FolderKanban, labelKey: "projects.title" },
    { to: "/credentials", icon: Key, labelKey: "credentials.title" },
    { to: "/settings", icon: Settings, labelKey: "settings.title" },
  ];

  const toggleLanguage = () => {
    const newLang = i18n.language === "en" ? "ko" : "en";
    i18n.changeLanguage(newLang);
  };

  const handleSignOut = async () => {
    try {
      await signoutMutation.mutateAsync();
      setAccount(null);
      navigate("/signin");
    } catch (error) {
      console.error("Failed to sign out:", error);
    }
  };

  const isActive = (path: string) => location.pathname === path;

  return (
    <aside className="fixed top-0 left-0 h-screen w-64 bg-white dark:bg-gray-800 shadow-lg flex flex-col">
      {/* Logo Section */}
      <div className="p-6 border-b border-gray-200 dark:border-gray-700">
        <h1 className="text-2xl font-bold text-gray-900 dark:text-white">
          Biyard
        </h1>
        <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">
          Console
        </p>
      </div>

      {/* Navigation Menu */}
      <nav className="flex-1 p-4 space-y-2 overflow-y-auto">
        {navItems.map((item) => {
          const Icon = item.icon;
          const active = isActive(item.to);

          return (
            <Link
              key={item.to}
              to={item.to}
              className={`
                flex items-center px-4 py-3 rounded-lg transition-colors
                ${
                  active
                    ? "bg-blue-50 dark:bg-blue-900/20 text-blue-600 dark:text-blue-400"
                    : "text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700"
                }
              `}
            >
              <Icon className="h-5 w-5 mr-3" />
              <span className="font-medium">{t(item.labelKey)}</span>
            </Link>
          );
        })}
      </nav>

      {/* Bottom Actions */}
      <div className="p-4 border-t border-gray-200 dark:border-gray-700 space-y-2">
        {/* Language Toggle */}
        <button
          onClick={toggleLanguage}
          className="flex items-center w-full px-4 py-2 rounded-lg text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
          title={i18n.language === "en" ? "한국어" : "English"}
        >
          <Globe className="h-5 w-5 mr-3" />
          <span className="font-medium">
            {i18n.language === "en" ? "한국어" : "English"}
          </span>
        </button>

        {/* Theme Toggle */}
        <button
          onClick={toggleTheme}
          className="flex items-center w-full px-4 py-2 rounded-lg text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
        >
          {theme === "light" ? (
            <>
              <Moon className="h-5 w-5 mr-3" />
              <span className="font-medium">{t("dashboard.themeDark")}</span>
            </>
          ) : (
            <>
              <Sun className="h-5 w-5 mr-3" />
              <span className="font-medium">{t("dashboard.themeLight")}</span>
            </>
          )}
        </button>

        {/* Sign Out */}
        <button
          onClick={handleSignOut}
          className="flex items-center w-full px-4 py-2 rounded-lg text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors"
        >
          <LogOut className="h-5 w-5 mr-3" />
          <span className="font-medium">{t("dashboard.signOut")}</span>
        </button>
      </div>
    </aside>
  );
}
