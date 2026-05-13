/**
 * Theming surface for `<biyard-claim>`. Intentionally minimal — partners
 * style the widget with a handful of CSS custom properties on the host
 * element, similar to Stripe Elements and Reown AppKit.
 *
 * The widget defaults to the partner's surrounding fonts/colors as much as
 * possible (`inherit`, `currentColor`) and only opts into explicit colors
 * for the accent surface and the danger state.
 */
export const WIDGET_STYLES = /* css */ `
  :host {
    /* ── Partner-tunable variables ─────────────────────────────────────── */
    --biyard-color-accent: #111827;
    --biyard-color-accent-foreground: #ffffff;
    --biyard-color-bg: #ffffff;
    --biyard-color-surface: #f9fafb;
    --biyard-color-text: #111827;
    --biyard-color-muted: #6b7280;
    --biyard-color-border: #e5e7eb;
    --biyard-color-danger: #dc2626;
    --biyard-color-success: #059669;
    --biyard-font-family: inherit;
    --biyard-radius: 12px;
    --biyard-spacing: 4px;
    /* ──────────────────────────────────────────────────────────────────── */

    display: block;
    container-type: inline-size;
    container-name: biyard;
    font-family: var(--biyard-font-family);
    color: var(--biyard-color-text);
    line-height: 1.5;
  }

  :host([theme="dark"]) {
    --biyard-color-accent: #f9fafb;
    --biyard-color-accent-foreground: #111827;
    --biyard-color-bg: #0b0d10;
    --biyard-color-surface: #14171c;
    --biyard-color-text: #f3f4f6;
    --biyard-color-muted: #9ca3af;
    --biyard-color-border: #2a2f37;
    --biyard-color-danger: #f87171;
    --biyard-color-success: #34d399;
  }

  @media (prefers-color-scheme: dark) {
    :host([theme="auto"]) {
      --biyard-color-accent: #f9fafb;
      --biyard-color-accent-foreground: #111827;
      --biyard-color-bg: #0b0d10;
      --biyard-color-surface: #14171c;
      --biyard-color-text: #f3f4f6;
      --biyard-color-muted: #9ca3af;
      --biyard-color-border: #2a2f37;
      --biyard-color-danger: #f87171;
      --biyard-color-success: #34d399;
    }
  }

  /* ── Trigger button ─────────────────────────────────────────────────── */
  .trigger {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: calc(var(--biyard-spacing) * 2);
    padding: calc(var(--biyard-spacing) * 2.5) calc(var(--biyard-spacing) * 5);
    border-radius: var(--biyard-radius);
    border: 1px solid var(--biyard-color-accent);
    background: var(--biyard-color-accent);
    color: var(--biyard-color-accent-foreground);
    cursor: pointer;
    font: inherit;
    font-family: var(--biyard-font-family);
    font-size: 14px;
    font-weight: 600;
    line-height: 1.2;
    transition: filter 0.15s ease, transform 0.06s ease;
  }
  .trigger:hover {
    filter: brightness(0.95);
  }
  .trigger:active {
    transform: translateY(1px);
  }

  /* ── Inline container (mode="inline") ───────────────────────────────── */
  .inline {
    display: block;
    background: var(--biyard-color-bg);
    color: var(--biyard-color-text);
    border: 1px solid var(--biyard-color-border);
    border-radius: var(--biyard-radius);
    padding: calc(var(--biyard-spacing) * 5);
    min-width: 280px;
  }

  /* ── Modal overlay ─────────────────────────────────────────────────── */
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(17, 24, 39, 0.4);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 2147483600;
    padding: 16px;
    animation: biyard-fade 0.15s ease-out;
  }
  @keyframes biyard-fade {
    from { opacity: 0; }
    to { opacity: 1; }
  }
  .card {
    position: relative;
    width: min(420px, 100%);
    max-height: calc(100vh - 32px);
    overflow: auto;
    background: var(--biyard-color-bg);
    color: var(--biyard-color-text);
    border: 1px solid var(--biyard-color-border);
    border-radius: var(--biyard-radius);
    padding: calc(var(--biyard-spacing) * 6);
    box-shadow:
      0 1px 2px rgba(0, 0, 0, 0.05),
      0 20px 40px -10px rgba(0, 0, 0, 0.15);
    animation: biyard-pop 0.18s cubic-bezier(0.2, 0.9, 0.3, 1.15);
  }
  @keyframes biyard-pop {
    from {
      opacity: 0;
      transform: translateY(4px) scale(0.99);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  .close {
    position: absolute;
    top: 10px;
    right: 10px;
    width: 28px;
    height: 28px;
    border: none;
    background: transparent;
    color: var(--biyard-color-muted);
    cursor: pointer;
    border-radius: 6px;
    font: inherit;
    font-size: 18px;
    line-height: 1;
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }
  .close:hover {
    background: var(--biyard-color-surface);
    color: var(--biyard-color-text);
  }

  /* ── Content layout ────────────────────────────────────────────────── */
  .title {
    font-size: 15px;
    font-weight: 600;
    color: var(--biyard-color-text);
    margin: 0;
  }
  .subtitle {
    color: var(--biyard-color-muted);
    font-size: 13px;
    margin-top: 2px;
  }

  .amount {
    margin: calc(var(--biyard-spacing) * 4) 0;
    display: flex;
    align-items: baseline;
    gap: 8px;
    flex-wrap: wrap;
  }
  .amount-value {
    font-size: clamp(22px, 6cqi, 30px);
    font-weight: 700;
    line-height: 1.1;
    letter-spacing: -0.02em;
    overflow-wrap: anywhere;
    min-width: 0;
  }
  .amount-symbol {
    font-size: 14px;
    font-weight: 600;
    color: var(--biyard-color-muted);
  }

  .review {
    background: var(--biyard-color-surface);
    border: 1px solid var(--biyard-color-border);
    border-radius: calc(var(--biyard-radius) - 4px);
    padding: 12px 14px;
    font-size: 12.5px;
    color: var(--biyard-color-muted);
    line-height: 1.5;
  }
  .review-row {
    display: flex;
    justify-content: space-between;
    gap: 12px;
    padding: 4px 0;
  }
  .review-row .k {
    color: var(--biyard-color-muted);
  }
  .review-row .v {
    color: var(--biyard-color-text);
    font-weight: 500;
    text-align: right;
    overflow-wrap: anywhere;
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    font-size: 12px;
  }
  .contract-link {
    color: var(--biyard-color-text);
    text-decoration: none;
    border-bottom: 1px dashed var(--biyard-color-border);
  }
  .contract-link:hover {
    color: var(--biyard-color-accent);
    border-bottom-color: var(--biyard-color-accent);
  }

  .cta {
    width: 100%;
    margin-top: calc(var(--biyard-spacing) * 4);
    padding: calc(var(--biyard-spacing) * 3.25) calc(var(--biyard-spacing) * 4);
    border-radius: calc(var(--biyard-radius) - 2px);
    border: 1px solid var(--biyard-color-accent);
    background: var(--biyard-color-accent);
    color: var(--biyard-color-accent-foreground);
    cursor: pointer;
    font: inherit;
    font-family: var(--biyard-font-family);
    font-size: 14px;
    font-weight: 600;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    transition: filter 0.15s ease, transform 0.06s ease, opacity 0.15s ease;
  }
  .cta:hover:not([disabled]) {
    filter: brightness(0.95);
  }
  .cta:active:not([disabled]) {
    transform: translateY(1px);
  }
  .cta[disabled] {
    opacity: 0.55;
    cursor: not-allowed;
  }

  .spinner {
    width: 14px;
    height: 14px;
    border: 2px solid color-mix(in srgb, var(--biyard-color-accent-foreground) 30%, transparent);
    border-top-color: var(--biyard-color-accent-foreground);
    border-radius: 50%;
    animation: biyard-spin 0.7s linear infinite;
  }
  .skeleton {
    background: linear-gradient(
      90deg,
      var(--biyard-color-surface) 0%,
      color-mix(in srgb, var(--biyard-color-surface) 50%, var(--biyard-color-border)) 50%,
      var(--biyard-color-surface) 100%
    );
    background-size: 200% 100%;
    animation: biyard-shimmer 1.4s ease infinite;
    border-radius: 6px;
    height: 28px;
    width: 60%;
  }
  @keyframes biyard-spin { to { transform: rotate(360deg); } }
  @keyframes biyard-shimmer {
    from { background-position: 200% 0; }
    to { background-position: -200% 0; }
  }

  .alert {
    margin-top: calc(var(--biyard-spacing) * 3);
    font-size: 13px;
    line-height: 1.5;
    word-break: break-word;
    padding: 10px 12px;
    border-radius: 8px;
  }
  .alert.error {
    color: var(--biyard-color-danger);
    background: color-mix(in srgb, var(--biyard-color-danger) 8%, transparent);
    border: 1px solid color-mix(in srgb, var(--biyard-color-danger) 25%, transparent);
  }
  .alert.success {
    color: var(--biyard-color-success);
    background: color-mix(in srgb, var(--biyard-color-success) 8%, transparent);
    border: 1px solid color-mix(in srgb, var(--biyard-color-success) 25%, transparent);
  }
  .alert .tx {
    color: var(--biyard-color-muted);
    font-size: 11.5px;
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    margin-top: 4px;
  }

  /* ── Success panel (replaces CTA after a successful claim) ──────────── */
  .success-panel {
    margin-top: calc(var(--biyard-spacing) * 4);
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
    padding: calc(var(--biyard-spacing) * 4) calc(var(--biyard-spacing) * 3);
    border-radius: calc(var(--biyard-radius) - 2px);
    background: color-mix(in srgb, var(--biyard-color-success) 10%, transparent);
    border: 1px solid color-mix(in srgb, var(--biyard-color-success) 30%, transparent);
    color: var(--biyard-color-text);
    text-align: center;
  }
  .success-icon {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    background: var(--biyard-color-success);
    color: var(--biyard-color-accent-foreground);
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }
  .success-icon svg {
    width: 20px;
    height: 20px;
    display: block;
  }
  .success-text {
    font-size: 14px;
    font-weight: 600;
    color: var(--biyard-color-text);
  }
  .success-tx {
    color: var(--biyard-color-muted);
    font-size: 11.5px;
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  }

  /* ── Balance widget ─────────────────────────────────────────────────── */
  .balance-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 12px;
    gap: 8px;
  }
  .month-pill {
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.04em;
    text-transform: uppercase;
    color: var(--biyard-color-muted);
    background: var(--biyard-color-surface);
    border: 1px solid var(--biyard-color-border);
    padding: 3px 8px;
    border-radius: 999px;
  }
  .balance-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
    gap: 12px;
  }
  .balance-cell {
    background: var(--biyard-color-surface);
    border: 1px solid var(--biyard-color-border);
    border-radius: calc(var(--biyard-radius) - 4px);
    padding: 14px;
    min-width: 0;
  }
  .balance-cell-label {
    color: var(--biyard-color-muted);
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: 4px;
  }
  .balance-cell-value {
    font-size: clamp(16px, 5cqi, 22px);
    font-weight: 700;
    letter-spacing: -0.02em;
    line-height: 1.2;
    overflow-wrap: anywhere;
    min-width: 0;
  }
  .balance-cell-symbol {
    font-size: 12px;
    font-weight: 600;
    color: var(--biyard-color-muted);
  }
  .balance-cell-hint {
    color: var(--biyard-color-muted);
    font-size: 11px;
    margin-top: 6px;
    line-height: 1.4;
  }

  /* ── Transactions widget ────────────────────────────────────────────── */
  .tx-list {
    display: flex;
    flex-direction: column;
    margin: 4px 0 0;
  }
  .tx-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 0;
    border-bottom: 1px solid var(--biyard-color-border);
    gap: 12px;
    min-width: 0;
  }
  .tx-row:last-of-type { border-bottom: none; }
  .tx-left {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
    flex: 1;
  }
  .tx-type {
    font-size: 13px;
    font-weight: 600;
    color: var(--biyard-color-text);
  }
  .tx-meta {
    color: var(--biyard-color-muted);
    font-size: 12px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .tx-amount {
    font-weight: 600;
    font-size: 14px;
    white-space: nowrap;
  }
  .tx-amount.pos { color: var(--biyard-color-success); }
  .tx-amount.neg { color: var(--biyard-color-danger); }
  .tx-amount.neutral { color: var(--biyard-color-text); }

  .load-more {
    margin-top: 12px;
    width: 100%;
    padding: 10px;
    border-radius: 8px;
    border: 1px solid var(--biyard-color-border);
    background: var(--biyard-color-surface);
    color: var(--biyard-color-text);
    cursor: pointer;
    font: inherit;
    font-size: 13px;
  }
  .load-more:hover:not([disabled]) {
    background: var(--biyard-color-bg);
  }
  .load-more[disabled] { opacity: 0.6; cursor: not-allowed; }

  /* ── Monthly summary widget ─────────────────────────────────────────── */
  .summary-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  .summary-row {
    background: var(--biyard-color-surface);
    border: 1px solid var(--biyard-color-border);
    border-radius: calc(var(--biyard-radius) - 4px);
    padding: 12px 14px;
    display: grid;
    grid-template-columns: 1fr auto;
    gap: 8px 14px;
    align-items: baseline;
  }
  .summary-month {
    grid-row: 1 / 3;
    grid-column: 1;
    font-weight: 700;
    font-size: 15px;
    align-self: center;
    white-space: nowrap;
  }
  .summary-stats {
    grid-row: 1 / 3;
    grid-column: 2;
    display: flex;
    gap: 14px;
    align-items: baseline;
    flex-wrap: wrap;
    justify-content: flex-end;
  }

  @container biyard (max-width: 420px) {
    .summary-row {
      grid-template-columns: 1fr;
    }
    .summary-month {
      grid-row: 1;
      grid-column: 1;
    }
    .summary-stats {
      grid-row: 2;
      grid-column: 1;
      justify-content: space-between;
      gap: 10px;
    }
    .summary-claimed-pill {
      grid-row: 3;
      grid-column: 1;
    }
  }
  .summary-stat {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
  }
  .summary-stat-label {
    color: var(--biyard-color-muted);
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
  .summary-stat-value {
    font-size: 14px;
    font-weight: 600;
    white-space: nowrap;
  }
  .summary-claimed-pill {
    grid-row: 2;
    grid-column: 1;
    justify-self: start;
    font-size: 10px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--biyard-color-success);
    background: color-mix(in srgb, var(--biyard-color-success) 12%, transparent);
    border: 1px solid color-mix(in srgb, var(--biyard-color-success) 30%, transparent);
    padding: 2px 8px;
    border-radius: 999px;
  }

  .empty-state {
    text-align: center;
    color: var(--biyard-color-muted);
    font-size: 13px;
    padding: 12px 0;
  }

  /* ── Attribution footer ─────────────────────────────────────────────── */
  .attribution {
    margin-top: calc(var(--biyard-spacing) * 4);
    text-align: center;
    font-size: 11px;
    line-height: 1;
  }
  .attribution a {
    color: var(--biyard-color-muted);
    text-decoration: none;
    opacity: 0.7;
  }
  .attribution a:hover {
    opacity: 1;
    color: var(--biyard-color-text);
  }
`;
