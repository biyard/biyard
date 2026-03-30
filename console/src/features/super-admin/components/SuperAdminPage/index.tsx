import { useState } from "react";
import { Search, Send, FileText, Terminal } from "lucide-react";
import { formatNumber, formatUSD } from "@/lib/mock-data";
import { toast } from "sonner";

type PlanFilter = "all" | "Free" | "Growth" | "Enterprise";

interface Company {
  name: string;
  plan: "Free" | "Growth" | "Enterprise";
  projects: number;
  apiCalls: number;
  revenue: number;
  status: "active" | "trial" | "suspended";
  joined: string;
}

const companies: Company[] = [
  { name: "Le Mouton", plan: "Enterprise", projects: 3, apiCalls: 245000, revenue: 4500, status: "active", joined: "2025-08-15" },
  { name: "Cafe Blossom", plan: "Growth", projects: 2, apiCalls: 128000, revenue: 299, status: "active", joined: "2025-09-01" },
  { name: "RunPulse", plan: "Enterprise", projects: 5, apiCalls: 520000, revenue: 8200, status: "active", joined: "2025-07-20" },
  { name: "GreenWalk", plan: "Growth", projects: 1, apiCalls: 45000, revenue: 299, status: "active", joined: "2026-01-10" },
  { name: "FitLife Korea", plan: "Enterprise", projects: 4, apiCalls: 380000, revenue: 6800, status: "active", joined: "2025-10-05" },
  { name: "Seoul Bakery", plan: "Free", projects: 1, apiCalls: 800, revenue: 0, status: "active", joined: "2026-03-01" },
  { name: "K-Fashion Hub", plan: "Growth", projects: 2, apiCalls: 92000, revenue: 299, status: "active", joined: "2025-11-15" },
  { name: "HealthPlus", plan: "Growth", projects: 1, apiCalls: 67000, revenue: 299, status: "trial", joined: "2026-02-20" },
  { name: "TechGym", plan: "Free", projects: 1, apiCalls: 500, revenue: 0, status: "active", joined: "2026-03-15" },
  { name: "Brew Masters", plan: "Growth", projects: 2, apiCalls: 156000, revenue: 299, status: "suspended", joined: "2025-12-01" },
];

const PLAN_BADGE: Record<Company["plan"], string> = {
  Free: "bg-gray-100 text-gray-700 dark:bg-gray-700 dark:text-gray-300",
  Growth: "bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400",
  Enterprise: "bg-purple-100 text-purple-700 dark:bg-purple-900/30 dark:text-purple-400",
};

const STATUS_BADGE: Record<Company["status"], string> = {
  active: "bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400",
  trial: "bg-yellow-100 text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-400",
  suspended: "bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-400",
};

function formatApiCalls(n: number): string {
  if (n >= 1_000_000) return (n / 1_000_000).toFixed(1) + "M";
  if (n >= 1_000) return (n / 1_000).toFixed(1) + "K";
  return String(n);
}

export function SuperAdminPage() {
  const [search, setSearch] = useState("");
  const [planFilter, setPlanFilter] = useState<PlanFilter>("all");
  const [showAnnouncement, setShowAnnouncement] = useState(false);
  const [announcementText, setAnnouncementText] = useState("");
  const [showReport, setShowReport] = useState(false);
  const [reportMonth, setReportMonth] = useState("2026-03");
  const [showLogs, setShowLogs] = useState(false);

  const filteredCompanies = companies.filter((c) => {
    const matchesSearch =
      search === "" ||
      c.name.toLowerCase().includes(search.toLowerCase());
    const matchesPlan =
      planFilter === "all" || c.plan === planFilter;
    return matchesSearch && matchesPlan;
  });

  return (
    <div>
      <div className="mb-6">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          Super Admin
        </h1>
        <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
          Platform-wide management and tenant overview
        </p>
      </div>

      {/* Overview KPI Cards */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
          <p className="text-sm text-gray-500 dark:text-gray-400">Total Companies</p>
          <p className="mt-1 text-2xl font-bold text-gray-900 dark:text-white">127</p>
          <p className="mt-0.5 text-xs text-green-600 dark:text-green-400">+12 this month</p>
        </div>
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
          <p className="text-sm text-gray-500 dark:text-gray-400">Active Projects</p>
          <p className="mt-1 text-2xl font-bold text-gray-900 dark:text-white">342</p>
          <p className="mt-0.5 text-xs text-green-600 dark:text-green-400">+28 this month</p>
        </div>
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
          <p className="text-sm text-gray-500 dark:text-gray-400">Total API Calls (This Month)</p>
          <p className="mt-1 text-2xl font-bold text-gray-900 dark:text-white">2.4M</p>
          <p className="mt-0.5 text-xs text-green-600 dark:text-green-400">+18% vs last month</p>
        </div>
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
          <p className="text-sm text-gray-500 dark:text-gray-400">Monthly Revenue</p>
          <p className="mt-1 text-2xl font-bold text-gray-900 dark:text-white">$38,700</p>
          <p className="mt-0.5 text-xs text-green-600 dark:text-green-400">+8% vs last month</p>
        </div>
      </div>

      {/* Companies Table */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5 mb-6">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          Companies
        </h2>

        {/* Search & Filter */}
        <div className="flex flex-col sm:flex-row gap-4 mb-4">
          <div className="relative flex-1">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-gray-400" />
            <input
              type="text"
              placeholder="Search companies..."
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              className="w-full pl-10 pr-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>
          <div className="flex gap-1 rounded-lg border border-gray-300 dark:border-gray-600 p-1">
            {(["all", "Free", "Growth", "Enterprise"] as const).map((p) => (
              <button
                key={p}
                onClick={() => setPlanFilter(p)}
                className={`px-3 py-1.5 text-sm font-medium rounded-md transition-colors ${
                  planFilter === p
                    ? "bg-blue-600 text-white"
                    : "text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700"
                }`}
              >
                {p === "all" ? "All" : p}
              </button>
            ))}
          </div>
        </div>

        {/* Table */}
        <div className="overflow-x-auto">
          <table className="w-full text-sm">
            <thead>
              <tr className="text-left text-gray-500 dark:text-gray-400 border-b border-gray-200 dark:border-gray-700">
                <th className="pb-2 font-medium">Company Name</th>
                <th className="pb-2 font-medium">Plan</th>
                <th className="pb-2 font-medium text-right">Projects</th>
                <th className="pb-2 font-medium text-right">API Calls (30d)</th>
                <th className="pb-2 font-medium text-right">Revenue</th>
                <th className="pb-2 font-medium">Status</th>
                <th className="pb-2 font-medium">Joined</th>
                <th className="pb-2 font-medium text-right">Actions</th>
              </tr>
            </thead>
            <tbody>
              {filteredCompanies.map((company) => (
                <tr
                  key={company.name}
                  className="border-b border-gray-100 dark:border-gray-700/50 last:border-0"
                >
                  <td className="py-3 font-medium text-gray-900 dark:text-white">
                    {company.name}
                  </td>
                  <td className="py-3">
                    <span
                      className={`inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium ${PLAN_BADGE[company.plan]}`}
                    >
                      {company.plan}
                    </span>
                  </td>
                  <td className="py-3 text-right text-gray-900 dark:text-white">
                    {formatNumber(company.projects)}
                  </td>
                  <td className="py-3 text-right text-gray-900 dark:text-white">
                    {formatApiCalls(company.apiCalls)}
                  </td>
                  <td className="py-3 text-right text-gray-900 dark:text-white">
                    {formatUSD(company.revenue)}
                  </td>
                  <td className="py-3">
                    <span
                      className={`inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium capitalize ${STATUS_BADGE[company.status]}`}
                    >
                      {company.status}
                    </span>
                  </td>
                  <td className="py-3 text-gray-500 dark:text-gray-400">
                    {new Date(company.joined).toLocaleDateString("en-US")}
                  </td>
                  <td className="py-3 text-right">
                    <button className="px-3 py-1 text-sm font-medium text-blue-600 dark:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-900/20 rounded-md transition-colors">
                      View
                    </button>
                  </td>
                </tr>
              ))}
              {filteredCompanies.length === 0 && (
                <tr>
                  <td
                    colSpan={8}
                    className="py-8 text-center text-gray-400 dark:text-gray-500"
                  >
                    No companies found matching your criteria.
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        </div>
      </div>

      {/* Platform Health + Quick Actions */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Platform Health */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
          <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
            Platform Health
          </h2>
          <div className="grid grid-cols-2 gap-4">
            <div className="bg-gray-50 dark:bg-gray-900 rounded-lg p-4">
              <p className="text-xs text-gray-500 dark:text-gray-400 mb-1">API Avg Response Time</p>
              <p className="text-xl font-bold text-green-600 dark:text-green-400">42ms</p>
              <p className="text-xs text-green-600 dark:text-green-400 mt-0.5">Healthy</p>
            </div>
            <div className="bg-gray-50 dark:bg-gray-900 rounded-lg p-4">
              <p className="text-xs text-gray-500 dark:text-gray-400 mb-1">Error Rate</p>
              <p className="text-xl font-bold text-green-600 dark:text-green-400">0.12%</p>
              <p className="text-xs text-green-600 dark:text-green-400 mt-0.5">Below threshold</p>
            </div>
            <div className="bg-gray-50 dark:bg-gray-900 rounded-lg p-4">
              <p className="text-xs text-gray-500 dark:text-gray-400 mb-1">Uptime</p>
              <p className="text-xl font-bold text-green-600 dark:text-green-400">99.98%</p>
              <p className="text-xs text-green-600 dark:text-green-400 mt-0.5">Last 30 days</p>
            </div>
            <div className="bg-gray-50 dark:bg-gray-900 rounded-lg p-4">
              <p className="text-xs text-gray-500 dark:text-gray-400 mb-1">Active WebSocket Connections</p>
              <p className="text-xl font-bold text-gray-900 dark:text-white">{formatNumber(1247)}</p>
              <p className="text-xs text-gray-500 dark:text-gray-400 mt-0.5">Real-time</p>
            </div>
          </div>
        </div>

        {/* Quick Actions */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
          <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
            Quick Actions
          </h2>
          <div className="space-y-3">
            <div>
              <button
                onClick={() => setShowAnnouncement(!showAnnouncement)}
                className="w-full flex items-center gap-3 px-4 py-3 rounded-lg border border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors text-left"
              >
                <Send className="h-5 w-5 text-blue-600 dark:text-blue-400 flex-shrink-0" />
                <div>
                  <p className="text-sm font-medium text-gray-900 dark:text-white">
                    Send Platform Announcement
                  </p>
                  <p className="text-xs text-gray-500 dark:text-gray-400">
                    Broadcast a message to all tenant companies
                  </p>
                </div>
              </button>
              {showAnnouncement && (
                <div className="mt-2 px-4 pb-3 space-y-2">
                  <textarea
                    value={announcementText}
                    onChange={(e) => setAnnouncementText(e.target.value)}
                    placeholder="Type your announcement..."
                    rows={3}
                    className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none"
                  />
                  <button
                    onClick={() => {
                      if (!announcementText.trim()) {
                        toast.error("Please enter an announcement message");
                        return;
                      }
                      toast.success("Announcement sent to all tenants");
                      setAnnouncementText("");
                      setShowAnnouncement(false);
                    }}
                    className="px-4 py-1.5 bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium rounded-lg transition-colors"
                  >
                    Send
                  </button>
                </div>
              )}
            </div>
            <div>
              <button
                onClick={() => setShowReport(!showReport)}
                className="w-full flex items-center gap-3 px-4 py-3 rounded-lg border border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors text-left"
              >
                <FileText className="h-5 w-5 text-green-600 dark:text-green-400 flex-shrink-0" />
                <div>
                  <p className="text-sm font-medium text-gray-900 dark:text-white">
                    Generate Monthly Report
                  </p>
                  <p className="text-xs text-gray-500 dark:text-gray-400">
                    Export platform metrics and revenue summary
                  </p>
                </div>
              </button>
              {showReport && (
                <div className="mt-2 px-4 pb-3 flex items-center gap-3">
                  <select
                    value={reportMonth}
                    onChange={(e) => setReportMonth(e.target.value)}
                    className="px-3 py-1.5 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                  >
                    <option value="2026-03">March 2026</option>
                    <option value="2026-02">February 2026</option>
                    <option value="2026-01">January 2026</option>
                    <option value="2025-12">December 2025</option>
                    <option value="2025-11">November 2025</option>
                    <option value="2025-10">October 2025</option>
                  </select>
                  <button
                    onClick={() => {
                      toast.success(`Report for ${reportMonth} downloading...`);
                      setShowReport(false);
                    }}
                    className="px-4 py-1.5 bg-green-600 hover:bg-green-700 text-white text-sm font-medium rounded-lg transition-colors"
                  >
                    Download
                  </button>
                </div>
              )}
            </div>
            <div>
              <button
                onClick={() => setShowLogs(!showLogs)}
                className="w-full flex items-center gap-3 px-4 py-3 rounded-lg border border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors text-left"
              >
                <Terminal className="h-5 w-5 text-purple-600 dark:text-purple-400 flex-shrink-0" />
                <div>
                  <p className="text-sm font-medium text-gray-900 dark:text-white">
                    View System Logs
                  </p>
                  <p className="text-xs text-gray-500 dark:text-gray-400">
                    Access real-time application and error logs
                  </p>
                </div>
              </button>
              {showLogs && (
                <div className="mt-2 px-4 pb-3">
                  <pre className="bg-gray-900 text-green-400 text-xs font-mono p-4 rounded-lg overflow-x-auto max-h-48 overflow-y-auto leading-relaxed">
{`[2026-03-30 11:42:03] INFO  api::v1::purchases  - Purchase recorded: pur_x1y2z3 amount=15000 user=user_42
[2026-03-30 11:41:58] INFO  api::v1::activities  - Activity logged: act_a1b2c3 type=walking steps=8500
[2026-03-30 11:41:45] WARN  api::middleware::rate_limit - Rate limit approaching for tenant=Le_Mouton (87%)
[2026-03-30 11:41:30] INFO  api::v1::tokens      - Token mint: 100 LYT to user_42 tx=0xabc...def
[2026-03-30 11:41:12] INFO  api::v1::points      - Points awarded: 500 pts to user_42 reason=Referral
[2026-03-30 11:40:55] ERROR api::v1::webhooks    - Webhook delivery failed: hook_id=wh_789 status=timeout
[2026-03-30 11:40:30] INFO  api::health          - Health check OK: latency=42ms uptime=99.98%
[2026-03-30 11:40:01] INFO  api::scheduler       - Cron job completed: daily_points_summary`}
                  </pre>
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
