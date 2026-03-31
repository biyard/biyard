import { test, expect } from "@playwright/test";
import { goto } from "../utils.js";

test.describe("Settings", () => {
  test("displays account settings page with profile info", async ({
    page,
  }) => {
    await goto(page, "/settings");

    await expect(page.getByText("Account Settings")).toBeVisible();
    await expect(page.getByText("Profile")).toBeVisible();
    await expect(page.getByText("Playwright Test")).toBeVisible();
    await expect(page.getByText("test@biyard.co")).toBeVisible();
  });

  test("shows danger zone with delete account option", async ({ page }) => {
    await goto(page, "/settings");

    await expect(page.getByText("Delete Account").first()).toBeVisible();
  });
});
