import { useState } from "react";
import { Copy, Eye, Code, Palette } from "lucide-react";
import { toast } from "sonner";

type WidgetType = "points" | "leaderboard" | "progress" | "feed";
type ThemeOption = "light" | "dark";
type ColorOption = "blue" | "green" | "purple" | "red";
type LanguageOption = "en" | "ko";

interface WidgetConfig {
  id: WidgetType;
  title: string;
  description: string;
  preview: React.ReactNode;
  basePath: string;
  hasUserId: boolean;
}

const COLOR_HEX: Record<ColorOption, string> = {
  blue: "#3B82F6",
  green: "#22C55E",
  purple: "#8B5CF6",
  red: "#EF4444",
};

const widgets: WidgetConfig[] = [
  {
    id: "points",
    title: "Points Balance Widget",
    description: "Shows customer's current point balance",
    hasUserId: true,
    basePath: "https://widgets.biyard.co/points",
    preview: (
      <div className="bg-white dark:bg-gray-700 rounded-lg p-4 border border-gray-200 dark:border-gray-600 text-center">
        <p className="text-xs text-gray-500 dark:text-gray-400 mb-1">My Points</p>
        <p className="text-3xl font-bold text-blue-600 dark:text-blue-400">2,450</p>
        <p className="text-xs text-gray-400 dark:text-gray-500 mt-1">Silver Tier</p>
      </div>
    ),
  },
  {
    id: "leaderboard",
    title: "Leaderboard Widget",
    description: "Shows top customers ranked by points/activity",
    hasUserId: false,
    basePath: "https://widgets.biyard.co/leaderboard",
    preview: (
      <div className="bg-white dark:bg-gray-700 rounded-lg p-4 border border-gray-200 dark:border-gray-600">
        <p className="text-xs font-semibold text-gray-700 dark:text-gray-300 mb-2">Top Users</p>
        <table className="w-full text-xs">
          <tbody>
            {[
              { rank: 1, name: "Kim M.", points: "12,400" },
              { rank: 2, name: "Lee S.", points: "9,800" },
              { rank: 3, name: "Park J.", points: "8,200" },
            ].map((row) => (
              <tr key={row.rank} className="border-b border-gray-100 dark:border-gray-600 last:border-0">
                <td className="py-1 font-bold text-gray-500 dark:text-gray-400">#{row.rank}</td>
                <td className="py-1 text-gray-800 dark:text-gray-200">{row.name}</td>
                <td className="py-1 text-right text-blue-600 dark:text-blue-400">{row.points}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    ),
  },
  {
    id: "progress",
    title: "Reward Progress Widget",
    description: "Shows progress toward next reward tier",
    hasUserId: true,
    basePath: "https://widgets.biyard.co/progress",
    preview: (
      <div className="bg-white dark:bg-gray-700 rounded-lg p-4 border border-gray-200 dark:border-gray-600">
        <p className="text-xs font-semibold text-gray-700 dark:text-gray-300 mb-2">Reward Progress</p>
        <div className="flex items-center gap-2 mb-1">
          <span className="text-xs text-gray-500 dark:text-gray-400">Silver</span>
          <div className="flex-1 h-2 bg-gray-200 dark:bg-gray-600 rounded-full overflow-hidden">
            <div className="h-full bg-blue-500 rounded-full" style={{ width: "65%" }} />
          </div>
          <span className="text-xs text-gray-500 dark:text-gray-400">Gold</span>
        </div>
        <p className="text-xs text-center text-gray-500 dark:text-gray-400">6,500 / 10,000 pts</p>
      </div>
    ),
  },
  {
    id: "feed",
    title: "Activity Feed Widget",
    description: "Shows recent activities (purchases, rewards earned)",
    hasUserId: false,
    basePath: "https://widgets.biyard.co/feed",
    preview: (
      <div className="bg-white dark:bg-gray-700 rounded-lg p-4 border border-gray-200 dark:border-gray-600">
        <p className="text-xs font-semibold text-gray-700 dark:text-gray-300 mb-2">Recent Activity</p>
        <ul className="space-y-1.5 text-xs">
          <li className="flex items-center gap-2">
            <span className="w-1.5 h-1.5 rounded-full bg-green-500 flex-shrink-0" />
            <span className="text-gray-700 dark:text-gray-300">+250 pts earned</span>
            <span className="ml-auto text-gray-400">2m ago</span>
          </li>
          <li className="flex items-center gap-2">
            <span className="w-1.5 h-1.5 rounded-full bg-blue-500 flex-shrink-0" />
            <span className="text-gray-700 dark:text-gray-300">Reward redeemed</span>
            <span className="ml-auto text-gray-400">1h ago</span>
          </li>
          <li className="flex items-center gap-2">
            <span className="w-1.5 h-1.5 rounded-full bg-purple-500 flex-shrink-0" />
            <span className="text-gray-700 dark:text-gray-300">Tier upgraded</span>
            <span className="ml-auto text-gray-400">3h ago</span>
          </li>
        </ul>
      </div>
    ),
  },
];

function buildEmbedCode(
  widget: WidgetConfig,
  theme: ThemeOption,
  color: ColorOption,
  lang: LanguageOption,
  width: number,
  height: number,
): string {
  const params = new URLSearchParams();
  params.set("project", "PROJECT_ID");
  if (widget.hasUserId) params.set("user", "USER_ID");
  params.set("theme", theme);
  params.set("color", color);
  params.set("lang", lang);
  return `<iframe src="${widget.basePath}?${params.toString()}" width="${width}" height="${height}" frameborder="0" />`;
}

function buildSdkCode(
  widget: WidgetConfig,
  theme: ThemeOption,
  color: ColorOption,
  lang: LanguageOption,
): string {
  const userProp = widget.hasUserId ? '\n  userId="USER_ID"' : "";
  return `import { BiyardWidget } from '@biyard/sdk';

<BiyardWidget
  type="${widget.id}"
  projectId="PROJECT_ID"${userProp}
  theme="${theme}"
  color="${color}"
  lang="${lang}"
/>`;
}

export function WidgetsPage() {
  const [selectedWidget, setSelectedWidget] = useState<WidgetType>("points");
  const [theme, setTheme] = useState<ThemeOption>("light");
  const [color, setColor] = useState<ColorOption>("blue");
  const [lang, setLang] = useState<LanguageOption>("en");
  const [width, setWidth] = useState(320);
  const [height, setHeight] = useState(200);
  const [showSdk, setShowSdk] = useState(false);

  const widget = widgets.find((w) => w.id === selectedWidget)!;
  const embedCode = buildEmbedCode(widget, theme, color, lang, width, height);
  const sdkCode = buildSdkCode(widget, theme, color, lang);

  const copyToClipboard = (text: string, label: string) => {
    navigator.clipboard.writeText(text);
    toast.success(`${label} copied to clipboard`);
  };

  return (
    <div>
      <div className="mb-6">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          Embed Widgets
        </h1>
        <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
          Get embeddable widget code for your apps and websites
        </p>
      </div>

      {/* Widget Cards */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
        {widgets.map((w) => (
          <button
            key={w.id}
            onClick={() => setSelectedWidget(w.id)}
            className={`text-left rounded-lg shadow p-4 transition-all border-2 ${
              selectedWidget === w.id
                ? "border-blue-500 bg-blue-50 dark:bg-blue-900/20"
                : "border-transparent bg-white dark:bg-gray-800 hover:border-gray-300 dark:hover:border-gray-600"
            }`}
          >
            <h3 className="text-sm font-semibold text-gray-900 dark:text-white mb-1">
              {w.title}
            </h3>
            <p className="text-xs text-gray-500 dark:text-gray-400 mb-3">
              {w.description}
            </p>
            {w.preview}
          </button>
        ))}
      </div>

      {/* Configuration + Output */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Customization Section */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
          <div className="flex items-center gap-2 mb-4">
            <Palette className="h-5 w-5 text-gray-500 dark:text-gray-400" />
            <h2 className="text-lg font-semibold text-gray-900 dark:text-white">
              Customization
            </h2>
          </div>

          <div className="space-y-4">
            {/* Theme */}
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Theme
              </label>
              <div className="flex gap-1 rounded-lg border border-gray-300 dark:border-gray-600 p-1 w-fit">
                {(["light", "dark"] as const).map((t) => (
                  <button
                    key={t}
                    onClick={() => setTheme(t)}
                    className={`px-3 py-1.5 text-sm font-medium rounded-md transition-colors ${
                      theme === t
                        ? "bg-blue-600 text-white"
                        : "text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700"
                    }`}
                  >
                    {t.charAt(0).toUpperCase() + t.slice(1)}
                  </button>
                ))}
              </div>
            </div>

            {/* Primary Color */}
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Primary Color
              </label>
              <div className="flex gap-2">
                {(["blue", "green", "purple", "red"] as const).map((c) => (
                  <button
                    key={c}
                    onClick={() => setColor(c)}
                    className={`w-8 h-8 rounded-full border-2 transition-all ${
                      color === c
                        ? "border-gray-900 dark:border-white scale-110"
                        : "border-gray-300 dark:border-gray-600"
                    }`}
                    style={{ backgroundColor: COLOR_HEX[c] }}
                    title={c}
                  />
                ))}
              </div>
            </div>

            {/* Language */}
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Language
              </label>
              <div className="flex gap-1 rounded-lg border border-gray-300 dark:border-gray-600 p-1 w-fit">
                {([
                  { value: "en" as const, label: "English" },
                  { value: "ko" as const, label: "Korean" },
                ]).map((l) => (
                  <button
                    key={l.value}
                    onClick={() => setLang(l.value)}
                    className={`px-3 py-1.5 text-sm font-medium rounded-md transition-colors ${
                      lang === l.value
                        ? "bg-blue-600 text-white"
                        : "text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700"
                    }`}
                  >
                    {l.label}
                  </button>
                ))}
              </div>
            </div>

            {/* Width & Height */}
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Width (px)
                </label>
                <input
                  type="number"
                  value={width}
                  onChange={(e) => setWidth(Number(e.target.value))}
                  className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Height (px)
                </label>
                <input
                  type="number"
                  value={height}
                  onChange={(e) => setHeight(Number(e.target.value))}
                  className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>
            </div>
          </div>
        </div>

        {/* Embed Code Output */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
          {/* Tab Toggle */}
          <div className="flex items-center gap-4 mb-4">
            <button
              onClick={() => setShowSdk(false)}
              className={`flex items-center gap-1.5 text-sm font-medium pb-1 border-b-2 transition-colors ${
                !showSdk
                  ? "border-blue-600 text-blue-600 dark:text-blue-400"
                  : "border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300"
              }`}
            >
              <Eye className="h-4 w-4" />
              Embed Code
            </button>
            <button
              onClick={() => setShowSdk(true)}
              className={`flex items-center gap-1.5 text-sm font-medium pb-1 border-b-2 transition-colors ${
                showSdk
                  ? "border-blue-600 text-blue-600 dark:text-blue-400"
                  : "border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300"
              }`}
            >
              <Code className="h-4 w-4" />
              React / JS SDK
            </button>
          </div>

          {!showSdk ? (
            <div>
              <p className="text-sm text-gray-500 dark:text-gray-400 mb-2">
                Copy this iframe code and paste it into your HTML:
              </p>
              <div className="relative">
                <pre className="bg-gray-50 dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded-lg p-4 text-sm text-gray-800 dark:text-gray-200 overflow-x-auto whitespace-pre-wrap break-all">
                  {embedCode}
                </pre>
                <button
                  onClick={() => copyToClipboard(embedCode, "Embed code")}
                  className="absolute top-2 right-2 p-2 rounded-md bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
                  title="Copy code"
                >
                  <Copy className="h-4 w-4 text-gray-500 dark:text-gray-400" />
                </button>
              </div>
            </div>
          ) : (
            <div>
              <p className="text-sm text-gray-500 dark:text-gray-400 mb-2">
                Install the SDK and use the React component:
              </p>
              <div className="relative mb-3">
                <pre className="bg-gray-50 dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded-lg p-4 text-sm text-gray-800 dark:text-gray-200 overflow-x-auto">
                  npm install @biyard/sdk
                </pre>
                <button
                  onClick={() => copyToClipboard("npm install @biyard/sdk", "Install command")}
                  className="absolute top-2 right-2 p-2 rounded-md bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
                  title="Copy command"
                >
                  <Copy className="h-4 w-4 text-gray-500 dark:text-gray-400" />
                </button>
              </div>
              <div className="relative">
                <pre className="bg-gray-50 dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded-lg p-4 text-sm text-gray-800 dark:text-gray-200 overflow-x-auto whitespace-pre-wrap">
                  {sdkCode}
                </pre>
                <button
                  onClick={() => copyToClipboard(sdkCode, "SDK code")}
                  className="absolute top-2 right-2 p-2 rounded-md bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
                  title="Copy code"
                >
                  <Copy className="h-4 w-4 text-gray-500 dark:text-gray-400" />
                </button>
              </div>
            </div>
          )}

          {/* Live Preview */}
          <div className="mt-6">
            <div className="flex items-center gap-2 mb-2">
              <Eye className="h-4 w-4 text-gray-500 dark:text-gray-400" />
              <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300">
                Live Preview
              </h3>
            </div>
            <div
              className={`rounded-lg p-4 border ${
                theme === "dark"
                  ? "bg-gray-900 border-gray-700"
                  : "bg-gray-50 border-gray-200"
              }`}
              style={{ maxWidth: width }}
            >
              {widget.preview}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
