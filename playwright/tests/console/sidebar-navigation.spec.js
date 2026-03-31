import { test, expect } from "@playwright/test";
import { goto } from "../utils.js";

test.describe("Sidebar Navigation", () => {
  test("navigates between all console pages via sidebar", async ({ page }) => {
    await goto(page, "/dashboard");
    await expect(page.getByText("Biyard Console")).toBeVisible();

    // Navigate to Projects
    await page.getByRole("link", { name: "Projects" }).click();
    await page.waitForURL("**/projects");
    await expect(page.getByText("Projects").first()).toBeVisible();

    // Navigate to Credentials
    await page.getByRole("link", { name: "Credentials" }).click();
    await page.waitForURL("**/credentials");
    await expect(page.getByText("API Credentials")).toBeVisible();

    // Navigate to Settings
    await page.getByRole("link", { name: "Settings" }).click();
    await page.waitForURL("**/settings");
    await expect(page.getByText("Account Settings")).toBeVisible();

    // Navigate back to Dashboard
    await page.getByRole("link", { name: "Dashboard" }).click();
    await page.waitForURL("**/dashboard");
    await expect(
      page.getByText("Welcome to Biyard Console"),
    ).toBeVisible();
  });
});
