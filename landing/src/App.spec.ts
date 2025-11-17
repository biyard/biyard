import { test, expect } from '@playwright/test';

test.describe('App Smoke Tests', () => {
  test('should load the app and redirect to signin when not authenticated', async ({ page }) => {
    await page.goto('/');

    // Should redirect to /signin when not authenticated
    await expect(page).toHaveURL(/\/signin/);
  });

  test('signin page should have sign in form', async ({ page }) => {
    await page.goto('/signin');

    // Check for email input
    await expect(page.getByLabel(/email/i)).toBeVisible();

    // Check for password input
    await expect(page.getByLabel(/password/i)).toBeVisible();

    // Check for sign in button
    await expect(page.getByRole('button', { name: /sign in/i })).toBeVisible();
  });

  test('should navigate to signup page', async ({ page }) => {
    await page.goto('/signin');

    // Click on sign up link
    const signupLink = page.getByRole('link', { name: /sign up/i });
    if (await signupLink.isVisible()) {
      await signupLink.click();
      await expect(page).toHaveURL(/\/signup/);
    }
  });
});
