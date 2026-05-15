export { BiyardClaimElement, defineBiyardClaim } from "./biyard-claim";
export { BiyardBalanceElement, defineBiyardBalance } from "./biyard-balance";
export {
  BiyardTransactionsElement,
  defineBiyardTransactions,
} from "./biyard-transactions";
export {
  BiyardMonthlySummaryElement,
  defineBiyardMonthlySummary,
} from "./biyard-monthly-summary";

import { defineBiyardClaim } from "./biyard-claim";
import { defineBiyardBalance } from "./biyard-balance";
import { defineBiyardTransactions } from "./biyard-transactions";
import { defineBiyardMonthlySummary } from "./biyard-monthly-summary";

/**
 * Register every Biyard Web Component with its default tag name.
 *
 * Useful when the host page just wants all widgets available; individual
 * `define*` functions can still be called if you only need a subset.
 */
export function defineBiyardWidgets(): void {
  defineBiyardClaim();
  defineBiyardBalance();
  defineBiyardTransactions();
  defineBiyardMonthlySummary();
}
