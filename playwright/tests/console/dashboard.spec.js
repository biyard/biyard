import { test, expect } from "@playwright/test";
import { goto } from "../utils.js";

test.describe("Dashboard", () => {
  test("displays page header and welcome section", async ({ page }) => {
    await goto(page, "/dashboard");

    await expect(page.getByText("Biyard Console", { exact: true })).toBeVisible();
    await expect(
      page.getByText("Welcome to Biyard Console"),
    ).toBeVisible();
  });

  test("displays user account info", async ({ page }) => {
    await goto(page, "/dashboard");

    // Account info section shows test user details
    await expect(page.getByText("Playwright Test")).toBeVisible();
    await expect(page.getByText("test@biyard.co")).toBeVisible();
  });

  test("has quick action cards linking to features", async ({ page }) => {
    await goto(page, "/dashboard");

    await expect(page.getByText("My Projects")).toBeVisible();
    await expect(page.getByText("API Credentials")).toBeVisible();
    await expect(page.getByText("Account Settings")).toBeVisible();
  });
});
