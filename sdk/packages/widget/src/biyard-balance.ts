import type { TokenInfo, UserBalance } from "@biyard/sdk";

import { BiyardWidgetBase, escapeHtml, renderAttribution } from "./base";
import { formatTokenAmount } from "./format";
import { strings } from "./i18n";
import { WIDGET_STYLES } from "./styles";

/**
 * `<biyard-balance>` — read-only card that shows the user's current point
 * balance (via partner proxy) and on-chain token balance (via wallet RPC).
 *
 * ```html
 * <biyard-balance
 *   base-url="/api/biyard"
 *   month="2026-01"     <!-- optional; defaults to current month -->
 * ></biyard-balance>
 * ```
 *
 * Same theming + i18n + branding attributes as `<biyard-claim>`:
 *   `theme="auto|light|dark"`, `lang="en|ko"`, `branding="default|minimal|none"`.
 *
 * Emits `biyard-refresh` after each data load.
 */
export class BiyardBalanceElement extends BiyardWidgetBase {
  static get observedAttributes(): string[] {
    return ["base-url", "chain-id", "month", "theme", "lang", "branding", "title"];
  }

  private token: TokenInfo | null = null;
  private balance: UserBalance | null = null;
  private onChainBalance: string | null = null;
  private walletAddress: string | null = null;
  private loading = false;
  private error: string | null = null;

  protected override onConnected(): void {
    void this.load();
  }

  private async load(): Promise<void> {
    if (this.loading) return;
    this.loading = true;
    this.error = null;
    this.render();
    try {
      const client = this.getFetchOnlyClient();
      const month = this.getAttribute("month") ?? undefined;
      const [tokenResult, balanceResult, walletResult] = await Promise.allSettled([
        client.getTokenInfo(),
        client.getUserBalance(month),
        client.getWalletAddress(),
      ]);

      if (tokenResult.status === "fulfilled") this.token = tokenResult.value;
      if (balanceResult.status === "fulfilled") this.balance = balanceResult.value;
      if (walletResult.status === "fulfilled") this.walletAddress = walletResult.value;

      const failure = [tokenResult, balanceResult].find((r) => r.status === "rejected") as
        | PromiseRejectedResult
        | undefined;
      if (failure) {
        this.error = failure.reason instanceof Error ? failure.reason.message : String(failure.reason);
      }

      // On-chain balance (best-effort; wallet may not be connected).
      if (this.walletAddress && this.token?.contract_address) {
        try {
          const client2 = this.getClient(this.token);
          this.onChainBalance = await client2.getOnChainTokenBalance(
            this.walletAddress,
            this.token.contract_address,
          );
        } catch {
          this.onChainBalance = null;
        }
      }

      this.dispatchEvent(new CustomEvent("biyard-refresh", { bubbles: true, composed: true }));
    } finally {
      this.loading = false;
      this.render();
    }
  }

  protected render(): void {
    const t = strings(this.getLocale());
    const title = this.getAttribute("title") ?? t.balanceTitle;
    const symbol = this.token?.symbol ?? "";
    const decimals = this.token?.decimals ?? 18;
    const points = this.balance?.balance ?? null;
    const month = this.balance?.month ?? this.getAttribute("month") ?? "";
    const tokenBal = this.onChainBalance;

    this.root.innerHTML = `
      <style>${WIDGET_STYLES}</style>
      <div class="inline" part="card">
        <div class="balance-header">
          <h3 class="title">${escapeHtml(title)}</h3>
          ${month ? `<span class="month-pill">${escapeHtml(month)}</span>` : ""}
        </div>
        ${this.renderBody({
          points,
          symbol,
          decimals,
          tokenBal,
          loading: this.loading && points === null && tokenBal === null,
          hint:
            !this.walletAddress && tokenBal === null
              ? t.connectWalletHint
              : null,
          t,
        })}
        ${this.error ? `<div class="alert error">${escapeHtml(this.error)}</div>` : ""}
        ${renderAttribution(this.getBranding(), t)}
      </div>
    `;
  }

  private renderBody(args: {
    points: number | null;
    symbol: string;
    decimals: number;
    tokenBal: string | null;
    loading: boolean;
    hint: string | null;
    t: ReturnType<typeof strings>;
  }): string {
    if (args.loading) {
      return `<div class="balance-grid"><div class="skeleton"></div><div class="skeleton"></div></div>`;
    }
    const pointsCell = `
      <div class="balance-cell">
        <div class="balance-cell-label">${escapeHtml(args.t.pointsLabel)}</div>
        <div class="balance-cell-value">${
          args.points != null ? escapeHtml(args.points.toLocaleString()) : "—"
        }</div>
      </div>
    `;
    const tokenValue =
      args.tokenBal != null
        ? `${escapeHtml(formatTokenAmount(args.tokenBal, args.decimals, 4))}${
            args.symbol ? ` <span class="balance-cell-symbol">${escapeHtml(args.symbol)}</span>` : ""
          }`
        : "—";
    const tokenCell = `
      <div class="balance-cell">
        <div class="balance-cell-label">${escapeHtml(args.t.tokensLabel)}</div>
        <div class="balance-cell-value">${tokenValue}</div>
        ${args.hint ? `<div class="balance-cell-hint">${escapeHtml(args.hint)}</div>` : ""}
      </div>
    `;
    return `<div class="balance-grid">${pointsCell}${tokenCell}</div>`;
  }
}

export function defineBiyardBalance(tag = "biyard-balance"): void {
  if (typeof customElements === "undefined") return;
  if (!customElements.get(tag)) customElements.define(tag, BiyardBalanceElement);
}
