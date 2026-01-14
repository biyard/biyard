import { Coins } from "lucide-react";
import type { TabProps } from "./types";

export function TokensTab({ t }: TabProps) {
  return (
    <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-12 text-center">
      <Coins className="mx-auto h-12 w-12 text-gray-400" />
      <h3 className="mt-2 text-sm font-medium text-gray-900 dark:text-white">
        {t.noTokenTransactions}
      </h3>
      <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
        {t.noTokenTransactionsDescription}
      </p>
    </div>
  );
}
