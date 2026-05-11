/**
 * CDN entry: auto-registers `<biyard-claim>` on load. Used by
 * `<script type="module" src="https://cdn.biyard.io/widget.js"></script>`.
 */
import { defineBiyardClaim } from "./biyard-claim";

defineBiyardClaim();
