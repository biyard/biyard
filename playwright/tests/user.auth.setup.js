import { test, expect } from "@playwright/test";
import { goto } from "./utils.js";

test("create storage state", async ({ page }) => {
  const email = "test@biyard.co";
  const password = "qwer1234!@#$";

  await goto(page, "/signin");

  await page.locator("#email").fill(email);
  await page.locator("#password").fill(password);

  await page.getByRole("button", { name: "Sign In" }).click();

  // Wait for navigation to dashboard after successful sign-in
  await page.waitForURL("**/dashboard", { waitUntil: "load" });
  await expect(page.getByText("Biyard Console")).toBeVisible();

  // Save storage state for authenticated tests
  await page.context().storageState({ path: "user.json" });

  console.log("Auth setup completed for test@biyard.co");
});
