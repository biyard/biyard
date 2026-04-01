import { useState } from "react";
import { formatNumber } from "@/lib/mock-data";
import { toast } from "sonner";

const milestones = [
  { name: "Bronze", steps: 5000, emoji: "🥉", completed: true },
  { name: "Silver", steps: 10000, emoji: "🥈", completed: true },
  { name: "Gold", steps: 25000, emoji: "🥇", completed: true },
  { name: "Platinum", steps: 50000, emoji: "💎", completed: false },
  { name: "Diamond", steps: 100000, emoji: "👑", completed: false },
];

const leaderboard = [
  { rank: 1, name: "김민수", steps: 268000, tier: "Diamond" },
  { rank: 2, name: "이서연", steps: 245000, tier: "Diamond" },
  { rank: 3, name: "박지훈", steps: 198000, tier: "Platinum" },
  { rank: 4, name: "최유진", steps: 185000, tier: "Platinum" },
  { rank: 5, name: "정다은", steps: 172000, tier: "Platinum" },
  { rank: 6, name: "강현우", steps: 165000, tier: "Platinum" },
  { rank: 7, name: "윤서아", steps: 158000, tier: "Gold" },
  { rank: 8, name: "임재현", steps: 148000, tier: "Gold" },
  { rank: 9, name: "한소영", steps: 142000, tier: "Gold" },
  { rank: 10, name: "오준서", steps: 138000, tier: "Gold" },
];

const POINTS_PER_100_STEPS = 1;
const POINTS_PER_TOKEN = 100;

export function UserChallengePage() {
  const [totalSteps, setTotalSteps] = useState(32400);
  const [customSteps, setCustomSteps] = useState("");
  const [convertAmount, setConvertAmount] = useState("");

  const totalPoints = Math.floor(totalSteps / 100) * POINTS_PER_100_STEPS;

  const addSteps = (amount: number) => {
    setTotalSteps((prev) => prev + amount);
    toast.success(`Added ${formatNumber(amount)} steps!`);
  };

  const handleCustomAdd = () => {
    const val = parseInt(customSteps, 10);
    if (!val || val <= 0) {
      toast.error("Please enter a valid number of steps.");
      return;
    }
    addSteps(val);
    setCustomSteps("");
  };

  const handleConvert = () => {
    const pts = parseInt(convertAmount, 10);
    if (!pts || pts <= 0) {
      toast.error("Please enter a valid point amount.");
      return;
    }
    if (pts < POINTS_PER_TOKEN) {
      toast.error(`Minimum ${POINTS_PER_TOKEN} points required for conversion.`);
      return;
    }
    const tokens = pts / POINTS_PER_TOKEN;
    toast.success(
      `Converted ${formatNumber(pts)} points to ${formatNumber(tokens)} tokens!`
    );
    setConvertAmount("");
  };

  const tierColor = (tier: string) => {
    switch (tier) {
      case "Diamond": return "text-purple-600 dark:text-purple-400";
      case "Platinum": return "text-sky-600 dark:text-sky-400";
      case "Gold": return "text-amber-600 dark:text-amber-400";
      case "Silver": return "text-gray-500 dark:text-gray-400";
      default: return "text-orange-600 dark:text-orange-400";
    }
  };

  return (
    <div>
      {/* Header */}
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          Walking Challenge
        </h1>
        <p className="mt-1 text-gray-600 dark:text-gray-400">
          Track your steps, earn points, and convert to tokens.
        </p>
      </div>

      {/* KPI Cards */}
      <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-8">
        <div className="bg-white dark:bg-gray-800 rounded-xl shadow p-6">
          <p className="text-sm text-gray-500 dark:text-gray-400">Total Steps</p>
          <p className="mt-1 text-3xl font-bold text-indigo-600 dark:text-indigo-400">
            {formatNumber(totalSteps)}
          </p>
        </div>
        <div className="bg-white dark:bg-gray-800 rounded-xl shadow p-6">
          <p className="text-sm text-gray-500 dark:text-gray-400">Points Earned</p>
          <p className="mt-1 text-3xl font-bold text-emerald-600 dark:text-emerald-400">
            {formatNumber(totalPoints)}
          </p>
        </div>
      </div>

      {/* Add Steps */}
      <div className="bg-white dark:bg-gray-800 rounded-xl shadow p-6 mb-8">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          Add Steps
        </h2>
        <div className="flex flex-wrap gap-3 mb-4">
          {[500, 1000, 5000, 10000].map((amount) => (
            <button
              key={amount}
              onClick={() => addSteps(amount)}
              className="px-4 py-2 text-sm font-medium rounded-lg bg-indigo-50 dark:bg-indigo-900/30 text-indigo-700 dark:text-indigo-300 hover:bg-indigo-100 dark:hover:bg-indigo-900/50 transition-colors"
            >
              +{formatNumber(amount)}
            </button>
          ))}
        </div>
        <div className="flex gap-2">
          <input
            type="number"
            value={customSteps}
            onChange={(e) => setCustomSteps(e.target.value)}
            placeholder="Custom steps"
            className="flex-1 px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white placeholder-gray-400 focus:ring-2 focus:ring-indigo-500 focus:border-transparent outline-none"
          />
          <button
            onClick={handleCustomAdd}
            className="px-4 py-2 text-sm font-medium rounded-lg bg-indigo-600 text-white hover:bg-indigo-700 transition-colors"
          >
            Add
          </button>
        </div>
      </div>

      {/* Milestones */}
      <div className="bg-white dark:bg-gray-800 rounded-xl shadow p-6 mb-8">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          Milestones
        </h2>
        <div className="flex flex-wrap gap-3">
          {milestones.map((m) => {
            const reached = totalSteps >= m.steps;
            return (
              <div
                key={m.name}
                className={`flex items-center gap-2 px-4 py-3 rounded-lg border ${
                  reached
                    ? "border-emerald-300 dark:border-emerald-700 bg-emerald-50 dark:bg-emerald-900/20"
                    : "border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800"
                }`}
              >
                <span className="text-xl">{m.emoji}</span>
                <div>
                  <p className={`text-sm font-medium ${reached ? "text-emerald-700 dark:text-emerald-300" : "text-gray-500 dark:text-gray-400"}`}>
                    {m.name}
                  </p>
                  <p className="text-xs text-gray-400 dark:text-gray-500">
                    {formatNumber(m.steps)} steps
                  </p>
                </div>
                {reached && (
                  <span className="text-emerald-500 ml-1 font-bold">&#10003;</span>
                )}
              </div>
            );
          })}
        </div>
      </div>

      {/* Point Conversion */}
      <div className="bg-white dark:bg-gray-800 rounded-xl shadow p-6 mb-8">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-2">
          Convert Points to Tokens
        </h2>
        <p className="text-sm text-gray-500 dark:text-gray-400 mb-4">
          Rate: {POINTS_PER_TOKEN} points = 1 token
        </p>
        <div className="flex gap-2">
          <input
            type="number"
            value={convertAmount}
            onChange={(e) => setConvertAmount(e.target.value)}
            placeholder="Points to convert"
            className="flex-1 px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white placeholder-gray-400 focus:ring-2 focus:ring-emerald-500 focus:border-transparent outline-none"
          />
          <button
            onClick={handleConvert}
            className="px-4 py-2 text-sm font-medium rounded-lg bg-emerald-600 text-white hover:bg-emerald-700 transition-colors"
          >
            Convert to Token
          </button>
        </div>
      </div>

      {/* Leaderboard */}
      <div className="bg-white dark:bg-gray-800 rounded-xl shadow p-6">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          Leaderboard
        </h2>
        <div className="overflow-x-auto">
          <table className="w-full text-sm">
            <thead>
              <tr className="border-b border-gray-200 dark:border-gray-700">
                <th className="text-left py-3 px-2 font-medium text-gray-500 dark:text-gray-400">Rank</th>
                <th className="text-left py-3 px-2 font-medium text-gray-500 dark:text-gray-400">Name</th>
                <th className="text-right py-3 px-2 font-medium text-gray-500 dark:text-gray-400">Steps</th>
                <th className="text-right py-3 px-2 font-medium text-gray-500 dark:text-gray-400">Tier</th>
              </tr>
            </thead>
            <tbody>
              {leaderboard.map((entry) => (
                <tr key={entry.rank} className="border-b border-gray-100 dark:border-gray-700/50">
                  <td className="py-3 px-2 font-medium text-gray-900 dark:text-white">
                    {entry.rank <= 3 ? (
                      <span className="text-base">
                        {entry.rank === 1 ? "🥇" : entry.rank === 2 ? "🥈" : "🥉"}
                      </span>
                    ) : (
                      `#${entry.rank}`
                    )}
                  </td>
                  <td className="py-3 px-2 text-gray-900 dark:text-white">{entry.name}</td>
                  <td className="py-3 px-2 text-right text-gray-700 dark:text-gray-300">
                    {formatNumber(entry.steps)}
                  </td>
                  <td className={`py-3 px-2 text-right font-medium ${tierColor(entry.tier)}`}>
                    {entry.tier}
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
