"use client";

import { defineBiyardWidgets } from "@biyard/widget";
import {
  useEffect,
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

type Theme = "light" | "dark";
type Lang = "en" | "ko";
type Width = "narrow" | "medium" | "wide";

const WIDTH_PRESETS: Record<Width, { label: string; px: number }> = {
  narrow: { label: "Narrow (320px)", px: 320 },
  medium: { label: "Medium (480px)", px: 480 },
  wide: { label: "Wide (720px)", px: 720 },
};

export default function ClientComponents() {
  const [ready, setReady] = useState(false);
  const [theme, setTheme] = useState<Theme>("light");
  const [lang, setLang] = useState<Lang>("en");
  const [width, setWidth] = useState<Width>("medium");

  useEffect(() => {
    defineBiyardWidgets();
    setReady(true);
  }, []);

  if (!ready) {
    return <p style={{ color: "#888" }}>Loading widgets…</p>;
  }

  return (
    <div style={{ display: "grid", gap: 24 }}>
      <Toolbar
        theme={theme}
        setTheme={setTheme}
        lang={lang}
        setLang={setLang}
        width={width}
        setWidth={setWidth}
      />

      <ComponentCard
        title="<biyard-claim>"
        description="Modal trigger button. Opens an on-chain claim flow that signs with the user's wallet — your service never touches the private key."
        snippet={`<biyard-claim base-url="/api/biyard" month="2026-01" />`}
        width={width}
        theme={theme}
      >
        <biyard-claim
          base-url="/api/biyard"
          month="2026-01"
          theme={theme}
          lang={lang}
        />
      </ComponentCard>

      <ComponentCard
        title="<biyard-balance>"
        description="Off-chain point balance plus on-chain token balance side by side. Includes a Connect wallet button when no wallet is connected."
        snippet={`<biyard-balance base-url="/api/biyard" month="2026-01" />`}
        width={width}
        theme={theme}
      >
        <biyard-balance
          base-url="/api/biyard"
          month="2026-01"
          theme={theme}
          lang={lang}
        />
      </ComponentCard>

      <ComponentCard
        title="<biyard-monthly-summary>"
        description="Per-month earned / spent / claimed totals. Compact rows that adapt to container width."
        snippet={`<biyard-monthly-summary base-url="/api/biyard" />`}
        width={width}
        theme={theme}
      >
        <biyard-monthly-summary
          base-url="/api/biyard"
          theme={theme}
          lang={lang}
        />
      </ComponentCard>

      <ComponentCard
        title="<biyard-transactions>"
        description="Paginated point activity. Earned / spent / claimed entries with on-demand load more."
        snippet={`<biyard-transactions base-url="/api/biyard" limit="6" />`}
        width={width}
        theme={theme}
      >
        <biyard-transactions
          base-url="/api/biyard"
          limit={6}
          theme={theme}
          lang={lang}
        />
      </ComponentCard>
    </div>
  );
}

function Toolbar({
  theme,
  setTheme,
  lang,
  setLang,
  width,
  setWidth,
}: {
  theme: Theme;
  setTheme: (t: Theme) => void;
  lang: Lang;
  setLang: (l: Lang) => void;
  width: Width;
  setWidth: (w: Width) => void;
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

      <Group label="Container width">
        {(Object.keys(WIDTH_PRESETS) as Width[]).map((w) => (
          <ToggleBtn
            key={w}
            active={width === w}
            onClick={() => setWidth(w)}
          >
            {WIDTH_PRESETS[w].label}
          </ToggleBtn>
        ))}
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

function ComponentCard({
  title,
  description,
  snippet,
  width,
  theme,
  children,
}: {
  title: string;
  description: string;
  snippet: string;
  width: Width;
  theme: Theme;
  children: ReactNode;
}) {
  const [copied, setCopied] = useState(false);
  const isDark = theme === "dark";

  const onCopy = () => {
    navigator.clipboard.writeText(snippet);
    setCopied(true);
    setTimeout(() => setCopied(false), 1500);
  };

  const stageStyle: CSSProperties = {
    width: "100%",
    minHeight: 120,
    display: "flex",
    justifyContent: "center",
    alignItems: "flex-start",
    padding: 32,
    borderRadius: 12,
    background: isDark
      ? "linear-gradient(135deg, #14171c, #0b0d10)"
      : "linear-gradient(135deg, #f9fafb, #f3f4f6)",
    border: `1px solid ${isDark ? "#2a2f37" : "#e5e7eb"}`,
  };

  const widthBoxStyle: CSSProperties = {
    width: WIDTH_PRESETS[width].px,
    maxWidth: "100%",
    transition: "width 0.25s ease",
    textAlign: "center",
  };

  return (
    <article
      style={{
        background: "#ffffff",
        border: "1px solid #e5e7eb",
        borderRadius: 14,
        overflow: "hidden",
      }}
    >
      <header
        style={{
          padding: "18px 22px 14px",
          borderBottom: "1px solid #f3f4f6",
        }}
      >
        <h2
          style={{
            margin: "0 0 4px",
            fontSize: 15,
            fontWeight: 600,
            fontFamily:
              "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace",
            color: "#111827",
          }}
        >
          {title}
        </h2>
        <p
          style={{
            margin: 0,
            fontSize: 13,
            color: "#6b7280",
            lineHeight: 1.5,
          }}
        >
          {description}
        </p>
      </header>

      <div style={{ padding: 22 }}>
        <div style={stageStyle}>
          <div style={widthBoxStyle}>{children}</div>
        </div>
      </div>

      <footer
        style={{
          display: "flex",
          alignItems: "center",
          gap: 12,
          padding: "12px 18px",
          background: "#0b0d10",
          borderTop: "1px solid #e5e7eb",
        }}
      >
        <code
          style={{
            flex: 1,
            fontSize: 12.5,
            color: "#d1d5db",
            fontFamily:
              "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace",
            overflow: "auto",
            whiteSpace: "nowrap",
          }}
        >
          {snippet}
        </code>
        <button
          onClick={onCopy}
          style={{
            padding: "6px 12px",
            fontSize: 12,
            fontWeight: 500,
            borderRadius: 6,
            border: "1px solid #2a2f37",
            background: copied ? "#10d99c" : "#14171c",
            color: copied ? "#0b0d10" : "#d1d5db",
            cursor: "pointer",
            transition: "all 0.15s ease",
            whiteSpace: "nowrap",
            flexShrink: 0,
          }}
        >
          {copied ? "✓ Copied" : "Copy"}
        </button>
      </footer>
    </article>
  );
}
