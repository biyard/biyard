import { expect, Locator, Page } from "@playwright/test";

export async function click(
  page: Page,
  { text, label }: { text?: string; label?: string },
): Promise<Locator> {
  let selected: Locator;

  if (label) {
    selected = page.getByLabel(label, { exact: true });
  } else if (text) {
    selected = page.getByRole("button", { name: text, exact: true });
  } else {
    throw new Error("Either text, label must be provided");
  }

  await expect(selected).toBeVisible();
  await selected.click();
  await page.waitForLoadState("networkidle");

  return selected;
}

export async function fill(
  page: Page,
  {
    placeholder,
    label,
  }: {
    placeholder?: string;
    label?: string;
  },
  value: string,
): Promise<Locator> {
  const opt = { exact: true };

  let selected: Locator;

  if (placeholder) {
    selected = page.getByPlaceholder(placeholder, opt);
  } else if (label) {
    selected = page.getByLabel(label, opt);
  } else {
    throw new Error("unsupported selector");
  }
  await expect(selected).toBeVisible();

  await selected.fill(value);

  return selected;
}

export async function waitForVisible(
  page: Page,
  {
    text,
  }: {
    text?: string;
  },
): Promise<Locator> {
  const opt = { exact: true };

  let selected: Locator;

  if (text) {
    selected = page.getByText(text, opt);
  } else {
    throw new Error("unsupported selector");
  }
  await expect(selected).toBeVisible();

  return selected;
}
