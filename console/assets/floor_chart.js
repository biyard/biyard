// Floor Price Simulator chart bridge.
//
// Registers a tiny facade on `window.biyard.floorChart` so the Rust
// side can render/update/destroy a Chart.js line chart without having
// to wrap the whole Chart.js API in wasm_bindgen.
//
// Chart.js itself is loaded via a `document::Script` tag in the
// simulator dialog (`https://cdn.jsdelivr.net/npm/chart.js`). This
// file only provides the narrow glue surface.
(function () {
  window.biyard = window.biyard || {};
  var registry = Object.create(null);

  function render(canvasId, labels, treasury, supply, floor) {
    if (typeof window.Chart === "undefined") {
      // Chart.js is still loading from the CDN. Retry shortly —
      // this avoids a blank canvas on the very first open of the
      // dialog before any user action has been taken.
      setTimeout(function () {
        render(canvasId, labels, treasury, supply, floor);
      }, 100);
      return false;
    }

    var canvas = document.getElementById(canvasId);
    if (!canvas) {
      // Canvas isn't mounted yet (dialog just opened and the effect
      // fired before the DOM settled). Retry once.
      setTimeout(function () {
        render(canvasId, labels, treasury, supply, floor);
      }, 50);
      return false;
    }

    var existing = registry[canvasId];
    var data = {
      labels: labels,
      datasets: [
        {
          label: "Treasury",
          data: treasury,
          borderColor: "#10b981",
          backgroundColor: "rgba(16, 185, 129, 0.15)",
          yAxisID: "y",
          tension: 0.25,
          pointRadius: 3,
        },
        {
          label: "Supply",
          data: supply,
          borderColor: "#6366f1",
          backgroundColor: "rgba(99, 102, 241, 0.15)",
          yAxisID: "y",
          tension: 0.25,
          pointRadius: 3,
        },
        {
          label: "Floor Price",
          data: floor,
          borderColor: "#f59e0b",
          backgroundColor: "rgba(245, 158, 11, 0.15)",
          yAxisID: "y1",
          tension: 0.25,
          pointRadius: 3,
          borderDash: [6, 4],
        },
      ],
    };

    if (existing) {
      existing.data = data;
      existing.update("none");
      return true;
    }

    var ctx = canvas.getContext("2d");
    var chart = new window.Chart(ctx, {
      type: "line",
      data: data,
      options: {
        responsive: true,
        maintainAspectRatio: false,
        interaction: { mode: "index", intersect: false },
        plugins: {
          legend: {
            position: "top",
            labels: { color: "#94a3b8", boxWidth: 14 },
          },
          tooltip: { mode: "index", intersect: false },
        },
        scales: {
          x: {
            ticks: { color: "#94a3b8" },
            grid: { color: "rgba(148, 163, 184, 0.12)" },
          },
          y: {
            type: "linear",
            position: "left",
            title: { display: true, text: "Treasury / Supply", color: "#94a3b8" },
            ticks: { color: "#94a3b8" },
            grid: { color: "rgba(148, 163, 184, 0.12)" },
          },
          y1: {
            type: "linear",
            position: "right",
            title: { display: true, text: "Floor Price", color: "#94a3b8" },
            ticks: { color: "#94a3b8" },
            grid: { drawOnChartArea: false },
          },
        },
      },
    });

    registry[canvasId] = chart;
    return true;
  }

  function destroy(canvasId) {
    var existing = registry[canvasId];
    if (existing) {
      try {
        existing.destroy();
      } catch (e) {}
      delete registry[canvasId];
    }
  }

  window.biyard.floorChart = {
    render: render,
    destroy: destroy,
  };
})();
