import type { TokenInfo, UserBalance } from "@biyard/sdk";

import { BiyardWidgetBase, escapeHtml, renderAttribution, renderBrandHeader } from "./base";
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
  private connecting = false;
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
        ${renderBrandHeader(this.getBranding())}
        <div class="balance-header">
          <h3 class="title">${escapeHtml(title)}</h3>
          ${month ? `<span class="month-pill">${escapeHtml(month)}</span>` : ""}
        </div>
        ${this.renderWallet()}
        ${this.renderBody({
          points,
          symbol,
          decimals,
          tokenBal,
          loading: this.loading && points === null && tokenBal === null,
          showConnect: !this.walletAddress && tokenBal === null,
          connecting: this.connecting,
          t,
        })}
        ${this.error ? `<div class="alert error">${escapeHtml(this.error)}</div>` : ""}
        ${renderAttribution(this.getBranding(), t)}
      </div>
    `;

    this.root
      .querySelectorAll<HTMLButtonElement>('[data-action="connect-wallet"]')
      .forEach((btn) => {
        btn.addEventListener("click", () => void this.handleConnect());
      });
  }

  private async handleConnect(): Promise<void> {
    if (this.connecting || this.walletAddress) return;
    this.connecting = true;
    this.error = null;
    this.render();
    try {
      const client = this.getClient(this.token);
      this.walletAddress = await client.connectWallet();
      // Re-load to fetch on-chain balance now that the wallet is connected.
      await this.load();
    } catch (e) {
      this.error = e instanceof Error ? e.message : String(e);
    } finally {
      this.connecting = false;
      this.render();
    }
  }

  private renderWallet(): string {
    const connected = !!this.walletAddress;
    const truncated = connected
      ? `${this.walletAddress!.slice(0, 6)}…${this.walletAddress!.slice(-4)}`
      : "";
    const chainName = this.token?.chain_id
      ? chainLabel(this.token.chain_id)
      : "";
    return `
      <div class="wallet-row" data-connected="${connected}">
        <span class="wallet-dot" aria-hidden="true"></span>
        <span class="wallet-addr">${escapeHtml(truncated)}</span>
        ${chainName ? `<span class="wallet-chain">${escapeHtml(chainName)}</span>` : ""}
      </div>
    `;
  }

  private renderBody(args: {
    points: number | null;
    symbol: string;
    decimals: number;
    tokenBal: string | null;
    loading: boolean;
    showConnect: boolean;
    connecting: boolean;
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
    const connectAction = args.showConnect
      ? `
        <button
          class="connect-wallet"
          type="button"
          data-action="connect-wallet"
          ${args.connecting ? "disabled" : ""}
        >
          ${
            args.connecting
              ? `<span class="spinner" aria-hidden="true"></span><span>${escapeHtml(args.t.connectingWallet)}</span>`
              : escapeHtml(args.t.connectWalletCta)
          }
        </button>
        <div class="balance-cell-hint">${escapeHtml(args.t.connectWalletHint)}</div>
      `
      : "";
    const tokenCell = `
      <div class="balance-cell">
        <div class="balance-cell-label">${escapeHtml(args.t.tokensLabel)}</div>
        <div class="balance-cell-value">${tokenValue}</div>
        ${connectAction}
      </div>
    `;
    return `<div class="balance-grid">${pointsCell}${tokenCell}</div>`;
  }
}

function chainLabel(chainId: number): string {
  switch (chainId) {
    case 1:
      return "Ethereum";
    case 11155111:
      return "Sepolia";
    case 137:
      return "Polygon";
    case 80002:
      return "Polygon Amoy";
    case 8453:
      return "Base";
    case 84532:
      return "Base Sepolia";
    case 42161:
      return "Arbitrum";
    case 421614:
      return "Arbitrum Sepolia";
    case 10:
      return "Optimism";
    default:
      return `Chain ${chainId}`;
  }
}

export function defineBiyardBalance(tag = "biyard-balance"): void {
  if (typeof customElements === "undefined") return;
  if (!customElements.get(tag)) customElements.define(tag, BiyardBalanceElement);
}
