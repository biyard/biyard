import {
  ArrowLeft,
  Loader2,
  FolderX,
  Coins,
  Settings,
  LayoutDashboard,
  Star,
  Landmark,
  Users,
  ClipboardList,
} from "lucide-react";
import { useController, type TabType } from "./use-controller";
import { OverviewTab } from "./OverviewTab";
import { TokensTab } from "./TokensTab";
import { PointsTab } from "./PointsTab";
import { SettingsTab } from "./SettingsTab";
import { formatNumber, formatUSD } from "@/lib/mock-data";

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
    {
      id: "treasury",
      label: ctrl.t.treasury,
      icon: <Landmark className="h-4 w-4" />,
    },
    { id: "points", label: ctrl.t.points, icon: <Star className="h-4 w-4" /> },
    { id: "tokens", label: ctrl.t.token, icon: <Coins className="h-4 w-4" /> },
    {
      id: "users",
      label: ctrl.t.users,
      icon: <Users className="h-4 w-4" />,
    },
    {
      id: "audit",
      label: ctrl.t.audit,
      icon: <ClipboardList className="h-4 w-4" />,
    },
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
      {ctrl.activeTab === "treasury" && (
        <div className="space-y-6">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-6">
              <p className="text-sm text-gray-500 dark:text-gray-400">
                {ctrl.t.totalTreasury}
              </p>
              <p className="mt-2 text-3xl font-bold text-gray-900 dark:text-white">
                {formatUSD(24500)}
              </p>
              <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
                ₩{formatNumber(29400000)}
              </p>
            </div>
            <div className="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-6">
              <p className="text-sm text-gray-500 dark:text-gray-400">
                {ctrl.t.floorPrice}
              </p>
              <p className="mt-2 text-3xl font-bold text-gray-900 dark:text-white">
                $0.0245
              </p>
            </div>
          </div>
          <div className="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-6">
            <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-2">
              Monthly Revenue
            </h3>
            <p className="text-sm text-gray-500 dark:text-gray-400 mb-4">
              Treasury grows as customers make purchases through your API.
            </p>
            <div className="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 overflow-hidden">
              <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
                <thead className="bg-gray-50 dark:bg-gray-900">
                  <tr>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                      Month
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                      Type
                    </th>
                    <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                      Amount
                    </th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-gray-200 dark:divide-gray-700">
                  <tr>
                    <td className="px-6 py-4 text-sm text-gray-900 dark:text-white">2026-01</td>
                    <td className="px-6 py-4 text-sm text-green-600 dark:text-green-400">Inflow</td>
                    <td className="px-6 py-4 text-sm text-right text-gray-900 dark:text-white">{formatUSD(8200)}</td>
                  </tr>
                  <tr>
                    <td className="px-6 py-4 text-sm text-gray-900 dark:text-white">2026-02</td>
                    <td className="px-6 py-4 text-sm text-green-600 dark:text-green-400">Inflow</td>
                    <td className="px-6 py-4 text-sm text-right text-gray-900 dark:text-white">{formatUSD(7800)}</td>
                  </tr>
                  <tr>
                    <td className="px-6 py-4 text-sm text-gray-900 dark:text-white">2026-03</td>
                    <td className="px-6 py-4 text-sm text-green-600 dark:text-green-400">Inflow</td>
                    <td className="px-6 py-4 text-sm text-right text-gray-900 dark:text-white">{formatUSD(8500)}</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      )}
      {ctrl.activeTab === "users" && (
        <div className="space-y-6">
          <div className="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 overflow-hidden">
            <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
              <thead className="bg-gray-50 dark:bg-gray-900">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    {ctrl.t.userId}
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    {ctrl.t.pointsBalance}
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    {ctrl.t.tokenBalance}
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    {ctrl.t.lastActive}
                  </th>
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-200 dark:divide-gray-700">
                {[
                  { id: "USER-001", points: 2450, tokens: 24.5, date: "2026-03-29" },
                  { id: "USER-002", points: 1890, tokens: 18.9, date: "2026-03-28" },
                  { id: "USER-003", points: 1230, tokens: 12.3, date: "2026-03-27" },
                  { id: "USER-004", points: 980, tokens: 9.8, date: "2026-03-25" },
                  { id: "USER-005", points: 450, tokens: 4.5, date: "2026-03-20" },
                ].map((user) => (
                  <tr key={user.id}>
                    <td className="px-6 py-4 text-sm font-medium text-gray-900 dark:text-white">{user.id}</td>
                    <td className="px-6 py-4 text-sm text-gray-900 dark:text-white">{formatNumber(user.points)} pts</td>
                    <td className="px-6 py-4 text-sm text-gray-900 dark:text-white">{user.tokens} tokens</td>
                    <td className="px-6 py-4 text-sm text-gray-500 dark:text-gray-400">{user.date}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
          <p className="text-sm text-gray-500 dark:text-gray-400">5 active users</p>
        </div>
      )}
      {ctrl.activeTab === "audit" && (
        <div className="space-y-6">
          <div className="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 overflow-hidden">
            <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
              <thead className="bg-gray-50 dark:bg-gray-900">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    {ctrl.t.date}
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    {ctrl.t.action}
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    {ctrl.t.amount}
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                    {ctrl.t.auditDescription}
                  </th>
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-200 dark:divide-gray-700">
                {[
                  { date: "2026-03-29", action: "Mint", amount: "+5,000 LMT", desc: "Monthly challenge rewards", color: "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300" },
                  { date: "2026-03-28", action: "Award", amount: "+2,450 pts", desc: "Purchase reward - USER-001", color: "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300" },
                  { date: "2026-03-27", action: "Award", amount: "+1,890 pts", desc: "Purchase reward - USER-002", color: "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300" },
                  { date: "2026-03-25", action: "Burn", amount: "-500 LMT", desc: "Expired tokens cleanup", color: "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300" },
                  { date: "2026-03-20", action: "Mint", amount: "+3,000 LMT", desc: "Partnership allocation", color: "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300" },
                ].map((entry, i) => (
                  <tr key={i}>
                    <td className="px-6 py-4 text-sm text-gray-900 dark:text-white">{entry.date}</td>
                    <td className="px-6 py-4 text-sm">
                      <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${entry.color}`}>
                        {entry.action}
                      </span>
                    </td>
                    <td className="px-6 py-4 text-sm font-mono text-gray-900 dark:text-white">{entry.amount}</td>
                    <td className="px-6 py-4 text-sm text-gray-500 dark:text-gray-400">{entry.desc}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}
      {ctrl.activeTab === "settings" && <SettingsTab t={ctrl.t} />}
    </div>
  );
}
