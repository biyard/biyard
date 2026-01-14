import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { AuthProvider } from "./contexts/AuthContext";
import { ThemeProvider } from "./contexts/ThemeContext";
import { SignInPage } from "./features/auth/components/SignInPage";
import { SignUpPage } from "./features/auth/components/SignUpPage";
import { DashboardPage } from "./features/dashboard/components/DashboardPage";
import { SettingsPage } from "./features/settings/components/SettingsPage";
import { CredentialsPage } from "./features/credentials/components/CredentialsPage";
import { ProjectsPage } from "./features/projects/components/ProjectsPage";
import { ProjectDetailPage } from "./features/projects/components/ProjectDetailPage";
import { ProtectedRoute } from "./components/ProtectedRoute";
import { ConsoleLayout } from "./components/layout/ConsoleLayout";
import { Toaster } from "./components/ui/toaster";

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
      retry: 1,
    },
  },
});

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <ThemeProvider>
        <AuthProvider>
          <BrowserRouter>
            <Routes>
              <Route path="/" element={<Navigate to="/dashboard" replace />} />
              <Route path="/signin" element={<SignInPage />} />
              <Route path="/signup" element={<SignUpPage />} />
              <Route
                path="/dashboard"
                element={
                  <ProtectedRoute>
                    <ConsoleLayout>
                      <DashboardPage />
                    </ConsoleLayout>
                  </ProtectedRoute>
                }
              />
              <Route
                path="/settings"
                element={
                  <ProtectedRoute>
                    <ConsoleLayout>
                      <SettingsPage />
                    </ConsoleLayout>
                  </ProtectedRoute>
                }
              />
              <Route
                path="/credentials"
                element={
                  <ProtectedRoute>
                    <ConsoleLayout>
                      <CredentialsPage />
                    </ConsoleLayout>
                  </ProtectedRoute>
                }
              />
              <Route
                path="/projects"
                element={
                  <ProtectedRoute>
                    <ConsoleLayout>
                      <ProjectsPage />
                    </ConsoleLayout>
                  </ProtectedRoute>
                }
              />
              <Route
                path="/projects/:projectId"
                element={
                  <ProtectedRoute>
                    <ConsoleLayout>
                      <ProjectDetailPage />
                    </ConsoleLayout>
                  </ProtectedRoute>
                }
              />
            </Routes>
          </BrowserRouter>
          <Toaster />
        </AuthProvider>
      </ThemeProvider>
    </QueryClientProvider>
  );
}

export default App;
