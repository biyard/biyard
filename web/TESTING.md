# Testing Guide

This document describes the testing strategy and conventions for the Biyard web application.

## Testing Philosophy

We follow a **component-colocated testing** approach where test files are placed alongside the components they test. This makes tests easier to find, maintain, and ensures they stay in sync with the code.

## Directory Structure

Tests should be placed in the same directory as the component they test:

```
src/
├── features/
│   ├── projects/
│   │   └── components/
│   │       └── ProjectsPage/
│   │           ├── index.tsx          # Component implementation
│   │           ├── index.spec.ts      # Playwright E2E tests
│   │           ├── use-controller.tsx # Controller logic
│   │           └── i18n.tsx          # Internationalization
│   └── credentials/
│       └── components/
│           └── CredentialsPage/
│               ├── index.tsx
│               ├── index.spec.ts
│               └── ...
└── App.tsx
    └── App.spec.ts                    # App-level smoke tests
```

## Test Types

### End-to-End Tests (Playwright)

We use [Playwright](https://playwright.dev/) for end-to-end testing. Test files follow the naming convention `*.spec.ts`.

**Example:**
```typescript
// src/features/projects/components/ProjectsPage/index.spec.ts
import { test, expect } from '@playwright/test';

test.describe('ProjectsPage', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/projects');
  });

  test('should display page header', async ({ page }) => {
    await expect(page.getByRole('heading', { name: /projects/i })).toBeVisible();
  });
});
```

### Test Organization

Each test file should be organized into logical sections:

1. **Page Layout** - Basic page structure and elements
2. **User Interactions** - Forms, dialogs, buttons
3. **Data Display** - Tables, lists, cards
4. **Actions** - Create, update, delete operations
5. **Navigation** - Links, routing, back buttons
6. **Loading States** - Spinners, skeletons
7. **Error Handling** - API failures, validation errors
8. **Accessibility** - ARIA labels, keyboard navigation

## Running Tests

### Run all tests
```bash
pnpm test
```

### Run tests in headed mode (see browser)
```bash
pnpm exec playwright test --headed
```

### Run specific test file
```bash
pnpm exec playwright test src/features/projects/components/ProjectsPage/index.spec.ts
```

### Run tests in debug mode
```bash
pnpm exec playwright test --debug
```

### List all discovered tests
```bash
pnpm exec playwright test --list
```

### View test report
```bash
pnpm exec playwright show-report
```

## Writing Tests

### Best Practices

1. **Use semantic selectors**
   - Prefer `getByRole()`, `getByLabel()`, `getByText()` over CSS selectors
   - This makes tests more resilient and improves accessibility

```typescript
// Good
await page.getByRole('button', { name: /create new project/i });
await page.getByLabel(/project name/i);

// Avoid
await page.locator('.btn-create');
await page.locator('#project-name');
```

2. **Test user behavior, not implementation**
   - Focus on what users see and do
   - Don't test internal state or implementation details

3. **Use beforeEach for setup**
   - Navigate to the page before each test
   - Set up authentication if needed

```typescript
test.beforeEach(async ({ page }) => {
  // TODO: Setup authentication
  await page.goto('/projects');
});
```

4. **Handle conditional visibility**
   - Some elements may not always be present
   - Use conditional checks when needed

```typescript
if (await element.isVisible()) {
  await expect(element).toBeVisible();
}
```

5. **Group related tests**
   - Use `test.describe()` to group related test cases
   - This improves readability and organization

6. **Test error states**
   - Verify validation errors are shown
   - Test API failure scenarios (when possible)

7. **Test accessibility**
   - Verify ARIA labels
   - Test keyboard navigation
   - Check focus management

## Test Configuration

The Playwright configuration is in `playwright.config.ts`:

- **Test Directory**: `./src` (searches for `**/*.spec.ts`)
- **Base URL**: `http://localhost:5000`
- **Browser**: Chromium (Chrome)
- **Dev Server**: Automatically starts with `pnpm dev`
- **Retries**: 2 retries in CI, 0 in local development
- **Parallelization**: Fully parallel (workers based on CPU cores)

## Authentication in Tests

Currently, authentication setup is marked as TODO in test files:

```typescript
test.beforeEach(async ({ page }) => {
  // TODO: Setup authentication before each test
  // This should use a test account or mock authentication
  await page.goto('/projects');
});
```

Future improvements:
- Create test fixtures for authenticated sessions
- Use Playwright's `storageState` to reuse authentication
- Mock authentication for faster test execution

## Continuous Integration

Tests are configured to run in CI with:
- 2 retries for flaky tests
- Single worker to avoid resource contention
- HTML reporter for test results

## Domain Models in Tests

When testing, be aware that components use domain model classes:

- `Credential` class with methods like `isActive()`, `getStatusColorClass()`
- `Project` class with methods like `hasManualTokenSupply()`, `getFormattedTokenSupply()`

Tests should verify the **user-facing behavior** that these methods enable, not the methods themselves.

## Coverage

We aim for:
- **Critical paths**: 100% coverage
- **User interactions**: All major flows tested
- **Error handling**: Key error scenarios covered
- **Accessibility**: Basic ARIA and keyboard navigation

## Debugging Tests

### Visual debugging
```bash
pnpm exec playwright test --debug
```

This opens Playwright Inspector where you can:
- Step through tests
- Inspect locators
- View screenshots
- See console logs

### Trace viewer
```bash
pnpm exec playwright show-trace
```

View detailed execution traces including:
- Screenshots at each step
- Network requests
- Console logs
- Timeline of actions

## Future Improvements

- [ ] Add unit tests for domain models
- [ ] Add unit tests for hooks
- [ ] Add visual regression testing
- [ ] Set up authentication fixtures
- [ ] Add API mocking for isolated tests
- [ ] Increase test coverage metrics
- [ ] Add component tests with Vitest + Testing Library

## Resources

- [Playwright Documentation](https://playwright.dev/)
- [Playwright Best Practices](https://playwright.dev/docs/best-practices)
- [Testing Library Queries](https://testing-library.com/docs/queries/about)
