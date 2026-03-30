import { UserNav } from "../components/user-nav";

const holdings = [
  {
    token: "LMT",
    brand: "Le Mouton",
    price: 0.0245,
    change: +3.2,
    amount: 8.5,
  },
  {
    token: "CBT",
    brand: "Cafe Blossom",
    price: 0.0246,
    change: +1.8,
    amount: 3.24,
  },
  {
    token: "RPT",
    brand: "RunPulse",
    price: 0.0179,
    change: +5.4,
    amount: 1.5,
  },
];

const activities = [
  { icon: "\u{1F4C8}", text: "LMT \uD1A0\uD070 \uAC00\uCE58 +3.2% \uC0C1\uC2B9", time: "1\uC2DC\uAC04 \uC804" },
  { icon: "\u{1F6CD}\uFE0F", text: "Le Mouton \uAD6C\uB9E4 \u2192 2,580\uC6D0 \uC801\uB9BD", time: "3\uC2DC\uAC04 \uC804" },
  { icon: "\u{1F3C3}", text: "\uAC77\uAE30 \uCC4C\uB9B0\uC9C0 85 \uD3EC\uC778\uD2B8 \uD68D\uB4DD", time: "\uC5B4\uC81C" },
  { icon: "\u{1F504}", text: "200 \uD3EC\uC778\uD2B8 \u2192 2 LMT \uC804\uD658", time: "2\uC77C \uC804" },
  { icon: "\u{1F5F3}\uFE0F", text: "'\uB9AC\uC6CC\uB4DC 2\uBC30' \uC81C\uC548\uC5D0 \uD22C\uD45C \uC644\uB8CC", time: "3\uC77C \uC804" },
];

const totalValue =
  holdings.reduce((sum, h) => sum + h.price * h.amount, 0);
const totalKRW = totalValue * 1380;

// Simple bar chart data (7 days of mock portfolio values)
const chartData = [0.28, 0.26, 0.29, 0.31, 0.27, 0.33, totalValue];
const chartMax = Math.max(...chartData);

export function WalletPage() {
  return (
    <div className="min-h-screen bg-[#0a0e17]">
      <UserNav />

      <div className="max-w-5xl mx-auto px-4 py-8 space-y-6">
        {/* Portfolio Value Hero */}
        <div className="rounded-2xl p-8 bg-gradient-to-r from-[#0f1420] to-[#121a2e] border border-gray-800">
          <p className="text-sm text-gray-400 mb-1">Total Portfolio Value</p>
          <div className="flex items-baseline gap-4">
            <h2 className="text-4xl font-bold text-white">
              ${totalValue.toFixed(4)}
            </h2>
            <span className="text-lg text-gray-400">
              {"\u2248"} {totalKRW.toFixed(0)}\uC6D0
            </span>
          </div>
          <span className="inline-block mt-2 text-sm font-medium text-[#00d4aa]">
            +4.7% (24h)
          </span>
        </div>

        {/* Holdings Table */}
        <div className="bg-[#0f1420] rounded-2xl border border-gray-800 overflow-hidden">
          <div className="px-6 py-4 border-b border-gray-800">
            <h3 className="text-lg font-semibold text-white">Holdings</h3>
          </div>
          <table className="w-full">
            <thead>
              <tr className="text-sm text-gray-500 border-b border-gray-800">
                <th className="text-left px-6 py-3 font-medium">Token</th>
                <th className="text-left px-6 py-3 font-medium">Brand</th>
                <th className="text-right px-6 py-3 font-medium">Price</th>
                <th className="text-right px-6 py-3 font-medium">24h</th>
                <th className="text-right px-6 py-3 font-medium">Holdings</th>
                <th className="text-right px-6 py-3 font-medium">Value</th>
              </tr>
            </thead>
            <tbody>
              {holdings.map((h) => (
                <tr
                  key={h.token}
                  className="border-b border-gray-800/50 hover:bg-[#0a0e17]/50 transition-colors"
                >
                  <td className="px-6 py-4 text-white font-semibold">
                    {h.token}
                  </td>
                  <td className="px-6 py-4 text-gray-400">{h.brand}</td>
                  <td className="px-6 py-4 text-right text-white">
                    ${h.price.toFixed(4)}
                  </td>
                  <td
                    className={`px-6 py-4 text-right font-medium ${
                      h.change >= 0 ? "text-[#00d4aa]" : "text-red-400"
                    }`}
                  >
                    {h.change >= 0 ? "+" : ""}
                    {h.change}%
                  </td>
                  <td className="px-6 py-4 text-right text-white">
                    {h.amount} {h.token}
                  </td>
                  <td className="px-6 py-4 text-right text-white font-medium">
                    ${(h.price * h.amount).toFixed(4)}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>

        {/* Price Chart + Recent Activity */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {/* Simple Bar Chart */}
          <div className="bg-[#0f1420] rounded-2xl border border-gray-800 p-6">
            <h3 className="text-lg font-semibold text-white mb-4">
              7-Day Portfolio
            </h3>
            <div className="flex items-end gap-2 h-40">
              {chartData.map((val, i) => (
                <div key={i} className="flex-1 flex flex-col items-center gap-1">
                  <div
                    className="w-full rounded-t-md bg-gradient-to-t from-blue-600 to-blue-400 transition-all"
                    style={{
                      height: `${(val / chartMax) * 100}%`,
                      minHeight: "4px",
                    }}
                  />
                  <span className="text-[10px] text-gray-500">
                    {["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"][i]}
                  </span>
                </div>
              ))}
            </div>
          </div>

          {/* Recent Activity */}
          <div className="bg-[#0f1420] rounded-2xl border border-gray-800 p-6">
            <h3 className="text-lg font-semibold text-white mb-4">
              Recent Activity
            </h3>
            <div className="space-y-4">
              {activities.map((a, i) => (
                <div key={i} className="flex items-start gap-3">
                  <span className="text-lg">{a.icon}</span>
                  <div className="flex-1 min-w-0">
                    <p className="text-sm text-white">{a.text}</p>
                    <p className="text-xs text-gray-500">{a.time}</p>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
