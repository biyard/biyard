import { useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate, Link } from "react-router-dom";
import { useAuth } from "../../auth/contexts/AuthContext";
import { useWithdrawal } from "../../auth/api/use-withdrawal";
import { ArrowLeft, AlertTriangle, Loader2 } from "lucide-react";

export function SettingsPage() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { account, setAccount } = useAuth();
  const withdrawalMutation = useWithdrawal();
  const [showConfirmDialog, setShowConfirmDialog] = useState(false);

  const handleWithdrawal = async () => {
    try {
      await withdrawalMutation.mutateAsync();
      setAccount(null);
      navigate("/signin");
    } catch (error) {
      console.error("Withdrawal failed:", error);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900">
      {/* Header */}
      <header className="bg-white shadow dark:bg-gray-800">
        <div className="px-4 mx-auto max-w-7xl sm:px-6 lg:px-8">
          <div className="flex items-center py-6">
            <Link
              to="/dashboard"
              className="p-2 mr-4 text-gray-600 rounded-md dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700"
            >
              <ArrowLeft className="w-5 h-5" />
            </Link>
            <div>
              <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
                {t("account.accountSettings")}
              </h1>
            </div>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="py-6 mx-auto max-w-3xl sm:px-6 lg:px-8">
        {/* Profile Section */}
        <div className="py-6 px-4 sm:px-0">
          <div className="p-6 bg-white rounded-lg shadow dark:bg-gray-800">
            <h2 className="mb-4 text-xl font-semibold text-gray-900 dark:text-white">
              {t("account.profile")}
            </h2>
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                  {t("auth.name")}
                </label>
                <p className="mt-1 text-sm text-gray-900 dark:text-white">
                  {account?.name}
                </p>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                  {t("auth.email")}
                </label>
                <p className="mt-1 text-sm text-gray-900 dark:text-white">
                  {account?.email}
                </p>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                  Account ID
                </label>
                <p className="mt-1 font-mono text-sm text-gray-900 dark:text-white">
                  {account?.pk}
                </p>
              </div>
            </div>
          </div>
        </div>

        {/* Danger Zone */}
        <div className="py-6 px-4 sm:px-0">
          <div className="p-6 bg-white rounded-lg border-2 border-red-200 shadow dark:bg-gray-800 dark:border-red-900">
            <div className="flex items-start">
              <AlertTriangle className="mt-0.5 w-6 h-6 text-red-600 dark:text-red-400" />
              <div className="flex-1 ml-3">
                <h2 className="mb-2 text-xl font-semibold text-red-600 dark:text-red-400">
                  {t("account.withdrawal")}
                </h2>
                <p className="mb-4 text-sm text-gray-600 dark:text-gray-400">
                  {t("account.withdrawalWarning")}
                </p>
                <button
                  onClick={() => setShowConfirmDialog(true)}
                  className="py-2 px-4 text-white bg-red-600 rounded-md hover:bg-red-700 focus:ring-2 focus:ring-red-500 focus:ring-offset-2 focus:outline-none"
                >
                  {t("account.withdrawal")}
                </button>
              </div>
            </div>
          </div>
        </div>
      </main>

      {/* Confirmation Dialog */}
      {showConfirmDialog && (
        <div className="flex fixed inset-0 z-50 justify-center items-center p-4 bg-black bg-opacity-50">
          <div className="p-6 w-full max-w-md bg-white rounded-lg dark:bg-gray-800">
            <div className="flex items-center mb-4">
              <AlertTriangle className="w-6 h-6 text-red-600 dark:text-red-400" />
              <h3 className="ml-2 text-lg font-semibold text-gray-900 dark:text-white">
                {t("account.confirmWithdrawal")}
              </h3>
            </div>
            <p className="mb-6 text-sm text-gray-600 dark:text-gray-400">
              {t("account.withdrawalWarning")}
            </p>
            <div className="flex justify-end space-x-3">
              <button
                onClick={() => setShowConfirmDialog(false)}
                disabled={withdrawalMutation.isPending}
                className="py-2 px-4 text-sm font-medium text-gray-700 bg-gray-100 rounded-md dark:text-gray-300 dark:bg-gray-700 hover:bg-gray-200 disabled:opacity-50 dark:hover:bg-gray-600"
              >
                {t("common.cancel")}
              </button>
              <button
                onClick={handleWithdrawal}
                disabled={withdrawalMutation.isPending}
                className="flex items-center py-2 px-4 text-sm font-medium text-white bg-red-600 rounded-md hover:bg-red-700 focus:ring-2 focus:ring-red-500 focus:ring-offset-2 focus:outline-none disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {withdrawalMutation.isPending ? (
                  <>
                    <Loader2 className="mr-2 -ml-1 w-4 h-4 animate-spin" />
                    {t("common.loading")}
                  </>
                ) : (
                  t("common.confirm")
                )}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
