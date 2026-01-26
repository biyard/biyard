import { Link } from "react-router-dom";
import { Settings, Key, FolderKanban } from "lucide-react";
import { useController } from "./use-controller";

export function DashboardPage() {
  const ctrl = useController();

  return (
    <div>
      {/* Page Header */}
      <div className="mb-6">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          {ctrl.t.title}
        </h1>
        <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
          {ctrl.t.tagline}
        </p>
      </div>

      {/* Welcome Section */}
      <div className="mb-6">
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
      <div className="mb-6">
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
      <div>
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
    </div>
  );
}
