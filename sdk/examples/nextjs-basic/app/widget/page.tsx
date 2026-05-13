"use client";

import dynamic from "next/dynamic";
import Link from "next/link";

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
      <h1 style={{ marginBottom: 4 }}>Biyard Web Components</h1>
      <p style={{ color: "#6b7280", marginTop: 0, maxWidth: 720 }}>
        Drop-in widgets you compose into your own page. Each one fetches its
        own data via the partner proxy, themes off CSS variables on the host,
        and shows a single tiny attribution line at the bottom.
      </p>
      <p>
        <Link href="/">← Back</Link>
      </p>

      <ClientWidgets />
    </main>
  );
}
