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

/**
 * Inline Biyard logomark — the official two-diamond "N" mark.
 * Path traced from landing/assets/biyard-logo.png via potrace, so this is
 * pixel-faithful to the brand asset (scales cleanly at any size).
 *
 * Uses `currentColor` so it inherits the surrounding text / accent color;
 * partners using the brand color get the official mint, others get whatever
 * flows from the host theme.
 */
export const BIYARD_LOGO_SVG = `
<svg viewBox="0 0 192 192" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
  <g transform="translate(0,192) scale(0.1,-0.1)" fill="currentColor">
    <path d="M386 1833 c-280 -637 -349 -794 -363 -825 l-16 -38 99 -222 c54 -123 148 -335 210 -473 l111 -250 120 -3 120 -3 78 178 c43 98 84 192 90 208 12 28 9 40 -39 149 -29 66 -57 121 -62 123 -5 2 -45 -79 -88 -179 -44 -101 -85 -189 -91 -196 -9 -10 -42 56 -155 310 -79 178 -147 332 -152 343 -7 16 10 64 77 215 169 383 209 470 215 470 4 0 33 -60 65 -132 71 -162 332 -751 521 -1176 l139 -313 119 3 119 3 39 90 c238 545 368 847 368 859 0 7 -55 133 -121 282 -67 148 -161 357 -209 464 l-87 195 -117 0 -117 0 -80 -180 c-43 -99 -84 -188 -89 -197 -6 -10 -10 -26 -10 -35 0 -20 101 -253 110 -253 3 0 44 87 91 193 47 105 90 191 95 190 7 -2 242 -506 293 -630 12 -30 7 -44 -124 -350 -76 -175 -143 -329 -150 -342 l-13 -25 -16 32 c-14 27 -398 892 -636 1435 l-74 167 -116 0 -115 0 -39 -87z"/>
  </g>
</svg>
`.trim();

/**
 * Card-level brand header: a small ⓑ logomark + "Biyard" wordmark, styled
 * with the brand gradient. Sits at the top of every widget card so users
 * immediately recognise this is a Biyard component, regardless of how the
 * partner has themed the surrounding page.
 */
export function renderBrandHeader(branding: "default" | "minimal" | "none"): string {
  if (branding === "none") return "";
  return `
    <div class="brand-header" part="brand-header">
      <span class="brand-mark" aria-hidden="true">${BIYARD_LOGO_SVG}</span>
      <span class="brand-name">Biyard</span>
    </div>
  `;
}

export function renderAttribution(
  branding: "default" | "minimal" | "none",
  strs: { attributionDefault: string; attributionMinimal: string },
): string {
  if (branding === "none") return "";
  const label = branding === "minimal" ? strs.attributionMinimal : strs.attributionDefault;
  return `
    <div class="attribution" part="attribution">
      <a href="https://biyard.co" target="_blank" rel="noopener">
        <span class="attribution-mark" aria-hidden="true">${BIYARD_LOGO_SVG}</span>
        <span>${escapeHtml(label)}</span>
      </a>
    </div>
  `;
}
