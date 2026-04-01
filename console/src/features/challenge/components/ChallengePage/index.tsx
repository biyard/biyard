import { useState } from "react";
import { useMode } from "@/contexts/ModeContext";
import { useBrands } from "@/contexts/BrandContext";
import {
  getChallenges,
  getUserChallenge,
  getLeaderboard,
  formatNumber,
  type UserChallenge,
} from "@/lib/mock-data";
import { toast } from "sonner";

const STEP_BUTTONS = [500, 1_000, 5_000, 10_000] as const;

export function ChallengePage() {
  const { isAdmin } = useMode();
  const { brands } = useBrands();

  const challenges = getChallenges();
  const leaderboard = getLeaderboard();

  const [userChallenge, setUserChallenge] = useState<UserChallenge>(
    getUserChallenge,
  );
  const [convertAmount, setConvertAmount] = useState<number>(0);

  const addSteps = (steps: number) => {
    setUserChallenge((prev) => ({
      ...prev,
      total_steps: prev.total_steps + steps,
      total_points: prev.total_points + Math.floor(steps / 100),
      pending_points: prev.pending_points + Math.floor(steps / 100),
    }));
    toast.success(`Added ${formatNumber(steps)} steps!`);
  };

  const handleCustomSteps = () => {
    const input = window.prompt("Enter custom step count:");
    if (!input) return;
    const steps = parseInt(input, 10);
    if (isNaN(steps) || steps <= 0) {
      toast.error("Please enter a valid positive number.");
      return;
    }
    addSteps(steps);
  };

  const handleConvert = () => {
    if (convertAmount <= 0) {
      toast.error("Enter a positive number of points to convert.");
      return;
    }
    if (convertAmount > userChallenge.pending_points) {
      toast.error("Not enough points.");
      return;
    }
    setUserChallenge((prev) => ({
      ...prev,
      pending_points: prev.pending_points - convertAmount,
      tokens_earned: prev.tokens_earned + convertAmount / prev.points_per_token,
    }));
    toast.success(
      `Converted ${formatNumber(convertAmount)} points to ${formatNumber(convertAmount / userChallenge.points_per_token)} tokens!`,
    );
    setConvertAmount(0);
  };

  if (isAdmin) {
    return (
      <div>
        <div className="mb-6">
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
            Walking Challenges
          </h1>
          <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
            Manage and monitor active challenges across {brands.length} brand(s).
          </p>
        </div>

        <div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
          {challenges.map((c) => (
            <div
              key={c.id}
              className="rounded-lg border border-gray-200 bg-white p-6 shadow dark:border-gray-700 dark:bg-gray-800"
            >
              <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
                {c.name}
              </h3>
              <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                {c.start_date} &mdash; {c.end_date}
              </p>

              <dl className="mt-4 grid grid-cols-2 gap-4 text-sm">
                <div>
                  <dt className="text-gray-500 dark:text-gray-400">
                    Participants
                  </dt>
                  <dd className="mt-1 font-semibold text-gray-900 dark:text-white">
                    {formatNumber(c.total_participants)}
                  </dd>
                </div>
                <div>
                  <dt className="text-gray-500 dark:text-gray-400">
                    Total Steps
                  </dt>
                  <dd className="mt-1 font-semibold text-gray-900 dark:text-white">
                    {formatNumber(c.total_steps_global)}
                  </dd>
                </div>
                <div className="col-span-2">
                  <dt className="text-gray-500 dark:text-gray-400">
                    Steps per Point
                  </dt>
                  <dd className="mt-1 font-semibold text-gray-900 dark:text-white">
                    {formatNumber(c.steps_per_point)}
                  </dd>
                </div>
              </dl>
            </div>
          ))}
        </div>
      </div>
    );
  }

  return (
    <div>
      <div className="mb-6">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          Walking Challenge
        </h1>
        <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
          Track your steps, earn points, and climb the leaderboard.
        </p>
      </div>

      {/* KPI Cards */}
      <div className="mb-6 grid grid-cols-1 gap-6 sm:grid-cols-2">
        <div className="rounded-lg bg-white p-6 shadow dark:bg-gray-800">
          <p className="text-sm font-medium text-gray-500 dark:text-gray-400">
            Total Steps
          </p>
          <p className="mt-2 text-3xl font-bold text-gray-900 dark:text-white">
            {formatNumber(userChallenge.total_steps)}
          </p>
        </div>
        <div className="rounded-lg bg-white p-6 shadow dark:bg-gray-800">
          <p className="text-sm font-medium text-gray-500 dark:text-gray-400">
            Points
          </p>
          <p className="mt-2 text-3xl font-bold text-blue-600 dark:text-blue-400">
            {formatNumber(userChallenge.total_points)}
          </p>
        </div>
      </div>

      {/* Step Input */}
      <div className="mb-6 rounded-lg bg-white p-6 shadow dark:bg-gray-800">
        <h2 className="mb-4 text-lg font-semibold text-gray-900 dark:text-white">
          Add Steps
        </h2>
        <div className="flex flex-wrap gap-3">
          {STEP_BUTTONS.map((s) => (
            <button
              key={s}
              onClick={() => addSteps(s)}
              className="rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
            >
              +{formatNumber(s)}
            </button>
          ))}
          <button
            onClick={handleCustomSteps}
            className="rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 dark:border-gray-600 dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600"
          >
            Custom
          </button>
        </div>
      </div>

      {/* Milestones */}
      <div className="mb-6 rounded-lg bg-white p-6 shadow dark:bg-gray-800">
        <h2 className="mb-4 text-lg font-semibold text-gray-900 dark:text-white">
          Milestones
        </h2>
        <ul className="space-y-3">
          {userChallenge.milestones.map((m, i) => (
            <li
              key={i}
              className={`flex items-center justify-between rounded-md border px-4 py-3 ${
                m.completed
                  ? "border-green-200 bg-green-50 dark:border-green-800 dark:bg-green-900/20"
                  : "border-gray-200 bg-gray-50 dark:border-gray-700 dark:bg-gray-800"
              }`}
            >
              <div className="flex items-center gap-3">
                <span className="text-xl">{m.emoji}</span>
                <span
                  className={`text-sm font-medium ${
                    m.completed
                      ? "text-green-700 dark:text-green-400"
                      : "text-gray-700 dark:text-gray-300"
                  }`}
                >
                  {m.name}
                </span>
              </div>
              <div className="flex items-center gap-3">
                <span className="text-xs text-gray-500 dark:text-gray-400">
                  +{formatNumber(m.reward)} pts
                </span>
                {m.completed ? (
                  <span className="text-green-600 dark:text-green-400">
                    &#10003;
                  </span>
                ) : (
                  <span className="h-5 w-5 rounded border border-gray-300 dark:border-gray-600" />
                )}
              </div>
            </li>
          ))}
        </ul>
      </div>

      {/* Point Conversion */}
      <div className="mb-6 rounded-lg bg-white p-6 shadow dark:bg-gray-800">
        <h2 className="mb-4 text-lg font-semibold text-gray-900 dark:text-white">
          Convert Points to Tokens
        </h2>
        <div className="flex items-center gap-3">
          <input
            type="number"
            min={0}
            value={convertAmount || ""}
            onChange={(e) => setConvertAmount(parseInt(e.target.value, 10) || 0)}
            placeholder="Amount"
            className="w-40 rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
          />
          <button
            onClick={handleConvert}
            className="rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
          >
            Convert to Token
          </button>
        </div>
        <div className="mt-3 rounded-md bg-blue-50 p-3 text-xs text-blue-800 dark:bg-blue-900/20 dark:text-blue-300">
          <p>
            Conversion rate: <strong>{userChallenge.points_per_token} points = 1 token</strong>
          </p>
          <p className="mt-1">
            Pending points: <strong>{formatNumber(userChallenge.pending_points)}</strong>
          </p>
        </div>
      </div>

      {/* Leaderboard */}
      <div className="rounded-lg bg-white shadow dark:bg-gray-800">
        <div className="px-6 py-4">
          <h2 className="text-lg font-semibold text-gray-900 dark:text-white">
            Leaderboard
          </h2>
        </div>
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
            <thead className="bg-gray-50 dark:bg-gray-700">
              <tr>
                {["Rank", "Name", "Steps", "Tier"].map((h) => (
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
              {leaderboard.map((entry) => {
                const tierColor: Record<string, string> = {
                  Diamond: "text-purple-600 dark:text-purple-400",
                  Platinum: "text-blue-600 dark:text-blue-400",
                  Gold: "text-yellow-600 dark:text-yellow-400",
                  Silver: "text-gray-500 dark:text-gray-300",
                  Bronze: "text-orange-600 dark:text-orange-400",
                };
                return (
                  <tr key={entry.rank}>
                    <td className="whitespace-nowrap px-6 py-4 text-sm font-medium text-gray-900 dark:text-white">
                      #{entry.rank}
                    </td>
                    <td className="whitespace-nowrap px-6 py-4 text-sm text-gray-900 dark:text-white">
                      {entry.name}
                    </td>
                    <td className="whitespace-nowrap px-6 py-4 text-sm text-gray-700 dark:text-gray-300">
                      {formatNumber(entry.steps)}
                    </td>
                    <td
                      className={`whitespace-nowrap px-6 py-4 text-sm font-semibold ${tierColor[entry.tier] || ""}`}
                    >
                      {entry.tier}
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}
