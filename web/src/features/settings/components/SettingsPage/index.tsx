import { useState } from 'react';
import { useNavigate, Link } from 'react-router-dom';
import { useAuth } from '../../../../contexts/AuthContext';
import { useWithdrawal } from '../../../auth/api/use-withdrawal';
import { ArrowLeft, AlertTriangle, Loader2 } from 'lucide-react';
import { useSettingsPageI18n } from './i18n';

export function SettingsPage() {
  const t = useSettingsPageI18n();
  const navigate = useNavigate();
  const { account, setAccount } = useAuth();
  const withdrawalMutation = useWithdrawal();
  const [showConfirmDialog, setShowConfirmDialog] = useState(false);

  const handleWithdrawal = async () => {
    try {
      await withdrawalMutation.mutateAsync();
      setAccount(null);
      navigate('/signin');
    } catch (error) {
      console.error('Withdrawal failed:', error);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900">
      {/* Header */}
      <header className="bg-white dark:bg-gray-800 shadow">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex items-center py-6">
            <Link
              to="/dashboard"
              className="mr-4 p-2 rounded-md text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700"
            >
              <ArrowLeft className="h-5 w-5" />
            </Link>
            <div>
              <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
                {t.accountSettings}
              </h1>
            </div>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="max-w-3xl mx-auto py-6 sm:px-6 lg:px-8">
        {/* Profile Section */}
        <div className="px-4 py-6 sm:px-0">
          <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-6">
            <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-4">
              {t.profile}
            </h2>
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                  {t.name}
                </label>
                <p className="mt-1 text-sm text-gray-900 dark:text-white">
                  {account?.name}
                </p>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                  {t.email}
                </label>
                <p className="mt-1 text-sm text-gray-900 dark:text-white">
                  {account?.email}
                </p>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                  Account ID
                </label>
                <p className="mt-1 text-sm text-gray-900 dark:text-white font-mono">
                  {account?.pk}
                </p>
              </div>
            </div>
          </div>
        </div>

        {/* Danger Zone */}
        <div className="px-4 py-6 sm:px-0">
          <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-6 border-2 border-red-200 dark:border-red-900">
            <div className="flex items-start">
              <AlertTriangle className="h-6 w-6 text-red-600 dark:text-red-400 mt-0.5" />
              <div className="ml-3 flex-1">
                <h2 className="text-xl font-semibold text-red-600 dark:text-red-400 mb-2">
                  {t.withdrawal}
                </h2>
                <p className="text-sm text-gray-600 dark:text-gray-400 mb-4">
                  {t.withdrawalWarning}
                </p>
                <button
                  onClick={() => setShowConfirmDialog(true)}
                  className="px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2"
                >
                  {t.withdrawal}
                </button>
              </div>
            </div>
          </div>
        </div>
      </main>

      {/* Confirmation Dialog */}
      {showConfirmDialog && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
          <div className="bg-white dark:bg-gray-800 rounded-lg max-w-md w-full p-6">
            <div className="flex items-center mb-4">
              <AlertTriangle className="h-6 w-6 text-red-600 dark:text-red-400" />
              <h3 className="ml-2 text-lg font-semibold text-gray-900 dark:text-white">
                {t.confirmWithdrawal}
              </h3>
            </div>
            <p className="text-sm text-gray-600 dark:text-gray-400 mb-6">
              {t.withdrawalWarning}
            </p>
            <div className="flex justify-end space-x-3">
              <button
                onClick={() => setShowConfirmDialog(false)}
                disabled={withdrawalMutation.isPending}
                className="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-700 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600 disabled:opacity-50"
              >
                {t.cancel}
              </button>
              <button
                onClick={handleWithdrawal}
                disabled={withdrawalMutation.isPending}
                className="px-4 py-2 text-sm font-medium text-white bg-red-600 rounded-md hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed flex items-center"
              >
                {withdrawalMutation.isPending ? (
                  <>
                    <Loader2 className="animate-spin -ml-1 mr-2 h-4 w-4" />
                    {t.loading}
                  </>
                ) : (
                  t.confirm
                )}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
