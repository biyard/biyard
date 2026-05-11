/**
 * CDN entry: auto-registers every Biyard Web Component on load. Used by
 * `<script type="module" src="https://cdn.biyard.io/widget.js"></script>`.
 */
import { defineBiyardWidgets } from "./index";

defineBiyardWidgets();
