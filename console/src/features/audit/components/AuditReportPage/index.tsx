import { useState } from "react";
import { Download } from "lucide-react";
import { formatUSD, formatNumber } from "@/lib/mock-data";
import { toast } from "sonner";

const periods = [
  { value: "2026-03", label: "March 2026" },
  { value: "2026-02", label: "February 2026" },
  { value: "2026-01", label: "January 2026" },
  { value: "2025-12", label: "December 2025" },
];

const reportData: Record<
  string,
  {
    openingTreasury: number;
    closingTreasury: number;
    netChange: number;
    netChangePct: number;
    tokensMinted: number;
    tokensBurned: number;
    netTokenChange: number;
    treasuryMovements: { date: string; type: string; amount: number; description: string }[];
    tokenOperations: { date: string; operation: string; amount: number; brand: string }[];
  }
> = {
  "2026-03": {
    openingTreasury: 68200,
    closingTreasury: 72600,
    netChange: 4400,
    netChangePct: 6.5,
    tokensMinted: 8000,
    tokensBurned: 1500,
    netTokenChange: 6500,
    treasuryMovements: [
      { date: "2026-03-05", type: "inflow", amount: 1250, description: "Le Mouton 월간 수익 배분" },
      { date: "2026-03-10", type: "inflow", amount: 890, description: "Cafe Blossom 월간 수익 배분" },
      { date: "2026-03-12", type: "outflow", amount: -320, description: "운영비 지출" },
      { date: "2026-03-15", type: "inflow", amount: 2100, description: "RunPulse 월간 수익 배분" },
      { date: "2026-03-20", type: "outflow", amount: -180, description: "가스비 정산" },
      { date: "2026-03-25", type: "inflow", amount: 660, description: "파트너십 수익" },
    ],
    tokenOperations: [
      { date: "2026-03-03", operation: "Mint", amount: 3000, brand: "Le Mouton (LMT)" },
      { date: "2026-03-08", operation: "Mint", amount: 2000, brand: "RunPulse (RPT)" },
      { date: "2026-03-11", operation: "Burn", amount: 500, brand: "Cafe Blossom (CBT)" },
      { date: "2026-03-18", operation: "Mint", amount: 3000, brand: "Cafe Blossom (CBT)" },
      { date: "2026-03-22", operation: "Burn", amount: 1000, brand: "Le Mouton (LMT)" },
      { date: "2026-03-28", operation: "Mint", amount: 0, brand: "RunPulse (RPT)" },
    ],
  },
  "2026-02": {
    openingTreasury: 64500,
    closingTreasury: 68200,
    netChange: 3700,
    netChangePct: 5.7,
    tokensMinted: 7200,
    tokensBurned: 1200,
    netTokenChange: 6000,
    treasuryMovements: [
      { date: "2026-02-05", type: "inflow", amount: 1100, description: "Le Mouton 월간 수익 배분" },
      { date: "2026-02-12", type: "inflow", amount: 780, description: "Cafe Blossom 월간 수익 배분" },
      { date: "2026-02-15", type: "outflow", amount: -280, description: "운영비 지출" },
      { date: "2026-02-20", type: "inflow", amount: 1900, description: "RunPulse 월간 수익 배분" },
      { date: "2026-02-25", type: "outflow", amount: -150, description: "가스비 정산" },
    ],
    tokenOperations: [
      { date: "2026-02-04", operation: "Mint", amount: 2500, brand: "Le Mouton (LMT)" },
      { date: "2026-02-10", operation: "Mint", amount: 1800, brand: "RunPulse (RPT)" },
      { date: "2026-02-16", operation: "Burn", amount: 700, brand: "Cafe Blossom (CBT)" },
      { date: "2026-02-22", operation: "Mint", amount: 2900, brand: "Cafe Blossom (CBT)" },
      { date: "2026-02-27", operation: "Burn", amount: 500, brand: "Le Mouton (LMT)" },
    ],
  },
  "2026-01": {
    openingTreasury: 61000,
    closingTreasury: 64500,
    netChange: 3500,
    netChangePct: 5.7,
    tokensMinted: 6500,
    tokensBurned: 1000,
    netTokenChange: 5500,
    treasuryMovements: [
      { date: "2026-01-05", type: "inflow", amount: 950, description: "Le Mouton 월간 수익 배분" },
      { date: "2026-01-12", type: "inflow", amount: 700, description: "Cafe Blossom 월간 수익 배분" },
      { date: "2026-01-18", type: "outflow", amount: -250, description: "운영비 지출" },
      { date: "2026-01-22", type: "inflow", amount: 1800, description: "RunPulse 월간 수익 배분" },
      { date: "2026-01-28", type: "outflow", amount: -130, description: "가스비 정산" },
    ],
    tokenOperations: [
      { date: "2026-01-06", operation: "Mint", amount: 2200, brand: "Le Mouton (LMT)" },
      { date: "2026-01-14", operation: "Mint", amount: 1500, brand: "RunPulse (RPT)" },
      { date: "2026-01-20", operation: "Burn", amount: 600, brand: "Cafe Blossom (CBT)" },
      { date: "2026-01-26", operation: "Mint", amount: 2800, brand: "Cafe Blossom (CBT)" },
    ],
  },
  "2025-12": {
    openingTreasury: 57800,
    closingTreasury: 61000,
    netChange: 3200,
    netChangePct: 5.5,
    tokensMinted: 6000,
    tokensBurned: 900,
    netTokenChange: 5100,
    treasuryMovements: [
      { date: "2025-12-05", type: "inflow", amount: 880, description: "Le Mouton 월간 수익 배분" },
      { date: "2025-12-10", type: "inflow", amount: 650, description: "Cafe Blossom 월간 수익 배분" },
      { date: "2025-12-15", type: "outflow", amount: -230, description: "운영비 지출" },
      { date: "2025-12-20", type: "inflow", amount: 1700, description: "RunPulse 월간 수익 배분" },
      { date: "2025-12-28", type: "outflow", amount: -120, description: "가스비 정산" },
    ],
    tokenOperations: [
      { date: "2025-12-03", operation: "Mint", amount: 2000, brand: "Le Mouton (LMT)" },
      { date: "2025-12-12", operation: "Mint", amount: 1400, brand: "RunPulse (RPT)" },
      { date: "2025-12-18", operation: "Burn", amount: 500, brand: "Cafe Blossom (CBT)" },
      { date: "2025-12-24", operation: "Mint", amount: 2600, brand: "Cafe Blossom (CBT)" },
    ],
  },
};

export function AuditReportPage() {
  const [selectedPeriod, setSelectedPeriod] = useState("2026-03");
  const data = reportData[selectedPeriod];
  const periodLabel = periods.find((p) => p.value === selectedPeriod)?.label ?? selectedPeriod;

  return (
    <div>
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between mb-6 gap-4">
        <div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
            Audit Report
          </h1>
          <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
            Monthly audit reports with downloadable summaries
          </p>
        </div>
        <div className="flex items-center gap-3">
          <select
            value={selectedPeriod}
            onChange={(e) => setSelectedPeriod(e.target.value)}
            className="rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-gray-900 dark:text-white px-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            {periods.map((p) => (
              <option key={p.value} value={p.value}>
                {p.label}
              </option>
            ))}
          </select>
          <button
            onClick={() => toast.success("Report downloaded as PDF")}
            className="inline-flex items-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 transition-colors"
          >
            <Download className="h-4 w-4" />
            Download Report
          </button>
        </div>
      </div>

      {/* Report Summary Cards */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 mb-6">
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
          <p className="text-sm text-gray-500 dark:text-gray-400">
            Opening Treasury
          </p>
          <p className="mt-1 text-2xl font-bold text-gray-900 dark:text-white">
            {formatUSD(data.openingTreasury)}
          </p>
        </div>
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
          <p className="text-sm text-gray-500 dark:text-gray-400">
            Closing Treasury
          </p>
          <p className="mt-1 text-2xl font-bold text-gray-900 dark:text-white">
            {formatUSD(data.closingTreasury)}
          </p>
        </div>
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
          <p className="text-sm text-gray-500 dark:text-gray-400">
            Net Change
          </p>
          <p className="mt-1 text-2xl font-bold text-green-600 dark:text-green-400">
            +{formatUSD(data.netChange)} (+{data.netChangePct}%)
          </p>
        </div>
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
          <p className="text-sm text-gray-500 dark:text-gray-400">
            Tokens Minted
          </p>
          <p className="mt-1 text-2xl font-bold text-gray-900 dark:text-white">
            {formatNumber(data.tokensMinted)}
          </p>
        </div>
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
          <p className="text-sm text-gray-500 dark:text-gray-400">
            Tokens Burned
          </p>
          <p className="mt-1 text-2xl font-bold text-gray-900 dark:text-white">
            {formatNumber(data.tokensBurned)}
          </p>
        </div>
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
          <p className="text-sm text-gray-500 dark:text-gray-400">
            Net Token Change
          </p>
          <p className="mt-1 text-2xl font-bold text-green-600 dark:text-green-400">
            +{formatNumber(data.netTokenChange)}
          </p>
        </div>
      </div>

      {/* Detailed Breakdown */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
        {/* Treasury Movements */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden">
          <div className="px-5 py-4 border-b border-gray-200 dark:border-gray-700">
            <h2 className="text-lg font-semibold text-gray-900 dark:text-white">
              Treasury Movements
            </h2>
            <p className="text-xs text-gray-400 dark:text-gray-500 mt-0.5">
              {periodLabel}
            </p>
          </div>
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr className="bg-gray-50 dark:bg-gray-700 text-left text-gray-500 dark:text-gray-400">
                  <th className="px-4 py-3 font-medium">Date</th>
                  <th className="px-4 py-3 font-medium">Type</th>
                  <th className="px-4 py-3 font-medium text-right">Amount</th>
                  <th className="px-4 py-3 font-medium">Description</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-200 dark:divide-gray-700">
                {data.treasuryMovements.map((m, i) => (
                  <tr key={i} className="hover:bg-gray-50 dark:hover:bg-gray-750">
                    <td className="px-4 py-3 text-gray-700 dark:text-gray-300 whitespace-nowrap">
                      {m.date}
                    </td>
                    <td className="px-4 py-3">
                      <span
                        className={`inline-block px-2.5 py-0.5 rounded-full text-xs font-medium capitalize ${
                          m.type === "inflow"
                            ? "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300"
                            : "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300"
                        }`}
                      >
                        {m.type}
                      </span>
                    </td>
                    <td
                      className={`px-4 py-3 text-right font-medium ${
                        m.amount >= 0
                          ? "text-green-600 dark:text-green-400"
                          : "text-red-600 dark:text-red-400"
                      }`}
                    >
                      {m.amount >= 0 ? "+" : ""}
                      {formatUSD(m.amount)}
                    </td>
                    <td className="px-4 py-3 text-gray-600 dark:text-gray-400">
                      {m.description}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>

        {/* Token Operations */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden">
          <div className="px-5 py-4 border-b border-gray-200 dark:border-gray-700">
            <h2 className="text-lg font-semibold text-gray-900 dark:text-white">
              Token Operations
            </h2>
            <p className="text-xs text-gray-400 dark:text-gray-500 mt-0.5">
              {periodLabel}
            </p>
          </div>
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr className="bg-gray-50 dark:bg-gray-700 text-left text-gray-500 dark:text-gray-400">
                  <th className="px-4 py-3 font-medium">Date</th>
                  <th className="px-4 py-3 font-medium">Operation</th>
                  <th className="px-4 py-3 font-medium text-right">Amount</th>
                  <th className="px-4 py-3 font-medium">Brand</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-200 dark:divide-gray-700">
                {data.tokenOperations.map((op, i) => (
                  <tr key={i} className="hover:bg-gray-50 dark:hover:bg-gray-750">
                    <td className="px-4 py-3 text-gray-700 dark:text-gray-300 whitespace-nowrap">
                      {op.date}
                    </td>
                    <td className="px-4 py-3">
                      <span
                        className={`inline-block px-2.5 py-0.5 rounded-full text-xs font-medium ${
                          op.operation === "Mint"
                            ? "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300"
                            : "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300"
                        }`}
                      >
                        {op.operation}
                      </span>
                    </td>
                    <td className="px-4 py-3 text-right font-medium text-gray-900 dark:text-white">
                      {formatNumber(op.amount)}
                    </td>
                    <td className="px-4 py-3 text-gray-600 dark:text-gray-400">
                      {op.brand}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </div>

      {/* Auditor Notes */}
      <div className="bg-gray-50 dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-5">
        <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">
          Auditor Notes
        </h3>
        <p className="text-sm text-gray-600 dark:text-gray-400 leading-relaxed">
          This report is automatically generated based on on-chain data and treasury records.
          All token operations are verified against the blockchain ledger.
          For questions, contact audit@biyard.co.
        </p>
      </div>
    </div>
  );
}
