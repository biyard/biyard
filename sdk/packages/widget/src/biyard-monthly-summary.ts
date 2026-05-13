import type { MonthlySummary } from "@biyard/sdk";

import { BiyardWidgetBase, escapeHtml, renderAttribution } from "./base";
import { strings } from "./i18n";
import { WIDGET_STYLES } from "./styles";

/**
 * `<biyard-monthly-summary>` — list of per-month point summaries
 * (earned/spent/balance + claimed flag).
 *
 * ```html
 * <biyard-monthly-summary base-url="/api/biyard"></biyard-monthly-summary>
 * ```
 *
 * Same theming/i18n/branding attributes.
 */
export class BiyardMonthlySummaryElement extends BiyardWidgetBase {
  static get observedAttributes(): string[] {
    return ["base-url", "theme", "lang", "branding", "title"];
  }

  private summaries: MonthlySummary[] = [];
  private loading = false;
  private error: string | null = null;

  protected override onConnected(): void {
    void this.load();
  }

  private async load(): Promise<void> {
    this.loading = true;
    this.error = null;
    this.render();
    try {
      const client = this.getFetchOnlyClient();
      const res = await client.getMonthlySummaries();
      this.summaries = res.months;
    } catch (e) {
      this.error = e instanceof Error ? e.message : String(e);
    } finally {
      this.loading = false;
      this.render();
    }
  }

  protected render(): void {
    const t = strings(this.getLocale());
    const title = this.getAttribute("title") ?? t.summaryTitle;
    const body = this.loading
      ? `<div class="empty-state">${escapeHtml(t.loading)}</div>`
      : this.summaries.length === 0
        ? `<div class="empty-state">${escapeHtml(t.empty)}</div>`
        : this.renderList(t);

    this.root.innerHTML = `
      <style>${WIDGET_STYLES}</style>
      <div class="inline" part="card">
        <h3 class="title">${escapeHtml(title)}</h3>
        <div style="margin-top:12px">${body}</div>
        ${this.error ? `<div class="alert error">${escapeHtml(this.error)}</div>` : ""}
        ${renderAttribution(this.getBranding(), t)}
      </div>
    `;
  }

  private renderList(t: ReturnType<typeof strings>): string {
    const rows = this.summaries.map((s) => {
      const earned = s.total_earned.toLocaleString();
      const spent = s.total_spent.toLocaleString();
      const balance = s.balance.toLocaleString();
      return `
        <div class="summary-row">
          <div class="summary-month">${escapeHtml(s.month)}</div>
          <div class="summary-stats">
            <div class="summary-stat">
              <span class="summary-stat-label">${escapeHtml(t.summaryEarned)}</span>
              <span class="summary-stat-value">+${escapeHtml(earned)}</span>
            </div>
            <div class="summary-stat">
              <span class="summary-stat-label">${escapeHtml(t.summarySpent)}</span>
              <span class="summary-stat-value">-${escapeHtml(spent)}</span>
            </div>
            <div class="summary-stat">
              <span class="summary-stat-label">${escapeHtml(t.summaryBalance)}</span>
              <span class="summary-stat-value">${escapeHtml(balance)}</span>
            </div>
          </div>
          ${s.exchanged ? `<span class="summary-claimed-pill">${escapeHtml(t.summaryExchanged)}</span>` : ""}
        </div>
      `;
    });
    return `<div class="summary-list">${rows.join("")}</div>`;
  }
}

export function defineBiyardMonthlySummary(tag = "biyard-monthly-summary"): void {
  if (typeof customElements === "undefined") return;
  if (!customElements.get(tag)) customElements.define(tag, BiyardMonthlySummaryElement);
}
