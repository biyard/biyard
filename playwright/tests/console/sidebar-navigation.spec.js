import { test, expect } from "@playwright/test";
import { goto } from "../utils.js";

test.describe("Sidebar Navigation", () => {
  test("navigates between all console pages via sidebar", async ({ page }) => {
    await goto(page, "/dashboard");
    await expect(page.getByText("Biyard Console", { exact: true }).first()).toBeVisible();

    // Navigate to Brands via sidebar
    await page.getByRole("link", { name: "Brands", exact: true }).click();
    await page.waitForURL("**/projects");
    await expect(page.getByText("Brands").first()).toBeVisible();

    // Navigate to Credentials via sidebar
    await page.getByRole("link", { name: "Credentials", exact: true }).click();
    await page.waitForURL("**/credentials");
    await expect(page.getByText("API Credentials").first()).toBeVisible();

    // Navigate to Settings via sidebar
    await page.getByRole("link", { name: "Settings", exact: true }).click();
    await page.waitForURL("**/settings");
    await expect(page.getByText("Account Settings").first()).toBeVisible();

    // Navigate back to Dashboard via sidebar
    await page.getByRole("link", { name: "Dashboard", exact: true }).click();
    await page.waitForURL("**/dashboard");
    await expect(
      page.getByText("Welcome to Biyard Console").first(),
    ).toBeVisible();
  });
});
