import { test, expect } from "@playwright/test";
import { goto } from "../utils.js";

test.describe("Dashboard", () => {
  test("displays page header and welcome section", async ({ page }) => {
    await goto(page, "/enterprise/overview");

    // New IA: the dashboard is titled "Enterprise Overview" and the
    // sidebar shows "Biyard Console" as a subtitle.
    await expect(
      page.getByText("Enterprise Overview", { exact: true }).first(),
    ).toBeVisible();
    await expect(page.getByText("Biyard Console").first()).toBeVisible();
  });

  test("displays user account info", async ({ page }) => {
    await goto(page, "/enterprise/overview");

    // Account info section shows test user details
    await expect(page.getByText("Playwright Test").first()).toBeVisible();
    await expect(page.getByText("test@biyard.co").first()).toBeVisible();
  });

  test("has quick stats and links to features", async ({ page }) => {
    await goto(page, "/enterprise/overview");

    // Dashboard now surfaces "My Brands" and "API Keys" stat tiles,
    // plus Recent Brands and Account Information sections.
    await expect(page.getByText("My Brands").first()).toBeVisible();
    await expect(page.getByText("API Keys").first()).toBeVisible();
    await expect(page.getByText("Account Information").first()).toBeVisible();
  });
});
