import { Link } from "react-router-dom";
import {
  ArrowLeft,
  Plus,
  Copy,
  Check,
  Key,
  Loader2,
  Trash2,
} from "lucide-react";
import { useController } from "./use-controller";

export function CredentialsPage() {
  const ctrl = useController();

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900">
      {/* Header */}
      <header className="bg-white shadow dark:bg-gray-800">
        <div className="px-4 mx-auto max-w-7xl sm:px-6 lg:px-8">
          <div className="flex justify-between items-center py-6">
            <div className="flex items-center">
              <Link
                to="/dashboard"
                className="p-2 mr-4 text-gray-600 rounded-md dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700"
              >
                <ArrowLeft className="w-5 h-5" />
              </Link>
              <div>
                <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
                  {ctrl.t.title}
                </h1>
                <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
                  {ctrl.t.description}
                </p>
              </div>
            </div>
            <button
              onClick={() => ctrl.showCreateDialog.set(true)}
              className="flex items-center py-2 px-4 text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:outline-none"
            >
              <Plus className="mr-2 w-5 h-5" />
              {ctrl.t.createNew}
            </button>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="py-6 mx-auto max-w-7xl sm:px-6 lg:px-8">
        <div className="px-4 sm:px-0">
          {ctrl.isLoading ? (
            <div className="p-12 text-center bg-white rounded-lg shadow dark:bg-gray-800">
              <Loader2 className="mx-auto w-12 h-12 text-gray-400 animate-spin" />
              <p className="mt-4 text-gray-600 dark:text-gray-400">
                {ctrl.t.loading}
              </p>
            </div>
          ) : !ctrl.credentials || ctrl.credentials.length === 0 ? (
            <div className="p-12 text-center bg-white rounded-lg shadow dark:bg-gray-800">
              <Key className="mx-auto w-12 h-12 text-gray-400" />
              <h3 className="mt-2 text-sm font-medium text-gray-900 dark:text-white">
                {ctrl.t.noCredentials}
              </h3>
              <div className="mt-6">
                <button
                  onClick={() => ctrl.showCreateDialog.set(true)}
                  className="inline-flex items-center py-2 px-4 text-sm font-medium text-white bg-blue-600 rounded-md border border-transparent shadow-sm hover:bg-blue-700 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:outline-none"
                >
                  <Plus className="mr-2 w-5 h-5" />
                  {ctrl.t.createNew}
                </button>
              </div>
            </div>
          ) : (
            <div className="overflow-hidden bg-white rounded-lg shadow dark:bg-gray-800">
              <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
                <thead className="bg-gray-50 dark:bg-gray-700">
                  <tr>
                    <th className="py-3 px-6 text-xs font-medium tracking-wider text-left text-gray-500 uppercase dark:text-gray-300">
                      {ctrl.t.name}
                    </th>
                    <th className="py-3 px-6 text-xs font-medium tracking-wider text-left text-gray-500 uppercase dark:text-gray-300">
                      {ctrl.t.apiKey}
                    </th>
                    <th className="py-3 px-6 text-xs font-medium tracking-wider text-left text-gray-500 uppercase dark:text-gray-300">
                      {ctrl.t.createdAt}
                    </th>
                    <th className="py-3 px-6 text-xs font-medium tracking-wider text-left text-gray-500 uppercase dark:text-gray-300">
                      {ctrl.t.status}
                    </th>
                    <th className="py-3 px-6 text-xs font-medium tracking-wider text-left text-gray-500 uppercase dark:text-gray-300">
                      {ctrl.t.actions}
                    </th>
                  </tr>
                </thead>
                <tbody className="bg-white divide-y divide-gray-200 dark:bg-gray-800 dark:divide-gray-700">
                  {ctrl.credentials.map((credential) => (
                    <tr key={credential.id}>
                      <td className="py-4 px-6 text-sm font-medium text-gray-900 whitespace-nowrap dark:text-white">
                        {credential.name}
                      </td>
                      <td className="py-4 px-6 text-sm text-gray-500 whitespace-nowrap dark:text-gray-400">
                        <div className="flex items-center">
                          <code className="mr-2">
                            {ctrl.maskKey(credential.api_key_prefix)}
                          </code>
                          <button
                            onClick={() =>
                              ctrl.handleCopyKey(credential.api_key_prefix)
                            }
                            className="p-1 rounded hover:bg-gray-100 dark:hover:bg-gray-700"
                            title={ctrl.t.copy}
                          >
                            {ctrl.copiedKey.get() ===
                            credential.api_key_prefix ? (
                              <Check className="w-4 h-4 text-green-600" />
                            ) : (
                              <Copy className="w-4 h-4" />
                            )}
                          </button>
                        </div>
                      </td>
                      <td className="py-4 px-6 text-sm text-gray-500 whitespace-nowrap dark:text-gray-400">
                        {new Date(
                          credential.created_at * 1000,
                        ).toLocaleDateString()}
                      </td>
                      <td className="py-4 px-6 whitespace-nowrap">
                        <span
                          className={`px-2 inline-flex text-xs leading-5 font-semibold rounded-full ${
                            credential.status === "Active"
                              ? "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200"
                              : "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200"
                          }`}
                        >
                          {credential.status === "Active"
                            ? ctrl.t.active
                            : ctrl.t.inactive}
                        </span>
                      </td>
                      <td className="py-4 px-6 text-sm whitespace-nowrap">
                        <button
                          onClick={() =>
                            ctrl.handleRevokeCredential(credential.id)
                          }
                          className="text-red-600 dark:text-red-400 hover:text-red-900 dark:hover:text-red-300"
                          disabled={ctrl.revokeMutation.isPending}
                        >
                          <Trash2 className="w-4 h-4" />
                        </button>
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
      {ctrl.showCreateDialog.get() && (
        <div className="flex fixed inset-0 z-50 justify-center items-center p-4 bg-black bg-opacity-50">
          <div className="p-6 w-full max-w-md bg-white rounded-lg dark:bg-gray-800">
            <h3 className="mb-4 text-lg font-semibold text-gray-900 dark:text-white">
              {ctrl.t.createNew}
            </h3>
            <div className="mb-4">
              <label className="block mb-2 text-sm font-medium text-gray-700 dark:text-gray-300">
                {ctrl.t.name}
              </label>
              <input
                type="text"
                value={ctrl.newCredentialName.get()}
                onChange={(e) => ctrl.newCredentialName.set(e.target.value)}
                className="py-2 px-3 w-full rounded-md border border-gray-300 shadow-sm dark:text-white dark:bg-gray-700 dark:border-gray-600 focus:border-blue-500 focus:ring-blue-500 focus:outline-none"
                placeholder="My API Key"
              />
            </div>
            <div className="flex justify-end space-x-3">
              <button
                onClick={() => {
                  ctrl.showCreateDialog.set(false);
                  ctrl.newCredentialName.set("");
                }}
                disabled={ctrl.createMutation.isPending}
                className="py-2 px-4 text-sm font-medium text-gray-700 bg-gray-100 rounded-md dark:text-gray-300 dark:bg-gray-700 hover:bg-gray-200 disabled:opacity-50 dark:hover:bg-gray-600"
              >
                {ctrl.t.cancel}
              </button>
              <button
                onClick={ctrl.handleCreateCredential}
                disabled={
                  !ctrl.newCredentialName.get().trim() ||
                  ctrl.createMutation.isPending
                }
                className="flex items-center py-2 px-4 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:outline-none disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {ctrl.createMutation.isPending ? (
                  <>
                    <Loader2 className="mr-2 -ml-1 w-4 h-4 animate-spin" />
                    {ctrl.t.loading}
                  </>
                ) : (
                  ctrl.t.generateKey
                )}
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Generated Key Dialog */}
      {ctrl.generatedKey.get() && (
        <div className="flex fixed inset-0 z-50 justify-center items-center p-4 bg-black bg-opacity-50">
          <div className="p-6 w-full max-w-md bg-white rounded-lg dark:bg-gray-800">
            <h3 className="mb-4 text-lg font-semibold text-gray-900 dark:text-white">
              {ctrl.t.keyGenerated}
            </h3>
            <div className="p-4 mb-4 bg-gray-50 rounded-md dark:bg-gray-900">
              <div className="flex justify-between items-center">
                <code className="text-sm text-gray-900 break-all dark:text-white">
                  {ctrl.generatedKey.get()}
                </code>
                <button
                  onClick={() =>
                    ctrl.handleCopyKey(ctrl.generatedKey.get() || "")
                  }
                  className="p-2 ml-2 rounded hover:bg-gray-200 dark:hover:bg-gray-700"
                >
                  {ctrl.copiedKey.get() === ctrl.generatedKey.get() ? (
                    <Check className="w-5 h-5 text-green-600" />
                  ) : (
                    <Copy className="w-5 h-5" />
                  )}
                </button>
              </div>
            </div>
            <p className="mb-4 text-sm text-red-600 dark:text-red-400">
              {ctrl.t.keyGeneratedWarning}
            </p>
            <div className="flex justify-end">
              <button
                onClick={() => ctrl.generatedKey.set(null)}
                className="py-2 px-4 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700"
              >
                {ctrl.t.close}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
