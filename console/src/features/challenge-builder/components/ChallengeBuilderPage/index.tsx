import { useState } from "react";
import { Copy, Plus, Pause, Play, Trash2, ArrowLeft } from "lucide-react";
import { toast } from "sonner";

interface Template {
  id: string;
  name: string;
  icon: string;
  description: string;
  activityType: string;
  defaultReward: number;
  unit: string;
  integrationMethod: string;
  popular: boolean;
}

interface ActiveChallenge {
  id: string;
  name: string;
  templateType: string;
  reward: number;
  unit: string;
  startDate: string;
  endDate: string;
  status: "active" | "paused";
}

const templates: Template[] = [
  {
    id: "walking",
    name: "Walking Challenge",
    icon: "\u{1F6B6}",
    description: "Track customer walking steps via GPS or pedometer",
    activityType: "steps",
    defaultReward: 1,
    unit: "steps per point",
    integrationMethod: "Mobile SDK (GPS/Pedometer)",
    popular: true,
  },
  {
    id: "running",
    name: "Running Challenge",
    icon: "\u{1F3C3}",
    description: "Track running distance with GPS verification",
    activityType: "distance",
    defaultReward: 10,
    unit: "meters per point",
    integrationMethod: "Mobile SDK (GPS)",
    popular: true,
  },
  {
    id: "purchase",
    name: "Purchase Reward",
    icon: "\u{1F6CD}\uFE0F",
    description: "Reward customers on every purchase automatically",
    activityType: "purchase",
    defaultReward: 2,
    unit: "% of purchase",
    integrationMethod: "POS API / Webhook",
    popular: true,
  },
  {
    id: "checkin",
    name: "Check-in",
    icon: "\u{1F4CD}",
    description: "Reward store visits via QR code or geofencing",
    activityType: "visit",
    defaultReward: 50,
    unit: "points per visit",
    integrationMethod: "QR Code / Geofence SDK",
    popular: false,
  },
  {
    id: "stamp",
    name: "Stamp Collection",
    icon: "\u{1F3AB}",
    description: "N visits = bonus reward (loyalty card style)",
    activityType: "stamp",
    defaultReward: 100,
    unit: "points per completion",
    integrationMethod: "POS API / Mobile SDK",
    popular: false,
  },
  {
    id: "sns",
    name: "SNS Share",
    icon: "\u{1F4F1}",
    description: "Reward social media sharing and engagement",
    activityType: "share",
    defaultReward: 30,
    unit: "points per share",
    integrationMethod: "Web SDK / API",
    popular: false,
  },
  {
    id: "custom",
    name: "Custom Action",
    icon: "\u26A1",
    description: "Define any action via API. Full flexibility.",
    activityType: "custom",
    defaultReward: 1,
    unit: "configurable",
    integrationMethod: "REST API",
    popular: false,
  },
];

export function ChallengeBuilderPage() {
  const [selectedTemplate, setSelectedTemplate] = useState<Template | null>(
    null
  );
  const [challengeName, setChallengeName] = useState("");
  const [rewardAmount, setRewardAmount] = useState<number | "">("");
  const [startDate, setStartDate] = useState("");
  const [endDate, setEndDate] = useState("");
  const [pointsPerToken, setPointsPerToken] = useState<number | "">(100);
  const [description, setDescription] = useState("");
  const [bannerText, setBannerText] = useState("");
  const [showCode, setShowCode] = useState(false);
  const [activeChallenges, setActiveChallenges] = useState<ActiveChallenge[]>([
    {
      id: "1",
      name: "Le Mouton Walking Challenge",
      templateType: "Walking Challenge",
      reward: 1,
      unit: "steps per point",
      startDate: "2026-01-01",
      endDate: "2026-06-30",
      status: "active",
    },
    {
      id: "2",
      name: "Cafe Blossom Stamp Rally",
      templateType: "Stamp Collection",
      reward: 100,
      unit: "points per completion",
      startDate: "2026-02-01",
      endDate: "2026-08-31",
      status: "active",
    },
  ]);

  const handleSelectTemplate = (template: Template) => {
    setSelectedTemplate(template);
    setRewardAmount(template.defaultReward);
    setShowCode(false);
    setChallengeName("");
    setStartDate("");
    setEndDate("");
    setPointsPerToken(100);
    setDescription("");
    setBannerText("");
  };

  const handleBackToTemplates = () => {
    setSelectedTemplate(null);
    setShowCode(false);
  };

  const handleCreateChallenge = (e: React.FormEvent) => {
    e.preventDefault();
    if (!selectedTemplate || !challengeName || !startDate || !endDate) {
      toast.error("Please fill in all required fields.");
      return;
    }

    const newChallenge: ActiveChallenge = {
      id: String(Date.now()),
      name: challengeName,
      templateType: selectedTemplate.name,
      reward: typeof rewardAmount === "number" ? rewardAmount : selectedTemplate.defaultReward,
      unit: selectedTemplate.unit,
      startDate,
      endDate,
      status: "active",
    };

    setActiveChallenges((prev) => [newChallenge, ...prev]);
    toast.success(`Challenge "${challengeName}" created successfully!`);
    setShowCode(true);
  };

  const handleToggleStatus = (id: string) => {
    setActiveChallenges((prev) =>
      prev.map((c) =>
        c.id === id
          ? { ...c, status: c.status === "active" ? "paused" : "active" }
          : c
      )
    );
  };

  const handleDelete = (id: string) => {
    setActiveChallenges((prev) => prev.filter((c) => c.id !== id));
    toast.success("Challenge deleted.");
  };

  const integrationCode = selectedTemplate
    ? `// Example: Sending activity data from your app
const response = await fetch('https://api.biyard.co/v1/projects/YOUR_PROJECT_ID/activities', {
  method: 'POST',
  headers: {
    'Authorization': 'Bearer YOUR_API_KEY',
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    meta_user_id: 'customer-123',
    activity_type: '${selectedTemplate.activityType}',  // from template
    value: 8500,             // activity value
    description: '${challengeName || "Daily activity"}'
  })
});`
    : "";

  const handleCopyCode = () => {
    navigator.clipboard.writeText(integrationCode);
    toast.success("Code copied to clipboard!");
  };

  const inputClass =
    "w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 px-3 py-2 text-sm text-gray-900 dark:text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500";

  return (
    <div>
      {/* Header */}
      <div className="mb-6">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          Challenge Templates
        </h1>
        <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
          Create challenges from templates and integrate via API — no Biyard
          involvement needed.
        </p>
      </div>

      {/* Section 1: Template Grid */}
      {!selectedTemplate && (
        <div className="mb-8">
          <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
            Choose a Template
          </h2>
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
            {templates.map((tpl) => (
              <div
                key={tpl.id}
                className="bg-white dark:bg-gray-800 rounded-lg shadow p-5 flex flex-col gap-3 border border-gray-200 dark:border-gray-700 hover:border-blue-400 dark:hover:border-blue-500 transition-colors"
              >
                <div className="flex items-start justify-between">
                  <span className="text-3xl">{tpl.icon}</span>
                  {tpl.popular && (
                    <span className="text-xs font-medium px-2 py-0.5 rounded-full bg-amber-100 text-amber-700 dark:bg-amber-900 dark:text-amber-300">
                      Popular
                    </span>
                  )}
                </div>
                <h3 className="font-semibold text-gray-900 dark:text-white text-sm">
                  {tpl.name}
                </h3>
                <p className="text-xs text-gray-500 dark:text-gray-400 flex-1">
                  {tpl.description}
                </p>
                <p className="text-xs text-gray-400 dark:text-gray-500">
                  Integration: {tpl.integrationMethod}
                </p>
                <button
                  onClick={() => handleSelectTemplate(tpl)}
                  className="mt-auto w-full flex items-center justify-center gap-1.5 px-3 py-2 bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium rounded-md transition-colors"
                >
                  <Plus className="h-4 w-4" />
                  Use Template
                </button>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Section 2: Configuration Form */}
      {selectedTemplate && (
        <div className="mb-8">
          <button
            onClick={handleBackToTemplates}
            className="flex items-center gap-1.5 text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white mb-4 transition-colors"
          >
            <ArrowLeft className="h-4 w-4" />
            Back to Templates
          </button>

          <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6 border border-gray-200 dark:border-gray-700">
            <div className="flex items-center gap-3 mb-6">
              <span className="text-3xl">{selectedTemplate.icon}</span>
              <div>
                <h2 className="text-lg font-semibold text-gray-900 dark:text-white">
                  {selectedTemplate.name}
                </h2>
                <p className="text-sm text-gray-500 dark:text-gray-400">
                  {selectedTemplate.description}
                </p>
              </div>
            </div>

            <form
              onSubmit={handleCreateChallenge}
              className="grid grid-cols-1 md:grid-cols-2 gap-4"
            >
              {/* Selected Template (read-only) */}
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Template
                </label>
                <input
                  type="text"
                  value={selectedTemplate.name}
                  readOnly
                  className={`${inputClass} bg-gray-50 dark:bg-gray-600 cursor-not-allowed`}
                />
              </div>

              {/* Integration Method (read-only) */}
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Integration Method
                </label>
                <input
                  type="text"
                  value={selectedTemplate.integrationMethod}
                  readOnly
                  className={`${inputClass} bg-gray-50 dark:bg-gray-600 cursor-not-allowed`}
                />
              </div>

              {/* Challenge Name */}
              <div className="md:col-span-2">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Challenge Name *
                </label>
                <input
                  type="text"
                  value={challengeName}
                  onChange={(e) => setChallengeName(e.target.value)}
                  placeholder="e.g. Summer Walking Challenge 2026"
                  className={inputClass}
                />
              </div>

              {/* Reward Amount */}
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Reward Amount
                </label>
                <input
                  type="number"
                  value={rewardAmount}
                  onChange={(e) =>
                    setRewardAmount(
                      e.target.value === "" ? "" : Number(e.target.value)
                    )
                  }
                  min={0}
                  className={inputClass}
                />
              </div>

              {/* Unit (read-only) */}
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Unit
                </label>
                <input
                  type="text"
                  value={selectedTemplate.unit}
                  readOnly
                  className={`${inputClass} bg-gray-50 dark:bg-gray-600 cursor-not-allowed`}
                />
              </div>

              {/* Start Date */}
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Start Date *
                </label>
                <input
                  type="date"
                  value={startDate}
                  onChange={(e) => setStartDate(e.target.value)}
                  className={inputClass}
                />
              </div>

              {/* End Date */}
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  End Date *
                </label>
                <input
                  type="date"
                  value={endDate}
                  onChange={(e) => setEndDate(e.target.value)}
                  className={inputClass}
                />
              </div>

              {/* Points per Token */}
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Points per Token
                </label>
                <input
                  type="number"
                  value={pointsPerToken}
                  onChange={(e) =>
                    setPointsPerToken(
                      e.target.value === "" ? "" : Number(e.target.value)
                    )
                  }
                  min={1}
                  className={inputClass}
                />
              </div>

              {/* Banner Text */}
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Banner Text
                </label>
                <input
                  type="text"
                  value={bannerText}
                  onChange={(e) => setBannerText(e.target.value)}
                  placeholder="e.g. Walk more, earn more!"
                  className={inputClass}
                />
              </div>

              {/* Description */}
              <div className="md:col-span-2">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Description
                </label>
                <textarea
                  value={description}
                  onChange={(e) => setDescription(e.target.value)}
                  rows={3}
                  placeholder="Promotional text for your challenge..."
                  className={`${inputClass} resize-none`}
                />
              </div>

              {/* Submit */}
              <div className="md:col-span-2">
                <button
                  type="submit"
                  className="w-full sm:w-auto px-6 py-2.5 bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium rounded-md transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                >
                  Create Challenge
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      {/* Section 3: Integration Code */}
      {showCode && selectedTemplate && (
        <div className="mb-8">
          <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6 border border-gray-200 dark:border-gray-700">
            <div className="flex items-center justify-between mb-4">
              <h2 className="text-lg font-semibold text-gray-900 dark:text-white">
                Integration Code
              </h2>
              <button
                onClick={handleCopyCode}
                className="flex items-center gap-1.5 px-3 py-1.5 text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-md transition-colors"
              >
                <Copy className="h-4 w-4" />
                Copy Code
              </button>
            </div>
            <pre className="bg-gray-900 text-gray-100 rounded-md p-4 text-sm overflow-x-auto">
              <code>{integrationCode}</code>
            </pre>
          </div>
        </div>
      )}

      {/* Section 4: Active Challenges */}
      <div>
        <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          Active Challenges
        </h2>
        {activeChallenges.length === 0 ? (
          <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-8 text-center">
            <p className="text-sm text-gray-500 dark:text-gray-400">
              No active challenges yet. Pick a template above to get started.
            </p>
          </div>
        ) : (
          <div className="bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden border border-gray-200 dark:border-gray-700">
            <div className="overflow-x-auto">
              <table className="w-full text-sm text-left">
                <thead className="bg-gray-50 dark:bg-gray-700 text-gray-600 dark:text-gray-300 text-xs uppercase">
                  <tr>
                    <th className="px-4 py-3">Name</th>
                    <th className="px-4 py-3">Template</th>
                    <th className="px-4 py-3">Reward</th>
                    <th className="px-4 py-3">Period</th>
                    <th className="px-4 py-3">Status</th>
                    <th className="px-4 py-3 text-right">Actions</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-gray-200 dark:divide-gray-700">
                  {activeChallenges.map((challenge) => (
                    <tr
                      key={challenge.id}
                      className="hover:bg-gray-50 dark:hover:bg-gray-750"
                    >
                      <td className="px-4 py-3 font-medium text-gray-900 dark:text-white">
                        {challenge.name}
                      </td>
                      <td className="px-4 py-3 text-gray-600 dark:text-gray-400">
                        {challenge.templateType}
                      </td>
                      <td className="px-4 py-3 text-gray-600 dark:text-gray-400">
                        {challenge.reward} {challenge.unit}
                      </td>
                      <td className="px-4 py-3 text-gray-600 dark:text-gray-400">
                        {challenge.startDate} ~ {challenge.endDate}
                      </td>
                      <td className="px-4 py-3">
                        <span
                          className={`inline-flex items-center gap-1 text-xs font-medium ${
                            challenge.status === "active"
                              ? "text-green-600 dark:text-green-400"
                              : "text-yellow-600 dark:text-yellow-400"
                          }`}
                        >
                          <span
                            className={`h-1.5 w-1.5 rounded-full ${
                              challenge.status === "active"
                                ? "bg-green-500"
                                : "bg-yellow-500"
                            }`}
                          />
                          {challenge.status}
                        </span>
                      </td>
                      <td className="px-4 py-3 text-right">
                        <div className="flex items-center justify-end gap-1">
                          <button
                            onClick={() => handleToggleStatus(challenge.id)}
                            className="p-1.5 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 rounded transition-colors"
                            title={
                              challenge.status === "active" ? "Pause" : "Resume"
                            }
                          >
                            {challenge.status === "active" ? (
                              <Pause className="h-4 w-4" />
                            ) : (
                              <Play className="h-4 w-4" />
                            )}
                          </button>
                          <button
                            onClick={() => handleDelete(challenge.id)}
                            className="p-1.5 text-gray-500 hover:text-red-600 dark:text-gray-400 dark:hover:text-red-400 rounded transition-colors"
                            title="Delete"
                          >
                            <Trash2 className="h-4 w-4" />
                          </button>
                        </div>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
