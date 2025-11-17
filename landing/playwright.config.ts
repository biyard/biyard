import { defineConfig, devices } from "@playwright/test";

/**
 * See https://playwright.dev/docs/test-configuration.
 */
export default defineConfig({
  // Test discovery: both /tests directory and component-colocated tests
  testDir: "./src",
  testMatch: "**/*.spec.ts",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: 1,
  reporter: [["html", { open: "never", host: "0.0.0.0" }]],
  use: {
    baseURL: process.env.BASE_URL || "http://localhost:5000",
    trace: "on",
    video: "on",
  },

  projects: [
    {
      name: "user-setup",
      testMatch: ["tests/user.setup.ts"],
    },
    {
      name: "user-test",
      dependencies: ["user-setup"],
      use: { ...devices["Desktop Chrome"], storageState: "user.json" },
    },
  ],

  /* Run your local dev server before starting the tests */
  webServer: process.env.CI
    ? undefined
    : {
        command: "pnpm dev",
        url: "http://localhost:5000",
        reuseExistingServer: !process.env.CI,
        timeout: 120000,
      },
});
