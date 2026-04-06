import { test, expect } from "@playwright/test";
import { goto } from "../utils.js";

test.describe("Auth Flow", () => {
  test("unauthenticated user is redirected to signin", async ({ browser }) => {
    const context = await browser.newContext({
      storageState: { cookies: [], origins: [] },
    });
    const page = await context.newPage();

    await page.goto("/dashboard");
    // Server-side auth guard renders the signin page; the URL may or may not
    // change depending on whether the redirect is server-side or client-side.
    await expect(page.getByText("Sign in with email")).toBeVisible({ timeout: 15000 });

    await context.close();
  });

  test("signin page has correct form elements", async ({ browser }) => {
    const context = await browser.newContext({
      storageState: { cookies: [], origins: [] },
    });
    const page = await context.newPage();

    await goto(page, "/signin");

    await expect(page.locator("#email")).toBeVisible();
    await expect(page.locator("#password")).toBeVisible();
    await expect(
      page.getByRole("button", { name: "Sign In" }),
    ).toBeVisible();
    await expect(page.getByText("Sign Up")).toBeVisible();

    await context.close();
  });

  test("signup page has correct form elements", async ({ browser }) => {
    const context = await browser.newContext({
      storageState: { cookies: [], origins: [] },
    });
    const page = await context.newPage();

    await goto(page, "/signup");

    await expect(page.locator("#name")).toBeVisible();
    await expect(page.locator("#email")).toBeVisible();
    await expect(page.locator("#password")).toBeVisible();
    await expect(page.locator("#confirm_password")).toBeVisible();
    await expect(
      page.getByRole("button", { name: "Sign Up" }),
    ).toBeVisible();

    await context.close();
  });
});
