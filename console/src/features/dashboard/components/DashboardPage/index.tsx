import { Link } from "react-router-dom";
import { Sun, Moon, Globe, Settings, Key, LogOut, FolderKanban } from "lucide-react";
import { useController } from "./use-controller";

export function DashboardPage() {
  const ctrl = useController();

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900">
      {/* Header */}
      <header className="bg-white dark:bg-gray-800 shadow">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center py-6">
            <div>
              <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
                {ctrl.t.title}
              </h1>
              <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
                {ctrl.t.tagline}
              </p>
            </div>
            <div className="flex items-center space-x-4">
              {/* Language Toggle */}
              <button
                onClick={ctrl.toggleLanguage}
                className="p-2 rounded-md text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700"
                title={ctrl.i18n.language === "en" ? "한국어" : "English"}
              >
                <Globe className="h-5 w-5" />
              </button>

              {/* Theme Toggle */}
              <button
                onClick={ctrl.theme.toggleTheme}
                className="p-2 rounded-md text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700"
                title={
                  ctrl.theme.theme === "light"
                    ? ctrl.t.themeDark
                    : ctrl.t.themeLight
                }
              >
                {ctrl.theme.theme === "light" ? (
                  <Moon className="h-5 w-5" />
                ) : (
                  <Sun className="h-5 w-5" />
                )}
              </button>

              {/* Sign Out */}
              <button
                onClick={ctrl.handleSignOut}
                className="flex items-center px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-md"
              >
                <LogOut className="h-4 w-4 mr-2" />
                {ctrl.t.signOut}
              </button>
            </div>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
        {/* Welcome Section */}
        <div className="px-4 py-6 sm:px-0">
          <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-6">
            <h2 className="text-2xl font-bold text-gray-900 dark:text-white mb-2">
              {ctrl.t.welcome}
            </h2>
            <p className="text-gray-600 dark:text-gray-400">
              {ctrl.t.myAccount}: {ctrl.account?.name} ({ctrl.account?.email})
            </p>
          </div>
        </div>

        {/* Quick Actions Grid */}
        <div className="px-4 py-6 sm:px-0">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            {/* My Projects Card */}
            <Link
              to="/projects"
              className="bg-white dark:bg-gray-800 shadow rounded-lg p-6 hover:shadow-lg transition-shadow"
            >
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <FolderKanban className="h-8 w-8 text-purple-600 dark:text-purple-400" />
                </div>
                <div className="ml-4">
                  <h3 className="text-lg font-medium text-gray-900 dark:text-white">
                    {ctrl.t.myProjects}
                  </h3>
                  <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
                    {ctrl.t.projectsDescription}
                  </p>
                </div>
              </div>
            </Link>

            {/* API Credentials Card */}
            <Link
              to="/credentials"
              className="bg-white dark:bg-gray-800 shadow rounded-lg p-6 hover:shadow-lg transition-shadow"
            >
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <Key className="h-8 w-8 text-blue-600 dark:text-blue-400" />
                </div>
                <div className="ml-4">
                  <h3 className="text-lg font-medium text-gray-900 dark:text-white">
                    {ctrl.t.apiCredentials}
                  </h3>
                  <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
                    {ctrl.t.credentialsDescription}
                  </p>
                </div>
              </div>
            </Link>

            {/* Account Settings Card */}
            <Link
              to="/settings"
              className="bg-white dark:bg-gray-800 shadow rounded-lg p-6 hover:shadow-lg transition-shadow"
            >
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <Settings className="h-8 w-8 text-gray-600 dark:text-gray-400" />
                </div>
                <div className="ml-4">
                  <h3 className="text-lg font-medium text-gray-900 dark:text-white">
                    {ctrl.t.accountSettings}
                  </h3>
                  <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
                    {ctrl.t.profile}, {ctrl.t.security}
                  </p>
                </div>
              </div>
            </Link>
          </div>
        </div>

        {/* Account Info */}
        <div className="px-4 py-6 sm:px-0">
          <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-6">
            <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">
              {ctrl.t.profile}
            </h3>
            <dl className="grid grid-cols-1 gap-4 sm:grid-cols-2">
              <div>
                <dt className="text-sm font-medium text-gray-500 dark:text-gray-400">
                  {ctrl.t.name}
                </dt>
                <dd className="mt-1 text-sm text-gray-900 dark:text-white">
                  {ctrl.account?.name}
                </dd>
              </div>
              <div>
                <dt className="text-sm font-medium text-gray-500 dark:text-gray-400">
                  {ctrl.t.email}
                </dt>
                <dd className="mt-1 text-sm text-gray-900 dark:text-white">
                  {ctrl.account?.email}
                </dd>
              </div>
              <div>
                <dt className="text-sm font-medium text-gray-500 dark:text-gray-400">
                  {ctrl.t.accountId}
                </dt>
                <dd className="mt-1 text-sm text-gray-900 dark:text-white font-mono">
                  {ctrl.account?.pk}
                </dd>
              </div>
              <div>
                <dt className="text-sm font-medium text-gray-500 dark:text-gray-400">
                  {ctrl.t.createdAt}
                </dt>
                <dd className="mt-1 text-sm text-gray-900 dark:text-white">
                  {ctrl.account?.created_at &&
                    new Date(ctrl.account.created_at * 1000).toLocaleString()}
                </dd>
              </div>
            </dl>
          </div>
        </div>
      </main>
    </div>
  );
}
