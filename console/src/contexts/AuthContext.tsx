import { createContext, useContext, useState, useEffect } from "react";
import type { ReactNode } from "react";
import type { Account } from "@/features/auth/dto/account";
import { useValidateSession } from "@/features/auth/hooks/use-validate-session";

interface AuthContextType {
  account: Account | null;
  setAccount: (account: Account | null) => void;
  isAuthenticated: boolean;
  isLoading: boolean;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [account, setAccount] = useState<Account | null>(null);
  const sessionQuery = useValidateSession();

  useEffect(() => {
    if (sessionQuery.data && !account) {
      setAccount(sessionQuery.data);
    }
  }, [sessionQuery.data, sessionQuery.isLoading, sessionQuery.error, account]);

  const isLoading = sessionQuery.isLoading || (sessionQuery.data && !account);

  return (
    <AuthContext.Provider
      value={{
        account,
        setAccount,
        isAuthenticated: !!account,
        isLoading: !!isLoading,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error("useAuth must be used within an AuthProvider");
  }
  return context;
}
