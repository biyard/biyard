import { formatNumber } from "@/lib/mock-data";
import {
  ResponsiveContainer,
  AreaChart,
  Area,
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
} from "recharts";

// Token price history (floor price trend - like a stock chart)
const priceHistory = [
  { date: "Sep", price: 0.0082 },
  { date: "Oct", price: 0.0098 },
  { date: "Nov", price: 0.0124 },
  { date: "Dec", price: 0.0156 },
  { date: "Jan", price: 0.0189 },
  { date: "Feb", price: 0.0218 },
  { date: "Mar", price: 0.0245 },
];

const weeklySteps = [
  { day: "Mon", steps: 4200 },
  { day: "Tue", steps: 6800 },
  { day: "Wed", steps: 5100 },
  { day: "Thu", steps: 8500 },
  { day: "Fri", steps: 3200 },
  { day: "Sat", steps: 2400 },
  { day: "Sun", steps: 2200 },
];

const holdings = [
  {
    token: "LMT",
    brand: "Le Mouton",
    amount: 8.5,
    price: 0.0245,
    change24h: +3.2,
    value: 8.5 * 0.0245,
  },
  {
    token: "CBT",
    brand: "Cafe Blossom",
    amount: 3.24,
    price: 0.0246,
    change24h: +1.8,
    value: 3.24 * 0.0246,
  },
  {
    token: "RPT",
    brand: "RunPulse",
    amount: 1.5,
    price: 0.0179,
    change24h: +5.4,
    value: 1.5 * 0.0179,
  },
];

const totalValue = holdings.reduce((s, h) => s + h.value, 0);
const totalValueKRW = totalValue * 1200;
const portfolioChange = +3.1; // % change this month

const recentTx = [
  { icon: "📈", text: "LMT 토큰 가치 +3.2% 상승", time: "1시간 전", type: "gain" },
  { icon: "🏃", text: "걷기 챌린지 → 85 포인트 획득", time: "2시간 전", type: "earn" },
  { icon: "🔄", text: "200 포인트 → 2 LMT 전환", time: "어제", type: "convert" },
  { icon: "🛍️", text: "Le Mouton 구매 → 2,580원 적립", time: "3일 전", type: "purchase" },
  { icon: "📈", text: "RPT 토큰 가치 +5.4% 상승", time: "3일 전", type: "gain" },
];

function fUSD(n: number) {
  return "$" + n.toFixed(4);
}

export function UserDashboardPage() {
  return (
    <div>
      {/* Header */}
      <div className="mb-6">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          My Portfolio
        </h1>
        <p className="mt-1 text-gray-600 dark:text-gray-400">
          Your token holdings and activity
        </p>
      </div>

      {/* Portfolio Value - Hero Card */}
      <div className="bg-gradient-to-br from-indigo-600 to-purple-700 rounded-2xl p-6 mb-6 text-white">
        <p className="text-sm text-indigo-200">Total Portfolio Value</p>
        <div className="flex items-end gap-3 mt-1">
          <span className="text-4xl font-extrabold">
            ${totalValue.toFixed(2)}
          </span>
          <span className="text-lg text-indigo-200 pb-0.5">
            ≈ ₩{formatNumber(Math.round(totalValueKRW))}
          </span>
        </div>
        <div className="mt-2 flex items-center gap-2">
          <span className="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-semibold bg-green-400/20 text-green-200">
            ▲ {portfolioChange}% this month
          </span>
          <span className="text-xs text-indigo-300">
            Holding {holdings.length} tokens across {holdings.length} brands
          </span>
        </div>
      </div>

      {/* Token Holdings Table */}
      <div className="bg-white dark:bg-gray-800 rounded-xl shadow mb-6 overflow-hidden">
        <div className="px-5 py-4 border-b border-gray-200 dark:border-gray-700">
          <h2 className="text-sm font-semibold text-gray-700 dark:text-gray-300">
            My Holdings
          </h2>
        </div>
        <div className="overflow-x-auto">
          <table className="w-full text-sm">
            <thead>
              <tr className="text-left text-xs text-gray-500 dark:text-gray-400 border-b border-gray-100 dark:border-gray-700">
                <th className="px-5 py-3 font-medium">Token</th>
                <th className="px-5 py-3 font-medium text-right">Price</th>
                <th className="px-5 py-3 font-medium text-right">24h</th>
                <th className="px-5 py-3 font-medium text-right">Holdings</th>
                <th className="px-5 py-3 font-medium text-right">Value</th>
              </tr>
            </thead>
            <tbody>
              {holdings.map((h) => (
                <tr
                  key={h.token}
                  className="border-b border-gray-50 dark:border-gray-700/50 last:border-0 hover:bg-gray-50 dark:hover:bg-gray-700/30"
                >
                  <td className="px-5 py-4">
                    <div>
                      <span className="font-semibold text-gray-900 dark:text-white">
                        {h.token}
                      </span>
                      <span className="ml-2 text-xs text-gray-400">
                        {h.brand}
                      </span>
                    </div>
                  </td>
                  <td className="px-5 py-4 text-right font-mono text-gray-900 dark:text-white">
                    {fUSD(h.price)}
                  </td>
                  <td className="px-5 py-4 text-right">
                    <span className="text-green-600 dark:text-green-400 font-medium">
                      +{h.change24h}%
                    </span>
                  </td>
                  <td className="px-5 py-4 text-right text-gray-900 dark:text-white font-medium">
                    {formatNumber(h.amount)}
                  </td>
                  <td className="px-5 py-4 text-right font-semibold text-gray-900 dark:text-white">
                    ${h.value.toFixed(4)}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      {/* Charts Row */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
        {/* Token Price Chart (like stock chart) */}
        <div className="bg-white dark:bg-gray-800 rounded-xl shadow p-5">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300">
              LMT Token Price
            </h3>
            <span className="text-xs text-green-600 dark:text-green-400 font-medium">
              +198.7% (6M)
            </span>
          </div>
          <ResponsiveContainer width="100%" height={220}>
            <AreaChart data={priceHistory}>
              <defs>
                <linearGradient id="priceGradient" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="5%" stopColor="#6366f1" stopOpacity={0.3} />
                  <stop offset="95%" stopColor="#6366f1" stopOpacity={0} />
                </linearGradient>
              </defs>
              <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />
              <XAxis dataKey="date" tick={{ fontSize: 12 }} />
              <YAxis tick={{ fontSize: 12 }} tickFormatter={(v) => `$${v}`} />
              <Tooltip formatter={(v) => [`$${Number(v).toFixed(4)}`, "Price"]} />
              <Area
                type="monotone"
                dataKey="price"
                stroke="#6366f1"
                strokeWidth={2}
                fill="url(#priceGradient)"
              />
            </AreaChart>
          </ResponsiveContainer>
        </div>

        {/* Weekly Steps */}
        <div className="bg-white dark:bg-gray-800 rounded-xl shadow p-5">
          <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-4">
            This Week's Activity
          </h3>
          <ResponsiveContainer width="100%" height={220}>
            <BarChart data={weeklySteps}>
              <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />
              <XAxis dataKey="day" tick={{ fontSize: 12 }} />
              <YAxis tick={{ fontSize: 12 }} />
              <Tooltip />
              <Bar
                dataKey="steps"
                fill="#10b981"
                name="Steps"
                radius={[4, 4, 0, 0]}
              />
            </BarChart>
          </ResponsiveContainer>
        </div>
      </div>

      {/* How Your Value Grows - Educational Card */}
      <div className="bg-gradient-to-r from-blue-50 to-indigo-50 dark:from-blue-900/20 dark:to-indigo-900/20 border border-blue-200 dark:border-blue-800 rounded-xl p-5 mb-6">
        <h3 className="text-sm font-semibold text-blue-800 dark:text-blue-300 mb-3">
          How Your Token Value Grows
        </h3>
        <div className="grid grid-cols-1 sm:grid-cols-4 gap-3 text-center text-xs">
          <div className="bg-white/60 dark:bg-gray-800/60 rounded-lg p-3">
            <div className="text-lg mb-1">🛍️</div>
            <p className="font-medium text-gray-700 dark:text-gray-300">More Purchases</p>
            <p className="text-gray-500 dark:text-gray-400">by all customers</p>
          </div>
          <div className="bg-white/60 dark:bg-gray-800/60 rounded-lg p-3">
            <div className="text-lg mb-1">🏦</div>
            <p className="font-medium text-gray-700 dark:text-gray-300">Treasury Grows</p>
            <p className="text-gray-500 dark:text-gray-400">revenue backs tokens</p>
          </div>
          <div className="bg-white/60 dark:bg-gray-800/60 rounded-lg p-3">
            <div className="text-lg mb-1">📈</div>
            <p className="font-medium text-gray-700 dark:text-gray-300">Floor Price ↑</p>
            <p className="text-gray-500 dark:text-gray-400">token value rises</p>
          </div>
          <div className="bg-white/60 dark:bg-gray-800/60 rounded-lg p-3">
            <div className="text-lg mb-1">💰</div>
            <p className="font-medium text-gray-700 dark:text-gray-300">You Benefit</p>
            <p className="text-gray-500 dark:text-gray-400">as a token holder</p>
          </div>
        </div>
      </div>

      {/* Recent Activity */}
      <div className="bg-white dark:bg-gray-800 rounded-xl shadow p-5">
        <h2 className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-4">
          Recent Activity
        </h2>
        <ul className="space-y-3">
          {recentTx.map((item, i) => (
            <li key={i} className="flex items-start gap-3">
              <span className="text-lg leading-5 flex-shrink-0">{item.icon}</span>
              <div className="flex-1 min-w-0">
                <p className="text-sm text-gray-900 dark:text-white">{item.text}</p>
                <p className="text-xs text-gray-400 dark:text-gray-500">{item.time}</p>
              </div>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}
