import { useState } from "react";
import { useBrands } from "@/contexts/BrandContext";
import { formatNumber, formatUSD } from "@/lib/mock-data";
import { toast } from "sonner";

const mockOperations = [
  { id: "1", date: "2026-03-28", brand: "Le Mouton", action: "mint", amount: 5000, reason: "Monthly challenge rewards", status: "completed" },
  { id: "2", date: "2026-03-25", brand: "RunPulse", action: "mint", amount: 10000, reason: "Partnership allocation", status: "completed" },
  { id: "3", date: "2026-03-20", brand: "Cafe Blossom", action: "burn", amount: 2000, reason: "Expired loyalty points", status: "completed" },
  { id: "4", date: "2026-03-15", brand: "Le Mouton", action: "mint", amount: 3000, reason: "Milestone bonus pool", status: "completed" },
  { id: "5", date: "2026-03-10", brand: "RunPulse", action: "burn", amount: 1500, reason: "Treasury rebalance", status: "pending" },
];

export function TokenAdminPage() {
  const { brands } = useBrands();

  const [selectedBrand, setSelectedBrand] = useState(brands[0]?.id ?? "");
  const [action, setAction] = useState<"mint" | "burn">("mint");
  const [amount, setAmount] = useState("");
  const [reason, setReason] = useState("");

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!amount || !reason) {
      toast.error("Please fill in all fields.");
      return;
    }
    const brandName = brands.find((b) => b.id === selectedBrand)?.name ?? selectedBrand;
    toast.success(
      `${action === "mint" ? "Minted" : "Burned"} ${formatNumber(Number(amount))} tokens for ${brandName}.`
    );
    setAmount("");
    setReason("");
  };

  return (
    <div>
      <div className="mb-6">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          Token Administration
        </h1>
        <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
          Mint and burn tokens across brands
        </p>
      </div>

      {/* Token Overview per Brand */}
      <div className="grid grid-cols-1 sm:grid-cols-3 gap-4 mb-6">
        {brands.map((brand) => (
          <div
            key={brand.id}
            className="bg-white dark:bg-gray-800 rounded-lg shadow p-5"
          >
            <p className="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
              {brand.name}
            </p>
            <div className="space-y-2">
              <div className="flex justify-between">
                <span className="text-xs text-gray-400 dark:text-gray-500">Total Supply</span>
                <span className="text-sm font-medium text-gray-900 dark:text-white">
                  {formatNumber(brand.total_supply)}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="text-xs text-gray-400 dark:text-gray-500">Circulating</span>
                <span className="text-sm font-medium text-gray-900 dark:text-white">
                  {formatNumber(brand.circulating_supply)}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="text-xs text-gray-400 dark:text-gray-500">Floor Price</span>
                <span className="text-sm font-medium text-gray-900 dark:text-white">
                  {formatUSD(brand.floor_price)}
                </span>
              </div>
            </div>
          </div>
        ))}
      </div>

      {/* Mint/Burn Form */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5 mb-6">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          Mint / Burn Tokens
        </h2>
        <form onSubmit={handleSubmit} className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-5 gap-4 items-end">
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Brand
            </label>
            <select
              value={selectedBrand}
              onChange={(e) => setSelectedBrand(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              {brands.map((b) => (
                <option key={b.id} value={b.id}>
                  {b.name}
                </option>
              ))}
            </select>
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Action
            </label>
            <div className="flex gap-1 rounded-lg border border-gray-300 dark:border-gray-600 p-1">
              {(["mint", "burn"] as const).map((a) => (
                <button
                  key={a}
                  type="button"
                  onClick={() => setAction(a)}
                  className={`flex-1 px-3 py-1.5 text-sm font-medium rounded-md transition-colors ${
                    action === a
                      ? a === "mint"
                        ? "bg-green-600 text-white"
                        : "bg-red-600 text-white"
                      : "text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700"
                  }`}
                >
                  {a.charAt(0).toUpperCase() + a.slice(1)}
                </button>
              ))}
            </div>
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Amount
            </label>
            <input
              type="number"
              min="0"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              placeholder="0"
              className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Reason
            </label>
            <input
              type="text"
              value={reason}
              onChange={(e) => setReason(e.target.value)}
              placeholder="Reason for operation"
              className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>

          <div>
            <button
              type="submit"
              className={`w-full px-4 py-2 text-sm font-medium text-white rounded-lg transition-colors ${
                action === "mint"
                  ? "bg-green-600 hover:bg-green-700"
                  : "bg-red-600 hover:bg-red-700"
              }`}
            >
              {action === "mint" ? "Mint Tokens" : "Burn Tokens"}
            </button>
          </div>
        </form>
      </div>

      {/* Recent Operations */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          Recent Operations
        </h2>
        <div className="overflow-x-auto">
          <table className="w-full text-sm">
            <thead>
              <tr className="text-left text-gray-500 dark:text-gray-400 border-b border-gray-200 dark:border-gray-700">
                <th className="pb-2 font-medium">Date</th>
                <th className="pb-2 font-medium">Brand</th>
                <th className="pb-2 font-medium">Action</th>
                <th className="pb-2 font-medium text-right">Amount</th>
                <th className="pb-2 font-medium">Reason</th>
                <th className="pb-2 font-medium">Status</th>
              </tr>
            </thead>
            <tbody>
              {mockOperations.map((op) => (
                <tr
                  key={op.id}
                  className="border-b border-gray-100 dark:border-gray-700/50 last:border-0"
                >
                  <td className="py-3 text-gray-500 dark:text-gray-400">
                    {new Date(op.date).toLocaleDateString("ko-KR")}
                  </td>
                  <td className="py-3 font-medium text-gray-900 dark:text-white">
                    {op.brand}
                  </td>
                  <td className="py-3">
                    <span
                      className={`inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium ${
                        op.action === "mint"
                          ? "bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400"
                          : "bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-400"
                      }`}
                    >
                      {op.action === "mint" ? "Mint" : "Burn"}
                    </span>
                  </td>
                  <td className="py-3 text-right text-gray-900 dark:text-white">
                    {formatNumber(op.amount)}
                  </td>
                  <td className="py-3 text-gray-500 dark:text-gray-400">
                    {op.reason}
                  </td>
                  <td className="py-3">
                    <span
                      className={`inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium ${
                        op.status === "completed"
                          ? "bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400"
                          : "bg-yellow-100 text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-400"
                      }`}
                    >
                      {op.status === "completed" ? "Completed" : "Pending"}
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
