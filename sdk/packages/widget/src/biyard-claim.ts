import { BiyardClaim, type TokenInfo } from "@biyard/sdk";

import { formatTokenAmount, shortHex } from "./format";
import { resolveLocale, strings, type Locale } from "./i18n";
import { WIDGET_STYLES } from "./styles";

/**
 * Convert a `YYYY-MM` month string to the on-chain month index used by
 * BrandToken (`(year - 1970) * 12 + (monthIndex0)` offset from `start_month`).
 * Mirrors `console/src/common/utils/time_utils.rs::month_index`.
 */
function monthToIndex(month: string, startMonth: string | null): bigint {
  const parse = (m: string): bigint | null => {
    const [y, mo] = m.split("-");
    if (!y || !mo) return null;
    const yi = BigInt(y);
    const mi = BigInt(mo);
    return (yi - 1970n) * 12n + (mi - 1n);
  };
  const target = parse(month);
  if (target === null) return 0n;
  const origin = startMonth ? parse(startMonth) : null;
  if (origin === null) return 0n;
  const diff = target - origin;
  return diff < 0n ? 0n : diff;
}

type Mode = "modal" | "inline";
type Branding = "default" | "minimal" | "none";

interface Status {
  kind: "none" | "error" | "success";
  message?: string;
  txHash?: string;
}

/**
 * Drop-in Biyard claim widget. Renders into the partner's site so the
 * partner controls all branding; Biyard's mark appears only as a small
 * footer link on the claim-review pane (and can be hidden entirely).
 *
 * ```html
 * <biyard-claim
 *   base-url="/api/biyard"
 *   chain-id="1001"
 *   month="2026-01"
 *   mode="modal"               <!-- "modal" | "inline" -->
 *   theme="auto"               <!-- "auto" | "light" | "dark" -->
 *   branding="default"         <!-- "default" | "minimal" | "none" -->
 *   label="Claim"              <!-- trigger label -->
 *   title="Claim your rewards" <!-- modal title -->
 *   subtitle="January 2026"    <!-- optional subtitle -->
 *   amount="..."               <!-- optional raw uint256 override -->
 *   symbol="CAFE"              <!-- optional symbol override -->
 *   decimals="18"              <!-- optional decimals override -->
 * ></biyard-claim>
 * ```
 *
 * Theming: set CSS variables on the host element (any combination):
 *   `--biyard-color-accent`, `--biyard-color-accent-foreground`,
 *   `--biyard-color-bg`, `--biyard-color-surface`,
 *   `--biyard-color-text`, `--biyard-color-muted`, `--biyard-color-border`,
 *   `--biyard-color-danger`, `--biyard-color-success`,
 *   `--biyard-font-family`, `--biyard-radius`, `--biyard-spacing`.
 *
 * Events:
 *   `biyard-claim-success` — `detail: { tx_hash }`
 *   `biyard-claim-error`   — `detail: { error }`
 *   `biyard-open`, `biyard-close`
 */
export class BiyardClaimElement extends HTMLElement {
  static get observedAttributes(): string[] {
    return [
      "base-url",
      "chain-id",
      "month",
      "mode",
      "theme",
      "branding",
      "lang",
      "label",
      "title",
      "subtitle",
      "amount",
      "symbol",
      "decimals",
    ];
  }

  private root: ShadowRoot;
  private open = false;
  private claiming = false;
  private status: Status = { kind: "none" };
  private token: TokenInfo | null = null;
  private claimable: { amount: string | null } = { amount: null };
  private metaLoading = false;
  private metaError: string | null = null;

  constructor() {
    super();
    this.root = this.attachShadow({ mode: "open" });
  }

  connectedCallback(): void {
    if (!this.hasAttribute("theme")) this.setAttribute("theme", "auto");
    this.render();
    // Inline mode auto-loads metadata on mount.
    if (this.getMode() === "inline") void this.loadMeta();
  }

  /**
   * Look for a prior on-chain Claimed event for this wallet + month and seed
   * the success state. Called after token metadata is available, and again
   * after the user connects their wallet (since `eth_accounts` returns
   * empty until then).
   *
   * On-chain is authoritative: a user who claimed on a different browser /
   * device still sees "already claimed" here.
   */
  private async checkOnChainHistory(): Promise<void> {
    const baseUrl = this.getAttribute("base-url");
    const month = this.getAttribute("month");
    const contract = this.token?.contract_address;
    const startMonth = this.token?.start_month ?? null;
    if (!baseUrl || !month || !contract) return;
    if (this.status.kind === "success") return; // already shown

    let client: BiyardClaim;
    try {
      client = this.getClient();
    } catch {
      return;
    }
    let wallet: string | null = null;
    try {
      wallet = await client.getWalletAddress();
    } catch {
      // Wallet probe failed (no extension, etc.) — non-fatal.
      return;
    }
    if (!wallet) {
      // No wallet connected yet. The widget calls checkOnChainHistory again
      // after a successful connect, so we don't need to poll.
      return;
    }
    try {
      const events = await client.getClaimHistory(wallet, contract);
      const targetIndex = monthToIndex(month, startMonth).toString();
      // eslint-disable-next-line no-console
      console.debug("[biyard-claim] history check", {
        wallet,
        contract,
        month,
        targetIndex,
        events,
      });
      const match = events.find((e) => e.month_index === targetIndex);
      if (match) {
        this.status = { kind: "success", txHash: match.tx_hash };
        this.render();
      }
    } catch (e) {
      // RPC errors shouldn't block claim flow, but they should be visible
      // so partners can diagnose misconfiguration (wrong chain, etc.).
      // eslint-disable-next-line no-console
      console.warn("[biyard-claim] on-chain history check failed:", e);
    }
  }

  disconnectedCallback(): void {
    document.removeEventListener("keydown", this.handleKeyDown);
  }

  attributeChangedCallback(): void {
    if (this.root.firstChild) this.render();
  }

  private getMode(): Mode {
    return this.getAttribute("mode") === "inline" ? "inline" : "modal";
  }

  private getBranding(): Branding {
    const b = this.getAttribute("branding");
    if (b === "minimal" || b === "none") return b;
    return "default";
  }

  private getLocale(): Locale {
    return resolveLocale(this.getAttribute("lang"));
  }

  private getClient(): BiyardClaim {
    const baseUrl = this.getAttribute("base-url");
    if (!baseUrl) throw new Error("<biyard-claim>: missing base-url attribute");
    const chainId = this.resolveChainId();
    if (!chainId)
      throw new Error(
        "<biyard-claim>: chain id is not available yet (token metadata not loaded and no `chain-id` attribute given)",
      );
    return new BiyardClaim({ baseUrl, chainId });
  }

  /**
   * Get a client for fetch-only operations (token info, claimable list) that
   * don't need a chain id. Falls back to chain id `0` since these calls go
   * through the partner proxy and never touch RPC.
   */
  private getFetchOnlyClient(): BiyardClaim {
    const baseUrl = this.getAttribute("base-url");
    if (!baseUrl) throw new Error("<biyard-claim>: missing base-url attribute");
    // BiyardClaim constructor requires `chainId`; pass 0 since these calls
    // don't invoke wallet/RPC code paths.
    return new BiyardClaim({ baseUrl, chainId: 0 });
  }

  /**
   * Resolve the EVM chain id. Priority:
   *   1. explicit `chain-id` attribute (lets the integrator override)
   *   2. value carried by the token metadata response from the partner proxy
   */
  private resolveChainId(): number {
    const attr = Number(this.getAttribute("chain-id"));
    if (Number.isFinite(attr) && attr > 0) return attr;
    return this.token?.chain_id ?? 0;
  }

  private async loadMeta(): Promise<void> {
    if (this.metaLoading) return;
    this.metaLoading = true;
    this.metaError = null;
    this.render();

    let client: BiyardClaim;
    try {
      // metadata fetches don't need chain id — they go through the proxy.
      client = this.getFetchOnlyClient();
    } catch (e) {
      this.metaError = e instanceof Error ? e.message : String(e);
      this.metaLoading = false;
      this.render();
      return;
    }

    const month = this.getAttribute("month");
    // We always fetch the token to discover chain_id, unless every relevant
    // field is provided as an attribute.
    const needsToken =
      !this.token &&
      (!this.getAttribute("chain-id") ||
        !this.getAttribute("symbol") ||
        !this.getAttribute("decimals"));
    const needsAmount = !this.getAttribute("amount") && month;

    try {
      const [tokenResult, claimableResult] = await Promise.allSettled([
        needsToken ? client.getTokenInfo() : Promise.resolve(this.token),
        needsAmount ? client.getClaimable() : Promise.resolve(null),
      ]);

      if (tokenResult.status === "fulfilled" && tokenResult.value) {
        this.token = tokenResult.value;
      } else if (tokenResult.status === "rejected") {
        this.metaError =
          tokenResult.reason instanceof Error
            ? tokenResult.reason.message
            : String(tokenResult.reason);
      }

      if (
        claimableResult.status === "fulfilled" &&
        claimableResult.value &&
        month
      ) {
        const row = claimableResult.value.months.find((m) => m.month === month);
        if (row) this.claimable = { amount: row.remaining };
      } else if (claimableResult.status === "rejected" && !this.metaError) {
        this.metaError =
          claimableResult.reason instanceof Error
            ? claimableResult.reason.message
            : String(claimableResult.reason);
      }
    } finally {
      this.metaLoading = false;
      this.render();
    }
    // After metadata loads, check on-chain history (non-blocking).
    void this.checkOnChainHistory();
  }

  private openModal = (): void => {
    if (this.open) return;
    this.open = true;
    this.status = { kind: "none" };
    document.addEventListener("keydown", this.handleKeyDown);
    this.dispatchEvent(
      new CustomEvent("biyard-open", { bubbles: true, composed: true }),
    );
    this.render();
    void this.loadMeta();
  };

  private closeModal = (): void => {
    if (!this.open) return;
    this.open = false;
    document.removeEventListener("keydown", this.handleKeyDown);
    this.dispatchEvent(
      new CustomEvent("biyard-close", { bubbles: true, composed: true }),
    );
    this.render();
  };

  private handleKeyDown = (e: KeyboardEvent): void => {
    if (e.key === "Escape") this.closeModal();
  };

  private handleClaim = async (): Promise<void> => {
    const month = this.getAttribute("month");
    if (!month || this.claiming) return;
    this.claiming = true;
    this.status = { kind: "none" };
    this.render();
    try {
      const client = this.getClient();
      const result = await client.claim(month);
      this.status = { kind: "success", txHash: result.tx_hash };
      this.dispatchEvent(
        new CustomEvent("biyard-claim-success", {
          bubbles: true,
          composed: true,
          detail: { tx_hash: result.tx_hash },
        }),
      );
    } catch (e) {
      const err = e instanceof Error ? e : new Error(String(e));
      this.status = { kind: "error", message: err.message };
      this.dispatchEvent(
        new CustomEvent("biyard-claim-error", {
          bubbles: true,
          composed: true,
          detail: { error: err },
        }),
      );
    } finally {
      this.claiming = false;
      this.render();
    }
  };

  private render(): void {
    if (this.getMode() === "inline") {
      this.root.innerHTML = `
        <style>${WIDGET_STYLES}</style>
        <div class="inline" part="card">
          ${this.renderBody()}
        </div>
      `;
      this.bindActions();
      return;
    }
    // modal mode: trigger button + optional overlay
    const t = strings(this.getLocale());
    this.root.innerHTML = `
      <style>${WIDGET_STYLES}</style>
      <button class="trigger" type="button" data-action="open">
        ${escapeHtml(this.getAttribute("label") ?? t.triggerLabel)}
      </button>
      ${
        this.open
          ? `
        <div class="overlay" data-action="overlay" role="dialog" aria-modal="true">
          <div class="card" part="card" data-stop>
            <button class="close" type="button" data-action="close" aria-label="${escapeHtml(t.close)}">×</button>
            ${this.renderBody()}
          </div>
        </div>
      `
          : ""
      }
    `;
    this.bindActions();
  }

  private renderBody(): string {
    const t = strings(this.getLocale());
    const title = this.getAttribute("title") ?? t.defaultTitle;
    const subtitle = this.getAttribute("subtitle");
    const month = this.getAttribute("month") ?? "";

    const symbol = this.getAttribute("symbol") ?? this.token?.symbol ?? "";
    const decimals = Number(
      this.getAttribute("decimals") ?? this.token?.decimals ?? 18,
    );
    const rawAmount =
      this.getAttribute("amount") ?? this.claimable.amount ?? null;
    const formattedAmount = rawAmount
      ? formatTokenAmount(rawAmount, decimals, 4)
      : null;

    const amountBlock =
      this.metaLoading && !formattedAmount
        ? `<div class="amount"><div class="skeleton"></div></div>`
        : formattedAmount
          ? `
            <div class="amount">
              <div class="amount-value">${escapeHtml(formattedAmount)}</div>
              ${symbol ? `<div class="amount-symbol">${escapeHtml(symbol)}</div>` : ""}
            </div>
          `
          : "";

    const review = this.renderReview(month);

    // On success, replace the CTA entirely with a success panel.
    const ctaOrSuccess =
      this.status.kind === "success"
        ? this.renderSuccessPanel()
        : `
            <button class="cta" type="button" data-action="claim" ${this.canClaim() ? "" : "disabled"}>
              ${
                this.claiming
                  ? `<span class="spinner" aria-hidden="true"></span><span>${escapeHtml(t.claiming)}</span>`
                  : `<span>${escapeHtml(t.defaultCta)}</span>`
              }
            </button>
          `;

    const errorAlert = this.metaError
      ? `<div class="alert error">${escapeHtml(this.metaError)}</div>`
      : this.status.kind === "error"
        ? `<div class="alert error">${escapeHtml(this.status.message ?? "")}</div>`
        : "";

    return `
      <div>
        <h3 class="title">${escapeHtml(title)}</h3>
        ${subtitle ? `<div class="subtitle">${escapeHtml(subtitle)}</div>` : ""}
      </div>
      ${amountBlock}
      ${review}
      ${ctaOrSuccess}
      ${errorAlert}
      ${this.renderAttribution()}
    `;
  }

  private renderReview(month: string): string {
    const t = strings(this.getLocale());
    const chainId = this.resolveChainId();
    const contract = this.token?.contract_address ?? null;
    const rows: string[] = [];
    if (month) {
      rows.push(
        `<div class="review-row"><span class="k">${escapeHtml(t.reviewMonth)}</span><span class="v">${escapeHtml(month)}</span></div>`,
      );
    }
    if (chainId) {
      rows.push(
        `<div class="review-row"><span class="k">${escapeHtml(t.reviewNetwork)}</span><span class="v">${escapeHtml(chainLabel(chainId))}</span></div>`,
      );
    }
    if (contract) {
      rows.push(
        `<div class="review-row"><span class="k">${escapeHtml(t.reviewContract)}</span><span class="v">${escapeHtml(shortHex(contract, 6, 4))}</span></div>`,
      );
    }
    if (rows.length === 0) return "";
    return `<div class="review">${rows.join("")}</div>`;
  }

  private renderSuccessPanel(): string {
    const t = strings(this.getLocale());
    const tx = this.status.txHash;
    return `
      <div class="success-panel" role="status" aria-live="polite">
        <div class="success-icon" aria-hidden="true">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="M5 12 L10 17 L19 7" />
          </svg>
        </div>
        <div class="success-text">${escapeHtml(t.claimedTitle)}</div>
        ${tx ? `<div class="success-tx">tx ${escapeHtml(shortHex(tx, 10, 8))}</div>` : ""}
      </div>
    `;
  }

  private renderAttribution(): string {
    const branding = this.getBranding();
    if (branding === "none") return "";
    const t = strings(this.getLocale());
    const label =
      branding === "minimal" ? t.attributionMinimal : t.attributionDefault;
    return `<div class="attribution"><a href="https://biyard.co" target="_blank" rel="noopener">${escapeHtml(label)}</a></div>`;
  }

  private canClaim(): boolean {
    if (this.claiming) return false;
    if (this.metaLoading) return false;
    if (this.status.kind === "success") return false;
    if (!this.getAttribute("month")) return false;
    if (!this.getAttribute("base-url")) return false;
    if (!this.resolveChainId()) return false;
    return true;
  }

  private bindActions(): void {
    this.root
      .querySelectorAll<HTMLElement>('[data-action="open"]')
      .forEach((el) => el.addEventListener("click", this.openModal));
    this.root
      .querySelectorAll<HTMLElement>('[data-action="close"]')
      .forEach((el) => el.addEventListener("click", this.closeModal));
    this.root
      .querySelectorAll<HTMLElement>('[data-action="overlay"]')
      .forEach((el) =>
        el.addEventListener("click", (e: Event) => {
          const target = e.target as HTMLElement;
          if (target.closest("[data-stop]")) return;
          this.closeModal();
        }),
      );
    this.root
      .querySelectorAll<HTMLElement>('[data-action="claim"]')
      .forEach((el) => el.addEventListener("click", this.handleClaim));
  }
}

function chainLabel(id: number): string {
  switch (id) {
    case 1:
      return "Ethereum";
    case 1001:
      return "Kaia Kairos";
    case 8217:
      return "Kaia";
    default:
      return `Chain ${id}`;
  }
}

function escapeHtml(s: string): string {
  return s
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#039;");
}

export function defineBiyardClaim(tag = "biyard-claim"): void {
  if (typeof customElements === "undefined") return;
  if (!customElements.get(tag)) {
    customElements.define(tag, BiyardClaimElement);
  }
}
