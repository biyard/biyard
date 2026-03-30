import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { AuthProvider } from "./contexts/AuthContext";
import { ThemeProvider } from "./contexts/ThemeContext";
import { ModeProvider } from "./contexts/ModeContext";
import { SignInPage } from "./features/auth/components/SignInPage";
import { SignUpPage } from "./features/auth/components/SignUpPage";
import { DashboardPage } from "./features/dashboard/components/DashboardPage";
import { SettingsPage } from "./features/settings/components/SettingsPage";
import { CredentialsPage } from "./features/credentials/components/CredentialsPage";
import { ProjectsPage } from "./features/projects/components/ProjectsPage";
import { ProjectDetailPage } from "./features/projects/components/ProjectDetailPage";
import { ROISimulatorPage } from "./features/roi-simulator/components/ROISimulatorPage";
import { DemoPage } from "./features/demo/components/DemoPage";
import { ChallengeBuilderPage } from "./features/challenge-builder/components/ChallengeBuilderPage";
import { WebhooksPage } from "./features/webhooks/components/WebhooksPage";
import { PricingPage } from "./features/pricing/components/PricingPage";
import { UsagePage } from "./features/usage/components/UsagePage";
import { SLAPage } from "./features/sla/components/SLAPage";
import { DeveloperPortalPage } from "./features/developer-portal/components/DeveloperPortalPage";
import { WidgetsPage } from "./features/widgets/components/WidgetsPage";
import { SuperAdminPage } from "./features/super-admin/components/SuperAdminPage";
import { TenantOnboardingPage } from "./features/tenant-onboarding/components/TenantOnboardingPage";
import { TeamPage } from "./features/team/components/TeamPage";
import { ExchangePage } from "./features/exchange/components/ExchangePage";
import { UserDashboardPage } from "./features/user-portal/components/UserDashboardPage";
import { UserChallengePage } from "./features/user-portal/components/UserChallengePage";
import { UserWalletPage } from "./features/user-portal/components/UserWalletPage";
import { UserDAOPage } from "./features/user-portal/components/UserDAOPage";
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

function ProtectedPage({ children }: { children: React.ReactNode }) {
  return (
    <ProtectedRoute>
      <ConsoleLayout>{children}</ConsoleLayout>
    </ProtectedRoute>
  );
}

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <ThemeProvider>
        <AuthProvider>
          <ModeProvider>
          <BrowserRouter>
            <Routes>
              <Route
                path="/"
                element={<Navigate to="/dashboard" replace />}
              />
              <Route path="/signin" element={<SignInPage />} />
              <Route path="/signup" element={<SignUpPage />} />

              {/* Public pages */}
              <Route path="/demo" element={<DemoPage />} />
              <Route path="/pricing" element={<PricingPage />} />
              <Route path="/roi-simulator" element={<ROISimulatorPage />} />
              <Route path="/sla" element={<SLAPage />} />

              {/* Console */}
              <Route path="/dashboard" element={<ProtectedPage><DashboardPage /></ProtectedPage>} />
              <Route path="/projects" element={<ProtectedPage><ProjectsPage /></ProtectedPage>} />
              <Route path="/projects/:projectId" element={<ProtectedPage><ProjectDetailPage /></ProtectedPage>} />

              {/* Build */}
              <Route path="/challenge-builder" element={<ProtectedPage><ChallengeBuilderPage /></ProtectedPage>} />
              <Route path="/widgets" element={<ProtectedPage><WidgetsPage /></ProtectedPage>} />

              {/* Integrate */}
              <Route path="/developer" element={<ProtectedPage><DeveloperPortalPage /></ProtectedPage>} />
              <Route path="/credentials" element={<ProtectedPage><CredentialsPage /></ProtectedPage>} />
              <Route path="/webhooks" element={<ProtectedPage><WebhooksPage /></ProtectedPage>} />

              {/* Account */}
              <Route path="/team" element={<ProtectedPage><TeamPage /></ProtectedPage>} />
              <Route path="/usage" element={<ProtectedPage><UsagePage /></ProtectedPage>} />
              <Route path="/settings" element={<ProtectedPage><SettingsPage /></ProtectedPage>} />
              <Route path="/setup" element={<ProtectedPage><TenantOnboardingPage /></ProtectedPage>} />

              {/* User Portal */}
              <Route path="/user/dashboard" element={<ProtectedPage><UserDashboardPage /></ProtectedPage>} />
              <Route path="/user/challenge" element={<ProtectedPage><UserChallengePage /></ProtectedPage>} />
              <Route path="/user/wallet" element={<ProtectedPage><UserWalletPage /></ProtectedPage>} />
              <Route path="/user/exchange" element={<ProtectedPage><ExchangePage /></ProtectedPage>} />
              <Route path="/user/dao" element={<ProtectedPage><UserDAOPage /></ProtectedPage>} />

              {/* Internal */}
              <Route path="/super-admin" element={<ProtectedPage><SuperAdminPage /></ProtectedPage>} />
            </Routes>
          </BrowserRouter>
          <Toaster />
          </ModeProvider>
        </AuthProvider>
      </ThemeProvider>
    </QueryClientProvider>
  );
}

export default App;
