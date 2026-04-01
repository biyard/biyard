import { useMode } from "@/contexts/ModeContext";
import { getWalletData, getTransactions, formatNumber } from "@/lib/mock-data";
import { toast } from "sonner";

const TYPE_BADGE: Record<string, string> = {
  earned:
    "bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400",
  convert:
    "bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-400",
  bonus:
    "bg-amber-100 text-amber-800 dark:bg-amber-900/30 dark:text-amber-400",
};

function TransactionTable() {
  const transactions = getTransactions();

  return (
    <div className="overflow-x-auto">
      <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
        <thead className="bg-gray-50 dark:bg-gray-700">
          <tr>
            {["Type", "Description", "Amount", "Date"].map((h) => (
              <th
                key={h}
                className="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-300"
              >
                {h}
              </th>
            ))}
          </tr>
        </thead>
        <tbody className="divide-y divide-gray-200 bg-white dark:divide-gray-700 dark:bg-gray-800">
          {transactions.map((tx) => (
            <tr key={tx.id}>
              <td className="whitespace-nowrap px-6 py-4">
                <span
                  className={`inline-flex rounded-full px-2 py-0.5 text-xs font-semibold leading-5 ${TYPE_BADGE[tx.type] || ""}`}
                >
                  {tx.type}
                </span>
              </td>
              <td className="whitespace-nowrap px-6 py-4 text-sm text-gray-900 dark:text-white">
                {tx.description}
              </td>
              <td
                className={`whitespace-nowrap px-6 py-4 text-sm font-medium ${
                  tx.amount >= 0
                    ? "text-green-600 dark:text-green-400"
                    : "text-red-600 dark:text-red-400"
                }`}
              >
                {tx.amount >= 0 ? "+" : ""}
                {formatNumber(tx.amount)}
              </td>
              <td className="whitespace-nowrap px-6 py-4 text-sm text-gray-500 dark:text-gray-400">
                {new Date(tx.created_at).toLocaleDateString("ko-KR")}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export function WalletPage() {
  const { isAdmin } = useMode();
  const wallet = getWalletData();

  const handleCopyReferral = () => {
    navigator.clipboard.writeText(wallet.referral_code);
    toast.success("Referral code copied!");
  };

  if (isAdmin) {
    return (
      <div>
        <div className="mb-6">
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
            Wallet &amp; Points
          </h1>
          <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
            Review all platform transactions.
          </p>
        </div>
        <div className="rounded-lg bg-white shadow dark:bg-gray-800">
          <div className="px-6 py-4">
            <h2 className="text-lg font-semibold text-gray-900 dark:text-white">
              Transaction History
            </h2>
          </div>
          <TransactionTable />
        </div>
      </div>
    );
  }

  return (
    <div>
      <div className="mb-6">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          Wallet &amp; Points
        </h1>
        <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
          View your tokens, points, and transaction history.
        </p>
      </div>

      {/* KPI Cards */}
      <div className="mb-6 grid grid-cols-1 gap-6 sm:grid-cols-2">
        <div className="rounded-lg bg-white p-6 shadow dark:bg-gray-800">
          <p className="text-sm font-medium text-gray-500 dark:text-gray-400">
            Total Tokens
          </p>
          <p className="mt-2 text-3xl font-bold text-gray-900 dark:text-white">
            {formatNumber(wallet.total_tokens)}
          </p>
        </div>
        <div className="rounded-lg bg-white p-6 shadow dark:bg-gray-800">
          <p className="text-sm font-medium text-gray-500 dark:text-gray-400">
            Pending Points
          </p>
          <p className="mt-2 text-3xl font-bold text-amber-600 dark:text-amber-400">
            {formatNumber(wallet.total_pending_points)}
          </p>
        </div>
      </div>

      {/* Token Balances */}
      <div className="mb-6 rounded-lg bg-white p-6 shadow dark:bg-gray-800">
        <h2 className="mb-4 text-lg font-semibold text-gray-900 dark:text-white">
          Token Balances
        </h2>
        <ul className="space-y-3">
          {Object.entries(wallet.balances).map(([brand, amount]) => (
            <li
              key={brand}
              className="flex items-center justify-between rounded-md border border-gray-200 px-4 py-3 dark:border-gray-700"
            >
              <span className="text-sm font-medium text-gray-900 dark:text-white">
                {brand}
              </span>
              <span className="text-sm font-semibold text-blue-600 dark:text-blue-400">
                {formatNumber(amount)} tokens
              </span>
            </li>
          ))}
        </ul>
      </div>

      {/* Transaction History */}
      <div className="mb-6 rounded-lg bg-white shadow dark:bg-gray-800">
        <div className="px-6 py-4">
          <h2 className="text-lg font-semibold text-gray-900 dark:text-white">
            Transaction History
          </h2>
        </div>
        <TransactionTable />
      </div>

      {/* Referral Program */}
      <div className="rounded-lg bg-white p-6 shadow dark:bg-gray-800">
        <h2 className="mb-4 text-lg font-semibold text-gray-900 dark:text-white">
          Referral Program
        </h2>
        <p className="mb-3 text-sm text-gray-600 dark:text-gray-400">
          Share your referral code and earn bonus tokens when friends join.
        </p>
        <div className="flex items-center gap-3">
          <code className="rounded-md bg-gray-100 px-4 py-2 text-sm font-mono text-gray-900 dark:bg-gray-700 dark:text-white">
            {wallet.referral_code}
          </code>
          <button
            onClick={handleCopyReferral}
            className="rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
          >
            Copy
          </button>
        </div>
      </div>
    </div>
  );
}
