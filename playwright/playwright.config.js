// @ts-check
import { defineConfig, devices } from "@playwright/test";
import { CONFIGS } from "./tests/config.js";

/**
 * @see https://playwright.dev/docs/test-configuration
 */
export default defineConfig({
  testDir: ".",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: [["html", { open: "never", host: "0.0.0.0" }]],
  timeout: CONFIGS.TIMEOUT,
  use: {
    baseURL: CONFIGS.BASE_URL,
    navigationTimeout: CONFIGS.TIMEOUT,
    locale: "en-US",
    trace: "on",
    video: "on",
    screenshot: "on",
  },

  projects: [
    {
      name: "auth-setup",
      testMatch: ["**/*.auth.setup.js"],
    },
    {
      name: "console-tests",
      testMatch: ["tests/console/**/*.spec.js"],
      dependencies: ["auth-setup"],
      use: {
        ...devices["Desktop Chrome"],
        viewport: {
          width: 1440,
          height: 950,
        },
        storageState: "user.json",
      },
    },
  ],
});
