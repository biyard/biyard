// Biyard STO Mockup — shared helpers & UI
// 자산 맵의 단위는 "STO (개별 발행 자산)"이며, 발행사는 보조 정보로만 표시.

const $ = (sel, ctx = document) => ctx.querySelector(sel);
const $$ = (sel, ctx = document) => Array.from(ctx.querySelectorAll(sel));

const D = window.STO_DATA;

function fmtNum(n) {
  return new Intl.NumberFormat("ko-KR").format(n);
}

function renderTopbar(activeKey) {
  const links = [
    { key: "assets", href: "assets.html", label: "STO 시장" },
    { key: "index", href: "biyard-index.html", label: "평가지표" },
    { key: "launchpad", href: "launchpad.html", label: "런치패드" },
    { key: "news", href: "news.html", label: "뉴스" },
    { key: "pricing", href: "pricing.html", label: "가격" },
  ];
  return `
    <header class="topbar">
      <div class="topbar-inner">
        <a class="brand-lockup" href="index.html" aria-label="Biyard">
          <img src="biyard-logo.png" alt="Biyard" />
          <span>Biyard</span>
          <small>STO</small>
        </a>
        <nav class="nav-links" aria-label="주요 화면">
          ${links
            .map(
              (l) =>
                `<a href="${l.href}" class="${l.key === activeKey ? "active" : ""}">${l.label}</a>`,
            )
            .join("")}
        </nav>
        <div class="topbar-actions">
          <input class="search-mini" type="search" placeholder="STO 검색..." id="topSearch" />
        </div>
      </div>
    </header>
  `;
}

function renderFooter() {
  return `
    <footer class="footer">
      © Biyard 2026 · 출처: DART, SEC EDGAR, 검증된 언론 매체. 공시 자료 기반 정보 제공이며 투자 자문이 아닙니다.
    </footer>
  `;
}

function mountChrome(activeKey) {
  document.body.insertAdjacentHTML("afterbegin", renderTopbar(activeKey));
  document.body.insertAdjacentHTML("beforeend", renderFooter());

  const ts = $("#topSearch");
  if (ts) {
    ts.addEventListener("keydown", (e) => {
      if (e.key === "Enter") {
        const q = ts.value.trim();
        location.href = `assets.html?q=${encodeURIComponent(q)}`;
      }
    });
  }
}

function sourceBadge(src) {
  const s = D.SOURCES[src];
  if (!s) return "";
  return `<span class="source-badge" title="${s.desc}">${s.label}</span>`;
}

function sourceList(sources) {
  if (!sources || !sources.length) return "";
  return `
    <ul class="source-list">
      ${sources
        .map(
          (s) => `
        <li>
          ${sourceBadge(s.src)}
          <span class="muted">${s.label}</span>
        </li>
      `,
        )
        .join("")}
    </ul>
  `;
}

function regionFlag(region, country) {
  if (region === "KR") return "🇰🇷";
  if (country) return country.split(" ")[0];
  return "🌐";
}

function categoryOf(key) {
  return D.CATEGORIES.find((c) => c.key === key) || { label: key, icon: "·" };
}

// ===== STO Card (compact, asset-level) =====
// 한 화면에 많이 보이게 콤팩트. 메인은 자산명. 발행사·카테고리는 작은 메타.
function stoCard(s) {
  const cat = categoryOf(s.category);
  const issuer = D.findIssuer(s.issuerId);
  const flag = regionFlag(s.region, s.country);
  const href = `detail.html?id=${s.id}`;
  return `
    <a class="sto-card" href="${href}">
      <div class="sto-card-head">
        <span class="sto-icon">${s.icon}</span>
        <span class="sto-flag">${flag}</span>
        <span class="sto-cat-chip">${cat.icon} ${cat.label}</span>
      </div>
      <div class="sto-name">${s.name}</div>
      <div class="sto-issuer">${issuer ? issuer.name : "—"}</div>
      <div class="sto-foot">
        <span class="sto-foot-item">${s.securityType || "—"}</span>
        <span class="sto-foot-sep">·</span>
        <span class="sto-foot-item">${s.issuedAt || "—"}</span>
      </div>
    </a>
  `;
}

// ===== STO Table Row (list format, like DART/Investing.com) =====
function stoRow(s) {
  const cat = categoryOf(s.category);
  const issuer = D.findIssuer(s.issuerId);
  const flag = regionFlag(s.region, s.country);
  const verified = s.verified
    ? `<span class="verified-badge" title="DART 공시 검증">✓ DART</span>`
    : "";
  const refBadge = s.referenceOnly
    ? `<span class="reference-badge" title="${s.referenceNote || "참고 비교군"}">참고</span>`
    : "";
  const yld =
    s.liquidation && typeof s.liquidation.reportedYieldPercent === "number"
      ? `<span class="yield-pill">${s.liquidation.reportedYieldPercent.toFixed(1)}%</span>`
      : "";
  const status = (s.status || "").includes("청산")
    ? `<span class="status-pill liquidated">${s.status}</span>`
    : s.status || "—";
  return `
    <tr onclick="location.href='detail.html?id=${s.id}'" style="cursor: pointer;">
      <td class="col-icon">${s.icon || "·"}</td>
      <td class="col-region">${flag}</td>
      <td class="col-cat"><span class="cat-tag">${cat.icon} ${cat.label}</span></td>
      <td class="col-name">
        <div class="row-name">${s.name} ${verified} ${refBadge}</div>
        <div class="row-underlying muted">${s.underlying || ""}</div>
      </td>
      <td class="col-issuer">${issuer ? issuer.name : "—"}</td>
      <td class="col-security">${s.securityType || "—"}</td>
      <td class="col-issued">${s.issuedAt || "—"}</td>
      <td class="col-status">${status}</td>
      <td class="col-yield">${yld}</td>
    </tr>
  `;
}

function stoTable(stos) {
  if (!stos || !stos.length) {
    return `<div class="empty" style="padding: 60px; text-align: center; color: var(--muted);">조건에 맞는 STO가 없습니다.</div>`;
  }
  return `
    <div class="sto-table-wrap">
      <table class="sto-table">
        <thead>
          <tr>
            <th class="col-icon"></th>
            <th class="col-region">국가</th>
            <th class="col-cat">카테고리</th>
            <th class="col-name">STO 자산명 / 기초자산</th>
            <th class="col-issuer">발행사</th>
            <th class="col-security">증권 종류</th>
            <th class="col-issued">발행 시기</th>
            <th class="col-status">상태</th>
            <th class="col-yield">수익률</th>
          </tr>
        </thead>
        <tbody>
          ${stos.map(stoRow).join("")}
        </tbody>
      </table>
    </div>
  `;
}

// ===== Issuer Card (separate, only for compare/dashboard pages if needed) =====
function issuerCard(i) {
  const cat = categoryOf(i.category);
  const flag = regionFlag(i.region, i.country);
  const stoCount = D.stosByIssuer(i.id).length;
  return `
    <div class="issuer-card">
      <div class="issuer-head">
        <span class="flag">${flag}</span>
        <strong>${i.name}</strong>
        <span class="cat-tag">${cat.icon} ${cat.label}</span>
      </div>
      <div class="issuer-meta muted">${i.description || ""}</div>
      <div class="issuer-stats">
        <span>등재 STO <strong>${stoCount}건</strong></span>
        <span>· 인가 <strong>${i.sandbox || "—"}</strong></span>
      </div>
    </div>
  `;
}

// ===== 카테고리별 투자 규모 집계 (KRW 기준) =====
// 외화는 환율 미적용 시 KRW 와 합산하지 않고 별도 표기.
// 환율은 정적이라 향후 OPENBANK·KEB 환율 API 연동 시 자동 변환 가능.
const FX_TO_KRW = {
  KRW: 1,
  USD: 1380, // 정적 mock — 운영 시 일별 환율로 대체
  EUR: 1480,
  SGD: 1020,
  CHF: 1550,
  JPY: 9.2,
};

function fmtKRW(v) {
  if (v == null) return "—";
  if (v >= 1_000_000_000_000) return `${(v / 1_000_000_000_000).toFixed(1)}조`;
  if (v >= 100_000_000) return `${(v / 100_000_000).toFixed(0)}억`;
  if (v >= 10_000_000) return `${(v / 10_000_000).toFixed(1)}천만`;
  if (v >= 10_000) return `${(v / 10_000).toFixed(0)}만`;
  return new Intl.NumberFormat("ko-KR").format(v);
}

function offeringInKRW(s) {
  const amt = s.offering?.amount;
  const cur = s.offering?.currency || "KRW";
  if (amt == null) return null;
  const fx = FX_TO_KRW[cur];
  if (fx == null) return null;
  return Math.round(amt * fx);
}

// 카테고리별 누적 모집액 + 건수 (region 필터 옵션)
function categoryScale(stos, region) {
  const filtered =
    region && region !== "all" ? stos.filter((s) => s.region === region) : stos;
  const map = {};
  for (const s of filtered) {
    const key = s.category;
    if (!map[key]) map[key] = { count: 0, totalKRW: 0, withAmount: 0 };
    map[key].count += 1;
    const krw = offeringInKRW(s);
    if (krw != null) {
      map[key].totalKRW += krw;
      map[key].withAmount += 1;
    }
  }
  return D.CATEGORIES.map((c) => ({
    ...c,
    ...(map[c.key] || { count: 0, totalKRW: 0, withAmount: 0 }),
  }))
    .filter((c) => c.count > 0)
    .sort((a, b) => b.totalKRW - a.totalKRW);
}

// 카테고리 색상 팔레트 (CATEGORIES 순서 기준)
const CAT_COLORS = {
  real_estate: "#00e5a0",
  music: "#5eb0ff",
  art: "#ffb547",
  livestock: "#ff8a4c",
  luxury: "#c084fc",
  infra: "#22d3ee",
  content: "#ff5f9e",
};
function colorOf(cat, idx) {
  return CAT_COLORS[cat] || `hsl(${(idx * 47) % 360},65%,60%)`;
}

// 파이 차트 — SVG donut + 우측 범례
function categoryScaleBar(rows) {
  // 함수명 유지 (호출자 영향 X) — 내부 구현만 파이 차트로 변경
  const withAmount = rows.filter((r) => r.totalKRW > 0);
  if (!withAmount.length) {
    return `<div class="empty" style="padding:30px; text-align:center; color:var(--muted);">집계할 모집액 데이터가 없습니다.</div>`;
  }
  const total = withAmount.reduce((sum, r) => sum + r.totalKRW, 0);

  // SVG donut: 360deg 를 비율로 분할
  const cx = 110,
    cy = 110,
    r = 90,
    innerR = 56;
  let startAngle = -Math.PI / 2; // 12시 방향 시작
  const arcs = withAmount.map((c, i) => {
    const ratio = c.totalKRW / total;
    const endAngle = startAngle + ratio * Math.PI * 2;
    const large = ratio > 0.5 ? 1 : 0;

    const x1 = cx + r * Math.cos(startAngle);
    const y1 = cy + r * Math.sin(startAngle);
    const x2 = cx + r * Math.cos(endAngle);
    const y2 = cy + r * Math.sin(endAngle);
    const x3 = cx + innerR * Math.cos(endAngle);
    const y3 = cy + innerR * Math.sin(endAngle);
    const x4 = cx + innerR * Math.cos(startAngle);
    const y4 = cy + innerR * Math.sin(startAngle);

    const path = `M ${x1} ${y1} A ${r} ${r} 0 ${large} 1 ${x2} ${y2} L ${x3} ${y3} A ${innerR} ${innerR} 0 ${large} 0 ${x4} ${y4} Z`;
    const color = colorOf(c.key, i);
    const tooltip = `${c.label}: ${fmtKRW(c.totalKRW)}원 (${(ratio * 100).toFixed(1)}%)`;
    startAngle = endAngle;
    return { path, color, ratio, tooltip, ...c };
  });

  return `
    <div class="cat-pie-wrap">
      <svg class="cat-pie" viewBox="0 0 220 220" width="220" height="220" aria-label="카테고리별 모집액 파이 차트">
        ${arcs
          .map(
            (a) =>
              `<path d="${a.path}" fill="${a.color}" stroke="var(--bg)" stroke-width="2"><title>${a.tooltip}</title></path>`,
          )
          .join("")}
        <text x="110" y="104" text-anchor="middle" font-size="11" fill="var(--muted)">누적 모집액</text>
        <text x="110" y="124" text-anchor="middle" font-size="18" font-weight="700" fill="var(--ink)" font-family="var(--font-mono)">${fmtKRW(total)}원</text>
      </svg>
      <ul class="cat-pie-legend">
        ${arcs
          .map((a) => {
            const incomplete = a.withAmount < a.count;
            return `
              <li>
                <span class="legend-swatch" style="background:${a.color}"></span>
                <span class="legend-label">${a.icon} ${a.label}</span>
                <span class="legend-pct">${(a.ratio * 100).toFixed(1)}%</span>
                <span class="legend-amt">${fmtKRW(a.totalKRW)}원</span>
                <span class="legend-count muted">${a.count}건${
                  incomplete ? ` (${a.withAmount} 집계)` : ""
                }</span>
              </li>
            `;
          })
          .join("")}
        ${rows
          .filter((r) => r.totalKRW === 0)
          .map(
            (c) => `
              <li class="legend-empty">
                <span class="legend-swatch" style="background:var(--panel-3)"></span>
                <span class="legend-label">${c.icon} ${c.label}</span>
                <span class="legend-pct muted">미공시</span>
                <span class="legend-amt muted">—</span>
                <span class="legend-count muted">${c.count}건</span>
              </li>
            `,
          )
          .join("")}
      </ul>
    </div>
  `;
}

// ===== 카테고리별 청산 통계 =====
// 청산 데이터 있는 STO 만 집계. 평균 수익률·연환산·운용일·건수.
function categoryLiquidationStats(category, region) {
  const list = D.STOS.filter((s) => {
    if (s.category !== category) return false;
    if (region && region !== "all" && s.region !== region) return false;
    return s.liquidation && typeof s.liquidation.reportedYieldPercent === "number";
  });
  if (!list.length) return null;
  const yields = list.map((s) => s.liquidation.reportedYieldPercent);
  const annualized = list
    .map((s) => s.liquidation.annualizedYieldPercent)
    .filter((v) => typeof v === "number");
  const days = list
    .map((s) => {
      const lv = s.livestock?.feedingPeriodDays;
      if (typeof lv === "number") return lv;
      const issued = s.issuedAt;
      const liq = s.liquidation.liquidatedAt;
      if (!issued || !liq) return null;
      const a = new Date(liq);
      const b = new Date(issued.replace(/\./g, "-"));
      const d = Math.round((a - b) / (1000 * 60 * 60 * 24));
      return isFinite(d) && d > 0 ? d : null;
    })
    .filter((v) => v != null);
  const avg = (arr) => arr.reduce((a, b) => a + b, 0) / arr.length;
  return {
    count: list.length,
    avgYield: avg(yields),
    minYield: Math.min(...yields),
    maxYield: Math.max(...yields),
    avgAnnualized: annualized.length ? avg(annualized) : null,
    avgDays: days.length ? Math.round(avg(days)) : null,
    items: list,
  };
}

// 같은 카테고리·청산된 STO 중 현재 STO 와 다른 것
function similarLiquidatedStos(currentSto, limit = 5) {
  return D.STOS.filter((s) => {
    if (s.id === currentSto.id) return false;
    if (s.category !== currentSto.category) return false;
    if (!(s.liquidation && typeof s.liquidation.reportedYieldPercent === "number")) return false;
    return true;
  }).slice(0, limit);
}

function toast(msg) {
  const el = document.createElement("div");
  el.className = "toast";
  el.innerHTML = `<span class="ico">●</span> ${msg}`;
  document.body.appendChild(el);
  setTimeout(() => el.remove(), 2400);
}

window.STO_UI = {
  $,
  $$,
  fmtNum,
  fmtKRW,
  offeringInKRW,
  categoryScale,
  categoryScaleBar,
  categoryLiquidationStats,
  similarLiquidatedStos,
  mountChrome,
  sourceBadge,
  sourceList,
  regionFlag,
  categoryOf,
  stoCard,
  stoRow,
  stoTable,
  issuerCard,
  toast,
};
