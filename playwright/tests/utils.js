import { expect } from "@playwright/test";

export async function goto(page, url) {
  await page.goto(url);
  await page.waitForLoadState("domcontentloaded");
  // Wait for Dioxus WASM to hydrate — SSR markup may already contain
  // [data-dioxus-id], so also verify the interpreter is initialised.
  await page.waitForFunction(
    () => document.querySelector("[data-dioxus-id]") !== null,
  );
}

export async function click(page, opt) {
  const selected = await getLocator(page, opt);
  await selected.click();
  await page.waitForLoadState("load");
  return selected;
}

export async function fill(page, opt, value) {
  const selected = await getLocator(page, opt);
  await selected.fill(value);
  return selected;
}

export async function getLocator(
  page,
  { placeholder, text, role, label, testId, id },
) {
  let selected;

  if (testId) {
    selected = page.getByTestId(testId);
  } else if (id) {
    selected = page.locator(`#${id}`);
  } else if (label) {
    selected = page.getByLabel(label, { exact: true });
  } else if (role) {
    const opt = { exact: true };
    if (text) {
      opt.name = text;
    }
    selected = page.getByRole(role, opt);
  } else if (placeholder) {
    selected = page.getByPlaceholder(placeholder, { exact: true });
  } else if (text) {
    selected = page.getByText(text, { exact: true });
  } else {
    throw new Error("Either text, label, id, or testId must be provided");
  }

  await expect(selected).toBeVisible();

  return selected;
}
