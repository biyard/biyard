import { test, expect } from "@playwright/test";
import { goto } from "../utils.js";

test.describe("Settings", () => {
  test("displays account settings page with profile info", async ({
    page,
  }) => {
    await goto(page, "/account/profile");

    // Page title is "Profile" (personal scope). The page shows the
    // user's display name (editable input) and email (read-only input).
    await expect(
      page.getByText("Profile", { exact: true }).first(),
    ).toBeVisible();
    await expect(page.locator("input[type='text']").first()).toHaveValue(
      "Playwright Test",
    );
    await expect(page.locator("input[type='email']").first()).toHaveValue(
      "test@biyard.co",
    );
  });

  test("shows danger zone with delete account option", async ({ page }) => {
    await goto(page, "/account/profile");

    await expect(page.getByText("Danger Zone").first()).toBeVisible();
    await expect(page.getByText("Delete Account").first()).toBeVisible();
  });
});
