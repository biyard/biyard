import { useState } from "react";
import {
  ChevronDown,
  ChevronRight,
  Copy,
  ExternalLink,
  BookOpen,
  Zap,
  Code2,
  Terminal,
  Package,
  Smartphone,
  Globe,
  ShoppingCart,
} from "lucide-react";
import { toast } from "sonner";

// ---------------------------------------------------------------------------
// Data
// ---------------------------------------------------------------------------

const BASE_URL = "https://api.biyard.co";

interface Endpoint {
  method: "GET" | "POST" | "PUT" | "DELETE";
  path: string;
  description: string;
  request?: string;
  response: string;
}

interface EndpointGroup {
  name: string;
  description: string;
  endpoints: Endpoint[];
}

const endpointGroups: EndpointGroup[] = [
  {
    name: "Accounts",
    description: "User authentication and account management",
    endpoints: [
      {
        method: "POST",
        path: "/v1/accounts/signup",
        description: "Create a new account on the Biyard platform.",
        request: JSON.stringify(
          { email: "dev@example.com", password: "s3cur3!", name: "Jane Dev" },
          null,
          2,
        ),
        response: JSON.stringify(
          {
            id: "acc_9x8f7e6d",
            email: "dev@example.com",
            name: "Jane Dev",
            created_at: "2026-03-30T00:00:00Z",
          },
          null,
          2,
        ),
      },
      {
        method: "POST",
        path: "/v1/accounts/signin",
        description: "Authenticate and receive an access token.",
        request: JSON.stringify(
          { email: "dev@example.com", password: "s3cur3!" },
          null,
          2,
        ),
        response: JSON.stringify(
          {
            access_token: "eyJhbGciOi...",
            token_type: "Bearer",
            expires_in: 3600,
          },
          null,
          2,
        ),
      },
      {
        method: "GET",
        path: "/v1/accounts/me",
        description: "Retrieve the currently authenticated user profile.",
        response: JSON.stringify(
          {
            id: "acc_9x8f7e6d",
            email: "dev@example.com",
            name: "Jane Dev",
            created_at: "2026-03-30T00:00:00Z",
          },
          null,
          2,
        ),
      },
    ],
  },
  {
    name: "Projects",
    description: "Create and manage your blockchain projects",
    endpoints: [
      {
        method: "GET",
        path: "/v1/projects",
        description: "List all projects for the authenticated account.",
        response: JSON.stringify(
          {
            projects: [
              {
                id: "proj_abc123",
                name: "My Loyalty App",
                created_at: "2026-03-01T00:00:00Z",
              },
            ],
            total: 1,
          },
          null,
          2,
        ),
      },
      {
        method: "POST",
        path: "/v1/projects",
        description: "Create a new project.",
        request: JSON.stringify(
          {
            name: "My Loyalty App",
            description: "Reward customers with points and tokens",
          },
          null,
          2,
        ),
        response: JSON.stringify(
          {
            id: "proj_abc123",
            name: "My Loyalty App",
            description: "Reward customers with points and tokens",
            created_at: "2026-03-30T00:00:00Z",
          },
          null,
          2,
        ),
      },
      {
        method: "GET",
        path: "/v1/projects/:id",
        description: "Get details of a specific project.",
        response: JSON.stringify(
          {
            id: "proj_abc123",
            name: "My Loyalty App",
            description: "Reward customers with points and tokens",
            created_at: "2026-03-30T00:00:00Z",
            stats: { total_users: 1250, total_points_issued: 540000 },
          },
          null,
          2,
        ),
      },
      {
        method: "DELETE",
        path: "/v1/projects/:id",
        description: "Delete a project. This action is irreversible.",
        response: JSON.stringify({ deleted: true }, null, 2),
      },
    ],
  },
  {
    name: "Purchases (Core)",
    description:
      "Record purchases and automatically distribute reward points and treasury contributions",
    endpoints: [
      {
        method: "POST",
        path: "/v1/projects/:id/purchases",
        description:
          "Record a purchase. Biyard automatically calculates reward points and treasury contribution based on the configured reward rate.",
        request: JSON.stringify(
          {
            meta_user_id: "user_42",
            amount: 15000,
            item_name: "Premium Latte",
            reward_rate: 0.05,
          },
          null,
          2,
        ),
        response: JSON.stringify(
          {
            id: "pur_x1y2z3",
            purchase_amount: 15000,
            reward_points: 750,
            treasury_contribution: 375,
            created_at: "2026-03-30T12:00:00Z",
          },
          null,
          2,
        ),
      },
    ],
  },
  {
    name: "Activities (Core)",
    description:
      "Award points for user activities such as walking, check-ins, and referrals",
    endpoints: [
      {
        method: "POST",
        path: "/v1/projects/:id/activities",
        description:
          "Record a user activity and award points. Supports custom activity types like walking, running, check-in, referral, and more.",
        request: JSON.stringify(
          {
            meta_user_id: "user_42",
            activity_type: "walking",
            value: 8500,
            description: "8,500 steps recorded today",
          },
          null,
          2,
        ),
        response: JSON.stringify(
          {
            id: "act_a1b2c3",
            points_earned: 85,
            total_points: 1635,
            activity_type: "walking",
            created_at: "2026-03-30T18:30:00Z",
          },
          null,
          2,
        ),
      },
    ],
  },
  {
    name: "Points",
    description:
      "Award, deduct, transfer, and query point balances and transaction history",
    endpoints: [
      {
        method: "POST",
        path: "/v1/projects/:id/points",
        description:
          "Award, deduct, transfer, or exchange points for a user.",
        request: JSON.stringify(
          {
            action: "award",
            meta_user_id: "user_42",
            amount: 500,
            reason: "Referral bonus",
          },
          null,
          2,
        ),
        response: JSON.stringify(
          {
            id: "pt_d4e5f6",
            action: "award",
            amount: 500,
            balance: 2135,
            created_at: "2026-03-30T19:00:00Z",
          },
          null,
          2,
        ),
      },
      {
        method: "GET",
        path: "/v1/projects/:id/points",
        description: "Get monthly aggregated point statistics for a project.",
        response: JSON.stringify(
          {
            month: "2026-03",
            total_awarded: 54000,
            total_deducted: 12000,
            net: 42000,
            unique_users: 320,
          },
          null,
          2,
        ),
      },
      {
        method: "GET",
        path: "/v1/projects/:id/points/:user_id",
        description: "Get the point balance for a specific user.",
        response: JSON.stringify(
          {
            meta_user_id: "user_42",
            balance: 2135,
            lifetime_earned: 4200,
            lifetime_spent: 2065,
          },
          null,
          2,
        ),
      },
      {
        method: "GET",
        path: "/v1/projects/:id/points/transactions",
        description:
          "List point transactions with pagination and optional filters.",
        response: JSON.stringify(
          {
            transactions: [
              {
                id: "pt_d4e5f6",
                action: "award",
                amount: 500,
                meta_user_id: "user_42",
                reason: "Referral bonus",
                created_at: "2026-03-30T19:00:00Z",
              },
            ],
            total: 128,
            page: 1,
            per_page: 20,
          },
          null,
          2,
        ),
      },
    ],
  },
  {
    name: "Tokens",
    description:
      "Query token information, mint new tokens, and check user token balances",
    endpoints: [
      {
        method: "GET",
        path: "/v1/projects/:id/tokens",
        description: "Get token configuration and supply information.",
        response: JSON.stringify(
          {
            name: "LoyalToken",
            symbol: "LYT",
            total_supply: 1000000,
            circulating_supply: 245000,
            decimals: 18,
          },
          null,
          2,
        ),
      },
      {
        method: "PUT",
        path: "/v1/projects/:id/tokens",
        description: "Mint new tokens to a user wallet.",
        request: JSON.stringify(
          { meta_user_id: "user_42", amount: 100, reason: "Point exchange" },
          null,
          2,
        ),
        response: JSON.stringify(
          {
            tx_hash: "0xabc...def",
            amount: 100,
            new_balance: 350,
            status: "confirmed",
          },
          null,
          2,
        ),
      },
      {
        method: "GET",
        path: "/v1/projects/:id/tokens/balance/:user_id",
        description: "Get token balance for a specific user.",
        response: JSON.stringify(
          { meta_user_id: "user_42", balance: 350, symbol: "LYT" },
          null,
          2,
        ),
      },
    ],
  },
  {
    name: "Treasury",
    description: "View project treasury balance and contribution history",
    endpoints: [
      {
        method: "GET",
        path: "/v1/projects/:id/treasury",
        description:
          "Get the current treasury balance, total contributions, and recent activity.",
        response: JSON.stringify(
          {
            balance: 125000,
            total_contributions: 187500,
            total_disbursements: 62500,
            currency: "KRW",
            last_updated: "2026-03-30T20:00:00Z",
          },
          null,
          2,
        ),
      },
    ],
  },
];

interface SDK {
  name: string;
  language: string;
  install: string;
  usage: string;
  available: boolean;
}

const sdks: SDK[] = [
  {
    name: "JavaScript / TypeScript",
    language: "bash",
    install: "npm install @biyard/sdk",
    usage: `import { Biyard } from "@biyard/sdk";

const client = new Biyard({
  apiKey: process.env.BIYARD_API_KEY,
  projectId: "proj_abc123",
});

// Record a purchase
const purchase = await client.purchases.create({
  meta_user_id: "user_42",
  amount: 15000,
  item_name: "Premium Latte",
  reward_rate: 0.05,
});

console.log(\`Awarded \${purchase.reward_points} points\`);`,
    available: true,
  },
  {
    name: "Python",
    language: "bash",
    install: "pip install biyard",
    usage: `from biyard import Biyard

client = Biyard(
    api_key=os.environ["BIYARD_API_KEY"],
    project_id="proj_abc123",
)

# Record an activity
activity = client.activities.create(
    meta_user_id="user_42",
    activity_type="walking",
    value=8500,
    description="8,500 steps recorded today",
)

print(f"Earned {activity.points_earned} points")`,
    available: false,
  },
  {
    name: "Java",
    language: "xml",
    install: `<dependency>
  <groupId>co.biyard</groupId>
  <artifactId>biyard-sdk</artifactId>
  <version>1.0.0</version>
</dependency>`,
    usage: `import co.biyard.Biyard;
import co.biyard.models.Purchase;

Biyard client = Biyard.builder()
    .apiKey(System.getenv("BIYARD_API_KEY"))
    .projectId("proj_abc123")
    .build();

Purchase purchase = client.purchases().create(
    PurchaseRequest.builder()
        .metaUserId("user_42")
        .amount(15000)
        .itemName("Premium Latte")
        .rewardRate(0.05)
        .build()
);

System.out.println("Points: " + purchase.getRewardPoints());`,
    available: false,
  },
];

interface IntegrationExample {
  title: string;
  description: string;
  icon: React.ReactNode;
  code: string;
}

const integrationExamples: IntegrationExample[] = [
  {
    title: "POS Integration",
    description:
      "Send purchase data from your Point-of-Sale system to automatically reward customers with points.",
    icon: <ShoppingCart className="w-6 h-6" />,
    code: `// After a successful POS transaction
async function onPurchaseComplete(transaction) {
  const response = await fetch(
    "${BASE_URL}/v1/projects/proj_abc123/purchases",
    {
      method: "POST",
      headers: {
        "Authorization": "Bearer YOUR_API_KEY",
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        meta_user_id: transaction.customerId,
        amount: transaction.total,
        item_name: transaction.items.map(i => i.name).join(", "),
        reward_rate: 0.05,
      }),
    }
  );

  const result = await response.json();
  // Display on POS screen
  showReceipt(\`You earned \${result.reward_points} points!\`);
}`,
  },
  {
    title: "Mobile App",
    description:
      "Track walking, running, or other physical activities and send them to Biyard to reward users with points.",
    icon: <Smartphone className="w-6 h-6" />,
    code: `// React Native / Expo example
import { Pedometer } from "expo-sensors";

async function submitDailySteps() {
  const { steps } = await Pedometer.getStepCountAsync(
    startOfDay, endOfDay
  );

  const res = await fetch(
    "${BASE_URL}/v1/projects/proj_abc123/activities",
    {
      method: "POST",
      headers: {
        "Authorization": "Bearer YOUR_API_KEY",
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        meta_user_id: currentUser.id,
        activity_type: "walking",
        value: steps,
        description: \`\${steps.toLocaleString()} steps today\`,
      }),
    }
  );

  const data = await res.json();
  Alert.alert("Points Earned!", \`+\${data.points_earned} pts\`);
}`,
  },
  {
    title: "Web App",
    description:
      "Display a user's point balance and transaction history directly inside your web application.",
    icon: <Globe className="w-6 h-6" />,
    code: `// React component using React Query
import { useQuery } from "@tanstack/react-query";

function PointsBalance({ userId }) {
  const { data, isLoading } = useQuery({
    queryKey: ["points", userId],
    queryFn: async () => {
      const res = await fetch(
        \`${BASE_URL}/v1/projects/proj_abc123/points/\${userId}\`,
        {
          headers: {
            "Authorization": "Bearer YOUR_API_KEY",
          },
        }
      );
      return res.json();
    },
  });

  if (isLoading) return <Spinner />;

  return (
    <div className="points-card">
      <h3>Your Points</h3>
      <p className="balance">{data.balance.toLocaleString()}</p>
      <p className="lifetime">
        Lifetime earned: {data.lifetime_earned.toLocaleString()}
      </p>
    </div>
  );
}`,
  },
];

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

const methodColors: Record<string, string> = {
  GET: "bg-emerald-100 text-emerald-800 dark:bg-emerald-900/40 dark:text-emerald-400",
  POST: "bg-blue-100 text-blue-800 dark:bg-blue-900/40 dark:text-blue-400",
  PUT: "bg-amber-100 text-amber-800 dark:bg-amber-900/40 dark:text-amber-400",
  DELETE: "bg-red-100 text-red-800 dark:bg-red-900/40 dark:text-red-400",
};

function MethodBadge({ method }: { method: string }) {
  return (
    <span
      className={`inline-flex items-center justify-center w-16 rounded-md px-2 py-0.5 text-xs font-bold uppercase tracking-wide ${methodColors[method] ?? ""}`}
    >
      {method}
    </span>
  );
}

function copyToClipboard(text: string) {
  navigator.clipboard.writeText(text).then(() => {
    toast.success("Copied to clipboard");
  });
}

// ---------------------------------------------------------------------------
// Sub-components
// ---------------------------------------------------------------------------

function CodeBlock({ code, className }: { code: string; className?: string }) {
  return (
    <div className={`relative group ${className ?? ""}`}>
      <button
        onClick={() => copyToClipboard(code)}
        className="absolute top-2 right-2 p-1.5 rounded-md bg-gray-700/60 text-gray-400 opacity-0 group-hover:opacity-100 transition-opacity hover:text-white hover:bg-gray-600"
        aria-label="Copy code"
      >
        <Copy className="w-4 h-4" />
      </button>
      <pre className="overflow-x-auto rounded-lg bg-gray-900 p-4 text-sm leading-relaxed text-green-400 font-mono">
        <code>{code}</code>
      </pre>
    </div>
  );
}

function QuickStartStep({
  step,
  title,
  description,
  code,
  link,
}: {
  step: number;
  title: string;
  description: string;
  code?: string;
  link?: { href: string; label: string };
}) {
  return (
    <div className="relative pl-10">
      {/* Step number */}
      <div className="absolute left-0 top-0 flex h-7 w-7 items-center justify-center rounded-full bg-blue-600 text-xs font-bold text-white">
        {step}
      </div>
      <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
        {title}
      </h3>
      <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
        {description}
      </p>
      {link && (
        <a
          href={link.href}
          className="mt-2 inline-flex items-center gap-1 text-sm font-medium text-blue-600 hover:text-blue-500 dark:text-blue-400"
        >
          {link.label}
          <ExternalLink className="w-3.5 h-3.5" />
        </a>
      )}
      {code && <CodeBlock code={code} className="mt-3" />}
    </div>
  );
}

function EndpointCard({ endpoint }: { endpoint: Endpoint }) {
  const [open, setOpen] = useState(false);

  return (
    <div className="border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden">
      <button
        onClick={() => setOpen(!open)}
        className="flex w-full items-center gap-3 px-4 py-3 text-left hover:bg-gray-50 dark:hover:bg-gray-800/60 transition-colors"
      >
        <MethodBadge method={endpoint.method} />
        <code className="text-sm font-semibold text-gray-800 dark:text-gray-200 font-mono">
          {endpoint.path}
        </code>
        <span className="ml-auto text-xs text-gray-500 dark:text-gray-400 hidden sm:inline">
          {endpoint.description}
        </span>
        {open ? (
          <ChevronDown className="w-4 h-4 text-gray-400 shrink-0" />
        ) : (
          <ChevronRight className="w-4 h-4 text-gray-400 shrink-0" />
        )}
      </button>

      {open && (
        <div className="border-t border-gray-200 dark:border-gray-700 bg-gray-50/50 dark:bg-gray-800/30 px-4 py-4 space-y-4">
          <p className="text-sm text-gray-700 dark:text-gray-300">
            {endpoint.description}
          </p>

          {endpoint.request && (
            <div>
              <h4 className="mb-1.5 text-xs font-semibold uppercase tracking-wider text-gray-500 dark:text-gray-400">
                Request Body
              </h4>
              <CodeBlock code={endpoint.request} />
            </div>
          )}

          <div>
            <h4 className="mb-1.5 text-xs font-semibold uppercase tracking-wider text-gray-500 dark:text-gray-400">
              Response
            </h4>
            <CodeBlock code={endpoint.response} />
          </div>
        </div>
      )}
    </div>
  );
}

function EndpointGroupAccordion({ group }: { group: EndpointGroup }) {
  const [open, setOpen] = useState(false);

  return (
    <div className="rounded-xl border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 overflow-hidden shadow-sm">
      <button
        onClick={() => setOpen(!open)}
        className="flex w-full items-center justify-between px-5 py-4 text-left hover:bg-gray-50 dark:hover:bg-gray-700/40 transition-colors"
      >
        <div>
          <h3 className="text-base font-semibold text-gray-900 dark:text-white">
            {group.name}
          </h3>
          <p className="mt-0.5 text-sm text-gray-500 dark:text-gray-400">
            {group.description}
          </p>
        </div>
        <div className="flex items-center gap-2 shrink-0 ml-4">
          <span className="rounded-full bg-gray-100 dark:bg-gray-700 px-2 py-0.5 text-xs font-medium text-gray-600 dark:text-gray-300">
            {group.endpoints.length}{" "}
            {group.endpoints.length === 1 ? "endpoint" : "endpoints"}
          </span>
          {open ? (
            <ChevronDown className="w-5 h-5 text-gray-400" />
          ) : (
            <ChevronRight className="w-5 h-5 text-gray-400" />
          )}
        </div>
      </button>

      {open && (
        <div className="border-t border-gray-200 dark:border-gray-700 px-5 py-4 space-y-3">
          {group.endpoints.map((ep, i) => (
            <EndpointCard key={i} endpoint={ep} />
          ))}
        </div>
      )}
    </div>
  );
}

function SDKCard({ sdk }: { sdk: SDK }) {
  if (!sdk.available) {
    return (
      <div className="relative rounded-xl border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 p-5 shadow-sm flex flex-col opacity-50">
        <span className="absolute top-3 right-3 rounded-full bg-amber-100 dark:bg-amber-900/40 px-2.5 py-0.5 text-xs font-semibold text-amber-800 dark:text-amber-400">
          Coming Soon
        </span>
        <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-3">
          {sdk.name}
        </h3>
        <div className="flex-1 flex items-center justify-center">
          <p className="text-sm text-gray-500 dark:text-gray-400 text-center py-8">
            SDK coming soon. Join the waitlist.
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="relative rounded-xl border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 p-5 shadow-sm flex flex-col">
      <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-3">
        {sdk.name}
      </h3>

      <div className="mb-3">
        <p className="mb-1 text-xs font-semibold uppercase tracking-wider text-gray-500 dark:text-gray-400">
          Install
        </p>
        <CodeBlock code={sdk.install} />
      </div>

      <div className="flex-1">
        <p className="mb-1 text-xs font-semibold uppercase tracking-wider text-gray-500 dark:text-gray-400">
          Usage
        </p>
        <CodeBlock code={sdk.usage} />
      </div>
    </div>
  );
}

function IntegrationCard({ example }: { example: IntegrationExample }) {
  const [open, setOpen] = useState(false);

  return (
    <div className="rounded-xl border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 overflow-hidden shadow-sm">
      <button
        onClick={() => setOpen(!open)}
        className="flex w-full items-center gap-4 px-5 py-4 text-left hover:bg-gray-50 dark:hover:bg-gray-700/40 transition-colors"
      >
        <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-blue-50 text-blue-600 dark:bg-blue-900/30 dark:text-blue-400 shrink-0">
          {example.icon}
        </div>
        <div className="flex-1 min-w-0">
          <h3 className="text-base font-semibold text-gray-900 dark:text-white">
            {example.title}
          </h3>
          <p className="mt-0.5 text-sm text-gray-500 dark:text-gray-400 truncate">
            {example.description}
          </p>
        </div>
        {open ? (
          <ChevronDown className="w-5 h-5 text-gray-400 shrink-0" />
        ) : (
          <ChevronRight className="w-5 h-5 text-gray-400 shrink-0" />
        )}
      </button>

      {open && (
        <div className="border-t border-gray-200 dark:border-gray-700 px-5 py-4">
          <p className="mb-3 text-sm text-gray-700 dark:text-gray-300">
            {example.description}
          </p>
          <CodeBlock code={example.code} />
        </div>
      )}
    </div>
  );
}

// ---------------------------------------------------------------------------
// Page
// ---------------------------------------------------------------------------

export function DeveloperPortalPage() {
  return (
    <div className="max-w-5xl">
      {/* Page Header */}
      <div className="mb-10">
        <div className="flex items-center gap-3 mb-2">
          <div className="flex h-10 w-10 items-center justify-center rounded-xl bg-blue-600 text-white">
            <BookOpen className="w-5 h-5" />
          </div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
            Developer Portal
          </h1>
        </div>
        <p className="text-base text-gray-600 dark:text-gray-400 max-w-2xl">
          Everything you need to integrate Biyard into your application.
          Manage points, tokens, and treasury operations through a simple REST
          API.
        </p>
      </div>

      {/* ----------------------------------------------------------------- */}
      {/* 1. Quick Start */}
      {/* ----------------------------------------------------------------- */}
      <section className="mb-14">
        <div className="flex items-center gap-2 mb-6">
          <Zap className="w-5 h-5 text-amber-500" />
          <h2 className="text-2xl font-bold text-gray-900 dark:text-white">
            Quick Start
          </h2>
        </div>
        <p className="mb-8 text-sm text-gray-600 dark:text-gray-400">
          Go from zero to your first API call in under five minutes.
        </p>

        <div className="space-y-8 border-l-2 border-blue-200 dark:border-blue-800 ml-3">
          <QuickStartStep
            step={1}
            title="Get your API Key"
            description="Create an API credential in the Console. You will need it to authenticate every request."
            link={{ href: "/credentials", label: "Go to Credentials" }}
          />
          <QuickStartStep
            step={2}
            title="Make your first API call"
            description="Record a purchase and see reward points calculated automatically."
            code={`curl -X POST ${BASE_URL}/v1/projects/proj_abc123/purchases \\
  -H "Authorization: Bearer YOUR_API_KEY" \\
  -H "Content-Type: application/json" \\
  -d '{
    "meta_user_id": "user_42",
    "amount": 15000,
    "item_name": "Premium Latte",
    "reward_rate": 0.05
  }'`}
          />
          <QuickStartStep
            step={3}
            title="Award points for activities"
            description="Track user activities like walking, check-ins, or referrals and award points."
            code={`curl -X POST ${BASE_URL}/v1/projects/proj_abc123/activities \\
  -H "Authorization: Bearer YOUR_API_KEY" \\
  -H "Content-Type: application/json" \\
  -d '{
    "meta_user_id": "user_42",
    "activity_type": "walking",
    "value": 8500,
    "description": "8,500 steps recorded today"
  }'`}
          />
          <QuickStartStep
            step={4}
            title="Check treasury"
            description="View how much has accumulated in your project treasury from purchase contributions."
            code={`curl ${BASE_URL}/v1/projects/proj_abc123/treasury \\
  -H "Authorization: Bearer YOUR_API_KEY"`}
          />
        </div>
      </section>

      {/* ----------------------------------------------------------------- */}
      {/* 2. API Reference */}
      {/* ----------------------------------------------------------------- */}
      <section className="mb-14">
        <div className="flex items-center gap-2 mb-6">
          <Code2 className="w-5 h-5 text-blue-500" />
          <h2 className="text-2xl font-bold text-gray-900 dark:text-white">
            API Reference
          </h2>
        </div>
        <p className="mb-6 text-sm text-gray-600 dark:text-gray-400">
          Explore every endpoint. Click to expand request and response examples.
        </p>

        <div className="space-y-4">
          {endpointGroups.map((group) => (
            <EndpointGroupAccordion key={group.name} group={group} />
          ))}
        </div>
      </section>

      {/* ----------------------------------------------------------------- */}
      {/* 3. SDKs */}
      {/* ----------------------------------------------------------------- */}
      <section className="mb-14">
        <div className="flex items-center gap-2 mb-6">
          <Package className="w-5 h-5 text-purple-500" />
          <h2 className="text-2xl font-bold text-gray-900 dark:text-white">
            SDKs
          </h2>
        </div>
        <p className="mb-6 text-sm text-gray-600 dark:text-gray-400">
          Use our official SDKs for a smoother integration experience.
        </p>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-5">
          {sdks.map((sdk) => (
            <SDKCard key={sdk.name} sdk={sdk} />
          ))}
        </div>
      </section>

      {/* ----------------------------------------------------------------- */}
      {/* 4. Integration Examples */}
      {/* ----------------------------------------------------------------- */}
      <section className="mb-14">
        <div className="flex items-center gap-2 mb-6">
          <Terminal className="w-5 h-5 text-green-500" />
          <h2 className="text-2xl font-bold text-gray-900 dark:text-white">
            Integration Examples
          </h2>
        </div>
        <p className="mb-6 text-sm text-gray-600 dark:text-gray-400">
          Real-world patterns to get you started quickly.
        </p>

        <div className="space-y-4">
          {integrationExamples.map((example) => (
            <IntegrationCard key={example.title} example={example} />
          ))}
        </div>
      </section>

      {/* Footer CTA */}
      <div className="rounded-xl border border-blue-200 dark:border-blue-800 bg-blue-50 dark:bg-blue-900/20 p-6 text-center mb-8">
        <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-2">
          Need help integrating?
        </h3>
        <p className="text-sm text-gray-600 dark:text-gray-400 mb-4">
          Our developer relations team is here to help you build on Biyard.
        </p>
        <a
          href="mailto:support@biyard.co"
          className="inline-flex items-center gap-2 rounded-lg bg-blue-600 px-5 py-2.5 text-sm font-medium text-white hover:bg-blue-700 transition-colors"
        >
          Contact Support
          <ExternalLink className="w-4 h-4" />
        </a>
      </div>
    </div>
  );
}
