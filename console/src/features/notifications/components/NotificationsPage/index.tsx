import { useState } from "react";
import { AlertTriangle, Settings, Activity, Bell } from "lucide-react";

interface Notification {
  id: string;
  type: "alert" | "system" | "activity";
  title: string;
  description: string;
  time: string;
  read: boolean;
}

const initialNotifications: Notification[] = [
  { id: "1", type: "alert", title: "스트레스 임계치 경고", description: "Le Mouton 트레저리가 설정된 임계치(20%)에 근접했습니다.", time: "2026-03-29T15:30:00Z", read: false },
  { id: "2", type: "system", title: "API 사용량 알림", description: "이번 달 API 호출이 80%에 도달했습니다. 업그레이드를 고려해주세요.", time: "2026-03-29T10:00:00Z", read: false },
  { id: "3", type: "activity", title: "DAO 투표 마감 임박", description: "'리워드 배수 2배 증가' 제안이 3일 후 마감됩니다. 현재 찬성 78%.", time: "2026-03-28T18:00:00Z", read: false },
  { id: "4", type: "alert", title: "대량 토큰 이동 감지", description: "RunPulse에서 10,000 RPT 토큰이 이동되었습니다.", time: "2026-03-28T14:00:00Z", read: true },
  { id: "5", type: "system", title: "시스템 업데이트 완료", description: "v2.1.0 업데이트가 성공적으로 적용되었습니다.", time: "2026-03-27T09:00:00Z", read: true },
  { id: "6", type: "activity", title: "신규 브랜드 등록 요청", description: "GreenWalk 브랜드가 플랫폼 등록을 요청했습니다.", time: "2026-03-26T16:30:00Z", read: true },
  { id: "7", type: "alert", title: "챌린지 참여율 급증", description: "Cafe Blossom 음료 스탬프 챌린지 참여율이 전주 대비 150% 증가했습니다.", time: "2026-03-25T11:00:00Z", read: true },
];

function getIcon(type: Notification["type"]) {
  switch (type) {
    case "alert":
      return <AlertTriangle className="h-5 w-5 text-amber-500" />;
    case "system":
      return <Settings className="h-5 w-5 text-blue-500" />;
    case "activity":
      return <Activity className="h-5 w-5 text-green-500" />;
  }
}

function formatRelativeTime(dateStr: string): string {
  const now = new Date("2026-03-30T00:00:00Z");
  const date = new Date(dateStr);
  const diffMs = now.getTime() - date.getTime();
  const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
  const diffDays = Math.floor(diffHours / 24);

  if (diffHours < 1) return "Just now";
  if (diffHours < 24) return `${diffHours}h ago`;
  if (diffDays === 1) return "1 day ago";
  return `${diffDays} days ago`;
}

export function NotificationsPage() {
  const [filter, setFilter] = useState<"all" | "alert" | "system" | "activity">("all");
  const [notifications, setNotifications] = useState<Notification[]>(initialNotifications);

  const filteredNotifications = notifications.filter(
    (n) => filter === "all" || n.type === filter
  );

  const unreadCount = notifications.filter((n) => !n.read).length;

  const markAsRead = (id: string) => {
    setNotifications((prev) =>
      prev.map((n) => (n.id === id ? { ...n, read: true } : n))
    );
  };

  const markAllAsRead = () => {
    setNotifications((prev) => prev.map((n) => ({ ...n, read: true })));
  };

  return (
    <div>
      <div className="mb-6 flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
            Notifications
          </h1>
          <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
            Alerts, system events, and activity updates
          </p>
        </div>
        <div className="flex items-center gap-3">
          {unreadCount > 0 && (
            <span className="flex items-center gap-1.5 text-sm text-gray-500 dark:text-gray-400">
              <Bell className="h-4 w-4" />
              {unreadCount} unread
            </span>
          )}
          <button
            onClick={markAllAsRead}
            disabled={unreadCount === 0}
            className="px-3 py-1.5 text-sm font-medium text-blue-600 dark:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-900/20 rounded-md transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            Mark all as read
          </button>
        </div>
      </div>

      {/* Filter Tabs */}
      <div className="flex gap-1 rounded-lg border border-gray-300 dark:border-gray-600 p-1 mb-6 w-fit">
        {(["all", "alert", "system", "activity"] as const).map((f) => (
          <button
            key={f}
            onClick={() => setFilter(f)}
            className={`px-4 py-1.5 text-sm font-medium rounded-md transition-colors ${
              filter === f
                ? "bg-blue-600 text-white"
                : "text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700"
            }`}
          >
            {f === "all" ? "All" : f.charAt(0).toUpperCase() + f.slice(1)}
          </button>
        ))}
      </div>

      {/* Notification Cards */}
      <div className="space-y-3">
        {filteredNotifications.map((notification) => (
          <div
            key={notification.id}
            className={`bg-white dark:bg-gray-800 rounded-lg shadow p-4 flex items-start gap-4 transition-colors ${
              !notification.read
                ? "border-l-4 border-l-blue-500"
                : "border-l-4 border-l-transparent"
            }`}
          >
            <div className="flex-shrink-0 mt-0.5">
              {getIcon(notification.type)}
            </div>
            <div className="flex-1 min-w-0">
              <div className="flex items-center gap-2">
                <h3 className="text-sm font-semibold text-gray-900 dark:text-white">
                  {notification.title}
                </h3>
                {!notification.read && (
                  <span className="h-2 w-2 rounded-full bg-blue-500 flex-shrink-0" />
                )}
              </div>
              <p className="mt-0.5 text-sm text-gray-600 dark:text-gray-400">
                {notification.description}
              </p>
              <p className="mt-1 text-xs text-gray-400 dark:text-gray-500">
                {formatRelativeTime(notification.time)}
              </p>
            </div>
            {!notification.read && (
              <button
                onClick={() => markAsRead(notification.id)}
                className="flex-shrink-0 px-3 py-1 text-xs font-medium text-blue-600 dark:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-900/20 rounded-md transition-colors"
              >
                Mark as read
              </button>
            )}
          </div>
        ))}
        {filteredNotifications.length === 0 && (
          <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-8 text-center text-gray-400 dark:text-gray-500">
            No notifications in this category.
          </div>
        )}
      </div>
    </div>
  );
}
