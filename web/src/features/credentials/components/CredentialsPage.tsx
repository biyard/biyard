import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Link } from 'react-router-dom';
import { ArrowLeft, Plus, Copy, Check, Key, Loader2, Trash2 } from 'lucide-react';
import { useListCredentials } from '../api/use-list-credentials';
import { useCreateCredential } from '../api/use-create-credential';
import { useRevokeCredential } from '../api/use-revoke-credential';

export function CredentialsPage() {
  const { t } = useTranslation();
  const [showCreateDialog, setShowCreateDialog] = useState(false);
  const [newCredentialName, setNewCredentialName] = useState('');
  const [generatedKey, setGeneratedKey] = useState<string | null>(null);
  const [copiedKey, setCopiedKey] = useState<string | null>(null);

  const { data: credentials, isLoading } = useListCredentials();
  const createMutation = useCreateCredential();
  const revokeMutation = useRevokeCredential();

  const handleCreateCredential = async () => {
    if (!newCredentialName.trim()) return;

    try {
      const response = await createMutation.mutateAsync({
        name: newCredentialName,
      });

      setGeneratedKey(response.api_key || null);
      setNewCredentialName('');
      setShowCreateDialog(false);
    } catch (error) {
      console.error('Failed to create credential:', error);
    }
  };

  const handleRevokeCredential = async (credentialId: string) => {
    if (!confirm(t('credentials.confirmRevoke') || 'Are you sure you want to revoke this credential?')) {
      return;
    }

    try {
      await revokeMutation.mutateAsync(credentialId);
    } catch (error) {
      console.error('Failed to revoke credential:', error);
    }
  };

  const handleCopyKey = (key: string) => {
    navigator.clipboard.writeText(key);
    setCopiedKey(key);
    setTimeout(() => setCopiedKey(null), 2000);
  };

  const maskKey = (key: string) => {
    if (key.length <= 16) return key;
    return `${key.substring(0, 12)}...${key.substring(key.length - 4)}`;
  };

  const extractCredentialId = (pk: string) => {
    // pk format is "Credential(uuid)"
    const match = pk.match(/Credential\(([^)]+)\)/);
    return match ? match[1] : pk;
  };

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900">
      {/* Header */}
      <header className="bg-white dark:bg-gray-800 shadow">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex items-center justify-between py-6">
            <div className="flex items-center">
              <Link
                to="/dashboard"
                className="mr-4 p-2 rounded-md text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700"
              >
                <ArrowLeft className="h-5 w-5" />
              </Link>
              <div>
                <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
                  {t('credentials.title')}
                </h1>
                <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
                  {t('credentials.description')}
                </p>
              </div>
            </div>
            <button
              onClick={() => setShowCreateDialog(true)}
              className="flex items-center px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
            >
              <Plus className="h-5 w-5 mr-2" />
              {t('credentials.createNew')}
            </button>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
        <div className="px-4 sm:px-0">
          {isLoading ? (
            <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-12 text-center">
              <Loader2 className="mx-auto h-12 w-12 text-gray-400 animate-spin" />
              <p className="mt-4 text-gray-600 dark:text-gray-400">{t('common.loading')}</p>
            </div>
          ) : !credentials || credentials.length === 0 ? (
            <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-12 text-center">
              <Key className="mx-auto h-12 w-12 text-gray-400" />
              <h3 className="mt-2 text-sm font-medium text-gray-900 dark:text-white">
                {t('credentials.noCredentials')}
              </h3>
              <div className="mt-6">
                <button
                  onClick={() => setShowCreateDialog(true)}
                  className="inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                >
                  <Plus className="h-5 w-5 mr-2" />
                  {t('credentials.createNew')}
                </button>
              </div>
            </div>
          ) : (
            <div className="bg-white dark:bg-gray-800 shadow rounded-lg overflow-hidden">
              <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
                <thead className="bg-gray-50 dark:bg-gray-700">
                  <tr>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                      {t('credentials.name')}
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                      {t('credentials.apiKey')}
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                      {t('credentials.createdAt')}
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                      {t('credentials.status')}
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                      {t('common.actions')}
                    </th>
                  </tr>
                </thead>
                <tbody className="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
                  {credentials.map((credential) => (
                    <tr key={credential.pk}>
                      <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white">
                        {credential.name}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                        <div className="flex items-center">
                          <code className="mr-2">{maskKey(credential.api_key_prefix)}</code>
                          <button
                            onClick={() => handleCopyKey(credential.api_key_prefix)}
                            className="p-1 rounded hover:bg-gray-100 dark:hover:bg-gray-700"
                            title={t('credentials.copy')}
                          >
                            {copiedKey === credential.api_key_prefix ? (
                              <Check className="h-4 w-4 text-green-600" />
                            ) : (
                              <Copy className="h-4 w-4" />
                            )}
                          </button>
                        </div>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                        {new Date(credential.created_at * 1000).toLocaleDateString()}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap">
                        <span
                          className={`px-2 inline-flex text-xs leading-5 font-semibold rounded-full ${
                            credential.status === 'Active'
                              ? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200'
                              : 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200'
                          }`}
                        >
                          {credential.status === 'Active' ? t('credentials.active') : t('credentials.inactive')}
                        </span>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm">
                        {credential.status === 'Active' && (
                          <button
                            onClick={() => handleRevokeCredential(extractCredentialId(credential.pk))}
                            disabled={revokeMutation.isPending}
                            className="text-red-600 hover:text-red-900 dark:text-red-400 dark:hover:text-red-300 disabled:opacity-50"
                          >
                            <Trash2 className="h-4 w-4" />
                          </button>
                        )}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </div>
      </main>

      {/* Create Credential Dialog */}
      {showCreateDialog && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
          <div className="bg-white dark:bg-gray-800 rounded-lg max-w-md w-full p-6">
            <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
              {t('credentials.createNew')}
            </h3>
            <div className="mb-4">
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                {t('credentials.name')}
              </label>
              <input
                type="text"
                value={newCredentialName}
                onChange={(e) => setNewCredentialName(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
                placeholder="My API Key"
              />
            </div>
            <div className="flex justify-end space-x-3">
              <button
                onClick={() => {
                  setShowCreateDialog(false);
                  setNewCredentialName('');
                }}
                disabled={createMutation.isPending}
                className="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-700 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600 disabled:opacity-50"
              >
                {t('common.cancel')}
              </button>
              <button
                onClick={handleCreateCredential}
                disabled={!newCredentialName.trim() || createMutation.isPending}
                className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed flex items-center"
              >
                {createMutation.isPending ? (
                  <>
                    <Loader2 className="animate-spin -ml-1 mr-2 h-4 w-4" />
                    {t('common.loading')}
                  </>
                ) : (
                  t('credentials.generateKey')
                )}
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Generated Key Dialog */}
      {generatedKey && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
          <div className="bg-white dark:bg-gray-800 rounded-lg max-w-md w-full p-6">
            <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
              {t('credentials.keyGenerated')}
            </h3>
            <div className="bg-gray-50 dark:bg-gray-900 p-4 rounded-md mb-4">
              <div className="flex items-center justify-between">
                <code className="text-sm text-gray-900 dark:text-white break-all">
                  {generatedKey}
                </code>
                <button
                  onClick={() => handleCopyKey(generatedKey)}
                  className="ml-2 p-2 rounded hover:bg-gray-200 dark:hover:bg-gray-700"
                >
                  {copiedKey === generatedKey ? (
                    <Check className="h-5 w-5 text-green-600" />
                  ) : (
                    <Copy className="h-5 w-5" />
                  )}
                </button>
              </div>
            </div>
            <p className="text-sm text-red-600 dark:text-red-400 mb-4">
              {t('credentials.keyGenerated')}
            </p>
            <div className="flex justify-end">
              <button
                onClick={() => setGeneratedKey(null)}
                className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700"
              >
                {t('common.close')}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
