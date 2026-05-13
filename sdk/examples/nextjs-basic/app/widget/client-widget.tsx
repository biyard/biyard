"use client";

import { defineBiyardWidgets } from "@biyard/widget";
import {
  useEffect,
  useMemo,
  useState,
  type CSSProperties,
  type ReactNode,
} from "react";

declare module "react" {
  namespace JSX {
    interface IntrinsicElements {
      "biyard-claim": React.DetailedHTMLProps<
        React.HTMLAttributes<HTMLElement> & {
          "base-url": string;
          month: string;
          "chain-id"?: string | number;
          mode?: "modal" | "inline";
          theme?: "auto" | "light" | "dark";
          branding?: "default" | "minimal" | "none";
          lang?: string;
          label?: string;
          title?: string;
          subtitle?: string;
        },
        HTMLElement
      >;
      "biyard-balance": React.DetailedHTMLProps<
        React.HTMLAttributes<HTMLElement> & {
          "base-url": string;
          month?: string;
          "chain-id"?: string | number;
          theme?: "auto" | "light" | "dark";
          branding?: "default" | "minimal" | "none";
          lang?: string;
          title?: string;
        },
        HTMLElement
      >;
      "biyard-transactions": React.DetailedHTMLProps<
        React.HTMLAttributes<HTMLElement> & {
          "base-url": string;
          limit?: string | number;
          month?: string;
          theme?: "auto" | "light" | "dark";
          branding?: "default" | "minimal" | "none";
          lang?: string;
          title?: string;
        },
        HTMLElement
      >;
      "biyard-monthly-summary": React.DetailedHTMLProps<
        React.HTMLAttributes<HTMLElement> & {
          "base-url": string;
          theme?: "auto" | "light" | "dark";
          branding?: "default" | "minimal" | "none";
          lang?: string;
          title?: string;
        },
        HTMLElement
      >;
    }
  }
}

type Device = "mobile" | "tablet" | "desktop";
type Theme = "light" | "dark";
type Lang = "en" | "ko";
type PageId = "home" | "shop" | "account" | "activity";

const DEVICE_PRESETS: Record<
  Device,
  { label: string; width: number; height: number; emoji: string }
> = {
  mobile: { label: "Mobile", width: 390, height: 780, emoji: "📱" },
  tablet: { label: "Tablet", width: 820, height: 920, emoji: "📲" },
  desktop: { label: "Desktop", width: 1180, height: 920, emoji: "🖥️" },
};

const PARTNER_ACCENT = { value: "#6366f1", fg: "#ffffff" };

export default function ClientWidgets() {
  const [ready, setReady] = useState(false);
  const [device, setDevice] = useState<Device>("desktop");
  const [theme, setTheme] = useState<Theme>("light");
  const [lang, setLang] = useState<Lang>("en");
  const [page, setPage] = useState<PageId>("home");

  useEffect(() => {
    defineBiyardWidgets();
    setReady(true);
  }, []);

  const accent = PARTNER_ACCENT;
  const preset = DEVICE_PRESETS[device];

  const frameStyle = useMemo<CSSProperties>(() => {
    const isDark = theme === "dark";
    return {
      width: preset.width,
      height: preset.height,
      maxWidth: "100%",
      background: isDark ? "#0b0d10" : "#ffffff",
      color: isDark ? "#f3f4f6" : "#111827",
      borderRadius: device === "mobile" ? 32 : device === "tablet" ? 20 : 12,
      border: `1px solid ${isDark ? "#2a2f37" : "#e5e7eb"}`,
      boxShadow:
        "0 10px 30px rgba(0,0,0,0.08), 0 2px 6px rgba(0,0,0,0.04)",
      overflow: "hidden",
      transition:
        "width 0.25s ease, height 0.25s ease, background 0.2s ease, color 0.2s ease, border-color 0.2s ease",
      display: "flex",
      flexDirection: "column",
    };
  }, [theme, preset, device]);

  if (!ready) {
    return <p style={{ color: "#888" }}>Loading widgets…</p>;
  }

  return (
    <div style={{ display: "grid", gap: 20 }}>
      <Toolbar
        device={device}
        setDevice={setDevice}
        theme={theme}
        setTheme={setTheme}
        lang={lang}
        setLang={setLang}
      />

      <div
        style={{
          display: "flex",
          justifyContent: "center",
          padding: "24px 0",
          background: "#f3f4f6",
          borderRadius: 16,
        }}
      >
        <div style={frameStyle}>
          <FrameChrome device={device} theme={theme} lang={lang} page={page} />
          <PartnerSite
            device={device}
            theme={theme}
            lang={lang}
            accent={accent}
            page={page}
            setPage={setPage}
          />
        </div>
      </div>

      <p style={{ color: "#6b7280", fontSize: 12, margin: 0 }}>
        Switch pages inside the preview — Biyard widgets only appear where they
        belong: a claim CTA on the home page, a balance card on the account
        page, and richer activity on the activity page.
      </p>
    </div>
  );
}

function Toolbar({
  device,
  setDevice,
  theme,
  setTheme,
  lang,
  setLang,
}: {
  device: Device;
  setDevice: (d: Device) => void;
  theme: Theme;
  setTheme: (t: Theme) => void;
  lang: Lang;
  setLang: (l: Lang) => void;
}) {
  return (
    <div
      style={{
        display: "flex",
        flexWrap: "wrap",
        gap: 16,
        padding: 16,
        background: "#ffffff",
        border: "1px solid #e5e7eb",
        borderRadius: 12,
      }}
    >
      <Group label="Device">
        {(Object.keys(DEVICE_PRESETS) as Device[]).map((d) => (
          <ToggleBtn
            key={d}
            active={device === d}
            onClick={() => setDevice(d)}
          >
            <span style={{ marginRight: 6 }}>{DEVICE_PRESETS[d].emoji}</span>
            {DEVICE_PRESETS[d].label}
          </ToggleBtn>
        ))}
      </Group>

      <Group label="Theme">
        <ToggleBtn active={theme === "light"} onClick={() => setTheme("light")}>
          ☀️ Light
        </ToggleBtn>
        <ToggleBtn active={theme === "dark"} onClick={() => setTheme("dark")}>
          🌙 Dark
        </ToggleBtn>
      </Group>

      <Group label="Language">
        <ToggleBtn active={lang === "en"} onClick={() => setLang("en")}>
          EN
        </ToggleBtn>
        <ToggleBtn active={lang === "ko"} onClick={() => setLang("ko")}>
          KO
        </ToggleBtn>
      </Group>
    </div>
  );
}

function Group({ label, children }: { label: string; children: ReactNode }) {
  return (
    <div style={{ display: "flex", flexDirection: "column", gap: 6 }}>
      <span
        style={{
          fontSize: 11,
          fontWeight: 600,
          color: "#6b7280",
          textTransform: "uppercase",
          letterSpacing: 0.4,
        }}
      >
        {label}
      </span>
      <div style={{ display: "flex", gap: 6, flexWrap: "wrap" }}>
        {children}
      </div>
    </div>
  );
}

function ToggleBtn({
  active,
  onClick,
  children,
}: {
  active: boolean;
  onClick: () => void;
  children: ReactNode;
}) {
  return (
    <button
      onClick={onClick}
      style={{
        padding: "6px 12px",
        fontSize: 13,
        fontWeight: 500,
        borderRadius: 8,
        border: `1px solid ${active ? "#111827" : "#e5e7eb"}`,
        background: active ? "#111827" : "#ffffff",
        color: active ? "#ffffff" : "#374151",
        cursor: "pointer",
        transition: "all 0.15s ease",
      }}
    >
      {children}
    </button>
  );
}

function pagePathFor(page: PageId, lang: Lang): string {
  if (lang === "ko") {
    return page === "home"
      ? ""
      : page === "shop"
      ? "/스토어"
      : page === "account"
      ? "/마이페이지"
      : "/활동";
  }
  return page === "home" ? "" : `/${page}`;
}

function FrameChrome({
  device,
  theme,
  lang,
  page,
}: {
  device: Device;
  theme: Theme;
  lang: Lang;
  page: PageId;
}) {
  const isDark = theme === "dark";
  const bar = isDark ? "#14171c" : "#f9fafb";
  const muted = isDark ? "#9ca3af" : "#6b7280";

  if (device === "mobile") {
    return (
      <div
        style={{
          height: 32,
          background: bar,
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          padding: "0 20px",
          fontSize: 11,
          fontWeight: 600,
          color: isDark ? "#f3f4f6" : "#111827",
          flexShrink: 0,
        }}
      >
        <span>9:41</span>
        <div
          style={{
            width: 80,
            height: 6,
            borderRadius: 4,
            background: muted,
            opacity: 0.4,
          }}
        />
        <span style={{ fontSize: 10 }}>100%</span>
      </div>
    );
  }

  return (
    <div
      style={{
        height: 36,
        background: bar,
        display: "flex",
        alignItems: "center",
        gap: 6,
        padding: "0 14px",
        flexShrink: 0,
      }}
    >
      <Dot color="#ef4444" />
      <Dot color="#f59e0b" />
      <Dot color="#10b981" />
      <div
        style={{
          flex: 1,
          textAlign: "center",
          fontSize: 12,
          color: muted,
          fontFamily:
            "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace",
        }}
      >
        acme-shop.com{pagePathFor(page, lang)}
      </div>
    </div>
  );
}

function Dot({ color }: { color: string }) {
  return (
    <div
      style={{
        width: 10,
        height: 10,
        borderRadius: "50%",
        background: color,
      }}
    />
  );
}

function PartnerSite({
  device,
  theme,
  lang,
  accent,
  page,
  setPage,
}: {
  device: Device;
  theme: Theme;
  lang: Lang;
  accent: { value: string; fg: string };
  page: PageId;
  setPage: (p: PageId) => void;
}) {
  const isDark = theme === "dark";
  const isMobile = device === "mobile";

  const copy =
    lang === "ko"
      ? {
          brand: "ACME 스토어",
          nav: [
            { id: "home" as PageId, label: "홈" },
            { id: "shop" as PageId, label: "스토어" },
            { id: "account" as PageId, label: "마이페이지" },
            { id: "activity" as PageId, label: "활동" },
          ],
          footer: "ACME 스토어 · 멤버십 데모",
        }
      : {
          brand: "ACME Store",
          nav: [
            { id: "home" as PageId, label: "Home" },
            { id: "shop" as PageId, label: "Shop" },
            { id: "account" as PageId, label: "Account" },
            { id: "activity" as PageId, label: "Activity" },
          ],
          footer: "ACME Store · Membership demo",
        };

  const siteVars: CSSProperties = {
    ["--biyard-color-accent" as string]: accent.value,
    ["--biyard-color-accent-foreground" as string]: accent.fg,
    ["--biyard-radius" as string]: "14px",
    ["--site-accent" as string]: accent.value,
    ["--site-accent-fg" as string]: accent.fg,
    ["--site-bg" as string]: isDark ? "#0b0d10" : "#ffffff",
    ["--site-surface" as string]: isDark ? "#14171c" : "#f9fafb",
    ["--site-border" as string]: isDark ? "#2a2f37" : "#e5e7eb",
    ["--site-text" as string]: isDark ? "#f3f4f6" : "#111827",
    ["--site-muted" as string]: isDark ? "#9ca3af" : "#6b7280",
  };

  return (
    <div
      style={{
        ...siteVars,
        flex: 1,
        overflowY: "auto",
        background: "var(--site-bg)",
        color: "var(--site-text)",
        fontFamily:
          "ui-sans-serif, system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif",
      }}
    >
      <style>{`
        biyard-balance,
        biyard-transactions,
        biyard-monthly-summary {
          display: block;
          width: 100%;
        }
      `}</style>
      <PartnerNav
        brand={copy.brand}
        nav={copy.nav}
        active={page}
        onNav={setPage}
        isMobile={isMobile}
      />

      <div
        style={{
          padding: isMobile ? "20px 16px 32px" : "32px 40px 48px",
          maxWidth: 1100,
          margin: "0 auto",
        }}
      >
        {page === "home" && (
          <HomePage theme={theme} lang={lang} isMobile={isMobile} />
        )}
        {page === "shop" && <ShopPage lang={lang} isMobile={isMobile} />}
        {page === "account" && <AccountPage theme={theme} lang={lang} />}
        {page === "activity" && (
          <ActivityPage theme={theme} lang={lang} />
        )}

        <footer
          style={{
            fontSize: 11,
            color: "var(--site-muted)",
            textAlign: "center",
            paddingTop: 32,
          }}
        >
          {copy.footer}
        </footer>
      </div>
    </div>
  );
}

function PartnerNav({
  brand,
  nav,
  active,
  onNav,
  isMobile,
}: {
  brand: string;
  nav: { id: PageId; label: string }[];
  active: PageId;
  onNav: (p: PageId) => void;
  isMobile: boolean;
}) {
  return (
    <nav
      style={{
        display: "flex",
        alignItems: "center",
        justifyContent: "space-between",
        padding: isMobile ? "12px 16px" : "16px 40px",
        borderBottom: "1px solid var(--site-border)",
        background: "var(--site-bg)",
        position: "sticky",
        top: 0,
        zIndex: 1,
        gap: 12,
      }}
    >
      <div
        style={{
          display: "flex",
          alignItems: "center",
          gap: 10,
          flexShrink: 0,
        }}
      >
        <div
          style={{
            width: 28,
            height: 28,
            borderRadius: 8,
            background: "var(--site-accent)",
            color: "var(--site-accent-fg)",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            fontWeight: 700,
            fontSize: 13,
          }}
        >
          A
        </div>
        {!isMobile && (
          <span style={{ fontWeight: 600, fontSize: 15 }}>{brand}</span>
        )}
      </div>
      <div
        style={{
          display: "flex",
          gap: isMobile ? 4 : 8,
          overflowX: "auto",
          flex: 1,
          justifyContent: isMobile ? "flex-start" : "center",
        }}
      >
        {nav.map((n) => {
          const isActive = active === n.id;
          return (
            <button
              key={n.id}
              onClick={() => onNav(n.id)}
              style={{
                padding: isMobile ? "6px 10px" : "8px 14px",
                fontSize: 13,
                fontWeight: isActive ? 600 : 500,
                borderRadius: 8,
                border: "none",
                background: isActive
                  ? "var(--site-surface)"
                  : "transparent",
                color: isActive
                  ? "var(--site-accent)"
                  : "var(--site-muted)",
                cursor: "pointer",
                whiteSpace: "nowrap",
              }}
            >
              {n.label}
            </button>
          );
        })}
      </div>
      <div
        style={{
          width: 28,
          height: 28,
          borderRadius: "50%",
          background: "var(--site-surface)",
          border: "1px solid var(--site-border)",
          flexShrink: 0,
        }}
      />
    </nav>
  );
}

function HomePage({
  theme,
  lang,
  isMobile,
}: {
  theme: Theme;
  lang: Lang;
  isMobile: boolean;
}) {
  const copy =
    lang === "ko"
      ? {
          heroTitle: "오늘도 한 잔, 더 깊은 한 모금",
          heroSub: "스페셜티 원두와 콜드브루를 매달 집으로 보내드려요.",
          ctaShop: "스토어 둘러보기",
          banner: "이번 달 적립 포인트가 도착했어요",
          bannerSub: "1월 멤버십 리워드를 받아보세요.",
          picks: "이번 주 추천",
          pickItems: ["에티오피아 예가체프", "콜롬비아 게이샤", "콜드브루 1L"],
        }
      : {
          heroTitle: "A better cup, every morning",
          heroSub: "Specialty beans and cold brew, shipped monthly.",
          ctaShop: "Browse the shop",
          banner: "Your January points are ready",
          bannerSub: "Claim this month's membership rewards.",
          picks: "Picks of the week",
          pickItems: ["Ethiopia Yirgacheffe", "Colombia Geisha", "Cold brew 1L"],
        };

  return (
    <div style={{ display: "grid", gap: isMobile ? 24 : 36 }}>
      <section
        style={{
          padding: isMobile ? "24px 20px" : "48px 36px",
          borderRadius: 18,
          background:
            "linear-gradient(135deg, var(--site-accent), color-mix(in srgb, var(--site-accent) 60%, #000))",
          color: "var(--site-accent-fg)",
          display: "grid",
          gap: 16,
        }}
      >
        <h1
          style={{
            margin: 0,
            fontSize: isMobile ? 22 : 30,
            fontWeight: 700,
            lineHeight: 1.25,
          }}
        >
          {copy.heroTitle}
        </h1>
        <p style={{ margin: 0, fontSize: 14, opacity: 0.9 }}>{copy.heroSub}</p>
        <div>
          <button
            style={{
              padding: "10px 18px",
              borderRadius: 10,
              border: "none",
              background: "rgba(255,255,255,0.18)",
              color: "var(--site-accent-fg)",
              fontWeight: 600,
              fontSize: 13,
              cursor: "pointer",
              backdropFilter: "blur(4px)",
            }}
          >
            {copy.ctaShop}
          </button>
        </div>
      </section>

      <section
        style={{
          padding: 16,
          borderRadius: 14,
          background: "var(--site-surface)",
          border: "1px solid var(--site-border)",
          display: "flex",
          flexDirection: isMobile ? "column" : "row",
          alignItems: isMobile ? "stretch" : "center",
          gap: 16,
        }}
      >
        <div style={{ flex: 1 }}>
          <div style={{ fontSize: 14, fontWeight: 600 }}>{copy.banner}</div>
          <div
            style={{
              fontSize: 13,
              color: "var(--site-muted)",
              marginTop: 4,
            }}
          >
            {copy.bannerSub}
          </div>
        </div>
        <biyard-claim
          base-url="/api/biyard"
          month="2026-01"
          theme={theme}
          lang={lang}
          branding="minimal"
        />
      </section>

      <section>
        <h2 style={{ margin: "0 0 14px", fontSize: 16, fontWeight: 600 }}>
          {copy.picks}
        </h2>
        <div
          style={{
            display: "grid",
            gridTemplateColumns: isMobile
              ? "repeat(2, 1fr)"
              : "repeat(3, 1fr)",
            gap: isMobile ? 12 : 14,
          }}
        >
          {copy.pickItems.slice(0, isMobile ? 2 : 3).map((name, i) => (
            <ProductCard key={i} name={name} />
          ))}
        </div>
      </section>
    </div>
  );
}

function ShopPage({ lang, isMobile }: { lang: Lang; isMobile: boolean }) {
  const copy =
    lang === "ko"
      ? {
          title: "스토어",
          subtitle: "스페셜티 원두와 굿즈를 만나보세요.",
          items: [
            { name: "에티오피아 예가체프 250g", price: "₩18,000" },
            { name: "콜롬비아 게이샤 200g", price: "₩28,000" },
            { name: "콜드브루 1L", price: "₩12,000" },
            { name: "ACME 머그컵", price: "₩14,000" },
            { name: "드립 키트", price: "₩42,000" },
            { name: "원두 정기구독 (3개월)", price: "₩96,000" },
          ],
        }
      : {
          title: "Shop",
          subtitle: "Specialty beans and merch.",
          items: [
            { name: "Ethiopia Yirgacheffe 250g", price: "$18.00" },
            { name: "Colombia Geisha 200g", price: "$28.00" },
            { name: "Cold brew 1L", price: "$12.00" },
            { name: "ACME mug", price: "$14.00" },
            { name: "Pour-over kit", price: "$42.00" },
            { name: "Beans subscription (3mo)", price: "$96.00" },
          ],
        };

  return (
    <div style={{ display: "grid", gap: 20 }}>
      <header>
        <h1 style={{ margin: "0 0 4px", fontSize: 24, fontWeight: 700 }}>
          {copy.title}
        </h1>
        <p style={{ margin: 0, fontSize: 14, color: "var(--site-muted)" }}>
          {copy.subtitle}
        </p>
      </header>
      <div
        style={{
          display: "grid",
          gridTemplateColumns: isMobile ? "repeat(2, 1fr)" : "repeat(3, 1fr)",
          gap: 14,
        }}
      >
        {copy.items.map((it) => (
          <ProductCard key={it.name} name={it.name} price={it.price} />
        ))}
      </div>
    </div>
  );
}

function AccountPage({
  theme,
  lang,
}: {
  theme: Theme;
  lang: Lang;
}) {
  const copy =
    lang === "ko"
      ? {
          title: "마이페이지",
          subtitle: "주문, 멤버십, 계정 설정",
          welcome: "Jiwon 님",
          tier: "골드 멤버",
          rewardsLabel: "내 리워드",
          rewardsSub: "이번 달 적립된 포인트입니다.",
          ordersLabel: "최근 주문",
          orderItems: [
            { name: "에스프레소 원두 1kg", price: "₩32,000", status: "배송 완료" },
            { name: "콜드브루 기프트 세트", price: "₩48,000", status: "배송 중" },
            { name: "ACME 머그컵", price: "₩14,000", status: "준비 중" },
          ],
        }
      : {
          title: "Account",
          subtitle: "Orders, membership, settings",
          welcome: "Jiwon",
          tier: "Gold member",
          rewardsLabel: "Your rewards",
          rewardsSub: "Points earned this month.",
          ordersLabel: "Recent orders",
          orderItems: [
            { name: "Espresso beans 1kg", price: "$24.00", status: "Delivered" },
            { name: "Cold brew gift set", price: "$36.00", status: "Shipping" },
            { name: "ACME mug", price: "$14.00", status: "Preparing" },
          ],
        };

  return (
    <div style={{ display: "grid", gap: 24 }}>
      <header>
        <h1 style={{ margin: "0 0 4px", fontSize: 24, fontWeight: 700 }}>
          {copy.title}
        </h1>
        <p style={{ margin: 0, fontSize: 14, color: "var(--site-muted)" }}>
          {copy.subtitle}
        </p>
      </header>

      <div style={{ display: "flex", flexDirection: "column", gap: 20 }}>
        <ProfileCard name={copy.welcome} tier={copy.tier} />

        <SectionCard title={copy.rewardsLabel} subtitle={copy.rewardsSub}>
          <biyard-balance
            base-url="/api/biyard"
            month="2026-01"
            theme={theme}
            lang={lang}
            branding="minimal"
            title=""
          />
        </SectionCard>

        <SectionCard title={copy.ordersLabel}>
          <div style={{ display: "grid", gap: 10 }}>
            {copy.orderItems.map((o) => (
              <OrderRow
                key={o.name}
                name={o.name}
                price={o.price}
                status={o.status}
              />
            ))}
          </div>
        </SectionCard>
      </div>
    </div>
  );
}

function ActivityPage({ theme, lang }: { theme: Theme; lang: Lang }) {
  const copy =
    lang === "ko"
      ? {
          title: "활동",
          subtitle: "포인트 적립과 사용 내역을 한눈에 확인하세요.",
          summary: "월별 요약",
          transactions: "트랜잭션",
        }
      : {
          title: "Activity",
          subtitle: "Your points earned and spent at a glance.",
          summary: "Monthly summary",
          transactions: "Transactions",
        };

  return (
    <div style={{ display: "grid", gap: 24 }}>
      <header>
        <h1 style={{ margin: "0 0 4px", fontSize: 24, fontWeight: 700 }}>
          {copy.title}
        </h1>
        <p style={{ margin: 0, fontSize: 14, color: "var(--site-muted)" }}>
          {copy.subtitle}
        </p>
      </header>

      <SectionCard title={copy.summary}>
        <biyard-monthly-summary
          base-url="/api/biyard"
          theme={theme}
          lang={lang}
          branding="minimal"
          title=""
        />
      </SectionCard>

      <SectionCard title={copy.transactions}>
        <biyard-transactions
          base-url="/api/biyard"
          limit={6}
          theme={theme}
          lang={lang}
          branding="minimal"
          title=""
        />
      </SectionCard>
    </div>
  );
}

function ProductCard({ name, price }: { name: string; price?: string }) {
  return (
    <div
      style={{
        background: "var(--site-surface)",
        border: "1px solid var(--site-border)",
        borderRadius: 12,
        padding: 12,
        display: "grid",
        gap: 8,
      }}
    >
      <div
        style={{
          aspectRatio: "4 / 3",
          borderRadius: 8,
          background:
            "linear-gradient(135deg, color-mix(in srgb, var(--site-accent) 25%, transparent), color-mix(in srgb, var(--site-accent) 5%, transparent))",
        }}
      />
      <div
        style={{
          fontSize: 13,
          fontWeight: 500,
          lineHeight: 1.3,
          wordBreak: "keep-all",
        }}
      >
        {name}
      </div>
      {price && (
        <div style={{ fontSize: 12, color: "var(--site-muted)" }}>{price}</div>
      )}
    </div>
  );
}

function ProfileCard({ name, tier }: { name: string; tier: string }) {
  return (
    <div
      style={{
        display: "flex",
        alignItems: "center",
        gap: 14,
        padding: 16,
        borderRadius: 14,
        background: "var(--site-surface)",
        border: "1px solid var(--site-border)",
      }}
    >
      <div
        style={{
          width: 44,
          height: 44,
          borderRadius: "50%",
          background: "var(--site-accent)",
          color: "var(--site-accent-fg)",
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          fontWeight: 700,
          fontSize: 16,
        }}
      >
        {name.slice(0, 1)}
      </div>
      <div style={{ flex: 1, minWidth: 0 }}>
        <div style={{ fontSize: 14, fontWeight: 600 }}>{name}</div>
        <div style={{ fontSize: 12, color: "var(--site-muted)" }}>{tier}</div>
      </div>
    </div>
  );
}

function SectionCard({
  title,
  subtitle,
  children,
}: {
  title: string;
  subtitle?: string;
  children: ReactNode;
}) {
  return (
    <section
      style={{
        background: "var(--site-surface)",
        border: "1px solid var(--site-border)",
        borderRadius: 14,
        padding: 18,
        display: "grid",
        gap: 12,
      }}
    >
      <header>
        <h2
          style={{
            margin: 0,
            fontSize: 14,
            fontWeight: 600,
            color: "var(--site-text)",
          }}
        >
          {title}
        </h2>
        {subtitle && (
          <p
            style={{
              margin: "4px 0 0",
              fontSize: 12,
              color: "var(--site-muted)",
            }}
          >
            {subtitle}
          </p>
        )}
      </header>
      <div>{children}</div>
    </section>
  );
}

function OrderRow({
  name,
  price,
  status,
}: {
  name: string;
  price: string;
  status: string;
}) {
  return (
    <div
      style={{
        display: "flex",
        alignItems: "center",
        gap: 12,
        padding: "8px 0",
        borderBottom: "1px dashed var(--site-border)",
      }}
    >
      <div
        style={{
          width: 36,
          height: 36,
          borderRadius: 8,
          background: "var(--site-bg)",
          border: "1px solid var(--site-border)",
          flexShrink: 0,
        }}
      />
      <div style={{ flex: 1, minWidth: 0 }}>
        <div
          style={{
            fontSize: 13,
            fontWeight: 500,
            whiteSpace: "nowrap",
            overflow: "hidden",
            textOverflow: "ellipsis",
          }}
        >
          {name}
        </div>
        <div style={{ fontSize: 11, color: "var(--site-muted)" }}>{status}</div>
      </div>
      <div style={{ fontSize: 13, fontWeight: 600 }}>{price}</div>
    </div>
  );
}
