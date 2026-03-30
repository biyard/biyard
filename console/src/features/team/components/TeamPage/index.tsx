import { useState } from "react";
import {
  UserPlus,
  Mail,
  Shield,
  Eye,
  Code,
  Crown,
  MoreVertical,
} from "lucide-react";
import { toast } from "sonner";

interface Member {
  id: string;
  name: string;
  email: string;
  role: "owner" | "admin" | "developer" | "viewer";
  lastActive: string;
  status: "active" | "inactive";
}

interface PendingInvite {
  id: string;
  email: string;
  role: "admin" | "developer" | "viewer";
  sentAt: string;
}

const INITIAL_MEMBERS: Member[] = [
  {
    id: "1",
    name: "Jihwan Park",
    email: "jhpark@biyard.co",
    role: "owner",
    lastActive: "2026-03-30T10:00:00Z",
    status: "active",
  },
  {
    id: "2",
    name: "Kim Minsoo",
    email: "minsoo@lemouton.com",
    role: "admin",
    lastActive: "2026-03-29T15:30:00Z",
    status: "active",
  },
  {
    id: "3",
    name: "Lee Seoyeon",
    email: "seoyeon@lemouton.com",
    role: "developer",
    lastActive: "2026-03-28T09:00:00Z",
    status: "active",
  },
  {
    id: "4",
    name: "Park Jiwoo",
    email: "jiwoo@lemouton.com",
    role: "viewer",
    lastActive: "2026-03-20T14:00:00Z",
    status: "inactive",
  },
];

const INITIAL_INVITES: PendingInvite[] = [
  {
    id: "inv1",
    email: "newdev@lemouton.com",
    role: "developer",
    sentAt: "2026-03-28T12:00:00Z",
  },
  {
    id: "inv2",
    email: "analytics@partner.com",
    role: "viewer",
    sentAt: "2026-03-27T09:00:00Z",
  },
];

const ROLE_BADGE: Record<
  string,
  { label: string; bg: string; text: string; icon: React.ReactNode }
> = {
  owner: {
    label: "Owner",
    bg: "bg-purple-100 dark:bg-purple-900/30",
    text: "text-purple-700 dark:text-purple-300",
    icon: <Crown className="w-3 h-3" />,
  },
  admin: {
    label: "Admin",
    bg: "bg-blue-100 dark:bg-blue-900/30",
    text: "text-blue-700 dark:text-blue-300",
    icon: <Shield className="w-3 h-3" />,
  },
  developer: {
    label: "Developer",
    bg: "bg-green-100 dark:bg-green-900/30",
    text: "text-green-700 dark:text-green-300",
    icon: <Code className="w-3 h-3" />,
  },
  viewer: {
    label: "Viewer",
    bg: "bg-gray-100 dark:bg-gray-700",
    text: "text-gray-600 dark:text-gray-300",
    icon: <Eye className="w-3 h-3" />,
  },
};

const ROLE_PERMISSIONS = [
  {
    role: "Owner",
    icon: <Crown className="w-5 h-5 text-purple-600" />,
    permissions: [
      "Full access",
      "Billing management",
      "Team management",
      "Delete project",
    ],
  },
  {
    role: "Admin",
    icon: <Shield className="w-5 h-5 text-blue-600" />,
    permissions: [
      "Project settings",
      "API keys",
      "Challenges management",
      "View analytics",
    ],
  },
  {
    role: "Developer",
    icon: <Code className="w-5 h-5 text-green-600" />,
    permissions: ["API keys (own)", "View docs", "Test sandbox"],
  },
  {
    role: "Viewer",
    icon: <Eye className="w-5 h-5 text-gray-500" />,
    permissions: ["Read-only dashboard", "View analytics"],
  },
];

function formatRelativeTime(dateStr: string): string {
  const now = new Date("2026-03-30T12:00:00Z");
  const date = new Date(dateStr);
  const diffMs = now.getTime() - date.getTime();
  const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
  const diffDays = Math.floor(diffHours / 24);

  if (diffHours < 1) return "Just now";
  if (diffHours < 24) return `${diffHours}h ago`;
  if (diffDays === 1) return "Yesterday";
  return `${diffDays} days ago`;
}

function formatDate(dateStr: string): string {
  return new Date(dateStr).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  });
}

export function TeamPage() {
  const [members, setMembers] = useState<Member[]>(INITIAL_MEMBERS);
  const [changingRoleId, setChangingRoleId] = useState<string | null>(null);
  const [pendingInvites, setPendingInvites] =
    useState<PendingInvite[]>(INITIAL_INVITES);
  const [inviteEmail, setInviteEmail] = useState("");
  const [inviteRole, setInviteRole] = useState<"admin" | "developer" | "viewer">(
    "developer"
  );
  const [showPermissions, setShowPermissions] = useState(false);
  const [openMenuId, setOpenMenuId] = useState<string | null>(null);

  const handleSendInvite = () => {
    if (!inviteEmail.trim()) {
      toast.error("Please enter an email address");
      return;
    }
    if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(inviteEmail)) {
      toast.error("Please enter a valid email address");
      return;
    }
    setPendingInvites((prev) => [
      ...prev,
      {
        id: `inv-${Date.now()}`,
        email: inviteEmail,
        role: inviteRole,
        sentAt: new Date().toISOString(),
      },
    ]);
    toast.success(`Invitation sent to ${inviteEmail}`);
    setInviteEmail("");
  };

  const handleResendInvite = (email: string) => {
    toast.success(`Invitation resent to ${email}`);
  };

  const handleCancelInvite = (id: string) => {
    setPendingInvites((prev) => prev.filter((inv) => inv.id !== id));
    toast.success("Invitation cancelled");
  };

  return (
    <div>
      <div className="mb-6">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          Team Management
        </h1>
        <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
          Manage team members, roles, and permissions for your project.
        </p>
      </div>

      {/* Invite Member Form */}
      <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-6 mb-6">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center gap-2">
          <UserPlus className="w-5 h-5 text-blue-600" />
          Invite Member
        </h2>
        <div className="flex flex-col sm:flex-row gap-3">
          <div className="flex-1">
            <input
              type="email"
              value={inviteEmail}
              onChange={(e) => setInviteEmail(e.target.value)}
              placeholder="colleague@company.com"
              className="w-full px-4 py-2.5 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none"
            />
          </div>
          <select
            value={inviteRole}
            onChange={(e) =>
              setInviteRole(e.target.value as "admin" | "developer" | "viewer")
            }
            className="px-4 py-2.5 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none"
          >
            <option value="admin">Admin</option>
            <option value="developer">Developer</option>
            <option value="viewer">Viewer</option>
          </select>
          <button
            onClick={handleSendInvite}
            className="px-6 py-2.5 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg transition-colors flex items-center justify-center gap-2"
          >
            <Mail className="w-4 h-4" />
            Send Invite
          </button>
        </div>
      </div>

      {/* Current Team Members */}
      <div className="bg-white dark:bg-gray-800 shadow rounded-lg overflow-hidden mb-6">
        <div className="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
          <h2 className="text-lg font-semibold text-gray-900 dark:text-white">
            Team Members ({members.length})
          </h2>
        </div>
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead>
              <tr className="border-b border-gray-200 dark:border-gray-700">
                <th className="text-left px-6 py-3 text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                  Name
                </th>
                <th className="text-left px-6 py-3 text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                  Email
                </th>
                <th className="text-left px-6 py-3 text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                  Role
                </th>
                <th className="text-left px-6 py-3 text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                  Last Active
                </th>
                <th className="text-left px-6 py-3 text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                  Status
                </th>
                <th className="text-right px-6 py-3 text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                  Actions
                </th>
              </tr>
            </thead>
            <tbody className="divide-y divide-gray-200 dark:divide-gray-700">
              {members.map((member) => {
                const badge = ROLE_BADGE[member.role];
                return (
                  <tr
                    key={member.id}
                    className="hover:bg-gray-50 dark:hover:bg-gray-750"
                  >
                    <td className="px-6 py-4 text-sm font-medium text-gray-900 dark:text-white whitespace-nowrap">
                      {member.name}
                    </td>
                    <td className="px-6 py-4 text-sm text-gray-600 dark:text-gray-400 whitespace-nowrap">
                      {member.email}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span
                        className={`inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-xs font-medium ${badge.bg} ${badge.text}`}
                      >
                        {badge.icon}
                        {badge.label}
                      </span>
                    </td>
                    <td className="px-6 py-4 text-sm text-gray-600 dark:text-gray-400 whitespace-nowrap">
                      {formatRelativeTime(member.lastActive)}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span
                        className={`inline-flex items-center gap-1.5 text-xs font-medium ${
                          member.status === "active"
                            ? "text-green-600 dark:text-green-400"
                            : "text-gray-400"
                        }`}
                      >
                        <span
                          className={`w-1.5 h-1.5 rounded-full ${
                            member.status === "active"
                              ? "bg-green-500"
                              : "bg-gray-400"
                          }`}
                        />
                        {member.status === "active" ? "Active" : "Inactive"}
                      </span>
                    </td>
                    <td className="px-6 py-4 text-right whitespace-nowrap">
                      <div className="relative inline-block">
                        <button
                          onClick={() =>
                            setOpenMenuId(
                              openMenuId === member.id ? null : member.id
                            )
                          }
                          disabled={member.role === "owner"}
                          className="p-1.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 disabled:opacity-30 disabled:cursor-not-allowed transition-colors"
                        >
                          <MoreVertical className="w-4 h-4" />
                        </button>
                        {openMenuId === member.id && member.role !== "owner" && (
                          <div className="absolute right-0 mt-1 w-48 bg-white dark:bg-gray-700 border border-gray-200 dark:border-gray-600 rounded-lg shadow-lg z-10">
                            {changingRoleId === member.id ? (
                              <div className="px-3 py-2">
                                <p className="text-xs font-medium text-gray-500 dark:text-gray-400 mb-1">Select new role</p>
                                <select
                                  defaultValue={member.role}
                                  onChange={(e) => {
                                    const newRole = e.target.value as Member["role"];
                                    setMembers((prev) =>
                                      prev.map((m) =>
                                        m.id === member.id ? { ...m, role: newRole } : m
                                      )
                                    );
                                    toast.success(`${member.name}'s role changed to ${newRole}`);
                                    setChangingRoleId(null);
                                    setOpenMenuId(null);
                                  }}
                                  className="w-full px-2 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                >
                                  <option value="admin">Admin</option>
                                  <option value="developer">Developer</option>
                                  <option value="viewer">Viewer</option>
                                </select>
                              </div>
                            ) : (
                              <>
                                <button
                                  onClick={() => {
                                    setChangingRoleId(member.id);
                                  }}
                                  className="block w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-600 first:rounded-t-lg"
                                >
                                  Change Role
                                </button>
                                <button
                                  onClick={() => {
                                    if (window.confirm(`Are you sure you want to remove ${member.name} from the team?`)) {
                                      setMembers((prev) => prev.filter((m) => m.id !== member.id));
                                      toast.success(`${member.name} removed from team`);
                                    }
                                    setOpenMenuId(null);
                                  }}
                                  className="block w-full text-left px-4 py-2 text-sm text-red-600 dark:text-red-400 hover:bg-gray-100 dark:hover:bg-gray-600 last:rounded-b-lg"
                                >
                                  Remove
                                </button>
                              </>
                            )}
                          </div>
                        )}
                      </div>
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      </div>

      {/* Pending Invites */}
      {pendingInvites.length > 0 && (
        <div className="bg-white dark:bg-gray-800 shadow rounded-lg overflow-hidden mb-6">
          <div className="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
            <h2 className="text-lg font-semibold text-gray-900 dark:text-white">
              Pending Invites ({pendingInvites.length})
            </h2>
          </div>
          <div className="divide-y divide-gray-200 dark:divide-gray-700">
            {pendingInvites.map((invite) => {
              const badge = ROLE_BADGE[invite.role];
              return (
                <div
                  key={invite.id}
                  className="px-6 py-4 flex items-center justify-between"
                >
                  <div className="flex items-center gap-4">
                    <div className="w-9 h-9 rounded-full bg-gray-100 dark:bg-gray-700 flex items-center justify-center">
                      <Mail className="w-4 h-4 text-gray-400" />
                    </div>
                    <div>
                      <p className="text-sm font-medium text-gray-900 dark:text-white">
                        {invite.email}
                      </p>
                      <p className="text-xs text-gray-400">
                        Sent {formatDate(invite.sentAt)}
                      </p>
                    </div>
                    <span
                      className={`inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-xs font-medium ${badge.bg} ${badge.text}`}
                    >
                      {badge.icon}
                      {badge.label}
                    </span>
                  </div>
                  <div className="flex items-center gap-2">
                    <button
                      onClick={() => handleResendInvite(invite.email)}
                      className="px-3 py-1.5 text-xs font-medium text-blue-600 dark:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-900/20 rounded-lg transition-colors"
                    >
                      Resend
                    </button>
                    <button
                      onClick={() => handleCancelInvite(invite.id)}
                      className="px-3 py-1.5 text-xs font-medium text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 rounded-lg transition-colors"
                    >
                      Cancel
                    </button>
                  </div>
                </div>
              );
            })}
          </div>
        </div>
      )}

      {/* Role Permissions */}
      <div className="bg-white dark:bg-gray-800 shadow rounded-lg overflow-hidden">
        <button
          onClick={() => setShowPermissions(!showPermissions)}
          className="w-full px-6 py-4 flex items-center justify-between text-left"
        >
          <h2 className="text-lg font-semibold text-gray-900 dark:text-white flex items-center gap-2">
            <Shield className="w-5 h-5 text-gray-400" />
            Role Permissions
          </h2>
          <svg
            className={`w-5 h-5 text-gray-400 transition-transform ${
              showPermissions ? "rotate-180" : ""
            }`}
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M19 9l-7 7-7-7"
            />
          </svg>
        </button>

        {showPermissions && (
          <div className="px-6 pb-6 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
            {ROLE_PERMISSIONS.map((rp) => (
              <div
                key={rp.role}
                className="border border-gray-200 dark:border-gray-700 rounded-lg p-4"
              >
                <div className="flex items-center gap-2 mb-3">
                  {rp.icon}
                  <h3 className="font-semibold text-gray-900 dark:text-white text-sm">
                    {rp.role}
                  </h3>
                </div>
                <ul className="space-y-1.5">
                  {rp.permissions.map((perm) => (
                    <li
                      key={perm}
                      className="text-xs text-gray-600 dark:text-gray-400 flex items-start gap-1.5"
                    >
                      <span className="text-green-500 mt-0.5">&#10003;</span>
                      {perm}
                    </li>
                  ))}
                </ul>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
