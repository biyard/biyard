/**
 * Biyard Landing — IntersectionObserver reveal animations.
 * Simple, reliable, no GSAP dependency.
 */
(function () {
  "use strict";

  function setup() {
    var els = document.querySelectorAll(".reveal");
    if (els.length === 0) { setTimeout(setup, 300); return; }

    var observer = new IntersectionObserver(function (entries) {
      entries.forEach(function (entry) {
        if (entry.isIntersecting) entry.target.classList.add("active");
      });
    }, { threshold: 0.1 });

    els.forEach(function (el) { observer.observe(el); });
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", function () { setTimeout(setup, 200); });
  } else {
    setTimeout(setup, 200);
  }

  // Re-observe on SPA navigation
  var mo = new MutationObserver(function () {
    var fresh = document.querySelectorAll(".reveal:not(.active)");
    if (fresh.length > 0) setup();
  });
  mo.observe(document.body, { childList: true, subtree: true });
})();
