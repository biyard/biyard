import { test } from "@playwright/test";
import { click, fill } from "./utils";

test("create user storage state", async ({ page }) => {
  const email = "test@biyard.co";
  const password = "qwer1234!@#$";

  await page.goto("/");
  await page.waitForLoadState("networkidle");

  await fill(page, { placeholder: "Enter your email" }, email);
  await fill(page, { placeholder: "Enter your password" }, password);

  await click(page, { text: "Sign In" });

  await page.waitForURL("/dashboard");

  // Save Playwright storage state for authenticated tests
  await page.context().storageState({ path: "user.json" });

  console.log("✅ Global authenticated user setup completed");
  console.log(`📄 Test user saved: ${email}`);
  console.log(`🔐 Storage state saved to: user.json`);
});
