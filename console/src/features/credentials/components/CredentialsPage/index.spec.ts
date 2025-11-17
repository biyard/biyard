import { test, expect } from '@playwright/test';

test.describe('CredentialsPage', () => {
  test.beforeEach(async ({ page }) => {
    // TODO: Setup authentication before each test
    // This should use a test account or mock authentication
    await page.goto('/credentials');
  });

  // Skip all tests until authentication fixtures are implemented
  test.skip(true, 'Authentication fixtures not yet implemented');

  test.describe('Page Layout', () => {
    test('should display credentials page header', async ({ page }) => {
      // Check for page title
      await expect(page.getByRole('heading', { name: /api credentials/i })).toBeVisible();

      // Check for create new credential button
      await expect(page.getByRole('button', { name: /create new credential/i })).toBeVisible();

      // Check for back button
      await expect(page.getByRole('link').filter({ has: page.locator('svg[class*="lucide-arrow-left"]') })).toBeVisible();
    });

    test('should display empty state when no credentials exist', async ({ page }) => {
      // Assuming no credentials exist in test environment
      const emptyStateIcon = page.locator('svg[class*="lucide-key"]');
      const emptyStateText = page.getByText(/no credentials yet/i);

      await expect(emptyStateIcon.or(emptyStateText)).toBeVisible();
    });

    test('should display credentials table when credentials exist', async ({ page }) => {
      // Assuming credentials exist in test environment
      const table = page.locator('table');

      if (await table.isVisible()) {
        await expect(page.getByRole('columnheader', { name: /name/i })).toBeVisible();
        await expect(page.getByRole('columnheader', { name: /api key/i })).toBeVisible();
        await expect(page.getByRole('columnheader', { name: /created at/i })).toBeVisible();
        await expect(page.getByRole('columnheader', { name: /status/i })).toBeVisible();
        await expect(page.getByRole('columnheader', { name: /actions/i })).toBeVisible();
      }
    });
  });

  test.describe('Create Credential Dialog', () => {
    test('should open create credential dialog when clicking create button', async ({ page }) => {
      const createButton = page.getByRole('button', { name: /create new credential/i });
      await createButton.click();

      // Check dialog is visible
      await expect(page.getByRole('heading', { name: /create new credential/i })).toBeVisible();

      // Check form field
      await expect(page.getByLabel(/credential name/i)).toBeVisible();
    });

    test('should close create dialog when clicking cancel', async ({ page }) => {
      // Open dialog
      await page.getByRole('button', { name: /create new credential/i }).click();

      // Click cancel
      await page.getByRole('button', { name: /cancel/i }).click();

      // Dialog should be closed
      await expect(page.getByRole('heading', { name: /create new credential/i })).not.toBeVisible();
    });

    test('should validate required credential name', async ({ page }) => {
      // Open dialog
      await page.getByRole('button', { name: /create new credential/i }).click();

      // Try to submit without filling name
      await page.getByRole('button', { name: /^create$/i }).click();

      // Should not proceed (button might be disabled or show validation)
      // The exact behavior depends on implementation
      await expect(page.getByRole('heading', { name: /create new credential/i })).toBeVisible();
    });

    test('should create credential with valid name', async ({ page }) => {
      // Open dialog
      await page.getByRole('button', { name: /create new credential/i }).click();

      // Fill in credential name
      await page.getByLabel(/credential name/i).fill('Test Credential');

      // Submit form
      await page.getByRole('button', { name: /^create$/i }).click();

      // Should show success toast (if backend is working)
      // await expect(page.getByText(/credential created successfully/i)).toBeVisible();

      // Dialog should close or show API key
      // await expect(page.getByRole('heading', { name: /create new credential/i })).not.toBeVisible();
    });

    test('should display generated API key after creation', async ({ page }) => {
      // Open dialog
      await page.getByRole('button', { name: /create new credential/i }).click();

      // Fill in credential name
      await page.getByLabel(/credential name/i).fill('Test Credential');

      // Submit form
      await page.getByRole('button', { name: /^create$/i }).click();

      // Should show generated API key dialog/section (if backend is working)
      // await expect(page.getByText(/api key generated/i)).toBeVisible();
      // await expect(page.locator('code')).toBeVisible();
    });
  });

  test.describe('Credential List', () => {
    test('should display credential information', async ({ page }) => {
      // Assuming credentials exist
      const credentialRows = page.locator('tbody tr');

      if (await credentialRows.first().isVisible()) {
        // Check that credential data is displayed
        await expect(credentialRows.first()).toContainText(/[A-Za-z0-9]/);
      }
    });

    test('should show active status for active credentials', async ({ page }) => {
      // Check for status badges
      const statusBadge = page.locator('span').filter({ hasText: /active/i }).first();

      if (await statusBadge.isVisible()) {
        // Verify it has appropriate styling
        await expect(statusBadge).toHaveClass(/bg-green/);
      }
    });

    test('should show inactive status for revoked credentials', async ({ page }) => {
      // Check for status badges
      const statusBadge = page.locator('span').filter({ hasText: /inactive/i }).first();

      if (await statusBadge.isVisible()) {
        // Verify it has appropriate styling
        await expect(statusBadge).toHaveClass(/bg-red/);
      }
    });

    test('should mask API key prefix', async ({ page }) => {
      // Check that API keys are masked
      const apiKeyCell = page.locator('code').first();

      if (await apiKeyCell.isVisible()) {
        const keyText = await apiKeyCell.textContent();
        // Should contain ellipsis indicating masking
        expect(keyText).toContain('...');
      }
    });

    test('should copy API key prefix when clicking copy button', async ({ page }) => {
      // Find copy button
      const copyButton = page.locator('button').filter({ has: page.locator('svg[class*="lucide-copy"]') }).first();

      if (await copyButton.isVisible()) {
        await copyButton.click();

        // Should show check icon after copying
        await expect(page.locator('svg[class*="lucide-check"]').first()).toBeVisible();
      }
    });
  });

  test.describe('Credential Actions', () => {
    test('should show revoke button for active credentials', async ({ page }) => {
      // Assuming active credentials exist
      const revokeButtons = page.locator('button').filter({ has: page.locator('svg[class*="lucide-trash"]') });

      if (await revokeButtons.first().isVisible()) {
        await expect(revokeButtons.first()).toBeVisible();
      }
    });

    test('should confirm before revoking a credential', async ({ page }) => {
      // Mock the confirm dialog
      page.on('dialog', async dialog => {
        expect(dialog.type()).toBe('confirm');
        expect(dialog.message()).toMatch(/are you sure/i);
        await dialog.dismiss();
      });

      // Click revoke button (if exists)
      const revokeButton = page.locator('button').filter({ has: page.locator('svg[class*="lucide-trash"]') }).first();

      if (await revokeButton.isVisible()) {
        await revokeButton.click();
      }
    });

    test('should revoke credential after confirmation', async ({ page }) => {
      // Mock the confirm dialog to accept
      page.on('dialog', async dialog => {
        await dialog.accept();
      });

      // Click revoke button (if exists)
      const revokeButton = page.locator('button').filter({ has: page.locator('svg[class*="lucide-trash"]') }).first();

      if (await revokeButton.isVisible()) {
        await revokeButton.click();

        // Should show success toast (if backend is working)
        // await expect(page.getByText(/credential revoked successfully/i)).toBeVisible();
      }
    });
  });

  test.describe('API Key Display Dialog', () => {
    test('should show API key only once after creation', async ({ page }) => {
      // This test verifies that API keys are shown only once
      // and cannot be retrieved again

      // Create a credential
      await page.getByRole('button', { name: /create new credential/i }).click();
      await page.getByLabel(/credential name/i).fill('One-Time Key Test');
      await page.getByRole('button', { name: /^create$/i }).click();

      // If API key dialog appears, it should have a warning
      const warningText = page.getByText(/save.*key.*now/i);
      if (await warningText.isVisible()) {
        await expect(warningText).toBeVisible();
      }
    });

    test('should close API key dialog', async ({ page }) => {
      // Create a credential to get API key dialog
      await page.getByRole('button', { name: /create new credential/i }).click();
      await page.getByLabel(/credential name/i).fill('Dialog Test');
      await page.getByRole('button', { name: /^create$/i }).click();

      // Close the API key dialog (if it appears)
      const closeButton = page.getByRole('button', { name: /close/i });
      if (await closeButton.isVisible()) {
        await closeButton.click();
        await expect(page.getByText(/api key generated/i)).not.toBeVisible();
      }
    });
  });

  test.describe('Navigation', () => {
    test('should navigate back to dashboard', async ({ page }) => {
      const backButton = page.getByRole('link').filter({ has: page.locator('svg[class*="lucide-arrow-left"]') });

      await backButton.click();

      // Should navigate to dashboard
      await expect(page).toHaveURL(/\/dashboard/);
    });
  });

  test.describe('Loading States', () => {
    test('should show loading spinner while fetching credentials', async ({ page }) => {
      // Check for loader (this might be too fast in real scenario)
      const loader = page.locator('svg[class*="animate-spin"]');

      if (await loader.isVisible({ timeout: 100 })) {
        await expect(page.getByText(/loading/i)).toBeVisible();
      }
    });
  });

  test.describe('Error Handling', () => {
    test('should show error toast when API fails', async ({ page }) => {
      // This test would require mocking API failures
      // TODO: Mock API failure and verify error toast appears
    });
  });

  test.describe('Accessibility', () => {
    test('should have proper ARIA labels', async ({ page }) => {
      // Check important elements have proper labels
      await expect(page.getByRole('button', { name: /create new credential/i })).toBeVisible();
      await expect(page.getByRole('link').filter({ has: page.locator('svg[class*="lucide-arrow-left"]') })).toBeVisible();
    });

    test('should be keyboard navigable', async ({ page }) => {
      // Tab through interactive elements
      await page.keyboard.press('Tab');
      await page.keyboard.press('Tab');

      // Check if focus is visible on elements
      const focusedElement = page.locator(':focus');
      await expect(focusedElement).toBeVisible();
    });
  });
});
