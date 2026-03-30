import { Link, useLocation } from "react-router";

export function UserNav() {
  const location = useLocation();
  const user = JSON.parse(localStorage.getItem("biyard-user") || "{}");

  const handleSignOut = () => {
    localStorage.removeItem("biyard-user");
    window.location.href = "/";
  };

  return (
    <nav className="border-b border-gray-800 bg-[#0a0e17]">
      <div className="max-w-5xl mx-auto px-4 h-14 flex items-center justify-between">
        <div className="flex items-center gap-6">
          <Link to="/" className="text-lg font-bold text-white">
            Biyard
          </Link>
          <div className="flex gap-1">
            <Link
              to="/wallet"
              className={`px-3 py-1.5 rounded-lg text-sm font-medium transition-colors ${
                location.pathname === "/wallet"
                  ? "bg-gray-800 text-white"
                  : "text-gray-400 hover:text-white"
              }`}
            >
              Wallet
            </Link>
            <Link
              to="/dao"
              className={`px-3 py-1.5 rounded-lg text-sm font-medium transition-colors ${
                location.pathname === "/dao"
                  ? "bg-gray-800 text-white"
                  : "text-gray-400 hover:text-white"
              }`}
            >
              DAO
            </Link>
          </div>
        </div>
        <div className="flex items-center gap-3">
          <span className="text-sm text-gray-400">{user.email || ""}</span>
          <button
            onClick={handleSignOut}
            className="text-sm text-red-400 hover:text-red-300"
          >
            Sign Out
          </button>
        </div>
      </div>
    </nav>
  );
}
