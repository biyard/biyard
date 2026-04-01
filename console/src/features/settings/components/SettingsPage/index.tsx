import { AlertTriangle, Loader2 } from "lucide-react";
import { useController } from "./use-controller";

export function SettingsPage() {
  const ctrl = useController();

  return (
    <div className="max-w-3xl">
      {/* Page Header */}
      <div className="mb-6">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          {ctrl.t.accountSettings}
        </h1>
      </div>

      {/* Profile Section */}
      <div className="mb-6">
          <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-6">
            <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-4">
              {ctrl.t.profile}
            </h2>
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                  {ctrl.t.name}
                </label>
                <p className="mt-1 text-sm text-gray-900 dark:text-white">
                  {ctrl.account?.name}
                </p>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                  {ctrl.t.email}
                </label>
                <p className="mt-1 text-sm text-gray-900 dark:text-white">
                  {ctrl.account?.email}
                </p>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                  {ctrl.t.accountId}
                </label>
                <p className="mt-1 text-sm text-gray-900 dark:text-white font-mono">
                  {ctrl.account?.pk}
                </p>
              </div>
            </div>
          </div>
        </div>

      {/* Danger Zone */}
      <div>
        <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-6 border-2 border-red-200 dark:border-red-900">
          <div className="flex items-start">
            <AlertTriangle className="h-6 w-6 text-red-600 dark:text-red-400 mt-0.5" />
            <div className="ml-3 flex-1">
              <h2 className="text-xl font-semibold text-red-600 dark:text-red-400 mb-2">
                {ctrl.t.withdrawal}
              </h2>
              <p className="text-sm text-gray-600 dark:text-gray-400 mb-4">
                {ctrl.t.withdrawalWarning}
              </p>
              <button
                onClick={() => ctrl.showConfirmDialog.set(true)}
                className="px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2"
              >
                {ctrl.t.withdrawal}
              </button>
            </div>
          </div>
        </div>
      </div>

      {/* Confirmation Dialog */}
      {ctrl.showConfirmDialog.get() && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
          <div className="bg-white dark:bg-gray-800 rounded-lg max-w-md w-full p-6">
            <div className="flex items-center mb-4">
              <AlertTriangle className="h-6 w-6 text-red-600 dark:text-red-400" />
              <h3 className="ml-2 text-lg font-semibold text-gray-900 dark:text-white">
                {ctrl.t.confirmWithdrawal}
              </h3>
            </div>
            <p className="text-sm text-gray-600 dark:text-gray-400 mb-6">
              {ctrl.t.withdrawalWarning}
            </p>
            <div className="flex justify-end space-x-3">
              <button
                onClick={() => ctrl.showConfirmDialog.set(false)}
                disabled={ctrl.withdrawalMutation.isPending}
                className="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-700 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600 disabled:opacity-50"
              >
                {ctrl.t.cancel}
              </button>
              <button
                onClick={ctrl.handleWithdrawal}
                disabled={ctrl.withdrawalMutation.isPending}
                className="px-4 py-2 text-sm font-medium text-white bg-red-600 rounded-md hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed flex items-center"
              >
                {ctrl.withdrawalMutation.isPending ? (
                  <>
                    <Loader2 className="animate-spin -ml-1 mr-2 h-4 w-4" />
                    {ctrl.t.loading}
                  </>
                ) : (
                  ctrl.t.confirm
                )}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
