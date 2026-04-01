import { test, expect } from "@playwright/test";
import { goto } from "../utils.js";

test.describe("Projects", () => {
  test("displays the projects page", async ({ page }) => {
    await goto(page, "/projects");
    await expect(page.getByText("Projects").first()).toBeVisible();
  });

  test("shows create new project button", async ({ page }) => {
    await goto(page, "/projects");
    await expect(
      page.getByRole("button", { name: /Create New/ }),
    ).toBeVisible();
  });

  test("opens and closes create project dialog", async ({ page }) => {
    await goto(page, "/projects");

    // Open create dialog
    await page.getByRole("button", { name: /Create New/ }).click();
    await expect(page.getByText("Create Project")).toBeVisible();

    // Verify form fields exist
    await expect(page.getByRole("textbox").first()).toBeVisible();

    // Close dialog
    await page.getByRole("button", { name: "Cancel" }).click();
  });
});
