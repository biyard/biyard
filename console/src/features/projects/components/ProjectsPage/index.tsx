import { Plus, Loader2, Trash2, FolderOpen } from "lucide-react";
import { useController } from "./use-controller";

export function ProjectsPage() {
  const ctrl = useController();

  return (
    <div>
      {/* Page Header */}
      <div className="mb-6 flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
            {ctrl.t.description}
          </h1>
        </div>
        <button
          onClick={() => ctrl.showCreateDialog.set(true)}
          className="flex items-center px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
        >
          <Plus className="h-5 w-5 mr-2" />
          {ctrl.t.createNew}
        </button>
      </div>

      {/* Main Content */}
      <div>
          {ctrl.isLoading ? (
            <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-12 text-center">
              <Loader2 className="mx-auto h-12 w-12 text-gray-400 animate-spin" />
              <p className="mt-4 text-gray-600 dark:text-gray-400">
                {ctrl.t.loading}
              </p>
            </div>
          ) : !ctrl.projects || ctrl.projects.length === 0 ? (
            <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-12 text-center">
              <FolderOpen className="mx-auto h-12 w-12 text-gray-400" />
              <h3 className="mt-2 text-sm font-medium text-gray-900 dark:text-white">
                {ctrl.t.noProjects}
              </h3>
              <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
                {ctrl.t.noProjectsDescription}
              </p>
              <div className="mt-6">
                <button
                  onClick={() => ctrl.showCreateDialog.set(true)}
                  className="inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700"
                >
                  <Plus className="h-5 w-5 mr-2" />
                  {ctrl.t.createNew}
                </button>
              </div>
            </div>
          ) : (
            <div className="bg-white dark:bg-gray-800 shadow rounded-lg overflow-hidden">
              <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
                <thead className="bg-gray-50 dark:bg-gray-700">
                  <tr>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase">
                      {ctrl.t.projectId}
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase">
                      {ctrl.t.projectName}
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase">
                      {ctrl.t.monthlyTokenSupply}
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase">
                      {ctrl.t.status}
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase">
                      {ctrl.t.actions}
                    </th>
                  </tr>
                </thead>
                <tbody className="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
                  {ctrl.projects.map((project) => (
                    <tr key={project.id}>
                      <td className="px-6 py-4 whitespace-nowrap">
                        <code className="text-xs text-gray-600 dark:text-gray-400 bg-gray-100 dark:bg-gray-700 px-2 py-1 rounded">
                          {project.id}
                        </code>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap">
                        <div className="text-sm font-medium text-gray-900 dark:text-white">
                          {project.name}
                        </div>
                        {project.description && (
                          <div className="text-sm text-gray-500 dark:text-gray-400">
                            {project.description}
                          </div>
                        )}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                        {project.getFormattedTokenSupply()}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap">
                        <span className={`px-2 inline-flex text-xs leading-5 font-semibold rounded-full ${project.getStatusColorClass()}`}>
                          {project.status}
                        </span>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm">
                        <button
                          onClick={() => ctrl.handleDeleteProject(project.id)}
                          disabled={ctrl.deleteMutation.isPending}
                          className="text-red-600 hover:text-red-900 dark:text-red-400 dark:hover:text-red-300"
                        >
                          <Trash2 className="h-4 w-4" />
                        </button>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
      </div>

      {/* Create Project Dialog */}
      {ctrl.showCreateDialog.get() && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
          <div className="bg-white dark:bg-gray-800 rounded-lg max-w-md w-full p-6">
            <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
              {ctrl.t.createProject}
            </h3>

            {ctrl.error.get() && (
              <div className="mb-4 p-3 bg-red-50 dark:bg-red-900/20 rounded-md">
                <p className="text-sm text-red-800 dark:text-red-400">
                  {ctrl.error.get()}
                </p>
              </div>
            )}

            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  {ctrl.t.projectName}
                </label>
                <input
                  type="text"
                  value={ctrl.projectName.get()}
                  onChange={(e) => ctrl.projectName.set(e.target.value)}
                  placeholder={ctrl.t.enterProjectName}
                  className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md dark:bg-gray-700 dark:text-white"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  {ctrl.t.projectDescription}
                </label>
                <input
                  type="text"
                  value={ctrl.description.get()}
                  onChange={(e) => ctrl.description.set(e.target.value)}
                  placeholder={ctrl.t.enterDescription}
                  className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md dark:bg-gray-700 dark:text-white"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  {ctrl.t.tokenSymbol}
                </label>
                <input
                  type="text"
                  value={ctrl.symbol.get()}
                  onChange={(e) => ctrl.symbol.set(e.target.value)}
                  placeholder={ctrl.t.enterSymbol}
                  maxLength={10}
                  className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md dark:bg-gray-700 dark:text-white"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  {ctrl.t.monthlyTokenSupply}
                </label>
                <input
                  type="number"
                  value={ctrl.monthlyTokenSupply.get()}
                  onChange={(e) => ctrl.monthlyTokenSupply.set(e.target.value)}
                  placeholder={ctrl.t.enterTokenSupply}
                  className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md dark:bg-gray-700 dark:text-white"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  {ctrl.t.tokenDecimals}
                </label>
                <input
                  type="number"
                  value={ctrl.decimals.get()}
                  onChange={(e) => ctrl.decimals.set(e.target.value)}
                  placeholder={ctrl.t.enterDecimals}
                  min={0}
                  max={18}
                  className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md dark:bg-gray-700 dark:text-white"
                />
              </div>
            </div>

            <div className="flex justify-end space-x-3 mt-6">
              <button
                onClick={() => {
                  ctrl.showCreateDialog.set(false);
                  ctrl.error.set("");
                }}
                disabled={ctrl.createMutation.isPending}
                className="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-700 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600"
              >
                {ctrl.t.cancel}
              </button>
              <button
                onClick={ctrl.handleCreateProject}
                disabled={ctrl.createMutation.isPending}
                className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 disabled:opacity-50"
              >
                {ctrl.createMutation.isPending ? ctrl.t.creating : ctrl.t.createProject}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
