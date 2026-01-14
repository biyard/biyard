import {
  Loader2,
  Star,
  ArrowUpRight,
  ArrowDownRight,
  ArrowLeftRight,
  Repeat,
} from "lucide-react";
import type { PointTransaction } from "../../../points/models/point-transaction";
import type { TabProps } from "./types";

interface PointsTabProps extends TabProps {
  transactions: PointTransaction[];
  isLoading: boolean;
}

export function PointsTab({ transactions, isLoading, t }: PointsTabProps) {
  if (isLoading) {
    return (
      <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-12 text-center">
        <Loader2 className="mx-auto h-12 w-12 text-gray-400 animate-spin" />
        <p className="mt-4 text-gray-600 dark:text-gray-400">
          Loading transactions...
        </p>
      </div>
    );
  }

  if (!transactions || transactions.length === 0) {
    return (
      <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-12 text-center">
        <Star className="mx-auto h-12 w-12 text-gray-400" />
        <h3 className="mt-2 text-sm font-medium text-gray-900 dark:text-white">
          {t.noTransactions}
        </h3>
        <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
          {t.noTransactionsDescription}
        </p>
      </div>
    );
  }

  const getTransactionIcon = (type: string) => {
    switch (type) {
      case "Award":
        return <ArrowUpRight className="h-4 w-4 text-green-500" />;
      case "Deduct":
        return <ArrowDownRight className="h-4 w-4 text-red-500" />;
      case "Transfer":
        return <ArrowLeftRight className="h-4 w-4 text-blue-500" />;
      case "Exchange":
        return <Repeat className="h-4 w-4 text-purple-500" />;
      default:
        return <Star className="h-4 w-4 text-gray-500" />;
    }
  };

  return (
    <div className="bg-white dark:bg-gray-800 shadow rounded-lg overflow-hidden">
      <div className="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
        <h3 className="text-lg font-medium text-gray-900 dark:text-white">
          Point Transactions
        </h3>
      </div>
      <div className="overflow-x-auto">
        <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
          <thead className="bg-gray-50 dark:bg-gray-700">
            <tr>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase">
                {t.transactionType}
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase">
                {t.user}
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase">
                {t.amount}
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase">
                {t.month}
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase">
                {t.description}
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase">
                {t.createdAt}
              </th>
            </tr>
          </thead>
          <tbody className="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
            {transactions.map((tx, index) => (
              <tr key={index}>
                <td className="px-6 py-4 whitespace-nowrap">
                  <div className="flex items-center">
                    {getTransactionIcon(tx.transactionType)}
                    <span
                      className={`ml-2 px-2 py-1 text-xs font-semibold rounded ${tx.getTransactionTypeColor()}`}
                    >
                      {tx.getTransactionTypeLabel()}
                    </span>
                  </div>
                </td>
                <td className="px-6 py-4 whitespace-nowrap">
                  <span className="text-sm text-gray-900 dark:text-white">
                    {tx.metaUserId}
                  </span>
                  {tx.targetUserId && (
                    <span className="text-sm text-gray-500 dark:text-gray-400">
                      {" "}
                      &rarr; {tx.targetUserId}
                    </span>
                  )}
                </td>
                <td className="px-6 py-4 whitespace-nowrap">
                  <span
                    className={`text-sm font-medium ${
                      tx.isPositive()
                        ? "text-green-600 dark:text-green-400"
                        : tx.isNegative()
                        ? "text-red-600 dark:text-red-400"
                        : "text-gray-900 dark:text-white"
                    }`}
                  >
                    {tx.getFormattedAmount()}
                  </span>
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                  {tx.month}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                  {tx.description || "-"}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                  {tx.getFormattedCreatedAt()}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
