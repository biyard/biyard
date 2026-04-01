import {
  ResponsiveContainer,
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
} from "recharts";
import { formatNumber } from "@/lib/mock-data";

// Generate 14 days of mock API call data
const dailyApiCalls = Array.from({ length: 14 }, (_, i) => {
  const date = new Date(2026, 2, 17 + i); // March 17-30
  return {
    date: `${date.getMonth() + 1}/${date.getDate()}`,
    calls: Math.floor(Math.random() * 2000 + 1500),
  };
});

const usageMetrics = [
  {
    label: "API Calls",
    current: 34521,
    limit: 50000,
    percent: 69,
    color: "bg-blue-500",
  },
  {
    label: "Active Users",
    current: 2847,
    limit: 5000,
    percent: 57,
    color: "bg-green-500",
  },
  {
    label: "Projects",
    current: 3,
    limit: 5,
    percent: 60,
    color: "bg-purple-500",
  },
  {
    label: "Storage",
    current: 2.1,
    limit: 10,
    percent: 21,
    unit: "GB",
    color: "bg-amber-500",
  },
];

const endpointData = [
  {
    endpoint: "/v1/accounts/*",
    calls: 8234,
    avgResponse: "45ms",
    errorRate: "0.2%",
  },
  {
    endpoint: "/v1/projects/*",
    calls: 5621,
    avgResponse: "62ms",
    errorRate: "0.1%",
  },
  {
    endpoint: "/v1/tokens/*",
    calls: 12456,
    avgResponse: "38ms",
    errorRate: "0.3%",
  },
  {
    endpoint: "/v1/points/*",
    calls: 6890,
    avgResponse: "55ms",
    errorRate: "0.1%",
  },
  {
    endpoint: "/v1/challenges/*",
    calls: 1320,
    avgResponse: "72ms",
    errorRate: "0.5%",
  },
];

export function UsagePage() {
  return (
    <div>
      {/* Header */}
      <div className="mb-6">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          Usage Dashboard
        </h1>
        <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
          Monitor your API usage and consumption metrics.
        </p>
      </div>

      {/* Current Plan Card */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6 mb-6">
        <div className="flex items-center justify-between flex-wrap gap-4">
          <div>
            <p className="text-sm text-gray-500 dark:text-gray-400">
              Current Plan
            </p>
            <h2 className="text-2xl font-bold text-gray-900 dark:text-white">
              Growth
            </h2>
          </div>
          <div className="text-right">
            <p className="text-sm text-gray-500 dark:text-gray-400">
              Billing Period
            </p>
            <p className="text-sm font-medium text-gray-900 dark:text-white">
              Mar 1 - Mar 31, 2026
            </p>
          </div>
          <div className="text-right">
            <p className="text-sm text-gray-500 dark:text-gray-400">
              Next Billing Date
            </p>
            <p className="text-sm font-medium text-gray-900 dark:text-white">
              April 1, 2026
            </p>
          </div>
        </div>
      </div>

      {/* Usage Summary KPI Cards */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
        {usageMetrics.map((metric) => (
          <div
            key={metric.label}
            className="bg-white dark:bg-gray-800 rounded-lg shadow p-5"
          >
            <p className="text-sm text-gray-500 dark:text-gray-400 mb-1">
              {metric.label}
            </p>
            <p className="text-2xl font-bold text-gray-900 dark:text-white">
              {metric.unit
                ? `${metric.current} ${metric.unit}`
                : formatNumber(metric.current)}
            </p>
            <p className="text-xs text-gray-400 dark:text-gray-500 mt-1">
              of{" "}
              {metric.unit
                ? `${metric.limit} ${metric.unit}`
                : formatNumber(metric.limit)}{" "}
              ({metric.percent}%)
            </p>
            <div className="mt-3 w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
              <div
                className={`${metric.color} h-2 rounded-full transition-all`}
                style={{ width: `${metric.percent}%` }}
              />
            </div>
          </div>
        ))}
      </div>

      {/* Daily API Calls Chart */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5 mb-6">
        <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-4">
          Daily API Calls (Last 14 Days)
        </h3>
        <ResponsiveContainer width="100%" height={300}>
          <BarChart data={dailyApiCalls}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="date" />
            <YAxis />
            <Tooltip />
            <Bar
              dataKey="calls"
              fill="#3b82f6"
              name="API Calls"
              radius={[4, 4, 0, 0]}
            />
          </BarChart>
        </ResponsiveContainer>
      </div>

      {/* API Calls by Endpoint Table */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden">
        <div className="px-5 py-4 border-b border-gray-200 dark:border-gray-700">
          <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300">
            API Calls by Endpoint
          </h3>
        </div>
        <div className="overflow-x-auto">
          <table className="w-full text-sm text-left">
            <thead className="bg-gray-50 dark:bg-gray-750 text-gray-500 dark:text-gray-400 uppercase text-xs">
              <tr>
                <th className="px-5 py-3">Endpoint</th>
                <th className="px-5 py-3 text-right">Calls</th>
                <th className="px-5 py-3 text-right">Avg Response Time</th>
                <th className="px-5 py-3 text-right">Error Rate</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-gray-200 dark:divide-gray-700">
              {endpointData.map((row) => (
                <tr
                  key={row.endpoint}
                  className="hover:bg-gray-50 dark:hover:bg-gray-750"
                >
                  <td className="px-5 py-3 font-mono text-gray-900 dark:text-white">
                    {row.endpoint}
                  </td>
                  <td className="px-5 py-3 text-right text-gray-700 dark:text-gray-300">
                    {formatNumber(row.calls)}
                  </td>
                  <td className="px-5 py-3 text-right text-gray-700 dark:text-gray-300">
                    {row.avgResponse}
                  </td>
                  <td className="px-5 py-3 text-right text-gray-700 dark:text-gray-300">
                    {row.errorRate}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}
