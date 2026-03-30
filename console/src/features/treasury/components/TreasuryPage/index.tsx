import { useState } from "react";
import { useMode } from "@/contexts/ModeContext";
import { useBrands } from "@/contexts/BrandContext";
import { getTreasuryData, formatUSD, formatKRW } from "@/lib/mock-data";

export function TreasuryPage() {
  const { isAdmin } = useMode();
  const { brands, selectedBrand } = useBrands();
  const [stressLevel, setStressLevel] = useState(10);

  const brand = selectedBrand ?? brands[0] ?? null;

  if (!brand) {
    return (
      <div className="text-center py-12 text-gray-500 dark:text-gray-400">
        No brand data available.
      </div>
    );
  }

  const data = getTreasuryData(brand.id, stressLevel);
  const stressedTreasury = data.treasury_usd;
  const scenarioFloorPrice = data.floor_price;

  return (
    <div>
      <div className="mb-6">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          Treasury
        </h1>
        <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
          {brand.name} &mdash; Treasury overview and analytics.
        </p>
      </div>

      {/* KPI Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
        <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-6 border border-gray-200 dark:border-gray-700">
          <h3 className="text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider mb-1">
            Treasury
          </h3>
          <p className="text-2xl font-bold text-gray-900 dark:text-white">
            {formatUSD(data.rows[data.rows.length - 1]?.treasury_usd ?? 0)}
          </p>
          {isAdmin && (
            <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
              ₩
              {formatKRW(
                (data.rows[data.rows.length - 1]?.treasury_usd ?? 0) * 1200
              )}
            </p>
          )}
        </div>

        <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-6 border border-gray-200 dark:border-gray-700">
          <h3 className="text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider mb-1">
            Floor Price
          </h3>
          <p className="text-2xl font-bold text-gray-900 dark:text-white">
            {formatUSD(data.rows[data.rows.length - 1]?.floor_price ?? 0)}
          </p>
          {isAdmin && (
            <p className="mt-1 text-xs text-gray-500 dark:text-gray-400 font-mono">
              P_floor = Treasury / Total Supply
            </p>
          )}
        </div>
      </div>

      {isAdmin && (
        <>
          {/* Stress Test */}
          <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-6 border border-gray-200 dark:border-gray-700 mb-8">
            <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
              Stress Test
            </h2>
            <div className="space-y-4">
              <div>
                <div className="flex items-center justify-between mb-2">
                  <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
                    Treasury Reduction
                  </label>
                  <span className="text-sm font-bold text-gray-900 dark:text-white">
                    {stressLevel}%
                  </span>
                </div>
                <input
                  type="range"
                  min={0}
                  max={100}
                  value={stressLevel}
                  onChange={(e) => setStressLevel(Number(e.target.value))}
                  className="w-full h-2 rounded-lg appearance-none cursor-pointer accent-blue-600"
                />
                <div className="flex justify-between text-xs text-gray-400 mt-1">
                  <span>0%</span>
                  <span>50%</span>
                  <span>100%</span>
                </div>
              </div>

              <div className="bg-gray-50 dark:bg-gray-900 rounded-lg p-4">
                <div className="flex items-center justify-between mb-3">
                  <div>
                    <p className="text-sm text-gray-500 dark:text-gray-400">
                      Scenario Treasury
                    </p>
                    <p className="text-lg font-bold text-gray-900 dark:text-white">
                      {formatUSD(stressedTreasury)}
                    </p>
                  </div>
                  <div className="text-right">
                    <p className="text-sm text-gray-500 dark:text-gray-400">
                      Scenario Floor Price
                    </p>
                    <p className="text-lg font-bold text-gray-900 dark:text-white">
                      {formatUSD(scenarioFloorPrice)}
                    </p>
                  </div>
                </div>
                <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-3 overflow-hidden">
                  <div
                    className="h-3 rounded-full transition-all duration-300"
                    style={{
                      width: `${100 - stressLevel}%`,
                      background:
                        "linear-gradient(to right, #22c55e, #eab308 50%, #ef4444)",
                    }}
                  />
                </div>
                <div className="flex justify-between text-xs text-gray-400 mt-1">
                  <span>Healthy</span>
                  <span>Critical</span>
                </div>
              </div>
            </div>
          </div>

          {/* Monthly Treasury Table */}
          <div className="bg-white dark:bg-gray-800 shadow rounded-lg overflow-hidden border border-gray-200 dark:border-gray-700">
            <div className="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
              <h2 className="text-lg font-semibold text-gray-900 dark:text-white">
                Monthly Treasury
              </h2>
            </div>
            <div className="overflow-x-auto">
              <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
                <thead className="bg-gray-50 dark:bg-gray-900">
                  <tr>
                    {[
                      "Month",
                      "Revenue (KRW)",
                      "Inflow (USD)",
                      "Outflow (USD)",
                      "Treasury (USD)",
                      "Floor Price",
                    ].map((h) => (
                      <th
                        key={h}
                        className="px-6 py-3 text-right first:text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider"
                      >
                        {h}
                      </th>
                    ))}
                  </tr>
                </thead>
                <tbody className="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
                  {data.rows.map((row) => (
                    <tr
                      key={row.month}
                      className="hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
                    >
                      <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white">
                        {row.month}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-right text-gray-900 dark:text-white">
                        ₩{formatKRW(row.revenue_krw)}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-right text-green-600 dark:text-green-400">
                        +{formatUSD(row.inflow_usd)}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-right text-red-600 dark:text-red-400">
                        -{formatUSD(row.outflow_usd)}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-right font-medium text-gray-900 dark:text-white">
                        {formatUSD(row.treasury_usd)}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-right text-gray-900 dark:text-white">
                        {formatUSD(row.floor_price)}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        </>
      )}
    </div>
  );
}
