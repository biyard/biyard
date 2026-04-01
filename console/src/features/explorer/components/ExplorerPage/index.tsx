import { useState } from "react";
import { Search } from "lucide-react";
import { formatNumber } from "@/lib/mock-data";

const mockTxns = [
  { hash: "0x7a3b...f82d", block: 18234567, type: "mint", from: "0x0000...0000", to: "0x1a2b...3c4d", amount: 5000, token: "LMT", time: "2026-03-29T14:30:00Z" },
  { hash: "0x9c4e...a12f", block: 18234550, type: "transfer", from: "0x1a2b...3c4d", to: "0x5e6f...7g8h", amount: 150, token: "LMT", time: "2026-03-29T10:20:00Z" },
  { hash: "0x2d5f...b34e", block: 18234530, type: "burn", from: "0x3i4j...5k6l", to: "0x0000...0000", amount: 2000, token: "CBT", time: "2026-03-28T18:00:00Z" },
  { hash: "0x8g1h...c56f", block: 18234510, type: "mint", from: "0x0000...0000", to: "0x7m8n...9o0p", amount: 10000, token: "RPT", time: "2026-03-28T12:00:00Z" },
  { hash: "0x4k2l...d78g", block: 18234490, type: "transfer", from: "0x7m8n...9o0p", to: "0x1q2r...3s4t", amount: 500, token: "RPT", time: "2026-03-27T16:45:00Z" },
  { hash: "0x6m3n...e90h", block: 18234470, type: "mint", from: "0x0000...0000", to: "0x5u6v...7w8x", amount: 3000, token: "LMT", time: "2026-03-27T09:30:00Z" },
  { hash: "0x0o4p...f12i", block: 18234450, type: "burn", from: "0x9y0z...1a2b", to: "0x0000...0000", amount: 1500, token: "RPT", time: "2026-03-26T11:00:00Z" },
  { hash: "0x3q5r...g34j", block: 18234430, type: "transfer", from: "0x3c4d...5e6f", to: "0x7g8h...9i0j", amount: 85, token: "CBT", time: "2026-03-25T15:00:00Z" },
];

const kpiCards = [
  { label: "Total Transactions", value: 15234 },
  { label: "Total Tokens Minted", value: 135000 },
  { label: "Total Tokens Burned", value: 12500 },
  { label: "Active Addresses", value: 4200 },
];

const typeBadge: Record<string, string> = {
  mint: "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300",
  burn: "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300",
  transfer: "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300",
};

function formatTime(iso: string): string {
  const d = new Date(iso);
  return d.toLocaleDateString("en-US", { month: "short", day: "numeric" }) +
    " " +
    d.toLocaleTimeString("en-US", { hour: "2-digit", minute: "2-digit" });
}

export function ExplorerPage() {
  const [search, setSearch] = useState("");

  const filtered = search.trim()
    ? mockTxns.filter(
        (tx) =>
          tx.hash.toLowerCase().includes(search.toLowerCase()) ||
          tx.from.toLowerCase().includes(search.toLowerCase()) ||
          tx.to.toLowerCase().includes(search.toLowerCase()) ||
          tx.token.toLowerCase().includes(search.toLowerCase())
      )
    : mockTxns;

  return (
    <div>
      <div className="mb-6">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          Blockchain Explorer
        </h1>
        <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
          On-chain transaction explorer for verifying token operations
        </p>
      </div>

      {/* Search Bar */}
      <div className="relative mb-6">
        <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-5 w-5 text-gray-400" />
        <input
          type="text"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          placeholder="Search by transaction hash, address, or token ID"
          className="w-full pl-10 pr-4 py-3 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-gray-900 dark:text-white placeholder-gray-400 dark:placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>

      {/* KPI Cards */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
        {kpiCards.map((card) => (
          <div
            key={card.label}
            className="bg-white dark:bg-gray-800 rounded-lg shadow p-5"
          >
            <p className="text-sm text-gray-500 dark:text-gray-400">
              {card.label}
            </p>
            <p className="mt-1 text-2xl font-bold text-gray-900 dark:text-white">
              {formatNumber(card.value)}
            </p>
          </div>
        ))}
      </div>

      {/* Recent Transactions Table */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden">
        <div className="px-5 py-4 border-b border-gray-200 dark:border-gray-700">
          <h2 className="text-lg font-semibold text-gray-900 dark:text-white">
            Recent On-Chain Transactions
          </h2>
        </div>
        <div className="overflow-x-auto">
          <table className="w-full text-sm">
            <thead>
              <tr className="bg-gray-50 dark:bg-gray-700 text-left text-gray-500 dark:text-gray-400">
                <th className="px-4 py-3 font-medium">Tx Hash</th>
                <th className="px-4 py-3 font-medium">Block</th>
                <th className="px-4 py-3 font-medium">Type</th>
                <th className="px-4 py-3 font-medium">From</th>
                <th className="px-4 py-3 font-medium">To</th>
                <th className="px-4 py-3 font-medium text-right">Amount</th>
                <th className="px-4 py-3 font-medium">Timestamp</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-gray-200 dark:divide-gray-700">
              {filtered.map((tx) => (
                <tr
                  key={tx.hash}
                  className="hover:bg-gray-50 dark:hover:bg-gray-750"
                >
                  <td className="px-4 py-3">
                    <span className="font-mono text-blue-600 dark:text-blue-400 cursor-pointer hover:underline">
                      {tx.hash}
                    </span>
                  </td>
                  <td className="px-4 py-3 font-mono text-gray-700 dark:text-gray-300">
                    {formatNumber(tx.block)}
                  </td>
                  <td className="px-4 py-3">
                    <span
                      className={`inline-block px-2.5 py-0.5 rounded-full text-xs font-medium capitalize ${typeBadge[tx.type]}`}
                    >
                      {tx.type}
                    </span>
                  </td>
                  <td className="px-4 py-3 font-mono text-gray-600 dark:text-gray-400">
                    {tx.from}
                  </td>
                  <td className="px-4 py-3 font-mono text-gray-600 dark:text-gray-400">
                    {tx.to}
                  </td>
                  <td className="px-4 py-3 text-right font-medium text-gray-900 dark:text-white">
                    {formatNumber(tx.amount)} {tx.token}
                  </td>
                  <td className="px-4 py-3 text-gray-500 dark:text-gray-400 whitespace-nowrap">
                    {formatTime(tx.time)}
                  </td>
                </tr>
              ))}
              {filtered.length === 0 && (
                <tr>
                  <td
                    colSpan={7}
                    className="px-4 py-8 text-center text-gray-400 dark:text-gray-500"
                  >
                    No transactions found matching your search.
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}
