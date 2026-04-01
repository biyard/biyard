import { createContext, useContext, useState, type ReactNode } from "react";

type Mode = "admin" | "user";

interface ModeContextType {
  mode: Mode;
  setMode: (mode: Mode) => void;
  isAdmin: boolean;
  isUser: boolean;
}

const ModeContext = createContext<ModeContextType | undefined>(undefined);

export function ModeProvider({ children }: { children: ReactNode }) {
  const [mode, setMode] = useState<Mode>(() => {
    return (localStorage.getItem("app-mode") as Mode) || "admin";
  });

  const handleSetMode = (newMode: Mode) => {
    setMode(newMode);
    localStorage.setItem("app-mode", newMode);
  };

  return (
    <ModeContext.Provider
      value={{
        mode,
        setMode: handleSetMode,
        isAdmin: mode === "admin",
        isUser: mode === "user",
      }}
    >
      {children}
    </ModeContext.Provider>
  );
}

export function useMode() {
  const context = useContext(ModeContext);
  if (!context) throw new Error("useMode must be used within ModeProvider");
  return context;
}
