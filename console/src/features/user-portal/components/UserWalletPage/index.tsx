import { formatNumber } from "@/lib/mock-data";
import { toast } from "sonner";

const tokenBalances = [
  { brand: "Le Mouton", symbol: "LMT", amount: 8.5 },
  { brand: "Cafe Blossom", symbol: "CBT", amount: 3.24 },
  { brand: "RunPulse", symbol: "RPT", amount: 1.5 },
];

const transactions = [
  { id: "t1", type: "earned", description: "Walking challenge - 8,500 steps", amount: 85, date: "Mar 28, 2026" },
  { id: "t2", type: "convert", description: "Points to LMT conversion", amount: 2, date: "Mar 27, 2026" },
  { id: "t3", type: "bonus", description: "Silver milestone reached", amount: 120, date: "Mar 25, 2026" },
  { id: "t4", type: "earned", description: "Walking challenge - 12,300 steps", amount: 123, date: "Mar 24, 2026" },
  { id: "t5", type: "bonus", description: "Sign-up bonus", amount: 10, date: "Mar 20, 2026" },
  { id: "t6", type: "convert", description: "Points to CBT conversion", amount: 1.24, date: "Mar 19, 2026" },
  { id: "t7", type: "earned", description: "Walking challenge - 5,600 steps", amount: 56, date: "Mar 18, 2026" },
  { id: "t8", type: "bonus", description: "Bronze milestone reached", amount: 50, date: "Mar 15, 2026" },
];

const REFERRAL_CODE = "REF-JHPARK-8A2F";

const typeBadge = (type: string) => {
  switch (type) {
    case "earned":
      return "bg-emerald-100 dark:bg-emerald-900/30 text-emerald-700 dark:text-emerald-300";
    case "convert":
      return "bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300";
    case "bonus":
      return "bg-amber-100 dark:bg-amber-900/30 text-amber-700 dark:text-amber-300";
    default:
      return "bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300";
  }
};

export function UserWalletPage() {
  const handleCopyReferral = () => {
    navigator.clipboard.writeText(REFERRAL_CODE);
    toast.success("Referral code copied to clipboard!");
  };

  return (
    <div>
      {/* Header */}
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          My Wallet
        </h1>
        <p className="mt-1 text-gray-600 dark:text-gray-400">
          Your tokens, points, and transaction history.
        </p>
      </div>

      {/* KPI Cards */}
      <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-8">
        <div className="bg-white dark:bg-gray-800 rounded-xl shadow p-6">
          <p className="text-sm text-gray-500 dark:text-gray-400">Total Tokens</p>
          <p className="mt-1 text-3xl font-bold text-indigo-600 dark:text-indigo-400">
            {formatNumber(13.24)}
          </p>
        </div>
        <div className="bg-white dark:bg-gray-800 rounded-xl shadow p-6">
          <p className="text-sm text-gray-500 dark:text-gray-400">Pending Points</p>
          <p className="mt-1 text-3xl font-bold text-amber-600 dark:text-amber-400">
            {formatNumber(124)}
          </p>
        </div>
      </div>

      {/* Token Balances */}
      <div className="bg-white dark:bg-gray-800 rounded-xl shadow p-6 mb-8">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          Token Balances
        </h2>
        <div className="space-y-3">
          {tokenBalances.map((token) => (
            <div
              key={token.symbol}
              className="flex items-center justify-between p-4 rounded-lg bg-gray-50 dark:bg-gray-700/50"
            >
              <div>
                <p className="text-sm font-medium text-gray-900 dark:text-white">
                  {token.brand}
                </p>
                <p className="text-xs text-gray-500 dark:text-gray-400">
                  {token.symbol}
                </p>
              </div>
              <p className="text-lg font-bold text-gray-900 dark:text-white">
                {formatNumber(token.amount)}
              </p>
            </div>
          ))}
        </div>
      </div>

      {/* Transaction History */}
      <div className="bg-white dark:bg-gray-800 rounded-xl shadow p-6 mb-8">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          Transaction History
        </h2>
        <div className="overflow-x-auto">
          <table className="w-full text-sm">
            <thead>
              <tr className="border-b border-gray-200 dark:border-gray-700">
                <th className="text-left py-3 px-2 font-medium text-gray-500 dark:text-gray-400">Type</th>
                <th className="text-left py-3 px-2 font-medium text-gray-500 dark:text-gray-400">Description</th>
                <th className="text-right py-3 px-2 font-medium text-gray-500 dark:text-gray-400">Amount</th>
                <th className="text-right py-3 px-2 font-medium text-gray-500 dark:text-gray-400">Date</th>
              </tr>
            </thead>
            <tbody>
              {transactions.map((tx) => (
                <tr key={tx.id} className="border-b border-gray-100 dark:border-gray-700/50">
                  <td className="py-3 px-2">
                    <span className={`inline-block px-2 py-0.5 text-xs font-medium rounded-full ${typeBadge(tx.type)}`}>
                      {tx.type}
                    </span>
                  </td>
                  <td className="py-3 px-2 text-gray-900 dark:text-white">
                    {tx.description}
                  </td>
                  <td className="py-3 px-2 text-right font-medium text-gray-900 dark:text-white">
                    {tx.type === "convert" ? `${formatNumber(tx.amount)} tokens` : `${formatNumber(tx.amount)} pts`}
                  </td>
                  <td className="py-3 px-2 text-right text-gray-500 dark:text-gray-400">
                    {tx.date}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      {/* Referral Program */}
      <div className="bg-white dark:bg-gray-800 rounded-xl shadow p-6">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-2">
          Referral Program
        </h2>
        <p className="text-sm text-gray-500 dark:text-gray-400 mb-4">
          Share your referral code and earn bonus points when friends join.
        </p>
        <div className="flex items-center gap-3">
          <div className="flex-1 px-4 py-3 rounded-lg bg-gray-100 dark:bg-gray-700 font-mono text-sm text-gray-900 dark:text-white tracking-wider">
            {REFERRAL_CODE}
          </div>
          <button
            onClick={handleCopyReferral}
            className="px-4 py-3 text-sm font-medium rounded-lg bg-indigo-600 text-white hover:bg-indigo-700 transition-colors"
          >
            Copy
          </button>
        </div>
      </div>
    </div>
  );
}
