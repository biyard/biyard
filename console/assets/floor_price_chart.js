// Floor Price Simulator chart helpers.
//
// Mounted at `window.biyard.simulator`. Loaded by the simulator dialog
// alongside the Chart.js CDN script. The Rust side calls
// `render_chart(canvas_id, payload_json)` whenever the simulator inputs
// change; this module owns the Chart.js instance lifecycle so the Rust
// side never has to hold a JS handle.

(function () {
  if (!window.biyard) window.biyard = {};
  if (!window.biyard.simulator) window.biyard.simulator = {};

  // canvas_id -> Chart instance, so repeated renders update in place
  // instead of stacking new charts on top of the same canvas.
  var charts = {};

  function waitForChartAndCanvas(canvas_id, cb, attempts) {
    var ready =
      typeof window.Chart !== "undefined" &&
      document.getElementById(canvas_id) !== null;
    if (ready) {
      cb();
      return;
    }
    if (attempts <= 0) return;
    setTimeout(function () {
      waitForChartAndCanvas(canvas_id, cb, attempts - 1);
    }, 50);
  }

  function destroy_chart(canvas_id) {
    var existing = charts[canvas_id];
    if (existing) {
      existing.destroy();
      delete charts[canvas_id];
    }
  }

  function formatCompact(v) {
    var n = Number(v);
    if (!isFinite(n)) return String(v);
    var abs = Math.abs(n);
    if (abs >= 1e12) return (n / 1e12).toFixed(abs >= 1e13 ? 0 : 1) + "T";
    if (abs >= 1e9) return (n / 1e9).toFixed(abs >= 1e10 ? 0 : 1) + "B";
    if (abs >= 1e6) return (n / 1e6).toFixed(abs >= 1e7 ? 0 : 1) + "M";
    if (abs >= 1e3) return (n / 1e3).toFixed(abs >= 1e4 ? 0 : 1) + "K";
    if (abs >= 1) return n.toFixed(0);
    if (abs === 0) return "0";
    return n.toFixed(4);
  }

  // Callback set by the Rust side. Signature: fn(month: number, value: number)
  var onTreasuryDrag = null;

  function set_on_treasury_drag(cb) {
    onTreasuryDrag = cb;
  }

  // ── Custom drag-to-edit for Treasury dataset ──────────────────────
  // We avoid chartjs-plugin-dragdata because Dioxus dialog event
  // handling prevents Chart.js from receiving mousemove/mouseup via
  // its internal event pipeline. Instead we listen directly on the
  // canvas and on the window for move/up events.
  function setupDrag(chart, labels) {
    var canvas = chart.canvas;
    var dragging = false;
    var dragIndex = -1;
    var TREASURY_DS = 1; // dataset index for Treasury

    canvas.style.touchAction = "none";

    function getCanvasPos(e) {
      var rect = canvas.getBoundingClientRect();
      return { x: e.clientX - rect.left, y: e.clientY - rect.top };
    }

    function findNearestTreasuryPoint(pos) {
      var meta = chart.getDatasetMeta(TREASURY_DS);
      var best = -1;
      var bestDist = Infinity;
      for (var i = 0; i < meta.data.length; i++) {
        var pt = meta.data[i];
        var dx = pt.x - pos.x;
        var dy = pt.y - pos.y;
        var d = Math.sqrt(dx * dx + dy * dy);
        if (d < bestDist && d < 20) {
          bestDist = d;
          best = i;
        }
      }
      return best;
    }

    function yPixelToValue(yPx) {
      var scale = chart.scales.y; // Treasury uses y axis
      return scale.getValueForPixel(yPx);
    }

    canvas.addEventListener("pointerdown", function (e) {
      var pos = getCanvasPos(e);
      var idx = findNearestTreasuryPoint(pos);
      if (idx < 0) return;
      dragging = true;
      dragIndex = idx;
      canvas.setPointerCapture(e.pointerId);
      e.preventDefault();
      e.stopPropagation();
    });

    canvas.addEventListener("pointermove", function (e) {
      if (!dragging) {
        // Show grab cursor when hovering over a treasury point
        var pos = getCanvasPos(e);
        var idx = findNearestTreasuryPoint(pos);
        canvas.style.cursor = idx >= 0 ? "grab" : "";
        return;
      }
      e.preventDefault();
      e.stopPropagation();
      var pos = getCanvasPos(e);
      var newVal = Math.max(0, Math.round(yPixelToValue(pos.y)));
      chart.data.datasets[TREASURY_DS].data[dragIndex] = newVal;
      chart.update("none");
    });

    canvas.addEventListener("pointerup", function (e) {
      if (!dragging) return;
      dragging = false;
      e.preventDefault();
      var pos = getCanvasPos(e);
      var newVal = Math.max(0, Math.round(yPixelToValue(pos.y)));
      chart.data.datasets[TREASURY_DS].data[dragIndex] = newVal;
      chart.update("none");
      if (onTreasuryDrag && labels[dragIndex] != null) {
        onTreasuryDrag(labels[dragIndex], newVal);
      }
      canvas.style.cursor = "";
      dragIndex = -1;
    });

    canvas.addEventListener("pointercancel", function () {
      dragging = false;
      canvas.style.cursor = "";
      dragIndex = -1;
    });
  }

  function render_chart(canvas_id, payload_json) {
    waitForChartAndCanvas(canvas_id, function () {
      var canvas = document.getElementById(canvas_id);
      if (!canvas) return;

      var payload;
      try {
        payload = JSON.parse(payload_json);
      } catch (e) {
        return;
      }

      var labels = payload.labels || [];
      var treasury = payload.treasury || [];
      var supply = payload.supply || [];
      var floor = payload.floor || [];
      var t = payload.t || {};

      var existing = charts[canvas_id];
      if (existing) {
        existing.data.labels = labels;
        existing.data.datasets[0].data = supply;
        existing.data.datasets[0].label = t.supply || "Supply";
        existing.data.datasets[1].data = treasury;
        existing.data.datasets[1].label = t.treasury || "Treasury";
        existing.data.datasets[2].data = floor;
        existing.data.datasets[2].label = t.floor || "Floor Price";
        existing.options.scales.y.title.text = t.y_left || "Treasury / Supply";
        existing.options.scales.y1.title.text = t.y_right || "Floor Price";
        existing.options.scales.x.title.text = t.x || "Month";
        existing.update("none");
        return;
      }

      var ctx = canvas.getContext("2d");
      var chart = new window.Chart(ctx, {
        type: "line",
        data: {
          labels: labels,
          datasets: [
            {
              type: "bar",
              label: t.supply || "Supply",
              data: supply,
              backgroundColor: "rgba(148, 163, 184, 0.35)",
              borderColor: "rgba(148, 163, 184, 0.6)",
              borderWidth: 1,
              yAxisID: "y2",
              order: 3,
            },
            {
              label: t.treasury || "Treasury",
              data: treasury,
              borderColor: "rgb(16, 185, 129)",
              backgroundColor: "rgba(16, 185, 129, 0.15)",
              yAxisID: "y",
              tension: 0.25,
              pointRadius: 5,
              pointHoverRadius: 8,
              pointHitRadius: 12,
              borderWidth: 2.5,
              order: 2,
            },
            {
              label: t.floor || "Floor Price",
              data: floor,
              borderColor: "rgb(99, 102, 241)",
              backgroundColor: "rgba(99, 102, 241, 0.2)",
              yAxisID: "y1",
              tension: 0.25,
              pointRadius: 2,
              borderWidth: 2.5,
              order: 1,
            },
          ],
        },
        options: {
          responsive: true,
          maintainAspectRatio: false,
          interaction: { mode: "index", intersect: false },
          plugins: {
            legend: { position: "bottom" },
            tooltip: {
              callbacks: {
                title: function (items) {
                  if (!items.length) return "";
                  return items[0].label + (t.month_suffix || "개월차");
                },
                label: function (ctx) {
                  var v = ctx.parsed.y;
                  return ctx.dataset.label + ": " + v.toLocaleString();
                },
              },
            },
          },
          scales: {
            x: {
              title: { display: true, text: t.x || "Month" },
            },
            y: {
              type: "linear",
              position: "left",
              title: {
                display: true,
                text: t.y_left || "Treasury / Supply",
              },
              ticks: {
                callback: function (v) {
                  return formatCompact(v);
                },
              },
            },
            y1: {
              type: "linear",
              position: "right",
              title: {
                display: true,
                text: t.y_right || "Floor Price",
              },
              grid: { drawOnChartArea: false },
              ticks: {
                callback: function (v) {
                  return formatCompact(v);
                },
              },
            },
            y2: {
              type: "linear",
              display: false,
              beginAtZero: true,
              grid: { drawOnChartArea: false },
            },
          },
        },
      });
      charts[canvas_id] = chart;

      // Set up custom drag handling for Treasury points
      setupDrag(chart, labels);
    }, 100);
  }

  window.biyard.simulator.render_chart = render_chart;
  window.biyard.simulator.destroy_chart = destroy_chart;
  window.biyard.simulator.set_on_treasury_drag = set_on_treasury_drag;
})();
