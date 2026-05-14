"use client";

import dynamic from "next/dynamic";

const ClientWidgets = dynamic(() => import("./client-widget"), { ssr: false });

export default function WidgetPage() {
  return (
    <main
      style={{
        maxWidth: 1320,
        margin: "48px auto",
        padding: "0 24px",
        color: "#111827",
      }}
    >
      <header style={{ marginBottom: 24 }}>
        <h1 style={{ margin: "0 0 6px", fontSize: 28, fontWeight: 700 }}>
          Biyard SDK live preview
        </h1>
        <p
          style={{
            color: "#6b7280",
            margin: 0,
            maxWidth: 720,
            fontSize: 14,
            lineHeight: 1.5,
          }}
        >
          See how Biyard&apos;s drop-in widgets look inside a real partner site
          across devices, themes, and locales. Each widget runs against the
          same Biyard infrastructure your service would.
        </p>
        <nav
          style={{
            display: "flex",
            gap: 8,
            marginTop: 20,
            borderBottom: "1px solid #e5e7eb",
          }}
        >
          <a
            href="/widget"
            style={{
              padding: "10px 16px",
              fontSize: 14,
              fontWeight: 600,
              color: "#111827",
              textDecoration: "none",
              borderBottom: "2px solid #111827",
              marginBottom: -1,
            }}
          >
            Live preview
          </a>
          <a
            href="/components"
            style={{
              padding: "10px 16px",
              fontSize: 14,
              fontWeight: 500,
              color: "#6b7280",
              textDecoration: "none",
              borderBottom: "2px solid transparent",
              marginBottom: -1,
            }}
          >
            Components
          </a>
        </nav>
      </header>

      <ClientWidgets />
    </main>
  );
}
