import { test, expect } from "@playwright/test";
import { goto } from "../utils.js";

test.describe("Credentials", () => {
  test("displays the credentials page", async ({ page }) => {
    await goto(page, "/credentials");
    await expect(page.getByText("API Credentials")).toBeVisible();
  });

  test("shows create new credential button", async ({ page }) => {
    await goto(page, "/credentials");
    await expect(
      page.getByRole("button", { name: /Create New/ }),
    ).toBeVisible();
  });

  test("opens and closes create credential dialog", async ({ page }) => {
    await goto(page, "/credentials");

    // Open create dialog
    await page.getByRole("button", { name: /Create New/ }).click();

    // Verify dialog form
    await expect(page.getByRole("textbox").first()).toBeVisible();

    // Close dialog
    await page.getByRole("button", { name: "Cancel" }).click();
  });
});
