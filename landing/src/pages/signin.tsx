import { useState } from "react";
import { useNavigate, Link } from "react-router";

export function SignInPage() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const navigate = useNavigate();

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    // Mock login - just navigate to wallet
    localStorage.setItem(
      "biyard-user",
      JSON.stringify({ email, name: email.split("@")[0] })
    );
    navigate("/wallet");
  };

  return (
    <div className="min-h-screen bg-[#0a0e17] flex items-center justify-center px-4">
      <div className="w-full max-w-md">
        <div className="text-center mb-8">
          <h1 className="text-3xl font-bold text-white">Biyard</h1>
          <p className="text-gray-400 mt-1">Launchpad Platform</p>
        </div>
        <div className="bg-[#0f1420] rounded-2xl p-8 border border-gray-800">
          <h2 className="text-xl font-bold text-white mb-6">Sign In</h2>
          <form onSubmit={handleSubmit} className="space-y-4">
            <div>
              <label className="block text-sm text-gray-400 mb-1">Email</label>
              <input
                type="email"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                required
                className="w-full px-4 py-3 bg-[#0a0e17] border border-gray-700 rounded-lg text-white focus:outline-none focus:border-blue-500"
              />
            </div>
            <div>
              <label className="block text-sm text-gray-400 mb-1">
                Password
              </label>
              <input
                type="password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                required
                className="w-full px-4 py-3 bg-[#0a0e17] border border-gray-700 rounded-lg text-white focus:outline-none focus:border-blue-500"
              />
            </div>
            <button
              type="submit"
              className="w-full py-3 bg-blue-600 hover:bg-blue-500 text-white font-semibold rounded-lg transition-colors"
            >
              Sign In
            </button>
          </form>
          <p className="text-center text-sm text-gray-500 mt-6">
            Don't have an account?{" "}
            <Link to="/signup" className="text-blue-400 hover:text-blue-300">
              Sign Up
            </Link>
          </p>
        </div>
      </div>
    </div>
  );
}
