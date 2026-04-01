import { useState, useMemo } from "react";
import {
  ResponsiveContainer,
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
} from "recharts";
import { formatUSD, formatKRW, formatNumber } from "@/lib/mock-data";

export function ROISimulatorPage() {
  const [monthlyRevenue, setMonthlyRevenue] = useState(100_000_000);
  const [sharePercent, setSharePercent] = useState(3);
  const [totalSupply, setTotalSupply] = useState(1_000_000);
  const [months, setMonths] = useState(12);

  const projectedTreasury = useMemo(
    () => (monthlyRevenue * sharePercent) / 100 / 1200 * months,
    [monthlyRevenue, sharePercent, months]
  );

  const floorPrice = useMemo(
    () => projectedTreasury / totalSupply,
    [projectedTreasury, totalSupply]
  );

  const month1Treasury = useMemo(
    () => (monthlyRevenue * sharePercent) / 100 / 1200,
    [monthlyRevenue, sharePercent]
  );

  const month1FloorPrice = useMemo(
    () => month1Treasury / totalSupply,
    [month1Treasury, totalSupply]
  );

  const floorPriceGrowth = useMemo(() => {
    if (month1FloorPrice === 0) return 0;
    return ((floorPrice - month1FloorPrice) / month1FloorPrice) * 100;
  }, [floorPrice, month1FloorPrice]);

  const chartData = useMemo(() => {
    const data = [];
    const monthlyTreasury = (monthlyRevenue * sharePercent) / 100 / 1200;
    for (let i = 1; i <= months; i++) {
      const cumTreasury = monthlyTreasury * i;
      data.push({
        month: `M${i}`,
        treasury: Math.round(cumTreasury * 100) / 100,
        floorPrice:
          Math.round((cumTreasury / totalSupply) * 1_000_000) / 1_000_000,
      });
    }
    return data;
  }, [monthlyRevenue, sharePercent, totalSupply, months]);

  return (
    <div>
      <div className="mb-8 text-center">
        <h1 className="text-4xl font-bold text-gray-900 dark:text-white">
          ROI Simulator
        </h1>
        <p className="mt-2 text-lg text-gray-600 dark:text-gray-400">
          See how revenue-backed tokens create value for your brand.
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        {/* Input Form */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
          <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-6">
            Parameters
          </h2>

          <div className="space-y-6">
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Monthly Revenue (KRW)
              </label>
              <input
                type="number"
                value={monthlyRevenue}
                onChange={(e) =>
                  setMonthlyRevenue(Number(e.target.value) || 0)
                }
                className="w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 px-3 py-2 text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
              <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                {formatKRW(monthlyRevenue)} KRW
              </p>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Treasury Share: {sharePercent}%
              </label>
              <input
                type="range"
                min={1}
                max={10}
                step={0.5}
                value={sharePercent}
                onChange={(e) => setSharePercent(Number(e.target.value))}
                className="w-full accent-blue-500"
              />
              <div className="flex justify-between text-xs text-gray-400">
                <span>1%</span>
                <span>10%</span>
              </div>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Token Total Supply
              </label>
              <input
                type="number"
                value={totalSupply}
                onChange={(e) =>
                  setTotalSupply(Number(e.target.value) || 1)
                }
                className="w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 px-3 py-2 text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
              <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                {formatNumber(totalSupply)} tokens
              </p>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Projection Period: {months} months
              </label>
              <input
                type="range"
                min={3}
                max={24}
                step={1}
                value={months}
                onChange={(e) => setMonths(Number(e.target.value))}
                className="w-full accent-blue-500"
              />
              <div className="flex justify-between text-xs text-gray-400">
                <span>3 months</span>
                <span>24 months</span>
              </div>
            </div>
          </div>
        </div>

        {/* Results */}
        <div className="space-y-6">
          <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
            <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
              <p className="text-sm text-gray-500 dark:text-gray-400">
                Projected Treasury
              </p>
              <p className="mt-1 text-2xl font-bold text-blue-600 dark:text-blue-400">
                {formatUSD(projectedTreasury)}
              </p>
              <p className="mt-0.5 text-xs text-gray-400">
                After {months} months
              </p>
            </div>

            <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
              <p className="text-sm text-gray-500 dark:text-gray-400">
                Floor Price
              </p>
              <p className="mt-1 text-2xl font-bold text-emerald-600 dark:text-emerald-400">
                {formatUSD(floorPrice)}
              </p>
              <p className="mt-0.5 text-xs text-gray-400">Per token</p>
            </div>

            <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
              <p className="text-sm text-gray-500 dark:text-gray-400">
                Floor Price Growth
              </p>
              <p className="mt-1 text-2xl font-bold text-purple-600 dark:text-purple-400">
                {floorPriceGrowth.toFixed(1)}%
              </p>
              <p className="mt-0.5 text-xs text-gray-400">vs Month 1</p>
            </div>
          </div>

          <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
            <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-4">
              Treasury Growth Projection
            </h3>
            <ResponsiveContainer width="100%" height={300}>
              <LineChart data={chartData}>
                <CartesianGrid strokeDasharray="3 3" />
                <XAxis dataKey="month" />
                <YAxis />
                <Tooltip
                  formatter={(value) => formatUSD(Number(value))}
                />
                <Line
                  type="monotone"
                  dataKey="treasury"
                  stroke="#3b82f6"
                  strokeWidth={2}
                  dot={{ r: 3 }}
                  name="Treasury (USD)"
                />
              </LineChart>
            </ResponsiveContainer>
          </div>
        </div>
      </div>
    </div>
  );
}
