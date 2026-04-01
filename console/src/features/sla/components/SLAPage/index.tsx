import { CheckCircle, AlertTriangle, XCircle } from "lucide-react";

type ServiceStatus = "operational" | "degraded" | "down";

const uptimeCards = [
  { label: "Current Month", value: "99.98%" },
  { label: "Last 90 Days", value: "99.95%" },
  { label: "Last 12 Months", value: "99.92%" },
];

const services: {
  name: string;
  status: ServiceStatus;
  uptime: string;
  lastIncident: string | null;
}[] = [
  {
    name: "API Gateway",
    status: "operational",
    uptime: "99.99%",
    lastIncident: null,
  },
  {
    name: "Authentication",
    status: "operational",
    uptime: "99.98%",
    lastIncident: "2026-03-15",
  },
  {
    name: "Token Service",
    status: "operational",
    uptime: "99.97%",
    lastIncident: "2026-02-28",
  },
  {
    name: "Points Service",
    status: "operational",
    uptime: "99.99%",
    lastIncident: null,
  },
  {
    name: "DynamoDB",
    status: "operational",
    uptime: "99.99%",
    lastIncident: null,
  },
  {
    name: "Blockchain Bridge",
    status: "operational",
    uptime: "99.91%",
    lastIncident: "2026-03-20",
  },
];

const incidents = [
  {
    date: "2026-03-20",
    title: "Blockchain Bridge 지연",
    description:
      "Ethereum 네트워크 혼잡으로 인한 브릿지 트랜잭션 지연. 30분 내 해결.",
    status: "Resolved" as const,
  },
  {
    date: "2026-03-15",
    title: "인증 서비스 일시 중단",
    description:
      "인증서 갱신 작업 중 5분간 로그인 지연 발생.",
    status: "Resolved" as const,
  },
  {
    date: "2026-02-28",
    title: "토큰 서비스 성능 저하",
    description:
      "높은 트래픽으로 인한 토큰 발행 지연. 스케일링 적용으로 해결.",
    status: "Resolved" as const,
  },
];

function StatusBadge({ status }: { status: ServiceStatus }) {
  switch (status) {
    case "operational":
      return (
        <span className="inline-flex items-center gap-1.5 text-sm text-green-600 dark:text-green-400">
          <span className="h-2 w-2 rounded-full bg-green-500" />
          Operational
        </span>
      );
    case "degraded":
      return (
        <span className="inline-flex items-center gap-1.5 text-sm text-yellow-600 dark:text-yellow-400">
          <span className="h-2 w-2 rounded-full bg-yellow-500" />
          Degraded
        </span>
      );
    case "down":
      return (
        <span className="inline-flex items-center gap-1.5 text-sm text-red-600 dark:text-red-400">
          <span className="h-2 w-2 rounded-full bg-red-500" />
          Down
        </span>
      );
  }
}

function StatusIcon({ status }: { status: ServiceStatus }) {
  switch (status) {
    case "operational":
      return <CheckCircle className="h-5 w-5 text-green-500" />;
    case "degraded":
      return <AlertTriangle className="h-5 w-5 text-yellow-500" />;
    case "down":
      return <XCircle className="h-5 w-5 text-red-500" />;
  }
}

export function SLAPage() {
  const allOperational = services.every((s) => s.status === "operational");

  return (
    <div>
      {/* Header */}
      <div className="mb-6">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          SLA Status
        </h1>
        <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
          Service availability and incident history.
        </p>
      </div>

      {/* Overall Status Banner */}
      <div
        className={`rounded-lg p-5 mb-6 flex items-center gap-3 ${
          allOperational
            ? "bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800"
            : "bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800"
        }`}
      >
        {allOperational ? (
          <CheckCircle className="h-6 w-6 text-green-500" />
        ) : (
          <AlertTriangle className="h-6 w-6 text-yellow-500" />
        )}
        <span
          className={`text-lg font-semibold ${
            allOperational
              ? "text-green-700 dark:text-green-300"
              : "text-yellow-700 dark:text-yellow-300"
          }`}
        >
          {allOperational
            ? "All Systems Operational"
            : "Some Systems Degraded"}
        </span>
      </div>

      {/* Uptime KPI Cards */}
      <div className="grid grid-cols-1 sm:grid-cols-3 gap-4 mb-6">
        {uptimeCards.map((card) => (
          <div
            key={card.label}
            className="bg-white dark:bg-gray-800 rounded-lg shadow p-5 text-center"
          >
            <p className="text-sm text-gray-500 dark:text-gray-400 mb-1">
              {card.label}
            </p>
            <p className="text-3xl font-bold text-gray-900 dark:text-white">
              {card.value}
            </p>
          </div>
        ))}
      </div>

      {/* Service Status Table */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden mb-6">
        <div className="px-5 py-4 border-b border-gray-200 dark:border-gray-700">
          <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300">
            Service Status
          </h3>
        </div>
        <div className="overflow-x-auto">
          <table className="w-full text-sm text-left">
            <thead className="bg-gray-50 dark:bg-gray-750 text-gray-500 dark:text-gray-400 uppercase text-xs">
              <tr>
                <th className="px-5 py-3">Service</th>
                <th className="px-5 py-3">Status</th>
                <th className="px-5 py-3 text-right">Uptime</th>
                <th className="px-5 py-3 text-right">Last Incident</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-gray-200 dark:divide-gray-700">
              {services.map((svc) => (
                <tr
                  key={svc.name}
                  className="hover:bg-gray-50 dark:hover:bg-gray-750"
                >
                  <td className="px-5 py-3 font-medium text-gray-900 dark:text-white flex items-center gap-2">
                    <StatusIcon status={svc.status} />
                    {svc.name}
                  </td>
                  <td className="px-5 py-3">
                    <StatusBadge status={svc.status} />
                  </td>
                  <td className="px-5 py-3 text-right text-gray-700 dark:text-gray-300">
                    {svc.uptime}
                  </td>
                  <td className="px-5 py-3 text-right text-gray-500 dark:text-gray-400">
                    {svc.lastIncident ?? "None"}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      {/* Recent Incidents */}
      <div>
        <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          Recent Incidents
        </h3>
        <div className="space-y-4">
          {incidents.map((incident) => (
            <div
              key={incident.date + incident.title}
              className="bg-white dark:bg-gray-800 rounded-lg shadow p-5"
            >
              <div className="flex items-center justify-between mb-2">
                <div className="flex items-center gap-2">
                  <span className="text-sm font-mono text-gray-500 dark:text-gray-400">
                    {incident.date}
                  </span>
                  <h4 className="text-sm font-semibold text-gray-900 dark:text-white">
                    {incident.title}
                  </h4>
                </div>
                <span className="inline-flex items-center gap-1.5 text-xs font-medium text-green-600 dark:text-green-400 bg-green-50 dark:bg-green-900/30 px-2.5 py-1 rounded-full">
                  <span className="h-1.5 w-1.5 rounded-full bg-green-500" />
                  {incident.status}
                </span>
              </div>
              <p className="text-sm text-gray-600 dark:text-gray-400">
                {incident.description}
              </p>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
