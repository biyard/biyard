import { test, expect } from "@playwright/test";
import { goto } from "../utils.js";

test.describe("Brands", () => {
  test("displays the brands page", async ({ page }) => {
    await goto(page, "/projects");
    await expect(page.getByText("Brands").first()).toBeVisible();
  });

  test("shows create new brand button", async ({ page }) => {
    await goto(page, "/projects");
    await expect(
      page.getByRole("button", { name: /Create New/ }).first(),
    ).toBeVisible();
  });

  test("opens and closes create brand dialog", async ({ page }) => {
    await goto(page, "/projects");

    // Open create dialog — retry to handle WASM hydration timing
    await expect(async () => {
      await page.getByRole("button", { name: /Create New/ }).first().click();
      await expect(page.getByText("Create Brand").first()).toBeVisible();
    }).toPass({ intervals: [500, 1000, 2000], timeout: 15000 });

    // Verify form fields exist
    await expect(page.getByRole("textbox").first()).toBeVisible();

    // Close dialog
    await page.getByRole("button", { name: "Cancel" }).click();
  });
});
