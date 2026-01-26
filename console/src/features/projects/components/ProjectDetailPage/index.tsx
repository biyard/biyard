import {
  ArrowLeft,
  Loader2,
  FolderX,
  Coins,
  Settings,
  LayoutDashboard,
  Star,
} from "lucide-react";
import { useController, type TabType } from "./use-controller";
import { OverviewTab } from "./OverviewTab";
import { TokensTab } from "./TokensTab";
import { PointsTab } from "./PointsTab";
import { SettingsTab } from "./SettingsTab";

export function ProjectDetailPage() {
  const ctrl = useController();

  if (ctrl.isLoading) {
    return (
      <div className="flex items-center justify-center min-h-96">
        <div className="text-center">
          <Loader2 className="mx-auto h-12 w-12 text-gray-400 animate-spin" />
          <p className="mt-4 text-gray-600 dark:text-gray-400">
            {ctrl.t.loading}
          </p>
        </div>
      </div>
    );
  }

  if (ctrl.isError || !ctrl.project) {
    return (
      <div className="flex items-center justify-center min-h-96">
        <div className="text-center">
          <FolderX className="mx-auto h-12 w-12 text-gray-400" />
          <h3 className="mt-2 text-lg font-medium text-gray-900 dark:text-white">
            {ctrl.t.notFound}
          </h3>
          <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
            {ctrl.t.notFoundDescription}
          </p>
          <button
            onClick={ctrl.handleBack}
            className="mt-4 inline-flex items-center px-4 py-2 text-sm font-medium text-blue-600 hover:text-blue-800"
          >
            <ArrowLeft className="h-4 w-4 mr-2" />
            {ctrl.t.backToProjects}
          </button>
        </div>
      </div>
    );
  }

  const tabs: { id: TabType; label: string; icon: React.ReactNode }[] = [
    {
      id: "overview",
      label: ctrl.t.overview,
      icon: <LayoutDashboard className="h-4 w-4" />,
    },
    { id: "tokens", label: ctrl.t.token, icon: <Coins className="h-4 w-4" /> },
    { id: "points", label: ctrl.t.points, icon: <Star className="h-4 w-4" /> },
    {
      id: "settings",
      label: ctrl.t.settings,
      icon: <Settings className="h-4 w-4" />,
    },
  ];

  return (
    <div>
      {/* Header */}
      <div className="mb-6">
        <button
          onClick={ctrl.handleBack}
          className="flex items-center text-sm text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 mb-4"
        >
          <ArrowLeft className="h-4 w-4 mr-1" />
          {ctrl.t.backToProjects}
        </button>

        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
              {ctrl.project.name}
            </h1>
            {ctrl.project.description && (
              <p className="mt-1 text-gray-500 dark:text-gray-400">
                {ctrl.project.description}
              </p>
            )}
          </div>
          <span
            className={`px-3 py-1 text-sm font-semibold rounded-full ${ctrl.project.getStatusColorClass()}`}
          >
            {ctrl.project.status}
          </span>
        </div>
      </div>

      {/* Tabs */}
      <div className="border-b border-gray-200 dark:border-gray-700 mb-6">
        <nav className="-mb-px flex space-x-8">
          {tabs.map((tab) => (
            <button
              key={tab.id}
              onClick={() => ctrl.setActiveTab(tab.id)}
              className={`flex items-center py-4 px-1 border-b-2 text-sm font-medium ${
                ctrl.activeTab === tab.id
                  ? "border-blue-500 text-blue-600 dark:text-blue-400"
                  : "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 dark:text-gray-400 dark:hover:text-gray-200"
              }`}
            >
              {tab.icon}
              <span className="ml-2">{tab.label}</span>
            </button>
          ))}
        </nav>
      </div>

      {/* Tab Content */}
      {ctrl.activeTab === "overview" && (
        <OverviewTab
          project={ctrl.project}
          projectId={ctrl.projectId || ""}
          token={ctrl.token}
          isLoadingToken={ctrl.isLoadingToken}
          pointTransactions={ctrl.pointTransactions}
          isLoadingPointTransactions={ctrl.isLoadingPointTransactions}
          t={ctrl.t}
        />
      )}
      {ctrl.activeTab === "tokens" && <TokensTab t={ctrl.t} />}
      {ctrl.activeTab === "points" && (
        <PointsTab
          transactions={ctrl.pointTransactions}
          isLoading={ctrl.isLoadingPointTransactions}
          t={ctrl.t}
        />
      )}
      {ctrl.activeTab === "settings" && <SettingsTab t={ctrl.t} />}
    </div>
  );
}
