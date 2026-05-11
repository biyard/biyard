"use client";

import dynamic from "next/dynamic";
import Link from "next/link";

const ClientWidget = dynamic(() => import("./client-widget"), { ssr: false });

export default function WidgetPage() {
  return (
    <main
      style={{
        maxWidth: 760,
        margin: "48px auto",
        padding: "0 24px",
        color: "#111827",
      }}
    >
      <h1 style={{ marginBottom: 4 }}>&lt;biyard-claim&gt; widget</h1>
      <p style={{ color: "#6b7280", marginTop: 0 }}>
        Drop-in Web Component. Theming follows the host page; the only Biyard
        mark is an 11px footer link.
      </p>
      <p>
        <Link href="/">← Back</Link>
      </p>

      <Section
        title="Default (modal mode, auto theme)"
        code={`<biyard-claim
  base-url="/api/biyard"
  month="2026-01">
</biyard-claim>`}
      >
        <ClientWidget month="2026-01" />
      </Section>

      <Section
        title="Inline mode — feels like a native partner block"
        code={`<biyard-claim
  mode="inline"
  base-url="/api/biyard"
  month="2026-01"
  title="January rewards"
  subtitle="Claim your BIYARD points">
</biyard-claim>`}
      >
        <ClientWidget
          month="2026-01"
          mode="inline"
          title="January rewards"
          subtitle="Claim your BIYARD points"
        />
      </Section>

      <Section
        title="Themed with partner accent (indigo) + branding=none"
        code={`<biyard-claim
  mode="inline"
  branding="none"
  base-url="/api/biyard"
  month="2026-01"
  style="--biyard-color-accent:#6366f1;
         --biyard-color-accent-foreground:#fff;
         --biyard-radius:16px">
</biyard-claim>`}
      >
        <ClientWidget
          month="2026-01"
          mode="inline"
          branding="none"
          title="Member rewards"
          subtitle="Available this month"
          style={
            {
              ["--biyard-color-accent" as string]: "#6366f1",
              ["--biyard-color-accent-foreground" as string]: "#fff",
              ["--biyard-radius" as string]: "16px",
            } as React.CSSProperties
          }
        />
      </Section>

      <Section
        title="Dark theme + Korean"
        code={`<biyard-claim
  mode="inline"
  theme="dark"
  lang="ko"
  base-url="/api/biyard"
  month="2026-01">
</biyard-claim>`}
      >
        <ClientWidget
          month="2026-01"
          mode="inline"
          theme="dark"
          lang="ko"
          title="BIYARD 받기"
          subtitle="지갑이 표시하는 컨트랙트 주소를 확인하세요"
        />
      </Section>
    </main>
  );
}

function Section({
  title,
  code,
  children,
}: {
  title: string;
  code: string;
  children: React.ReactNode;
}) {
  return (
    <section style={{ marginTop: 32 }}>
      <h2 style={{ fontSize: 14, fontWeight: 600, color: "#374151" }}>
        {title}
      </h2>
      <div
        style={{
          display: "grid",
          gridTemplateColumns: "minmax(260px, 1fr) minmax(0, 1.2fr)",
          gap: 24,
          alignItems: "start",
          marginTop: 8,
        }}
      >
        <div>{children}</div>
        <pre
          style={{
            background: "#f9fafb",
            border: "1px solid #e5e7eb",
            padding: 14,
            borderRadius: 8,
            overflow: "auto",
            fontSize: 12,
            lineHeight: 1.5,
            color: "#374151",
          }}
        >
          {code}
        </pre>
      </div>
    </section>
  );
}
