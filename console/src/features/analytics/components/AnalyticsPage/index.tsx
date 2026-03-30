import { useMode } from "@/contexts/ModeContext";
import {
  ResponsiveContainer,
  LineChart,
  Line,
  BarChart,
  Bar,
  AreaChart,
  Area,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
} from "recharts";

const treasuryTrend = [
  { month: "Sep", treasury: 4800 },
  { month: "Oct", treasury: 8200 },
  { month: "Nov", treasury: 12500 },
  { month: "Dec", treasury: 18900 },
  { month: "Jan", treasury: 28400 },
  { month: "Feb", treasury: 42100 },
  { month: "Mar", treasury: 72600 },
];

const userGrowth = [
  { month: "Sep", users: 320 },
  { month: "Oct", users: 580 },
  { month: "Nov", users: 950 },
  { month: "Dec", users: 1400 },
  { month: "Jan", users: 2100 },
  { month: "Feb", users: 3200 },
  { month: "Mar", users: 4200 },
];

const tokenCirculation = [
  { month: "Sep", circulating: 8000, locked: 42000 },
  { month: "Oct", circulating: 15000, locked: 40000 },
  { month: "Nov", circulating: 28000, locked: 37000 },
  { month: "Dec", circulating: 45000, locked: 33000 },
  { month: "Jan", circulating: 68000, locked: 28000 },
  { month: "Feb", circulating: 95000, locked: 22000 },
  { month: "Mar", circulating: 135000, locked: 15000 },
];

const pointsActivity = [
  { month: "Sep", awarded: 1200, exchanged: 300 },
  { month: "Oct", awarded: 2800, exchanged: 900 },
  { month: "Nov", awarded: 5200, exchanged: 1800 },
  { month: "Dec", awarded: 8100, exchanged: 3200 },
  { month: "Jan", awarded: 12400, exchanged: 5600 },
  { month: "Feb", awarded: 18500, exchanged: 9200 },
  { month: "Mar", awarded: 26000, exchanged: 14800 },
];

const myStepsWeekly = [
  { week: "W1", steps: 3200 },
  { week: "W2", steps: 5400 },
  { week: "W3", steps: 4100 },
  { week: "W4", steps: 6800 },
  { week: "W5", steps: 5200 },
  { week: "W6", steps: 7500 },
  { week: "W7", steps: 8400 },
];

const myPointsAccum = [
  { week: "W1", points: 32 },
  { week: "W2", points: 86 },
  { week: "W3", points: 127 },
  { week: "W4", points: 195 },
  { week: "W5", points: 247 },
  { week: "W6", points: 322 },
  { week: "W7", points: 406 },
];

function ChartCard({
  title,
  children,
}: {
  title: string;
  children: React.ReactNode;
}) {
  return (
    <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
      <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-4">
        {title}
      </h3>
      {children}
    </div>
  );
}

export function AnalyticsPage() {
  const { isAdmin } = useMode();

  return (
    <div>
      <div className="mb-6">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          Analytics
        </h1>
        <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
          {isAdmin
            ? "Platform-wide performance metrics and trends"
            : "Your personal activity and progress"}
        </p>
      </div>

      {isAdmin ? (
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          <ChartCard title="Treasury Trend (USD)">
            <ResponsiveContainer width="100%" height={300}>
              <LineChart data={treasuryTrend}>
                <CartesianGrid strokeDasharray="3 3" />
                <XAxis dataKey="month" />
                <YAxis />
                <Tooltip />
                <Legend />
                <Line
                  type="monotone"
                  dataKey="treasury"
                  stroke="#3b82f6"
                  strokeWidth={2}
                  dot={{ r: 4 }}
                  name="Treasury (USD)"
                />
              </LineChart>
            </ResponsiveContainer>
          </ChartCard>

          <ChartCard title="User Growth">
            <ResponsiveContainer width="100%" height={300}>
              <BarChart data={userGrowth}>
                <CartesianGrid strokeDasharray="3 3" />
                <XAxis dataKey="month" />
                <YAxis />
                <Tooltip />
                <Legend />
                <Bar
                  dataKey="users"
                  fill="#10b981"
                  name="Total Users"
                  radius={[4, 4, 0, 0]}
                />
              </BarChart>
            </ResponsiveContainer>
          </ChartCard>

          <ChartCard title="Token Circulation">
            <ResponsiveContainer width="100%" height={300}>
              <AreaChart data={tokenCirculation}>
                <CartesianGrid strokeDasharray="3 3" />
                <XAxis dataKey="month" />
                <YAxis />
                <Tooltip />
                <Legend />
                <Area
                  type="monotone"
                  dataKey="circulating"
                  stackId="1"
                  stroke="#f59e0b"
                  fill="#f59e0b"
                  fillOpacity={0.6}
                  name="Circulating"
                />
                <Area
                  type="monotone"
                  dataKey="locked"
                  stackId="1"
                  stroke="#8b5cf6"
                  fill="#8b5cf6"
                  fillOpacity={0.6}
                  name="Locked"
                />
              </AreaChart>
            </ResponsiveContainer>
          </ChartCard>

          <ChartCard title="Points Awarded vs Exchanged">
            <ResponsiveContainer width="100%" height={300}>
              <BarChart data={pointsActivity}>
                <CartesianGrid strokeDasharray="3 3" />
                <XAxis dataKey="month" />
                <YAxis />
                <Tooltip />
                <Legend />
                <Bar
                  dataKey="awarded"
                  fill="#3b82f6"
                  name="Awarded"
                  radius={[4, 4, 0, 0]}
                />
                <Bar
                  dataKey="exchanged"
                  fill="#10b981"
                  name="Exchanged"
                  radius={[4, 4, 0, 0]}
                />
              </BarChart>
            </ResponsiveContainer>
          </ChartCard>
        </div>
      ) : (
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          <ChartCard title="My Steps (Weekly)">
            <ResponsiveContainer width="100%" height={300}>
              <BarChart data={myStepsWeekly}>
                <CartesianGrid strokeDasharray="3 3" />
                <XAxis dataKey="week" />
                <YAxis />
                <Tooltip />
                <Legend />
                <Bar
                  dataKey="steps"
                  fill="#3b82f6"
                  name="Steps"
                  radius={[4, 4, 0, 0]}
                />
              </BarChart>
            </ResponsiveContainer>
          </ChartCard>

          <ChartCard title="My Points Accumulation">
            <ResponsiveContainer width="100%" height={300}>
              <LineChart data={myPointsAccum}>
                <CartesianGrid strokeDasharray="3 3" />
                <XAxis dataKey="week" />
                <YAxis />
                <Tooltip />
                <Legend />
                <Line
                  type="monotone"
                  dataKey="points"
                  stroke="#10b981"
                  strokeWidth={2}
                  dot={{ r: 4 }}
                  name="Cumulative Points"
                />
              </LineChart>
            </ResponsiveContainer>
          </ChartCard>
        </div>
      )}
    </div>
  );
}
