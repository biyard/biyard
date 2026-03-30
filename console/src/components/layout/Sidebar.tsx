import { Link, useLocation, useNavigate } from "react-router-dom";
import {
  LayoutDashboard,
  Hammer,
  Component,
  BookOpen,
  Key,
  Webhook,
  UsersRound,
  Activity,
  Settings,
  Sun,
  Moon,
  Globe,
  LogOut,
  FolderKanban,
  Crown,
  Footprints,
  Wallet,
  ArrowLeftRight,
  Vote,
  Building2,
  User,
} from "lucide-react";
import { useTheme } from "../../contexts/ThemeContext";
import { useAuth } from "../../contexts/AuthContext";
import { useMode } from "../../contexts/ModeContext";
import { useTranslation } from "react-i18next";
import { useSignout } from "../../features/auth/hooks/use-signout";

interface NavSection {
  title: string;
  items: { to: string; icon: typeof LayoutDashboard; label: string }[];
}

export function Sidebar() {
  const location = useLocation();
  const navigate = useNavigate();
  const { theme, toggleTheme } = useTheme();
  const { setAccount } = useAuth();
  const { mode, setMode } = useMode();
  const { i18n } = useTranslation();
  const signoutMutation = useSignout();

  const enterpriseSections: NavSection[] = [
    {
      title: "",
      items: [
        { to: "/dashboard", icon: LayoutDashboard, label: "Dashboard" },
        { to: "/projects", icon: FolderKanban, label: "Projects" },
      ],
    },
    {
      title: "Build",
      items: [
        { to: "/challenge-builder", icon: Hammer, label: "Challenge Templates" },
        { to: "/widgets", icon: Component, label: "Widgets" },
      ],
    },
    {
      title: "Integrate",
      items: [
        { to: "/developer", icon: BookOpen, label: "API Docs" },
        { to: "/credentials", icon: Key, label: "API Keys" },
        { to: "/webhooks", icon: Webhook, label: "Webhooks" },
      ],
    },
    {
      title: "Account",
      items: [
        { to: "/team", icon: UsersRound, label: "Team" },
        { to: "/usage", icon: Activity, label: "Usage & Billing" },
        { to: "/settings", icon: Settings, label: "Settings" },
      ],
    },
    {
      title: "",
      items: [
        { to: "/super-admin", icon: Crown, label: "Super Admin" },
      ],
    },
  ];

  const userSections: NavSection[] = [
    {
      title: "",
      items: [
        { to: "/user/dashboard", icon: LayoutDashboard, label: "My Dashboard" },
      ],
    },
    {
      title: "Activity",
      items: [
        { to: "/user/challenge", icon: Footprints, label: "Challenge" },
        { to: "/user/wallet", icon: Wallet, label: "Wallet" },
        { to: "/user/exchange", icon: ArrowLeftRight, label: "Exchange" },
      ],
    },
    {
      title: "Community",
      items: [
        { to: "/user/dao", icon: Vote, label: "DAO" },
      ],
    },
    {
      title: "",
      items: [
        { to: "/settings", icon: Settings, label: "Settings" },
      ],
    },
  ];

  const sections = mode === "admin" ? enterpriseSections : userSections;

  const toggleLanguage = () => {
    i18n.changeLanguage(i18n.language === "en" ? "ko" : "en");
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

  const handleModeSwitch = (newMode: "admin" | "user") => {
    setMode(newMode);
    navigate(newMode === "admin" ? "/dashboard" : "/user/dashboard");
  };

  const isActive = (path: string) => {
    if (path === "/dashboard" || path === "/user/dashboard") return location.pathname === path;
    return location.pathname.startsWith(path);
  };

  return (
    <aside className="fixed top-0 left-0 h-screen w-60 bg-white dark:bg-gray-800 shadow-lg flex flex-col">
      {/* Logo */}
      <div className="p-5 border-b border-gray-200 dark:border-gray-700">
        <h1 className="text-xl font-bold text-gray-900 dark:text-white">
          Biyard
        </h1>
        <p className="text-xs text-gray-500 dark:text-gray-400 mt-0.5">
          Launchpad Platform
        </p>
      </div>

      {/* Mode Switcher */}
      <div className="px-4 pt-3 pb-1">
        <div className="flex rounded-lg bg-gray-100 dark:bg-gray-700 p-1">
          <button
            onClick={() => handleModeSwitch("admin")}
            className={`flex-1 flex items-center justify-center gap-1.5 px-3 py-1.5 rounded-md text-xs font-medium transition-colors ${
              mode === "admin"
                ? "bg-white dark:bg-gray-600 text-blue-600 dark:text-blue-400 shadow-sm"
                : "text-gray-500 dark:text-gray-400"
            }`}
          >
            <Building2 className="h-3.5 w-3.5" />
            Enterprise
          </button>
          <button
            onClick={() => handleModeSwitch("user")}
            className={`flex-1 flex items-center justify-center gap-1.5 px-3 py-1.5 rounded-md text-xs font-medium transition-colors ${
              mode === "user"
                ? "bg-white dark:bg-gray-600 text-blue-600 dark:text-blue-400 shadow-sm"
                : "text-gray-500 dark:text-gray-400"
            }`}
          >
            <User className="h-3.5 w-3.5" />
            User
          </button>
        </div>
      </div>

      {/* Navigation */}
      <nav className="flex-1 px-3 py-3 overflow-y-auto space-y-4">
        {sections.map((section, si) => (
          <div key={si}>
            {section.title && (
              <p className="px-3 mb-1.5 text-[10px] font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500">
                {section.title}
              </p>
            )}
            <div className="space-y-0.5">
              {section.items.map((item) => {
                const Icon = item.icon;
                const active = isActive(item.to);
                return (
                  <Link
                    key={item.to}
                    to={item.to}
                    className={`flex items-center px-3 py-2 rounded-lg transition-colors text-sm ${
                      active
                        ? "bg-blue-50 dark:bg-blue-900/20 text-blue-600 dark:text-blue-400 font-semibold"
                        : "text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700"
                    }`}
                  >
                    <Icon className="h-4 w-4 mr-2.5 flex-shrink-0" />
                    <span className="truncate">{item.label}</span>
                  </Link>
                );
              })}
            </div>
          </div>
        ))}
      </nav>

      {/* Bottom */}
      <div className="p-3 border-t border-gray-200 dark:border-gray-700 space-y-0.5">
        <button
          onClick={toggleLanguage}
          className="flex items-center w-full px-3 py-1.5 rounded-lg text-xs text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
        >
          <Globe className="h-4 w-4 mr-2.5" />
          {i18n.language === "en" ? "한국어" : "English"}
        </button>
        <button
          onClick={toggleTheme}
          className="flex items-center w-full px-3 py-1.5 rounded-lg text-xs text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
        >
          {theme === "light" ? (
            <><Moon className="h-4 w-4 mr-2.5" />Dark Mode</>
          ) : (
            <><Sun className="h-4 w-4 mr-2.5" />Light Mode</>
          )}
        </button>
        <button
          onClick={handleSignOut}
          className="flex items-center w-full px-3 py-1.5 rounded-lg text-xs text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors"
        >
          <LogOut className="h-4 w-4 mr-2.5" />Sign Out
        </button>
      </div>
    </aside>
  );
}
