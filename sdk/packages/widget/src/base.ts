import { BiyardClaim } from "@biyard/sdk";

import { resolveLocale, type Locale } from "./i18n";

/**
 * Shared scaffolding for Biyard data widgets. Handles:
 *   - common attributes (`base-url`, `chain-id`, `theme`, `lang`, `branding`)
 *   - lazy `BiyardClaim` client construction
 *   - theme attribute defaulting to `auto`
 *
 * Concrete widgets implement `renderView()` and (optionally) `onConnected()`
 * to fetch data on mount.
 */
export abstract class BiyardWidgetBase extends HTMLElement {
  protected readonly root: ShadowRoot;

  constructor() {
    super();
    this.root = this.attachShadow({ mode: "open" });
  }

  connectedCallback(): void {
    if (!this.hasAttribute("theme")) this.setAttribute("theme", "auto");
    this.onConnected();
    this.render();
  }

  attributeChangedCallback(): void {
    if (this.root.firstChild) this.render();
  }

  /** Locale resolved from the `lang` attribute / `<html lang>` / navigator. */
  protected getLocale(): Locale {
    return resolveLocale(this.getAttribute("lang"));
  }

  protected getBranding(): "default" | "minimal" | "none" {
    const b = this.getAttribute("branding");
    if (b === "minimal" || b === "none") return b;
    return "default";
  }

  protected resolveChainId(token: { chain_id?: number | null } | null): number {
    const attr = Number(this.getAttribute("chain-id"));
    if (Number.isFinite(attr) && attr > 0) return attr;
    return token?.chain_id ?? 0;
  }

  /**
   * Build a fetch-only client (no chain id needed) for proxy GET calls.
   */
  protected getFetchOnlyClient(): BiyardClaim {
    const baseUrl = this.getAttribute("base-url");
    if (!baseUrl) throw new Error(`<${this.tagName.toLowerCase()}>: missing base-url attribute`);
    return new BiyardClaim({ baseUrl, chainId: 0 });
  }

  /**
   * Build a client with a resolved chain id for on-chain calls. Pass the
   * token info you already fetched.
   */
  protected getClient(token: { chain_id?: number | null } | null): BiyardClaim {
    const baseUrl = this.getAttribute("base-url");
    if (!baseUrl) throw new Error(`<${this.tagName.toLowerCase()}>: missing base-url attribute`);
    const chainId = this.resolveChainId(token);
    if (!chainId)
      throw new Error(
        `<${this.tagName.toLowerCase()}>: chain id is not available (token metadata not loaded and no chain-id attribute given)`,
      );
    return new BiyardClaim({ baseUrl, chainId });
  }

  protected abstract render(): void;
  protected onConnected(): void {}
}

export function escapeHtml(s: string): string {
  return s
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#039;");
}

export function renderAttribution(
  branding: "default" | "minimal" | "none",
  strs: { attributionDefault: string; attributionMinimal: string },
): string {
  if (branding === "none") return "";
  const label = branding === "minimal" ? strs.attributionMinimal : strs.attributionDefault;
  return `<div class="attribution"><a href="https://biyard.co" target="_blank" rel="noopener">${escapeHtml(label)}</a></div>`;
}
