import type { PointTransaction, TransactionType } from "@biyard/sdk";

import { BiyardWidgetBase, escapeHtml, renderAttribution, renderBrandHeader } from "./base";
import { strings } from "./i18n";
import { WIDGET_STYLES } from "./styles";

/**
 * `<biyard-transactions>` — paginated list of point transactions for the
 * current user. Shows AWARD/DEDUCT/TRANSFER/EXCHANGE with sign + description.
 *
 * ```html
 * <biyard-transactions
 *   base-url="/api/biyard"
 *   limit="10"
 *   month="2026-01"   <!-- optional filter -->
 * ></biyard-transactions>
 * ```
 *
 * Same theming/i18n/branding attributes as the other widgets.
 */
export class BiyardTransactionsElement extends BiyardWidgetBase {
  static get observedAttributes(): string[] {
    return ["base-url", "limit", "month", "theme", "lang", "branding", "title"];
  }

  private items: PointTransaction[] = [];
  private bookmark: string | null = null;
  private loading = false;
  private loadingMore = false;
  private error: string | null = null;

  protected override onConnected(): void {
    void this.loadInitial();
  }

  private getLimit(): number {
    const raw = Number(this.getAttribute("limit"));
    if (!Number.isFinite(raw) || raw <= 0) return 10;
    return Math.min(raw, 50);
  }

  private async loadInitial(): Promise<void> {
    this.items = [];
    this.bookmark = null;
    this.loading = true;
    this.error = null;
    this.render();
    try {
      const client = this.getFetchOnlyClient();
      const month = this.getAttribute("month") ?? undefined;
      const res = await client.getTransactions({ limit: this.getLimit(), month });
      this.items = res.items;
      this.bookmark = res.bookmark;
    } catch (e) {
      this.error = e instanceof Error ? e.message : String(e);
    } finally {
      this.loading = false;
      this.render();
    }
  }

  private loadMore = async (): Promise<void> => {
    if (this.loadingMore || !this.bookmark) return;
    this.loadingMore = true;
    this.render();
    try {
      const client = this.getFetchOnlyClient();
      const month = this.getAttribute("month") ?? undefined;
      const res = await client.getTransactions({
        limit: this.getLimit(),
        bookmark: this.bookmark,
        month,
      });
      this.items = this.items.concat(res.items);
      this.bookmark = res.bookmark;
    } catch (e) {
      this.error = e instanceof Error ? e.message : String(e);
    } finally {
      this.loadingMore = false;
      this.render();
    }
  };

  protected render(): void {
    const t = strings(this.getLocale());
    const title = this.getAttribute("title") ?? t.transactionsTitle;
    const body = this.loading
      ? `<div class="empty-state">${escapeHtml(t.loading)}</div>`
      : this.items.length === 0
        ? `<div class="empty-state">${escapeHtml(t.empty)}</div>`
        : this.renderList(t);

    this.root.innerHTML = `
      <style>${WIDGET_STYLES}</style>
      <div class="inline" part="card">
        ${renderBrandHeader(this.getBranding())}
        <h3 class="title">${escapeHtml(title)}</h3>
        ${body}
        ${
          this.bookmark
            ? `<button class="load-more" type="button" data-action="load-more" ${this.loadingMore ? "disabled" : ""}>${escapeHtml(this.loadingMore ? t.loading : t.loadMore)}</button>`
            : ""
        }
        ${this.error ? `<div class="alert error">${escapeHtml(this.error)}</div>` : ""}
        ${renderAttribution(this.getBranding(), t)}
      </div>
    `;

    this.root
      .querySelectorAll<HTMLElement>('[data-action="load-more"]')
      .forEach((el) => el.addEventListener("click", this.loadMore));
  }

  private renderList(t: ReturnType<typeof strings>): string {
    const rows = this.items.map((tx) => {
      const label = txTypeLabel(tx.transaction_type, t);
      const sign = txSign(tx.transaction_type, tx.amount);
      const cls = sign === ">" ? "pos" : sign === "<" ? "neg" : "neutral";
      const amount = `${sign === ">" ? "+" : sign === "<" ? "-" : ""}${Math.abs(tx.amount).toLocaleString()}`;
      const meta = [
        tx.month,
        tx.description ?? "",
      ]
        .filter(Boolean)
        .join(" · ");
      return `
        <div class="tx-row">
          <div class="tx-left">
            <div class="tx-type">${escapeHtml(label)}</div>
            <div class="tx-meta" title="${escapeHtml(meta)}">${escapeHtml(meta)}</div>
          </div>
          <div class="tx-amount ${cls}">${escapeHtml(amount)}</div>
        </div>
      `;
    });
    return `<div class="tx-list">${rows.join("")}</div>`;
  }
}

function txTypeLabel(type: TransactionType, t: ReturnType<typeof strings>): string {
  switch (type) {
    case "AWARD":
      return t.txTypeAward;
    case "DEDUCT":
      return t.txTypeDeduct;
    case "TRANSFER":
      return t.txTypeTransfer;
    case "EXCHANGE":
      return t.txTypeExchange;
    default:
      return type;
  }
}

/**
 * Sign character indicating direction for display class selection.
 * `>` = inflow, `<` = outflow, `=` = neutral (transfer).
 */
function txSign(type: TransactionType, amount: number): ">" | "<" | "=" {
  if (type === "AWARD") return ">";
  if (type === "DEDUCT" || type === "EXCHANGE") return "<";
  // TRANSFER: sign embedded in amount (positive = received, negative = sent).
  if (amount > 0) return ">";
  if (amount < 0) return "<";
  return "=";
}

export function defineBiyardTransactions(tag = "biyard-transactions"): void {
  if (typeof customElements === "undefined") return;
  if (!customElements.get(tag)) customElements.define(tag, BiyardTransactionsElement);
}
