import { useState } from "react";
import { ArrowDownUp } from "lucide-react";
import { formatNumber } from "@/lib/mock-data";
import { toast } from "sonner";

interface TokenBalance {
  symbol: string;
  name: string;
  balance: number;
}

interface ExchangeRecord {
  id: string;
  date: string;
  from: string;
  to: string;
  amount: number;
  rate: number;
  received: number;
  status: string;
}

const initialBalances: TokenBalance[] = [
  { symbol: "LMT", name: "Le Mouton", balance: 8.5 },
  { symbol: "CBT", name: "Cafe Blossom", balance: 3.24 },
  { symbol: "RPT", name: "RunPulse", balance: 1.5 },
];

const EXCHANGE_RATES: Record<string, number> = {
  "LMT-CBT": 0.85,
  "LMT-RPT": 1.2,
  "CBT-LMT": 1.18,
  "CBT-RPT": 1.41,
  "RPT-LMT": 0.83,
  "RPT-CBT": 0.71,
};

const initialExchanges: ExchangeRecord[] = [
  { id: "1", date: "2026-03-28", from: "LMT", to: "CBT", amount: 5, rate: 0.85, received: 4.25, status: "completed" },
  { id: "2", date: "2026-03-25", from: "RPT", to: "LMT", amount: 2, rate: 0.83, received: 1.66, status: "completed" },
  { id: "3", date: "2026-03-20", from: "CBT", to: "RPT", amount: 3, rate: 1.41, received: 4.23, status: "completed" },
];

const SYMBOLS = ["LMT", "CBT", "RPT"];

export function ExchangePage() {
  const [balances, setBalances] = useState<TokenBalance[]>(initialBalances);
  const [exchanges, setExchanges] = useState<ExchangeRecord[]>(initialExchanges);

  const [fromToken, setFromToken] = useState("LMT");
  const [toToken, setToToken] = useState("CBT");
  const [amount, setAmount] = useState<number | "">("");

  const rate = fromToken !== toToken ? EXCHANGE_RATES[`${fromToken}-${toToken}`] ?? 0 : 0;
  const calculatedAmount = typeof amount === "number" ? amount * rate : 0;

  const fromBalance = balances.find((b) => b.symbol === fromToken)?.balance ?? 0;
  const toBalance = balances.find((b) => b.symbol === toToken)?.balance ?? 0;

  const handleSwap = () => {
    setFromToken(toToken);
    setToToken(fromToken);
  };

  const handleExchange = () => {
    if (fromToken === toToken) {
      toast.error("Cannot exchange the same token.");
      return;
    }
    if (typeof amount !== "number" || amount <= 0) {
      toast.error("Enter a valid amount.");
      return;
    }
    if (amount > fromBalance) {
      toast.error("Insufficient balance.");
      return;
    }

    setBalances((prev) =>
      prev.map((b) => {
        if (b.symbol === fromToken) return { ...b, balance: b.balance - amount };
        if (b.symbol === toToken) return { ...b, balance: b.balance + calculatedAmount };
        return b;
      })
    );

    const record: ExchangeRecord = {
      id: String(Date.now()),
      date: new Date().toISOString().slice(0, 10),
      from: fromToken,
      to: toToken,
      amount,
      rate,
      received: calculatedAmount,
      status: "completed",
    };

    setExchanges((prev) => [record, ...prev]);
    toast.success(
      `Exchanged ${formatNumber(amount)} ${fromToken} for ${formatNumber(calculatedAmount)} ${toToken}`
    );
    setAmount("");
  };

  return (
    <div>
      <div className="mb-6">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          Token Exchange
        </h1>
        <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
          Exchange tokens between brands
        </p>
      </div>

      {/* Exchange Form */}
      <div className="max-w-md mx-auto mb-8">
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
          {/* From */}
          <div className="mb-1">
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              From
            </label>
            <div className="flex gap-2">
              <select
                value={fromToken}
                onChange={(e) => setFromToken(e.target.value)}
                className="rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 px-3 py-2 text-sm text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                {SYMBOLS.map((s) => (
                  <option key={s} value={s}>
                    {s}
                  </option>
                ))}
              </select>
              <input
                type="number"
                value={amount}
                onChange={(e) =>
                  setAmount(e.target.value === "" ? "" : Number(e.target.value))
                }
                placeholder="0.00"
                min={0}
                step="any"
                className="flex-1 rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 px-3 py-2 text-sm text-gray-900 dark:text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
            <p className="text-xs text-gray-400 dark:text-gray-500 mt-1">
              Available: {formatNumber(fromBalance)} {fromToken}
            </p>
          </div>

          {/* Swap Button */}
          <div className="flex justify-center my-3">
            <button
              type="button"
              onClick={handleSwap}
              className="p-2 rounded-full border border-gray-300 dark:border-gray-600 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
            >
              <ArrowDownUp className="h-4 w-4 text-gray-500 dark:text-gray-400" />
            </button>
          </div>

          {/* To */}
          <div className="mb-4">
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              To
            </label>
            <div className="flex gap-2">
              <select
                value={toToken}
                onChange={(e) => setToToken(e.target.value)}
                className="rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 px-3 py-2 text-sm text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                {SYMBOLS.map((s) => (
                  <option key={s} value={s}>
                    {s}
                  </option>
                ))}
              </select>
              <input
                type="text"
                value={
                  typeof amount === "number" && amount > 0
                    ? formatNumber(calculatedAmount)
                    : ""
                }
                readOnly
                placeholder="0.00"
                className="flex-1 rounded-md border border-gray-300 dark:border-gray-600 bg-gray-50 dark:bg-gray-600 px-3 py-2 text-sm text-gray-900 dark:text-white placeholder-gray-400"
              />
            </div>
            <p className="text-xs text-gray-400 dark:text-gray-500 mt-1">
              Available: {formatNumber(toBalance)} {toToken}
            </p>
          </div>

          {/* Rate */}
          {fromToken !== toToken && (
            <p className="text-xs text-gray-500 dark:text-gray-400 text-center mb-4">
              1 {fromToken} = {rate} {toToken}
            </p>
          )}

          <button
            type="button"
            onClick={handleExchange}
            className="w-full px-4 py-2.5 bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium rounded-md transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
          >
            Exchange
          </button>
        </div>
      </div>

      {/* My Balances */}
      <div className="mb-8">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          My Balances
        </h2>
        <div className="flex gap-4 overflow-x-auto pb-2">
          {balances.map((b) => (
            <div
              key={b.symbol}
              className="min-w-[180px] bg-white dark:bg-gray-800 rounded-lg shadow p-4 flex-shrink-0"
            >
              <p className="text-sm text-gray-500 dark:text-gray-400">
                {b.name}
              </p>
              <p className="mt-1 text-xl font-bold text-gray-900 dark:text-white">
                {formatNumber(b.balance)}{" "}
                <span className="text-sm font-medium text-gray-500">
                  {b.symbol}
                </span>
              </p>
            </div>
          ))}
        </div>
      </div>

      {/* Recent Exchanges */}
      <div>
        <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          Recent Exchanges
        </h2>
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow overflow-x-auto">
          <table className="w-full text-sm text-left">
            <thead className="bg-gray-50 dark:bg-gray-700 text-gray-500 dark:text-gray-400 text-xs uppercase">
              <tr>
                <th className="px-4 py-3">Date</th>
                <th className="px-4 py-3">From</th>
                <th className="px-4 py-3">To</th>
                <th className="px-4 py-3 text-right">Amount</th>
                <th className="px-4 py-3 text-right">Rate</th>
                <th className="px-4 py-3">Status</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-gray-100 dark:divide-gray-700">
              {exchanges.map((ex) => (
                <tr key={ex.id}>
                  <td className="px-4 py-3 text-gray-700 dark:text-gray-300">
                    {ex.date}
                  </td>
                  <td className="px-4 py-3 font-medium text-gray-900 dark:text-white">
                    {formatNumber(ex.amount)} {ex.from}
                  </td>
                  <td className="px-4 py-3 font-medium text-gray-900 dark:text-white">
                    {formatNumber(ex.received)} {ex.to}
                  </td>
                  <td className="px-4 py-3 text-right text-gray-700 dark:text-gray-300">
                    {formatNumber(ex.amount)}
                  </td>
                  <td className="px-4 py-3 text-right text-gray-700 dark:text-gray-300">
                    {ex.rate}
                  </td>
                  <td className="px-4 py-3">
                    <span className="inline-flex items-center gap-1 text-xs font-medium text-green-600 dark:text-green-400">
                      <span className="h-1.5 w-1.5 rounded-full bg-green-500" />
                      {ex.status}
                    </span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}
