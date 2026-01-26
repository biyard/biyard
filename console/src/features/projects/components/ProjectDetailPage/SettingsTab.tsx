import type { TabProps } from "./types";

export function SettingsTab({ t }: TabProps) {
  return (
    <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-6">
      <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">
        {t.settings}
      </h3>
      <p className="text-gray-500 dark:text-gray-400">
        Project settings will be available here.
      </p>
    </div>
  );
}
