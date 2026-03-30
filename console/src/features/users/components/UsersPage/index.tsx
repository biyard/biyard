import { useState } from "react";
import { Search } from "lucide-react";
import { formatNumber } from "@/lib/mock-data";

const mockUsers = [
  { id: "1", name: "김민수", email: "minsu@example.com", brand: "Le Mouton", tokens: 245.5, points: 12400, status: "active", joined: "2026-01-15" },
  { id: "2", name: "이서연", email: "seoyeon@example.com", brand: "RunPulse", tokens: 189.2, points: 9800, status: "active", joined: "2026-01-20" },
  { id: "3", name: "박지훈", email: "jihun@example.com", brand: "Le Mouton", tokens: 156.8, points: 8200, status: "active", joined: "2026-02-01" },
  { id: "4", name: "최유진", email: "yujin@example.com", brand: "Cafe Blossom", tokens: 134.1, points: 7100, status: "active", joined: "2026-02-10" },
  { id: "5", name: "정다은", email: "daeun@example.com", brand: "RunPulse", tokens: 98.5, points: 5200, status: "inactive", joined: "2026-02-15" },
  { id: "6", name: "강현우", email: "hyunwoo@example.com", brand: "Le Mouton", tokens: 87.3, points: 4600, status: "active", joined: "2026-02-20" },
  { id: "7", name: "윤서아", email: "seoa@example.com", brand: "Cafe Blossom", tokens: 76.2, points: 4100, status: "active", joined: "2026-03-01" },
  { id: "8", name: "임재현", email: "jaehyun@example.com", brand: "RunPulse", tokens: 65.4, points: 3500, status: "active", joined: "2026-03-05" },
  { id: "9", name: "한소영", email: "soyoung@example.com", brand: "Le Mouton", tokens: 45.1, points: 2400, status: "inactive", joined: "2026-03-10" },
  { id: "10", name: "오준서", email: "junseo@example.com", brand: "Cafe Blossom", tokens: 32.8, points: 1800, status: "active", joined: "2026-03-15" },
];

export function UsersPage() {
  const [search, setSearch] = useState("");
  const [statusFilter, setStatusFilter] = useState<"all" | "active" | "inactive">("all");

  const filteredUsers = mockUsers.filter((user) => {
    const matchesSearch =
      search === "" ||
      user.name.toLowerCase().includes(search.toLowerCase()) ||
      user.email.toLowerCase().includes(search.toLowerCase()) ||
      user.brand.toLowerCase().includes(search.toLowerCase());
    const matchesStatus =
      statusFilter === "all" || user.status === statusFilter;
    return matchesSearch && matchesStatus;
  });

  const totalUsers = mockUsers.length;
  const activeUsers = mockUsers.filter((u) => u.status === "active").length;
  const inactiveUsers = mockUsers.filter((u) => u.status === "inactive").length;

  return (
    <div>
      <div className="mb-6">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          User Management
        </h1>
        <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
          Manage platform users and their activity
        </p>
      </div>

      {/* KPI Cards */}
      <div className="grid grid-cols-1 sm:grid-cols-3 gap-4 mb-6">
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
          <p className="text-sm text-gray-500 dark:text-gray-400">Total Users</p>
          <p className="mt-1 text-2xl font-bold text-gray-900 dark:text-white">
            {formatNumber(totalUsers)}
          </p>
        </div>
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
          <p className="text-sm text-gray-500 dark:text-gray-400">Active Users</p>
          <p className="mt-1 text-2xl font-bold text-green-600 dark:text-green-400">
            {formatNumber(activeUsers)}
          </p>
        </div>
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
          <p className="text-sm text-gray-500 dark:text-gray-400">Inactive Users</p>
          <p className="mt-1 text-2xl font-bold text-gray-500 dark:text-gray-400">
            {formatNumber(inactiveUsers)}
          </p>
        </div>
      </div>

      {/* Search & Filter */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-5">
        <div className="flex flex-col sm:flex-row gap-4 mb-4">
          <div className="relative flex-1">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-gray-400" />
            <input
              type="text"
              placeholder="Search by name, email, or brand..."
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              className="w-full pl-10 pr-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>
          <div className="flex gap-1 rounded-lg border border-gray-300 dark:border-gray-600 p-1">
            {(["all", "active", "inactive"] as const).map((s) => (
              <button
                key={s}
                onClick={() => setStatusFilter(s)}
                className={`px-3 py-1.5 text-sm font-medium rounded-md transition-colors ${
                  statusFilter === s
                    ? "bg-blue-600 text-white"
                    : "text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700"
                }`}
              >
                {s.charAt(0).toUpperCase() + s.slice(1)}
              </button>
            ))}
          </div>
        </div>

        {/* Users Table */}
        <div className="overflow-x-auto">
          <table className="w-full text-sm">
            <thead>
              <tr className="text-left text-gray-500 dark:text-gray-400 border-b border-gray-200 dark:border-gray-700">
                <th className="pb-2 font-medium">Name</th>
                <th className="pb-2 font-medium">Email</th>
                <th className="pb-2 font-medium">Brand</th>
                <th className="pb-2 font-medium text-right">Tokens</th>
                <th className="pb-2 font-medium text-right">Points</th>
                <th className="pb-2 font-medium">Status</th>
                <th className="pb-2 font-medium">Joined</th>
                <th className="pb-2 font-medium text-right">Actions</th>
              </tr>
            </thead>
            <tbody>
              {filteredUsers.map((user) => (
                <tr
                  key={user.id}
                  className="border-b border-gray-100 dark:border-gray-700/50 last:border-0"
                >
                  <td className="py-3 font-medium text-gray-900 dark:text-white">
                    {user.name}
                  </td>
                  <td className="py-3 text-gray-500 dark:text-gray-400">
                    {user.email}
                  </td>
                  <td className="py-3 text-gray-500 dark:text-gray-400">
                    {user.brand}
                  </td>
                  <td className="py-3 text-right text-gray-900 dark:text-white">
                    {formatNumber(user.tokens)}
                  </td>
                  <td className="py-3 text-right text-gray-900 dark:text-white">
                    {formatNumber(user.points)}
                  </td>
                  <td className="py-3">
                    <span
                      className={`inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium ${
                        user.status === "active"
                          ? "bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400"
                          : "bg-gray-100 text-gray-600 dark:bg-gray-700 dark:text-gray-400"
                      }`}
                    >
                      {user.status === "active" ? "Active" : "Inactive"}
                    </span>
                  </td>
                  <td className="py-3 text-gray-500 dark:text-gray-400">
                    {new Date(user.joined).toLocaleDateString("ko-KR")}
                  </td>
                  <td className="py-3 text-right">
                    <button className="px-3 py-1 text-sm font-medium text-blue-600 dark:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-900/20 rounded-md transition-colors">
                      View
                    </button>
                  </td>
                </tr>
              ))}
              {filteredUsers.length === 0 && (
                <tr>
                  <td
                    colSpan={8}
                    className="py-8 text-center text-gray-400 dark:text-gray-500"
                  >
                    No users found matching your criteria.
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}
