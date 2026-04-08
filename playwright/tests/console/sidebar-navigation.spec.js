import { test, expect } from "@playwright/test";
import { goto } from "../utils.js";

test.describe("Sidebar Navigation", () => {
  test("navigates between enterprise pages via sidebar", async ({ page }) => {
    await goto(page, "/enterprise/overview");
    await expect(
      page.getByText("Enterprise Overview", { exact: true }).first(),
    ).toBeVisible();

    // Sidebar enterprise-scope links: Overview, General, Members, API Keys.
    // Settings (profile) lives inside the account menu, not the sidebar.

    // Navigate to General (enterprise settings)
    await page
      .getByRole("link", { name: "Settings", exact: true })
      .first()
      .click();
    await page.waitForURL("**/enterprise/settings/general");

    // Navigate to Members
    await page
      .getByRole("link", { name: "Members", exact: true })
      .first()
      .click();
    await page.waitForURL("**/enterprise/settings/members");

    // Navigate to API Keys
    await page
      .getByRole("link", { name: "API Keys", exact: true })
      .first()
      .click();
    await page.waitForURL("**/enterprise/settings/api-keys");
    await expect(page.getByText("API Keys").first()).toBeVisible();

    // Navigate back to Overview
    await page
      .getByRole("link", { name: "Overview", exact: true })
      .first()
      .click();
    await page.waitForURL("**/enterprise/overview");
    await expect(
      page.getByText("Enterprise Overview", { exact: true }).first(),
    ).toBeVisible();
  });
});
