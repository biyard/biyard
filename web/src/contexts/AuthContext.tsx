import { createContext, useContext, useState } from 'react';
import type { ReactNode } from 'react';
import type { Account } from '../types/account';

interface AuthContextType {
  account: Account | null;
  setAccount: (account: Account | null) => void;
  isAuthenticated: boolean;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [account, setAccount] = useState<Account | null>(null);

  return (
    <AuthContext.Provider
      value={{
        account,
        setAccount,
        isAuthenticated: !!account
      }}
    >
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
}
