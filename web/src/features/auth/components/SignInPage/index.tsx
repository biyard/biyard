import { useState } from "react";
import { useNavigate, Link } from "react-router-dom";
import { useSignin } from "../../api/use-signin";
import { useAuth } from "../../../../contexts/AuthContext";
import { Mail, Lock, Loader2 } from "lucide-react";
import { useSignInPageI18n } from "./i18n";

export function SignInPage() {
  const t = useSignInPageI18n();
  const navigate = useNavigate();
  const { setAccount } = useAuth();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");

  const signinMutation = useSignin();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");

    if (!email || !password) {
      setError(t.emailRequired);
      return;
    }

    try {
      const account = await signinMutation.mutateAsync({
        email,
        password,
      });
      setAccount(account);
      navigate("/dashboard");
    } catch (_err) {
      setError(t.signInError);
    }
  };

  return (
    <div className="flex justify-center items-center px-4 min-h-screen bg-gray-50 dark:bg-gray-900">
      <div className="space-y-8 w-full max-w-md">
        <div className="text-center">
          <h1 className="text-4xl font-bold text-gray-900 dark:text-white">
            {t.title}
          </h1>
          <p className="mt-2 text-sm text-gray-600 dark:text-gray-400">
            {t.tagline}
          </p>
          <h2 className="mt-6 text-3xl font-extrabold text-gray-900 dark:text-white">
            {t.signInWithEmail}
          </h2>
        </div>

        <form className="mt-8 space-y-6" onSubmit={handleSubmit}>
          {error && (
            <div className="p-4 bg-red-50 rounded-md dark:bg-red-900/20">
              <p className="text-sm text-red-800 dark:text-red-400">{error}</p>
            </div>
          )}

          <div className="space-y-4">
            <div>
              <label
                htmlFor="email"
                className="block text-sm font-medium text-gray-700 dark:text-gray-300"
              >
                {t.email}
              </label>
              <div className="relative mt-1">
                <div className="flex absolute inset-y-0 left-0 items-center pl-3 pointer-events-none">
                  <Mail className="w-5 h-5 text-gray-400" />
                </div>
                <input
                  id="email"
                  name="email"
                  type="email"
                  autoComplete="email"
                  required
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  className="block py-2 pr-3 pl-10 w-full placeholder-gray-400 rounded-md border border-gray-300 shadow-sm appearance-none dark:text-white dark:bg-gray-800 dark:border-gray-600 focus:border-blue-500 focus:ring-blue-500 focus:outline-none"
                  placeholder={t.enterEmail}
                />
              </div>
            </div>

            <div>
              <label
                htmlFor="password"
                className="block text-sm font-medium text-gray-700 dark:text-gray-300"
              >
                {t.password}
              </label>
              <div className="relative mt-1">
                <div className="flex absolute inset-y-0 left-0 items-center pl-3 pointer-events-none">
                  <Lock className="w-5 h-5 text-gray-400" />
                </div>
                <input
                  id="password"
                  name="password"
                  type="password"
                  autoComplete="current-password"
                  required
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  className="block py-2 pr-3 pl-10 w-full placeholder-gray-400 rounded-md border border-gray-300 shadow-sm appearance-none dark:text-white dark:bg-gray-800 dark:border-gray-600 focus:border-blue-500 focus:ring-blue-500 focus:outline-none"
                  placeholder={t.enterPassword}
                />
              </div>
            </div>
          </div>

          <div>
            <button
              type="submit"
              disabled={signinMutation.isPending}
              className="flex justify-center py-2 px-4 w-full text-sm font-medium text-white bg-blue-600 rounded-md border border-transparent shadow-sm hover:bg-blue-700 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:outline-none disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {signinMutation.isPending ? (
                <>
                  <Loader2 className="mr-2 -ml-1 w-5 h-5 animate-spin" />
                  {t.loading}
                </>
              ) : (
                t.signIn
              )}
            </button>
          </div>

          <div className="text-center">
            <p className="text-sm text-gray-600 dark:text-gray-400">
              {t.noAccount}{" "}
              <Link
                to="/signup"
                className="font-medium text-blue-600 dark:text-blue-400 hover:text-blue-500"
              >
                {t.signUp}
              </Link>
            </p>
          </div>
        </form>
      </div>
    </div>
  );
}
