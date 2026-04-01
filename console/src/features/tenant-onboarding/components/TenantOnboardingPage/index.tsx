import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { Check, Copy, ArrowLeft, ArrowRight, Rocket } from "lucide-react";
import { toast } from "sonner";

const INDUSTRIES = [
  "Fashion",
  "F&B",
  "Sports",
  "Tech",
  "Retail",
  "Healthcare",
  "Other",
];

const ACTIVITY_TYPES = [
  { label: "Walking", defaultReward: 10 },
  { label: "Running", defaultReward: 20 },
  { label: "Purchase", defaultReward: 50 },
  { label: "Check-in", defaultReward: 15 },
  { label: "Stamp", defaultReward: 5 },
  { label: "Custom", defaultReward: 10 },
];

const TOTAL_STEPS = 5;

export function TenantOnboardingPage() {
  const navigate = useNavigate();
  const [currentStep, setCurrentStep] = useState(0);

  // Step 1 state
  const [projectName, setProjectName] = useState("");
  const [industry, setIndustry] = useState("");
  const [description, setDescription] = useState("");
  const [monthlyTokenSupply, setMonthlyTokenSupply] = useState(10000);

  // Step 2 state
  const [purchaseRewardRate, setPurchaseRewardRate] = useState(2);
  const [activityType, setActivityType] = useState("Walking");
  const [activityReward, setActivityReward] = useState(10);

  // Step 3 state
  const [apiKeyCopied, setApiKeyCopied] = useState(false);

  // Step 4 state
  const [purchaseTestResult, setPurchaseTestResult] = useState<
    "idle" | "loading" | "success"
  >("idle");
  const [activityTestResult, setActivityTestResult] = useState<
    "idle" | "loading" | "success"
  >("idle");

  const mockApiKey = "by_live_a3f8c1d2e5b7g9h4j6k8m0n2p4r6t8";

  const handleCopyApiKey = () => {
    navigator.clipboard.writeText(mockApiKey).then(() => {
      setApiKeyCopied(true);
      toast.success("API key copied to clipboard");
      setTimeout(() => setApiKeyCopied(false), 3000);
    });
  };

  const handleActivityTypeChange = (type: string) => {
    setActivityType(type);
    const found = ACTIVITY_TYPES.find((a) => a.label === type);
    if (found) {
      setActivityReward(found.defaultReward);
    }
  };

  const runPurchaseTest = () => {
    setPurchaseTestResult("loading");
    setTimeout(() => {
      setPurchaseTestResult("success");
      toast.success("Purchase test completed successfully!");
    }, 1500);
  };

  const runActivityTest = () => {
    setActivityTestResult("loading");
    setTimeout(() => {
      setActivityTestResult("success");
      toast.success("Activity test completed successfully!");
    }, 1500);
  };

  const purchasePreview = Math.floor(100000 * (purchaseRewardRate / 100));

  const curlExample = `curl -X POST https://api.biyard.co/v1/points/issue \\
  -H "Authorization: Bearer ${mockApiKey}" \\
  -H "Content-Type: application/json" \\
  -d '{"user_id": "user_123", "amount": 100, "reason": "purchase"}'`;

  const testCurlPurchase = `curl -X POST https://api.biyard.co/v1/points/issue \\
  -H "Authorization: Bearer ${mockApiKey}" \\
  -d '{"user_id": "test_user", "amount": ${purchasePreview}, "type": "purchase", "purchase_amount": 100000}'`;

  const testCurlActivity = `curl -X POST https://api.biyard.co/v1/points/issue \\
  -H "Authorization: Bearer ${mockApiKey}" \\
  -d '{"user_id": "test_user", "amount": ${activityReward}, "type": "activity", "activity": "${activityType.toLowerCase()}"}'`;

  return (
    <div className="max-w-3xl mx-auto">
      {/* Progress Bar */}
      <div className="flex items-center justify-center gap-3 mb-8">
        {Array.from({ length: TOTAL_STEPS }).map((_, i) => (
          <div key={i} className="flex items-center gap-3">
            <div
              className={`w-3 h-3 rounded-full transition-colors ${
                i < currentStep
                  ? "bg-blue-600"
                  : i === currentStep
                    ? "bg-blue-600 ring-4 ring-blue-100 dark:ring-blue-900"
                    : "bg-gray-300 dark:bg-gray-600"
              }`}
            />
            {i < TOTAL_STEPS - 1 && (
              <div
                className={`w-12 h-0.5 ${
                  i < currentStep
                    ? "bg-blue-600"
                    : "bg-gray-300 dark:bg-gray-600"
                }`}
              />
            )}
          </div>
        ))}
      </div>

      {/* Step Content */}
      <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-8">
        {/* Step 0: Welcome */}
        {currentStep === 0 && (
          <div className="text-center py-8">
            <div className="flex justify-center mb-6">
              <Rocket className="w-16 h-16 text-blue-600" />
            </div>
            <h1 className="text-3xl font-bold text-gray-900 dark:text-white mb-3">
              Welcome to Biyard Platform!
            </h1>
            <p className="text-lg text-gray-600 dark:text-gray-400 mb-8">
              Set up your token economy in 5 minutes
            </p>
            <button
              onClick={() => setCurrentStep(1)}
              className="px-8 py-3 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg transition-colors text-lg"
            >
              Get Started
            </button>
          </div>
        )}

        {/* Step 1: Create Project */}
        {currentStep === 1 && (
          <div>
            <h2 className="text-2xl font-bold text-gray-900 dark:text-white mb-1">
              Create Your Project
            </h2>
            <p className="text-sm text-gray-500 dark:text-gray-400 mb-6">
              Tell us about your business so we can tailor your token economy.
            </p>

            <div className="space-y-5">
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Project Name
                </label>
                <input
                  type="text"
                  value={projectName}
                  onChange={(e) => setProjectName(e.target.value)}
                  placeholder="e.g., Le Mouton Rewards"
                  className="w-full px-4 py-2.5 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Industry
                </label>
                <select
                  value={industry}
                  onChange={(e) => setIndustry(e.target.value)}
                  className="w-full px-4 py-2.5 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none"
                >
                  <option value="">Select industry...</option>
                  {INDUSTRIES.map((ind) => (
                    <option key={ind} value={ind}>
                      {ind}
                    </option>
                  ))}
                </select>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Description
                </label>
                <textarea
                  value={description}
                  onChange={(e) => setDescription(e.target.value)}
                  placeholder="Briefly describe your project and goals..."
                  rows={3}
                  className="w-full px-4 py-2.5 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none resize-none"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Monthly Token Supply
                </label>
                <input
                  type="number"
                  value={monthlyTokenSupply}
                  onChange={(e) =>
                    setMonthlyTokenSupply(Number(e.target.value))
                  }
                  className="w-full px-4 py-2.5 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none"
                />
                <p className="mt-1 text-xs text-gray-400">
                  Total tokens available for distribution each month
                </p>
              </div>
            </div>
          </div>
        )}

        {/* Step 2: Configure Rewards */}
        {currentStep === 2 && (
          <div>
            <h2 className="text-2xl font-bold text-gray-900 dark:text-white mb-1">
              Configure Rewards
            </h2>
            <p className="text-sm text-gray-500 dark:text-gray-400 mb-6">
              Define how customers earn points through purchases and activities.
            </p>

            <div className="space-y-6">
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Purchase Reward Rate:{" "}
                  <span className="text-blue-600 font-bold">
                    {purchaseRewardRate}%
                  </span>
                </label>
                <input
                  type="range"
                  min={1}
                  max={5}
                  step={0.5}
                  value={purchaseRewardRate}
                  onChange={(e) =>
                    setPurchaseRewardRate(Number(e.target.value))
                  }
                  className="w-full h-2 bg-gray-200 dark:bg-gray-600 rounded-lg appearance-none cursor-pointer accent-blue-600"
                />
                <div className="flex justify-between text-xs text-gray-400 mt-1">
                  <span>1%</span>
                  <span>5%</span>
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Activity Type
                </label>
                <select
                  value={activityType}
                  onChange={(e) => handleActivityTypeChange(e.target.value)}
                  className="w-full px-4 py-2.5 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none"
                >
                  {ACTIVITY_TYPES.map((a) => (
                    <option key={a.label} value={a.label}>
                      {a.label}
                    </option>
                  ))}
                </select>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Activity Reward (points per action)
                </label>
                <input
                  type="number"
                  value={activityReward}
                  onChange={(e) => setActivityReward(Number(e.target.value))}
                  min={1}
                  className="w-full px-4 py-2.5 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none"
                />
              </div>

              <div className="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4">
                <p className="text-sm text-blue-800 dark:text-blue-300">
                  <span className="font-semibold">This means:</span> A customer
                  buying{" "}
                  <span className="font-mono font-bold">
                    &#8361;100,000
                  </span>{" "}
                  gets{" "}
                  <span className="font-mono font-bold text-blue-600 dark:text-blue-400">
                    {purchasePreview.toLocaleString()} points
                  </span>
                </p>
              </div>
            </div>
          </div>
        )}

        {/* Step 3: Get API Key */}
        {currentStep === 3 && (
          <div>
            <h2 className="text-2xl font-bold text-gray-900 dark:text-white mb-1">
              Your API Key
            </h2>
            <p className="text-sm text-gray-500 dark:text-gray-400 mb-6">
              Use this key to authenticate requests to the Biyard API.
            </p>

            <div className="space-y-5">
              <div className="bg-gray-50 dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded-lg p-4">
                <div className="flex items-center justify-between">
                  <code className="text-sm font-mono text-gray-800 dark:text-gray-200 break-all">
                    {mockApiKey}
                  </code>
                  <button
                    onClick={handleCopyApiKey}
                    className="ml-3 flex-shrink-0 p-2 text-gray-500 hover:text-blue-600 dark:text-gray-400 dark:hover:text-blue-400 transition-colors"
                    title="Copy API key"
                  >
                    {apiKeyCopied ? (
                      <Check className="w-5 h-5 text-green-500" />
                    ) : (
                      <Copy className="w-5 h-5" />
                    )}
                  </button>
                </div>
              </div>

              <div className="bg-amber-50 dark:bg-amber-900/20 border border-amber-200 dark:border-amber-800 rounded-lg p-4">
                <p className="text-sm text-amber-800 dark:text-amber-300 font-medium">
                  Save this key securely. You won't be able to see it again.
                </p>
              </div>

              <div>
                <p className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  Quick Start Example
                </p>
                <div className="bg-gray-900 rounded-lg p-4 overflow-x-auto">
                  <pre className="text-sm text-green-400 font-mono whitespace-pre-wrap">
                    {curlExample}
                  </pre>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Step 4: Integration Test */}
        {currentStep === 4 && (
          <div>
            <h2 className="text-2xl font-bold text-gray-900 dark:text-white mb-1">
              Test Your Integration
            </h2>
            <p className="text-sm text-gray-500 dark:text-gray-400 mb-6">
              Run a quick test to make sure everything is working.
            </p>

            <div className="space-y-6">
              {/* Purchase Test */}
              <div className="border border-gray-200 dark:border-gray-700 rounded-lg p-5">
                <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3">
                  Send Test Purchase
                </h3>
                <div className="bg-gray-900 rounded-lg p-3 overflow-x-auto mb-3">
                  <pre className="text-xs text-green-400 font-mono whitespace-pre-wrap">
                    {testCurlPurchase}
                  </pre>
                </div>
                <div className="flex items-center gap-3">
                  <button
                    onClick={runPurchaseTest}
                    disabled={purchaseTestResult === "loading"}
                    className="px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 text-white text-sm font-medium rounded-lg transition-colors"
                  >
                    {purchaseTestResult === "loading"
                      ? "Running..."
                      : "Run Test"}
                  </button>
                  {purchaseTestResult === "success" && (
                    <div className="flex items-center gap-1.5 text-green-600 dark:text-green-400">
                      <Check className="w-5 h-5" />
                      <span className="text-sm font-medium">
                        Success! {purchasePreview} points issued.
                      </span>
                    </div>
                  )}
                </div>
              </div>

              {/* Activity Test */}
              <div className="border border-gray-200 dark:border-gray-700 rounded-lg p-5">
                <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3">
                  Send Test Activity
                </h3>
                <div className="bg-gray-900 rounded-lg p-3 overflow-x-auto mb-3">
                  <pre className="text-xs text-green-400 font-mono whitespace-pre-wrap">
                    {testCurlActivity}
                  </pre>
                </div>
                <div className="flex items-center gap-3">
                  <button
                    onClick={runActivityTest}
                    disabled={activityTestResult === "loading"}
                    className="px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 text-white text-sm font-medium rounded-lg transition-colors"
                  >
                    {activityTestResult === "loading"
                      ? "Running..."
                      : "Run Test"}
                  </button>
                  {activityTestResult === "success" && (
                    <div className="flex items-center gap-1.5 text-green-600 dark:text-green-400">
                      <Check className="w-5 h-5" />
                      <span className="text-sm font-medium">
                        Success! {activityReward} points issued for{" "}
                        {activityType.toLowerCase()}.
                      </span>
                    </div>
                  )}
                </div>
              </div>

              {/* Results Summary */}
              {purchaseTestResult === "success" &&
                activityTestResult === "success" && (
                  <div className="bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg p-4 text-center">
                    <Check className="w-8 h-8 text-green-600 mx-auto mb-2" />
                    <p className="text-green-800 dark:text-green-300 font-semibold">
                      All tests passed! Your integration is ready.
                    </p>
                    <p className="text-sm text-green-600 dark:text-green-400 mt-1">
                      You can now start issuing points to your customers.
                    </p>
                  </div>
                )}
            </div>
          </div>
        )}
      </div>

      {/* Navigation */}
      <div className="flex items-center justify-between mt-6">
        <div>
          {currentStep > 0 && (
            <button
              onClick={() => setCurrentStep(currentStep - 1)}
              className="flex items-center gap-2 px-4 py-2.5 text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white font-medium transition-colors"
            >
              <ArrowLeft className="w-4 h-4" />
              Back
            </button>
          )}
        </div>

        <div>
          <button
            onClick={() => navigate("/dashboard")}
            className="text-sm text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors"
          >
            Skip Setup
          </button>
        </div>

        <div>
          {currentStep > 0 && currentStep < TOTAL_STEPS - 1 && (
            <button
              onClick={() => setCurrentStep(currentStep + 1)}
              className="flex items-center gap-2 px-6 py-2.5 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg transition-colors"
            >
              Next
              <ArrowRight className="w-4 h-4" />
            </button>
          )}
          {currentStep === TOTAL_STEPS - 1 && (
            <button
              onClick={() => toast.success("Onboarding complete! Redirecting to dashboard...")}
              className="flex items-center gap-2 px-6 py-2.5 bg-green-600 hover:bg-green-700 text-white font-medium rounded-lg transition-colors"
            >
              <Check className="w-4 h-4" />
              Finish Setup
            </button>
          )}
        </div>
      </div>
    </div>
  );
}
