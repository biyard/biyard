import { useMode } from "@/contexts/ModeContext";
import { getOnboardingStats } from "@/lib/mock-data";
import { useNavigate } from "react-router-dom";

export function OnboardingPage() {
  const { isAdmin } = useMode();
  const navigate = useNavigate();
  const steps = getOnboardingStats();

  if (!isAdmin) {
    return (
      <div className="flex items-center justify-center min-h-[60vh]">
        <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-10 text-center max-w-md w-full">
          <div className="text-6xl mb-4">&#10003;</div>
          <h2 className="text-2xl font-bold text-gray-900 dark:text-white mb-2">
            Onboarding Complete!
          </h2>
          <p className="text-gray-600 dark:text-gray-400 mb-6">
            You're all set. Start exploring the Biyard Launchpad platform.
          </p>
          <button
            onClick={() => navigate("/dashboard")}
            className="px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg transition-colors"
          >
            Get Started
          </button>
        </div>
      </div>
    );
  }

  return (
    <div>
      <div className="mb-6">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          Onboarding Analytics
        </h1>
        <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
          Funnel conversion rates for the user onboarding flow.
        </p>
      </div>

      <div className="bg-white dark:bg-gray-800 shadow rounded-lg overflow-hidden">
        <table className="w-full">
          <thead>
            <tr className="border-b border-gray-200 dark:border-gray-700">
              <th className="text-left px-6 py-4 text-sm font-semibold text-gray-700 dark:text-gray-300">
                Step
              </th>
              <th className="text-left px-6 py-4 text-sm font-semibold text-gray-700 dark:text-gray-300 w-24">
                Rate
              </th>
              <th className="text-left px-6 py-4 text-sm font-semibold text-gray-700 dark:text-gray-300">
                Progress
              </th>
            </tr>
          </thead>
          <tbody>
            {steps.map((step, index) => {
              const pctDisplay = Math.round(step.pct * 100);
              return (
                <tr
                  key={index}
                  className="border-b border-gray-100 dark:border-gray-700 last:border-b-0"
                >
                  <td className="px-6 py-4 text-sm font-medium text-gray-900 dark:text-white">
                    {step.label}
                  </td>
                  <td className="px-6 py-4 text-sm text-gray-700 dark:text-gray-300 font-mono">
                    {pctDisplay}%
                  </td>
                  <td className="px-6 py-4">
                    <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-5 overflow-hidden">
                      <div
                        className="bg-blue-500 h-5 rounded-full flex items-center justify-end pr-2 transition-all duration-500"
                        style={{ width: `${pctDisplay}%` }}
                      >
                        {pctDisplay >= 15 && (
                          <span className="text-xs font-semibold text-white">
                            {pctDisplay}%
                          </span>
                        )}
                      </div>
                    </div>
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
    </div>
  );
}
