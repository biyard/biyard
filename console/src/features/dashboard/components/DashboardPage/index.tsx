import {
  getEvents,
  formatUSD,
  formatNumber,
} from "@/lib/mock-data";
import {
  ResponsiveContainer,
  LineChart,
  Line,
  AreaChart,
  Area,
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
} from "recharts";

const tokenPriceHistory = [
  { month: "Sep", price: 0.0082 },
  { month: "Oct", price: 0.0098 },
  { month: "Nov", price: 0.0124 },
  { month: "Dec", price: 0.0156 },
  { month: "Jan", price: 0.0189 },
  { month: "Feb", price: 0.0218 },
  { month: "Mar", price: 0.0245 },
];

const treasuryTrend = [
  { month: "Sep", treasury: 4800 },
  { month: "Oct", treasury: 8200 },
  { month: "Nov", treasury: 12500 },
  { month: "Dec", treasury: 18900 },
  { month: "Jan", treasury: 28400 },
  { month: "Feb", treasury: 42100 },
  { month: "Mar", treasury: 72600 },
];

const apiUsage = [
  { day: "Mon", calls: 4120 },
  { day: "Tue", calls: 5340 },
  { day: "Wed", calls: 4890 },
  { day: "Thu", calls: 6210 },
  { day: "Fri", calls: 5780 },
  { day: "Sat", calls: 3950 },
  { day: "Sun", calls: 4231 },
];

export function DashboardPage() {
  const events = getEvents();

  return (
    <div>
      <div className="mb-6">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          Dashboard
        </h1>
        <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
          Token market overview and platform metrics
        </p>
      </div>

      {/* Token Market KPIs */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
          <p className="text-sm text-gray-500 dark:text-gray-400">Token Floor Price</p>
          <p className="mt-1 text-2xl font-bold text-gray-900 dark:text-white">$0.0245</p>
          <p className="mt-0.5 text-xs text-green-600 dark:text-green-400 font-medium">▲ 198.7% (6M)</p>
        </div>
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
          <p className="text-sm text-gray-500 dark:text-gray-400">Market Cap</p>
          <p className="mt-1 text-2xl font-bold text-gray-900 dark:text-white">{formatUSD(24500)}</p>
          <p className="mt-0.5 text-xs text-gray-400 dark:text-gray-500">Treasury-backed</p>
        </div>
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
          <p className="text-sm text-gray-500 dark:text-gray-400">Token Holders</p>
          <p className="mt-1 text-2xl font-bold text-gray-900 dark:text-white">{formatNumber(4200)}</p>
          <p className="mt-0.5 text-xs text-green-600 dark:text-green-400 font-medium">▲ 12% this month</p>
        </div>
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
          <p className="text-sm text-gray-500 dark:text-gray-400">API Calls</p>
          <p className="mt-1 text-2xl font-bold text-gray-900 dark:text-white">{formatNumber(34521)}</p>
          <p className="mt-0.5 text-xs text-gray-400 dark:text-gray-500">This month</p>
        </div>
      </div>

      {/* Token Price + Treasury Charts */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300">Token Floor Price</h3>
            <span className="text-xs px-2 py-0.5 rounded-full bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400 font-medium">All-time high</span>
          </div>
          <ResponsiveContainer width="100%" height={280}>
            <AreaChart data={tokenPriceHistory}>
              <defs>
                <linearGradient id="priceGrad" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="5%" stopColor="#10b981" stopOpacity={0.3} />
                  <stop offset="95%" stopColor="#10b981" stopOpacity={0} />
                </linearGradient>
              </defs>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="month" />
              <YAxis tickFormatter={(v) => `$${v}`} />
              <Tooltip formatter={(v) => [`$${Number(v).toFixed(4)}`, "Floor Price"]} />
              <Area type="monotone" dataKey="price" stroke="#10b981" strokeWidth={2} fill="url(#priceGrad)" />
            </AreaChart>
          </ResponsiveContainer>
        </div>

        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
          <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-4">Treasury Growth (USD)</h3>
          <ResponsiveContainer width="100%" height={280}>
            <LineChart data={treasuryTrend}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="month" />
              <YAxis />
              <Tooltip formatter={(v) => [formatUSD(Number(v)), "Treasury"]} />
              <Line type="monotone" dataKey="treasury" stroke="#3b82f6" strokeWidth={2} dot={{ r: 4 }} />
            </LineChart>
          </ResponsiveContainer>
        </div>
      </div>

      {/* Holder Economics + API Usage */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
        <div className="bg-gradient-to-br from-indigo-50 to-purple-50 dark:from-indigo-900/20 dark:to-purple-900/20 border border-indigo-200 dark:border-indigo-800 rounded-lg p-5">
          <h3 className="text-sm font-semibold text-indigo-800 dark:text-indigo-300 mb-4">Token Holder Economics</h3>
          <div className="space-y-3">
            <div className="flex justify-between"><span className="text-sm text-gray-600 dark:text-gray-400">Total Supply</span><span className="text-sm font-bold text-gray-900 dark:text-white">1,000,000 LMT</span></div>
            <div className="flex justify-between"><span className="text-sm text-gray-600 dark:text-gray-400">Circulating Supply</span><span className="text-sm font-bold text-gray-900 dark:text-white">45,000 LMT (4.5%)</span></div>
            <div className="flex justify-between"><span className="text-sm text-gray-600 dark:text-gray-400">Avg. Holding per User</span><span className="text-sm font-bold text-gray-900 dark:text-white">10.7 LMT</span></div>
            <div className="flex justify-between"><span className="text-sm text-gray-600 dark:text-gray-400">Holder Growth (30d)</span><span className="text-sm font-bold text-green-600 dark:text-green-400">+512 holders</span></div>
            <hr className="border-indigo-200 dark:border-indigo-700" />
            <p className="text-xs text-indigo-700 dark:text-indigo-400">
              Every purchase increases your treasury → raises floor price → benefits all token holders.
              Your holders are shareholders who grow your brand organically.
            </p>
          </div>
        </div>

        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
          <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-4">API Usage (Last 7 Days)</h3>
          <ResponsiveContainer width="100%" height={200}>
            <BarChart data={apiUsage}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="day" />
              <YAxis />
              <Tooltip />
              <Bar dataKey="calls" fill="#8b5cf6" name="API Calls" radius={[4, 4, 0, 0]} />
            </BarChart>
          </ResponsiveContainer>
        </div>
      </div>

      {/* Recent Activity */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
        <h2 className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-4">Recent Activity</h2>
        <ul className="space-y-3">
          {events.map((event, i) => (
            <li key={i} className="flex items-start gap-3 text-sm">
              <span className="text-lg leading-5 flex-shrink-0">{event.icon}</span>
              <div className="flex-1 min-w-0">
                <p className="text-gray-900 dark:text-white truncate">{event.text}</p>
                <p className="text-xs text-gray-400 dark:text-gray-500">{new Date(event.created_at).toLocaleDateString()}</p>
              </div>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}
