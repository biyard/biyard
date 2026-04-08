import { test, expect } from "@playwright/test";
import { goto } from "../utils.js";

test.describe("Credentials", () => {
  test("displays the credentials page", async ({ page }) => {
    await goto(page, "/enterprise/settings/api-keys");
    await expect(page.getByText("API Keys").first()).toBeVisible();
  });

  test("shows create new credential button", async ({ page }) => {
    await goto(page, "/enterprise/settings/api-keys");
    await expect(
      page.getByRole("button", { name: /Create New/ }).first(),
    ).toBeVisible();
  });

  test("opens and closes create credential dialog", async ({ page }) => {
    await goto(page, "/enterprise/settings/api-keys");

    // Open create dialog — retry to handle WASM hydration timing
    await expect(async () => {
      await page.getByRole("button", { name: /Create New/ }).first().click();
      await expect(page.getByRole("textbox").first()).toBeVisible();
    }).toPass({ intervals: [500, 1000, 2000], timeout: 15000 });

    // Close dialog
    await page.getByRole("button", { name: "Cancel" }).click();
  });
});
